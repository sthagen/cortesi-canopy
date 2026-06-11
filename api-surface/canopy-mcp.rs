// Ruskel skeleton - syntactically valid Rust with implementation omitted.
// settings: target=crates/canopy-mcp, visibility=public, auto_impls=false, blanket_impls=false

pub mod canopy_mcp {
    //! MCP and smoke-test helpers for canopy applications.

    pub mod error {
        //! Error types shared across the automation helpers.

        /// Result type used by `canopy-mcp`.
        pub type Result<T> = std::result::Result<T, Error>;

        /// Errors returned by `canopy-mcp`.
        #[derive(Debug, Error, Display)]
        pub enum Error {
            /// A canopy runtime error.
            Canopy(canopy::error::Error),
            /// A canopy command conversion error.
            Command(canopy::commands::CommandError),
            /// An I/O error.
            Io(io::Error),
            /// A JSON encoding or decoding error.
            Json(serde_json::Error),
            /// An MCP transport or protocol error.
            Tmcp(tmcp::Error),
            /// The application factory failed to build an app instance.
            App(String),
            /// A smoke suite did not resolve to any Luau scripts.
            NoScripts(std::path::PathBuf),
        }

        impl Error {
            /// Wrap an application-specific setup error.
            pub fn app(error: impl Display) -> Self {}
        }

        impl From<Error> for Error {
            fn from(source: CanopyError) -> Self {}
        }

        impl From<CommandError> for Error {
            fn from(source: CommandError) -> Self {}
        }

        impl From<Error> for Error {
            fn from(source: io::Error) -> Self {}
        }

        impl From<Error> for Error {
            fn from(source: serde_json::Error) -> Self {}
        }

        impl From<Error> for Error {
            fn from(source: tmcp::Error) -> Self {}
        }
    }

    pub mod script {
        //! Headless script-evaluation types and helpers.

        /// Shared application factory used by the automation helpers.
        pub type AppFactory =
            std::sync::Arc<dyn Fn() -> crate::Result<canopy::Canopy> + Send + Sync>;

        /// Convert a closure into a shared app factory.
        pub fn app_factory<F>(factory: F) -> AppFactory
        where
            F: Fn() -> crate::Result<canopy::Canopy> + Send + Sync + 'static, {
        }

        /// Request payload for the `script_eval` tool.
        #[derive(Deserialize, Debug, Clone, StructuralPartialEq, PartialEq)]
        pub struct ScriptEvalRequest {
            /// Luau source code to execute.
            pub script: String,
            /// Optional named fixture applied before evaluation.
            pub fixture: Option<String>,
            /// Optional evaluation timeout in milliseconds.
            pub timeout_ms: Option<u64>,
        }

        impl JsonSchema for ScriptEvalRequest {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

            fn inline_schema() -> bool {}
        }

        /// Structured typecheck diagnostic returned by `script_eval`.
        #[derive(
            Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Eq, Serialize, Deserialize,
        )]
        pub struct ScriptDiagnostic {
            /// Diagnostic severity such as `error` or `warning`.
            pub severity: String,
            /// One-based line number, or zero when the diagnostic is not source-bound.
            pub line: usize,
            /// One-based column number, or zero when the diagnostic is not source-bound.
            pub column: usize,
            /// Human-readable diagnostic message.
            pub message: String,
        }

        impl JsonSchema for ScriptDiagnostic {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

            fn inline_schema() -> bool {}
        }

        /// Assertion outcome recorded during script execution.
        #[derive(
            Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Eq, Serialize, Deserialize,
        )]
        pub struct ScriptAssertion {
            /// Whether the assertion passed.
            pub passed: bool,
            /// Assertion message emitted by the runtime.
            pub message: String,
        }

        impl JsonSchema for ScriptAssertion {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

            fn inline_schema() -> bool {}
        }

        /// Timing information for a script evaluation.
        #[derive(
            Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Eq, Serialize, Deserialize,
        )]
        pub struct ScriptTiming {
            /// Time spent constructing and rendering the headless app.
            pub build_ms: u64,
            /// Time spent executing the script and final render.
            pub exec_ms: u64,
            /// Total wall-clock time for the request.
            pub total_ms: u64,
        }

        impl ScriptTiming {
            /// Zeroed timing information for early errors.
            pub fn zero() -> Self {}
        }

        impl JsonSchema for ScriptTiming {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

            fn inline_schema() -> bool {}
        }

        /// Evaluation task state exposed to automation callers.
        #[derive(
            Serialize,
            Debug,
            Clone,
            Copy,
            StructuralPartialEq,
            PartialEq,
            Eq,
            Serialize,
            Deserialize,
        )]
        pub enum ScriptTaskState {
            /// Evaluation completed successfully.
            Completed,
            /// Evaluation failed before completion.
            Failed,
            /// Evaluation stopped at the cooperative timeout boundary.
            TimedOut,
        }

        impl JsonSchema for ScriptTaskState {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

            fn inline_schema() -> bool {}
        }

        /// Error details included in a failed script evaluation.
        #[derive(
            Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Serialize, Deserialize,
        )]
        pub struct ScriptErrorInfo {
            /// Error category such as `build`, `typecheck`, `timeout`, or `runtime`.
            pub error_type: String,
            /// Human-readable error message.
            pub message: String,
        }

        impl JsonSchema for ScriptErrorInfo {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

            fn inline_schema() -> bool {}
        }

        /// Structured response for the `script_eval` tool and smoke runner.
        #[derive(
            Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Serialize, Deserialize,
        )]
        pub struct ScriptEvalOutcome {
            /// Whether the script completed successfully.
            pub success: bool,
            /// Final task state for the evaluation.
            pub state: ScriptTaskState,
            /// Optional JSON-serializable script return value.
            pub value: Option<serde_json::Value>,
            /// Log lines emitted during evaluation.
            pub logs: Vec<String>,
            /// Assertion outcomes recorded during evaluation.
            pub assertions: Vec<ScriptAssertion>,
            /// Typecheck diagnostics captured before execution.
            pub diagnostics: Vec<ScriptDiagnostic>,
            /// Timing information for the request.
            pub timing: ScriptTiming,
            /// Error payload when evaluation fails.
            pub error: Option<ScriptErrorInfo>,
        }

        impl ScriptEvalOutcome {
            /// Encode the outcome as an MCP tool result.
            pub fn to_tool_result(&self) -> CallToolResult {}

            /// Build a failure payload with no result value.
            pub fn error_only(
                error_type: impl Into<String>,
                message: impl Into<String>,
                diagnostics: Vec<ScriptDiagnostic>,
                timing: ScriptTiming,
            ) -> Self {
            }
        }

        impl JsonSchema for ScriptEvalOutcome {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

            fn inline_schema() -> bool {}
        }

        impl From<ScriptEvalOutcome> for tmcp::schema::CallToolResult {
            fn from(outcome: ScriptEvalOutcome) -> Self {}
        }

        /// Headless evaluator that creates a fresh canopy app instance for each request.
        #[derive(Clone)]
        pub struct AppEvaluator {}

        impl AppEvaluator {
            /// Construct an evaluator with a default headless viewport size.
            pub fn new(factory: AppFactory) -> Self {}

            /// Override the headless viewport size used for evaluations.
            pub fn with_view_size(self, width: u32, height: u32) -> Self {}

            /// Render and return the app's Luau API definition.
            pub fn script_api(&self) -> Result<String> {}

            /// Return the evaluator's registered fixture catalog.
            pub fn fixtures(&self) -> Result<Vec<FixtureInfo>> {}

            /// Evaluate a Luau script, enforcing the requested timeout when present.
            pub fn evaluate_with_timeout(&self, request: &ScriptEvalRequest) -> ScriptEvalOutcome {}

            /// Evaluate a Luau script against a fresh headless app.
            pub fn evaluate(&self, request: &ScriptEvalRequest) -> ScriptEvalOutcome {}
        }

        /// Evaluate a Luau script against an existing live canopy app.
        pub fn evaluate_live(
            canopy: &mut canopy::Canopy,
            request: &ScriptEvalRequest,
        ) -> ScriptEvalOutcome {
        }
    }

    pub mod server {
        //! Stdio MCP server wrapper for script automation.

        /// Request payload for applying a named fixture to a live app.
        #[derive(
            Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Eq, Deserialize, Serialize,
        )]
        pub struct ApplyFixtureRequest {
            /// Registered fixture name.
            pub name: String,
        }

        impl JsonSchema for ApplyFixtureRequest {
            fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

            fn inline_schema() -> bool {}
        }

        /// Serve `script_eval` and `script_api` over stdio for an app factory.
        pub fn serve_stdio(
            factory: impl Fn() -> crate::Result<canopy::Canopy> + Send + Sync + 'static,
        ) -> crate::Result<()> {
        }

        /// Handle for a running live UDS MCP listener.
        pub struct UdsServerHandle {}

        impl UdsServerHandle {
            /// Stop the listener and remove the socket path.
            pub fn stop(self) -> Result<()> {}
        }

        impl Drop for UdsServerHandle {
            fn drop(&mut self) {}
        }

        /// Serve live MCP automation for a running canopy app over a Unix-domain socket.
        pub fn serve_uds(
            socket_path: impl AsRef<std::path::Path>,
            automation: canopy::AutomationHandle,
        ) -> crate::Result<UdsServerHandle> {
        }
    }

    pub mod smoke {
        //! Smoke-suite discovery and execution helpers.

        /// Configuration for a smoke-suite run.
        #[derive(Debug, Clone, StructuralPartialEq, PartialEq)]
        pub struct SuiteConfig {
            /// Root directory to scan for `.luau` scripts when no explicit script list is provided.
            pub suite_dir: std::path::PathBuf,
            /// Optional subset of scripts to run. Relative paths are resolved against `suite_dir`.
            pub scripts: Vec<std::path::PathBuf>,
            /// Optional timeout per script in milliseconds.
            pub timeout_ms: Option<u64>,
            /// Stop after the first failing script when true.
            pub fail_fast: bool,
        }

        impl SuiteConfig {
            /// Construct a config using a suite directory and default options.
            pub fn new(suite_dir: impl Into<PathBuf>) -> Self {}
        }

        /// Final status for a smoke script.
        #[derive(
            Serialize,
            Debug,
            Clone,
            Copy,
            StructuralPartialEq,
            PartialEq,
            Eq,
            Serialize,
            Deserialize,
        )]
        pub enum ScriptStatus {
            /// The script passed.
            Passed,
            /// The script failed.
            Failed,
        }

        /// Result of running one smoke script.
        #[derive(
            Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Serialize, Deserialize,
        )]
        pub struct ScriptResult {
            /// Script path on disk.
            pub path: std::path::PathBuf,
            /// Fixture derived for this script, if any.
            pub fixture: Option<String>,
            /// Pass or fail status.
            pub status: ScriptStatus,
            /// Total script duration in milliseconds.
            pub elapsed_ms: u64,
            /// Optional summary message.
            pub message: Option<String>,
            /// Structured script outcome.
            pub outcome: crate::script::ScriptEvalOutcome,
        }

        /// Aggregated result for a smoke suite.
        #[derive(
            Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Serialize, Deserialize,
        )]
        pub struct SuiteResult {
            /// Per-script results in execution order.
            pub scripts: Vec<ScriptResult>,
        }

        impl SuiteResult {
            /// Return true when all smoke scripts passed.
            pub fn success(&self) -> bool {}
        }

        /// Run a smoke suite against fresh headless app instances.
        pub fn run_suite(
            factory: impl Fn() -> crate::Result<canopy::Canopy> + Send + Sync + 'static,
            config: &SuiteConfig,
        ) -> crate::Result<SuiteResult> {
        }
    }

    /// Errors returned by `canopy-mcp`.
    #[derive(Debug, Error, Display)]
    pub enum Error {
        /// A canopy runtime error.
        Canopy(canopy::error::Error),
        /// A canopy command conversion error.
        Command(canopy::commands::CommandError),
        /// An I/O error.
        Io(io::Error),
        /// A JSON encoding or decoding error.
        Json(serde_json::Error),
        /// An MCP transport or protocol error.
        Tmcp(tmcp::Error),
        /// The application factory failed to build an app instance.
        App(String),
        /// A smoke suite did not resolve to any Luau scripts.
        NoScripts(std::path::PathBuf),
    }

    impl Error {
        /// Wrap an application-specific setup error.
        pub fn app(error: impl Display) -> Self {}
    }

    impl From<Error> for Error {
        fn from(source: CanopyError) -> Self {}
    }

    impl From<CommandError> for Error {
        fn from(source: CommandError) -> Self {}
    }

    impl From<Error> for Error {
        fn from(source: io::Error) -> Self {}
    }

    impl From<Error> for Error {
        fn from(source: serde_json::Error) -> Self {}
    }

    impl From<Error> for Error {
        fn from(source: tmcp::Error) -> Self {}
    }

    /// Result type used by `canopy-mcp`.
    pub type Result<T> = std::result::Result<T, Error>;

    /// Headless evaluator that creates a fresh canopy app instance for each request.
    #[derive(Clone)]
    pub struct AppEvaluator {}

    impl AppEvaluator {
        /// Construct an evaluator with a default headless viewport size.
        pub fn new(factory: AppFactory) -> Self {}

        /// Override the headless viewport size used for evaluations.
        pub fn with_view_size(self, width: u32, height: u32) -> Self {}

        /// Render and return the app's Luau API definition.
        pub fn script_api(&self) -> Result<String> {}

        /// Return the evaluator's registered fixture catalog.
        pub fn fixtures(&self) -> Result<Vec<FixtureInfo>> {}

        /// Evaluate a Luau script, enforcing the requested timeout when present.
        pub fn evaluate_with_timeout(&self, request: &ScriptEvalRequest) -> ScriptEvalOutcome {}

        /// Evaluate a Luau script against a fresh headless app.
        pub fn evaluate(&self, request: &ScriptEvalRequest) -> ScriptEvalOutcome {}
    }

    /// Assertion outcome recorded during script execution.
    #[derive(
        Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Eq, Serialize, Deserialize,
    )]
    pub struct ScriptAssertion {
        /// Whether the assertion passed.
        pub passed: bool,
        /// Assertion message emitted by the runtime.
        pub message: String,
    }

    impl JsonSchema for ScriptAssertion {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

        fn inline_schema() -> bool {}
    }

    /// Structured typecheck diagnostic returned by `script_eval`.
    #[derive(
        Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Eq, Serialize, Deserialize,
    )]
    pub struct ScriptDiagnostic {
        /// Diagnostic severity such as `error` or `warning`.
        pub severity: String,
        /// One-based line number, or zero when the diagnostic is not source-bound.
        pub line: usize,
        /// One-based column number, or zero when the diagnostic is not source-bound.
        pub column: usize,
        /// Human-readable diagnostic message.
        pub message: String,
    }

    impl JsonSchema for ScriptDiagnostic {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

        fn inline_schema() -> bool {}
    }

    /// Error details included in a failed script evaluation.
    #[derive(Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Serialize, Deserialize)]
    pub struct ScriptErrorInfo {
        /// Error category such as `build`, `typecheck`, `timeout`, or `runtime`.
        pub error_type: String,
        /// Human-readable error message.
        pub message: String,
    }

    impl JsonSchema for ScriptErrorInfo {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

        fn inline_schema() -> bool {}
    }

    /// Structured response for the `script_eval` tool and smoke runner.
    #[derive(Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Serialize, Deserialize)]
    pub struct ScriptEvalOutcome {
        /// Whether the script completed successfully.
        pub success: bool,
        /// Final task state for the evaluation.
        pub state: ScriptTaskState,
        /// Optional JSON-serializable script return value.
        pub value: Option<serde_json::Value>,
        /// Log lines emitted during evaluation.
        pub logs: Vec<String>,
        /// Assertion outcomes recorded during evaluation.
        pub assertions: Vec<ScriptAssertion>,
        /// Typecheck diagnostics captured before execution.
        pub diagnostics: Vec<ScriptDiagnostic>,
        /// Timing information for the request.
        pub timing: ScriptTiming,
        /// Error payload when evaluation fails.
        pub error: Option<ScriptErrorInfo>,
    }

    impl ScriptEvalOutcome {
        /// Encode the outcome as an MCP tool result.
        pub fn to_tool_result(&self) -> CallToolResult {}

        /// Build a failure payload with no result value.
        pub fn error_only(
            error_type: impl Into<String>,
            message: impl Into<String>,
            diagnostics: Vec<ScriptDiagnostic>,
            timing: ScriptTiming,
        ) -> Self {
        }
    }

    impl JsonSchema for ScriptEvalOutcome {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

        fn inline_schema() -> bool {}
    }

    impl From<ScriptEvalOutcome> for tmcp::schema::CallToolResult {
        fn from(outcome: ScriptEvalOutcome) -> Self {}
    }

    /// Request payload for the `script_eval` tool.
    #[derive(Deserialize, Debug, Clone, StructuralPartialEq, PartialEq)]
    pub struct ScriptEvalRequest {
        /// Luau source code to execute.
        pub script: String,
        /// Optional named fixture applied before evaluation.
        pub fixture: Option<String>,
        /// Optional evaluation timeout in milliseconds.
        pub timeout_ms: Option<u64>,
    }

    impl JsonSchema for ScriptEvalRequest {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

        fn inline_schema() -> bool {}
    }

    /// Evaluation task state exposed to automation callers.
    #[derive(
        Serialize, Debug, Clone, Copy, StructuralPartialEq, PartialEq, Eq, Serialize, Deserialize,
    )]
    pub enum ScriptTaskState {
        /// Evaluation completed successfully.
        Completed,
        /// Evaluation failed before completion.
        Failed,
        /// Evaluation stopped at the cooperative timeout boundary.
        TimedOut,
    }

    impl JsonSchema for ScriptTaskState {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

        fn inline_schema() -> bool {}
    }

    /// Timing information for a script evaluation.
    #[derive(
        Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Eq, Serialize, Deserialize,
    )]
    pub struct ScriptTiming {
        /// Time spent constructing and rendering the headless app.
        pub build_ms: u64,
        /// Time spent executing the script and final render.
        pub exec_ms: u64,
        /// Total wall-clock time for the request.
        pub total_ms: u64,
    }

    impl ScriptTiming {
        /// Zeroed timing information for early errors.
        pub fn zero() -> Self {}
    }

    impl JsonSchema for ScriptTiming {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

        fn inline_schema() -> bool {}
    }

    /// Convert a closure into a shared app factory.
    pub fn app_factory<F>(factory: F) -> AppFactory
    where
        F: Fn() -> crate::Result<canopy::Canopy> + Send + Sync + 'static, {
    }

    /// Evaluate a Luau script against an existing live canopy app.
    pub fn evaluate_live(
        canopy: &mut canopy::Canopy,
        request: &ScriptEvalRequest,
    ) -> ScriptEvalOutcome {
    }

    /// Request payload for applying a named fixture to a live app.
    #[derive(
        Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Eq, Deserialize, Serialize,
    )]
    pub struct ApplyFixtureRequest {
        /// Registered fixture name.
        pub name: String,
    }

    impl JsonSchema for ApplyFixtureRequest {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

        fn inline_schema() -> bool {}
    }

    /// Handle for a running live UDS MCP listener.
    pub struct UdsServerHandle {}

    impl UdsServerHandle {
        /// Stop the listener and remove the socket path.
        pub fn stop(self) -> Result<()> {}
    }

    impl Drop for UdsServerHandle {
        fn drop(&mut self) {}
    }

    /// Serve `script_eval` and `script_api` over stdio for an app factory.
    pub fn serve_stdio(
        factory: impl Fn() -> crate::Result<canopy::Canopy> + Send + Sync + 'static,
    ) -> crate::Result<()> {
    }

    /// Serve live MCP automation for a running canopy app over a Unix-domain socket.
    pub fn serve_uds(
        socket_path: impl AsRef<std::path::Path>,
        automation: canopy::AutomationHandle,
    ) -> crate::Result<UdsServerHandle> {
    }

    /// Result of running one smoke script.
    #[derive(Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Serialize, Deserialize)]
    pub struct ScriptResult {
        /// Script path on disk.
        pub path: std::path::PathBuf,
        /// Fixture derived for this script, if any.
        pub fixture: Option<String>,
        /// Pass or fail status.
        pub status: ScriptStatus,
        /// Total script duration in milliseconds.
        pub elapsed_ms: u64,
        /// Optional summary message.
        pub message: Option<String>,
        /// Structured script outcome.
        pub outcome: crate::script::ScriptEvalOutcome,
    }

    /// Final status for a smoke script.
    #[derive(
        Serialize, Debug, Clone, Copy, StructuralPartialEq, PartialEq, Eq, Serialize, Deserialize,
    )]
    pub enum ScriptStatus {
        /// The script passed.
        Passed,
        /// The script failed.
        Failed,
    }

    /// Configuration for a smoke-suite run.
    #[derive(Debug, Clone, StructuralPartialEq, PartialEq)]
    pub struct SuiteConfig {
        /// Root directory to scan for `.luau` scripts when no explicit script list is provided.
        pub suite_dir: std::path::PathBuf,
        /// Optional subset of scripts to run. Relative paths are resolved against `suite_dir`.
        pub scripts: Vec<std::path::PathBuf>,
        /// Optional timeout per script in milliseconds.
        pub timeout_ms: Option<u64>,
        /// Stop after the first failing script when true.
        pub fail_fast: bool,
    }

    impl SuiteConfig {
        /// Construct a config using a suite directory and default options.
        pub fn new(suite_dir: impl Into<PathBuf>) -> Self {}
    }

    /// Aggregated result for a smoke suite.
    #[derive(Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Serialize, Deserialize)]
    pub struct SuiteResult {
        /// Per-script results in execution order.
        pub scripts: Vec<ScriptResult>,
    }

    impl SuiteResult {
        /// Return true when all smoke scripts passed.
        pub fn success(&self) -> bool {}
    }

    /// Run a smoke suite against fresh headless app instances.
    pub fn run_suite(
        factory: impl Fn() -> crate::Result<canopy::Canopy> + Send + Sync + 'static,
        config: &SuiteConfig,
    ) -> crate::Result<SuiteResult> {
    }
}

