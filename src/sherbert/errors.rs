//! Error types

/// Result of a CLI function
pub type CliResult = Result<(), CliError>;

#[derive(Debug)]
/// The CLI error is the error type used at Sherbert's CLI-layer.
///
/// All errors from the lib side of Sherbert will get wrapped with this error.
/// Other errors (such as command-line argument validation) will create this
/// directly.
pub struct CliError {
    /// The error to display. This can be `None` in rare cases to exit with a
    /// code without displaying a message.
    pub error: Option<anyhow::Error>,
    /// The process exit code.
    pub exit_code: i32,
}

impl CliError {
    /// Create new error
    #[must_use]
    pub fn new(error: anyhow::Error, code: i32) -> CliError {
        CliError {
            error: Some(error),
            exit_code: code,
        }
    }

    /// Create new error from code
    #[must_use]
    pub fn code(code: i32) -> CliError {
        CliError {
            error: None,
            exit_code: code,
        }
    }
}

impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> CliError {
        CliError::new(err, 101)
    }
}

impl From<clap::Error> for CliError {
    fn from(err: clap::Error) -> CliError {
        let code = i32::from(err.use_stderr());
        CliError::new(err.into(), code)
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> CliError {
        CliError::new(err.into(), 1)
    }
}
