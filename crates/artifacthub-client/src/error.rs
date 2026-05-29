/// Error returned by Artifact Hub client operations.
#[derive(Debug, thiserror::Error)]
pub enum ArtifactHubError {
    /// The HTTP request could not be sent.
    #[error("Request failed: {0}")]
    Request(#[source] reqwest::Error),
    /// The response body could not be read.
    #[error("Failed to read response: {0}")]
    Body(#[source] reqwest::Error),
    /// Artifact Hub returned a non-success HTTP status.
    #[error("API error {status}: {message}")]
    Api {
        status: reqwest::StatusCode,
        message: String,
    },
    /// A JSON response could not be decoded into the expected type.
    #[error("{context}: {source}")]
    Json {
        context: &'static str,
        #[source]
        source: serde_json::Error,
    },
    /// A required response field was missing or had an unexpected type.
    #[error("No {field} found for {context}")]
    MissingField {
        field: &'static str,
        context: &'static str,
    },
}

/// Result type used by the Artifact Hub client.
pub type Result<T> = std::result::Result<T, ArtifactHubError>;

impl ArtifactHubError {
    pub(crate) fn json(context: &'static str, source: serde_json::Error) -> Self {
        Self::Json { context, source }
    }

    pub(crate) fn missing_field(field: &'static str, context: &'static str) -> Self {
        Self::MissingField { field, context }
    }
}

impl From<ArtifactHubError> for String {
    fn from(error: ArtifactHubError) -> Self {
        error.to_string()
    }
}
