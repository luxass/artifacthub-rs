use std::io::{BufRead, BufReader, Read, Write};
use std::ops::{Deref, DerefMut};
use std::process::{Child, Command, Stdio};

struct TestServer {
    child: Child,
}

impl Deref for TestServer {
    type Target = Child;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl DerefMut for TestServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn spawn_server() -> TestServer {
    spawn_server_with_args(&[])
}

fn spawn_server_with_args(args: &[&str]) -> TestServer {
    TestServer {
        child: Command::new(env!("CARGO_BIN_EXE_artifacthub-mcp"))
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn artifacthub-mcp binary"),
    }
}

fn send_request(stdin: &mut impl Write, id: u64, method: &str, params: serde_json::Value) {
    let msg = serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params
    });
    let bytes = format!("{}\n", serde_json::to_string(&msg).unwrap());
    stdin.write_all(bytes.as_bytes()).unwrap();
    stdin.flush().unwrap();
}

fn send_notification(stdin: &mut impl Write, method: &str, params: serde_json::Value) {
    let msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params
    });
    let bytes = format!("{}\n", serde_json::to_string(&msg).unwrap());
    stdin.write_all(bytes.as_bytes()).unwrap();
    stdin.flush().unwrap();
}

fn read_response(stdout: &mut BufReader<impl std::io::Read>) -> serde_json::Value {
    let mut line = String::new();
    stdout.read_line(&mut line).unwrap();
    serde_json::from_str(&line).unwrap()
}

fn drain_stderr(stderr: &mut BufReader<impl std::io::Read>) -> String {
    let mut out = String::new();
    stderr.read_to_string(&mut out).ok();
    out
}

fn initialize(stdin: &mut impl Write, stdout: &mut BufReader<impl std::io::Read>) {
    initialize_with_version(stdin, stdout, "2025-11-25", "2025-11-25");
}

fn initialize_with_version(
    stdin: &mut impl Write,
    stdout: &mut BufReader<impl std::io::Read>,
    protocol_version: &str,
    expected_version: &str,
) {
    send_request(
        stdin,
        1,
        "initialize",
        serde_json::json!({
            "protocolVersion": protocol_version,
            "capabilities": {},
            "clientInfo": { "name": "e2e-test", "version": "0.0.0" }
        }),
    );
    let resp = read_response(stdout);
    assert_eq!(resp["result"]["protocolVersion"], expected_version);
    assert!(
        resp["result"]["capabilities"]["tools"].is_object(),
        "initialize must advertise tools so capability-aware clients discover them: {resp}"
    );
    assert_eq!(resp["result"]["serverInfo"]["name"], "artifacthub-mcp");
    assert_eq!(
        resp["result"]["serverInfo"]["version"],
        env!("CARGO_PKG_VERSION")
    );

    send_notification(stdin, "notifications/initialized", serde_json::json!({}));
}

fn call_tool(
    stdin: &mut impl Write,
    stdout: &mut BufReader<impl std::io::Read>,
    id: u64,
    name: &str,
    args: serde_json::Value,
) -> serde_json::Value {
    send_request(
        stdin,
        id,
        "tools/call",
        serde_json::json!({
            "name": name,
            "arguments": args
        }),
    );
    read_response(stdout)
}

#[test]
fn e2e_stdio_tools_list() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());
    let mut stderr = BufReader::new(server.stderr.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    send_request(&mut stdin, 2, "tools/list", serde_json::json!({}));
    let resp = read_response(&mut stdout);

    assert!(
        resp.get("result").is_some(),
        "should have result: {:?}, stderr: {}",
        resp,
        drain_stderr(&mut stderr)
    );
    let tools = resp["result"]["tools"].as_array().unwrap();
    assert!(!tools.is_empty());
    let tool_names: Vec<_> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"search_packages"));
    assert!(tool_names.contains(&"get_package"));
}

#[test]
fn e2e_stdio_protocol_negotiation_matrix() {
    for (requested, expected) in [
        ("2024-11-05", "2024-11-05"),
        ("2025-03-26", "2025-03-26"),
        ("2025-06-18", "2025-06-18"),
        ("2025-11-25", "2025-11-25"),
        ("2026-07-28", "2025-11-25"),
        ("2099-01-01", "2025-11-25"),
    ] {
        let mut server = spawn_server();
        let mut stdin = server.stdin.take().unwrap();
        let mut stdout = BufReader::new(server.stdout.take().unwrap());
        initialize_with_version(&mut stdin, &mut stdout, requested, expected);
        send_request(&mut stdin, 2, "tools/list", serde_json::json!({}));
        let resp = read_response(&mut stdout);
        assert!(resp["result"].get("ttlMs").is_none());
        assert!(resp["result"].get("cacheScope").is_none());
        assert!(resp["result"].get("resultType").is_none());
        let tools = resp["result"]["tools"].as_array().unwrap();
        assert_eq!(
            tools.len(),
            15,
            "tool discovery failed for {requested}: {resp}"
        );
        for tool in tools {
            assert!(tool["name"].as_str().is_some());
            assert_eq!(tool["inputSchema"]["type"], "object");
        }
        let resp = call_tool(
            &mut stdin,
            &mut stdout,
            3,
            "get_server_info",
            serde_json::json!({}),
        );
        assert_eq!(resp["result"]["isError"], false);
        assert_eq!(
            resp["result"]["structuredContent"]["name"],
            "artifacthub-mcp"
        );
    }
}

// Newer protocols use server/discover with per-request metadata, not initialize.
#[test]
fn e2e_stdio_modern_discovery_cache_and_filters() {
    for (args, expected_count) in [
        (vec![], 15),
        (vec!["--tools", "get_server_info"], 1),
        (vec!["--exclude-tools", "search_packages"], 14),
    ] {
        let mut server = spawn_server_with_args(&args);
        let mut stdin = server.stdin.take().unwrap();
        let mut stdout = BufReader::new(server.stdout.take().unwrap());
        let meta = serde_json::json!({
            "io.modelcontextprotocol/protocolVersion": "2026-07-28",
            "io.modelcontextprotocol/clientCapabilities": {},
            "io.modelcontextprotocol/clientInfo": {"name": "modern-test", "version": "1"}
        });
        send_request(
            &mut stdin,
            1,
            "server/discover",
            serde_json::json!({"_meta": meta}),
        );
        let resp = read_response(&mut stdout);
        assert!(
            resp["result"]["capabilities"]["tools"].is_object(),
            "{resp}"
        );
        assert!(
            resp["result"]["supportedVersions"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!("2026-07-28"))
        );

        for id in [2, 3] {
            send_request(
                &mut stdin,
                id,
                "tools/list",
                serde_json::json!({"_meta": meta}),
            );
            let resp = read_response(&mut stdout);
            assert_eq!(resp["result"]["resultType"], "complete", "{resp}");
            assert_eq!(resp["result"]["ttlMs"], 0);
            assert_eq!(resp["result"]["cacheScope"], "private");
            let tools = resp["result"]["tools"].as_array().unwrap();
            assert_eq!(tools.len(), expected_count);
            assert!(tools.iter().any(|tool| tool["name"] == "get_server_info"));
            if !args.is_empty() {
                assert!(!tools.iter().any(|tool| tool["name"] == "search_packages"));
            }
        }
        send_request(
            &mut stdin,
            4,
            "tools/call",
            serde_json::json!({
                "_meta": meta, "name": "get_server_info", "arguments": {}
            }),
        );
        let resp = read_response(&mut stdout);
        assert_eq!(resp["result"]["resultType"], "complete", "{resp}");
        assert_eq!(resp["result"]["isError"], false);
        assert!(resp["result"].get("ttlMs").is_none());
        assert!(resp["result"].get("cacheScope").is_none());
        if !args.is_empty() {
            send_request(
                &mut stdin,
                5,
                "tools/call",
                serde_json::json!({
                    "_meta": meta, "name": "search_packages", "arguments": {"q": "nginx"}
                }),
            );
            let resp = read_response(&mut stdout);
            assert!(
                resp.get("error").is_some(),
                "disabled tool must be rejected: {resp}"
            );
        }
    }
}

#[test]
fn e2e_stdio_newer_protocol_falls_back() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize_with_version(&mut stdin, &mut stdout, "2026-07-28", "2025-11-25");
    send_request(&mut stdin, 2, "tools/list", serde_json::json!({}));
    let resp = read_response(&mut stdout);
    assert!(!resp["result"]["tools"].as_array().unwrap().is_empty());

    let resp = call_tool(
        &mut stdin,
        &mut stdout,
        3,
        "get_server_info",
        serde_json::json!({}),
    );
    assert_eq!(resp["result"]["isError"], false);
}

#[cfg(feature = "e2e")]
#[test]
fn e2e_stdio_search_packages() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(
        &mut stdin,
        &mut stdout,
        3,
        "search_packages",
        serde_json::json!({
            "q": "nginx",
            "kind": ["helm"],
            "limit": 5
        }),
    );

    assert!(
        resp["result"]["isError"].as_bool() != Some(true),
        "should not be error: {:?}",
        resp
    );
    let packages = resp["result"]["structuredContent"]["packages"]
        .as_array()
        .unwrap();
    assert!(!packages.is_empty());
}

#[cfg(feature = "e2e")]
#[test]
fn e2e_stdio_get_package() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(
        &mut stdin,
        &mut stdout,
        4,
        "get_package",
        serde_json::json!({
            "kind": "helm",
            "repo": "bitnami",
            "name": "nginx"
        }),
    );

    assert!(
        resp["result"]["isError"].as_bool() != Some(true),
        "should not be error: {:?}",
        resp
    );
    let pkg = &resp["result"]["structuredContent"];
    assert_eq!(pkg["name"].as_str().unwrap(), "nginx");
    assert!(!pkg["version"].as_str().unwrap().is_empty());
    assert_eq!(pkg["repository"]["name"].as_str().unwrap(), "bitnami");
}

#[cfg(feature = "e2e")]
#[test]
fn e2e_stdio_get_package_versions() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(
        &mut stdin,
        &mut stdout,
        5,
        "get_package_versions",
        serde_json::json!({
            "kind": "helm",
            "repo": "bitnami",
            "name": "nginx",
            "limit": 3
        }),
    );

    assert!(
        resp["result"]["isError"].as_bool() != Some(true),
        "should not be error: {:?}",
        resp
    );
    let result = &resp["result"]["structuredContent"];
    assert!(result["versions"].as_array().unwrap().len() <= 3);
    assert!(result["count"].as_u64().unwrap() >= 3);
}

#[cfg(feature = "e2e")]
#[test]
fn e2e_stdio_get_package_readme() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(
        &mut stdin,
        &mut stdout,
        6,
        "get_package_readme",
        serde_json::json!({
            "kind": "helm",
            "repo": "bitnami",
            "name": "nginx"
        }),
    );

    assert!(
        resp["result"]["isError"].as_bool() != Some(true),
        "should not be error: {:?}",
        resp
    );
    let readme = resp["result"]["structuredContent"]["readme"]
        .as_str()
        .unwrap();
    assert!(!readme.is_empty());
}

#[cfg(feature = "e2e")]
#[test]
fn e2e_stdio_search_repositories() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(
        &mut stdin,
        &mut stdout,
        7,
        "search_repositories",
        serde_json::json!({
            "name": "bitnami",
            "kind": ["helm"],
            "limit": 5
        }),
    );

    assert!(
        resp["result"]["isError"].as_bool() != Some(true),
        "should not be error: {:?}",
        resp
    );
    let repos = resp["result"]["structuredContent"]["repositories"]
        .as_array()
        .unwrap();
    assert!(!repos.is_empty());
}

#[test]
fn e2e_stdio_get_server_info() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(
        &mut stdin,
        &mut stdout,
        8,
        "get_server_info",
        serde_json::json!({}),
    );

    assert!(
        resp["result"]["isError"].as_bool() != Some(true),
        "should not be error: {:?}",
        resp
    );
    let info = &resp["result"]["structuredContent"];
    assert!(info["name"].as_str().is_some());
    assert!(info["version"].as_str().is_some());
}
