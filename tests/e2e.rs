#![cfg(feature = "e2e")]

use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};

fn spawn_server() -> Child {
    Command::new(env!("CARGO_BIN_EXE_artifacthub-mcp"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn artifacthub-mcp binary")
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
    send_request(stdin, 1, "initialize", serde_json::json!({
        "protocolVersion": "2025-11-25",
        "capabilities": {},
        "clientInfo": { "name": "e2e-test", "version": "0.0.0" }
    }));
    let _resp = read_response(stdout);

    send_notification(stdin, "notifications/initialized", serde_json::json!({}));
}

fn call_tool(stdin: &mut impl Write, stdout: &mut BufReader<impl std::io::Read>, id: u64, name: &str, args: serde_json::Value) -> serde_json::Value {
    send_request(stdin, id, "tools/call", serde_json::json!({
        "name": name,
        "arguments": args
    }));
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

    assert!(resp.get("result").is_some(), "should have result: {:?}, stderr: {}", resp, drain_stderr(&mut stderr));
    let tools = resp["result"]["tools"].as_array().unwrap();
    assert!(!tools.is_empty());
    let tool_names: Vec<_> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"search_packages"));
    assert!(tool_names.contains(&"get_package"));
}

#[test]
fn e2e_stdio_search_packages() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(&mut stdin, &mut stdout, 3, "search_packages", serde_json::json!({
        "q": "nginx",
        "kind": "helm",
        "limit": 5
    }));

    assert!(resp["result"]["isError"].as_bool() != Some(true), "should not be error: {:?}", resp);
    let packages = resp["result"]["structuredContent"]["packages"].as_array().unwrap();
    assert!(!packages.is_empty());
}

#[test]
fn e2e_stdio_get_package() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(&mut stdin, &mut stdout, 4, "get_package", serde_json::json!({
        "kind": "helm",
        "repo": "bitnami",
        "name": "nginx"
    }));

    assert!(resp["result"]["isError"].as_bool() != Some(true), "should not be error: {:?}", resp);
    let pkg = &resp["result"]["structuredContent"];
    assert_eq!(pkg["name"].as_str().unwrap(), "nginx");
    assert!(!pkg["version"].as_str().unwrap().is_empty());
    assert_eq!(pkg["repository"]["name"].as_str().unwrap(), "bitnami");
}

#[test]
fn e2e_stdio_get_package_versions() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(&mut stdin, &mut stdout, 5, "get_package_versions", serde_json::json!({
        "kind": "helm",
        "repo": "bitnami",
        "name": "nginx",
        "limit": 3
    }));

    assert!(resp["result"]["isError"].as_bool() != Some(true), "should not be error: {:?}", resp);
    let result = &resp["result"]["structuredContent"];
    assert!(result["versions"].as_array().unwrap().len() <= 3);
    assert!(result["count"].as_u64().unwrap() >= 3);
}

#[test]
fn e2e_stdio_get_package_readme() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(&mut stdin, &mut stdout, 6, "get_package_readme", serde_json::json!({
        "kind": "helm",
        "repo": "bitnami",
        "name": "nginx"
    }));

    assert!(resp["result"]["isError"].as_bool() != Some(true), "should not be error: {:?}", resp);
    let readme = resp["result"]["structuredContent"]["readme"].as_str().unwrap();
    assert!(!readme.is_empty());
}

#[test]
fn e2e_stdio_search_repositories() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(&mut stdin, &mut stdout, 7, "search_repositories", serde_json::json!({
        "name": "bitnami",
        "kind": "helm",
        "limit": 5
    }));

    assert!(resp["result"]["isError"].as_bool() != Some(true), "should not be error: {:?}", resp);
    let repos = resp["result"]["structuredContent"]["repositories"].as_array().unwrap();
    assert!(!repos.is_empty());
}

#[test]
fn e2e_stdio_get_server_info() {
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = BufReader::new(server.stdout.take().unwrap());

    initialize(&mut stdin, &mut stdout);

    let resp = call_tool(&mut stdin, &mut stdout, 8, "get_server_info", serde_json::json!({}));

    assert!(resp["result"]["isError"].as_bool() != Some(true), "should not be error: {:?}", resp);
    let info = &resp["result"]["structuredContent"];
    assert!(info["name"].as_str().is_some());
    assert!(info["version"].as_str().is_some());
}
