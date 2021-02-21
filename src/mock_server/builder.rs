use crate::mock_server::bare_server::{BareMockServer, RequestRecording};
use crate::mock_server::exposed_server::InnerServer;
use crate::MockServer;
use std::net::TcpListener;

/// A builder providing a fluent API to assemble a [`MockServer`] step-by-step.  
/// Use [`MockServer::builder`] to get started.
pub struct MockServerBuilder {
    listener: Option<TcpListener>,
    record_incoming_requests: bool,
}

impl MockServerBuilder {
    pub(super) fn new() -> Self {
        Self {
            listener: None,
            record_incoming_requests: true,
        }
    }

    /// Each instance of [`MockServer`] is, by default, running on a random
    /// port available on your local machine.
    /// With `MockServerBuilder::listener` you can choose to start the `MockServer`
    /// instance on a specific port you have already bound.
    ///
    /// ### Example:
    /// ```rust
    /// use wiremock::MockServer;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     // Arrange
    ///     let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    ///     let expected_server_address = listener
    ///         .local_addr()
    ///         .expect("Failed to get server address.");
    ///     let mock_server = MockServer::start_on(listener).await;
    ///
    ///     assert_eq!(&expected_server_address, mock_server.address());
    /// }
    /// ```
    pub fn listener(mut self, listener: TcpListener) -> Self {
        self.listener = Some(listener);
        self
    }

    /// By default, [`MockServer`] will record all incoming requests to display
    /// more meaningful error messages when your expectations are not verified.
    ///
    /// This can sometimes be undesirable (e.g. a long-lived server serving
    /// high volumes of traffic) - you can disable request recording using
    /// `MockServerBuilder::disable_request_recording`.
    pub fn disable_request_recording(mut self) -> Self {
        self.record_incoming_requests = false;
        self
    }

    /// Finalise the builder to get an instance of a [`BareMockServer`].
    pub(super) async fn build_bare(self) -> BareMockServer {
        let listener = if let Some(listener) = self.listener {
            listener
        } else {
            TcpListener::bind("127.0.0.1:0").expect("Failed to bind an OS port for a mock server.")
        };
        let recording = if self.record_incoming_requests {
            RequestRecording::Enabled
        } else {
            RequestRecording::Disabled
        };
        BareMockServer::start(listener, recording).await
    }

    /// Finalise the builder to get an instance of a [`MockServer`]!
    pub async fn build(self) -> MockServer {
        MockServer::new(InnerServer::Bare(self.build_bare().await))
    }
}