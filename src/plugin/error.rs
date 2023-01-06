extern crate thiserror;
use thiserror::Error;

#[derive(Error, Debug)]
#[repr(C)]
pub enum PluginErrorCodes {
    // #[error("3000")] : "3000" is when the plugin is not loaded
    #[error("3001")]
    FailedToLoad,

    // #[error("3002")] : "3002" for invalid parameters
    #[error("3002")]
    ParametersError,

    // #[error("3003")] : "3003" for invalid plugin
    #[error("3003")]
    InvalidPlugin,

    // #[error("3004")] : "3004" for no such file
    #[error("3004")]
    NoSuchFile,

    // #[error("3005")] : "3005" for permission denied
    #[error("3005")]
    PermissionDenied,

    // #[error("3006")] : "3006" for missing symbol
    #[error("3006")]
    MissingSymbol,

    // #[error("3007")] : "3007" for failed to initialize
    #[error("3007")]
    FailedToInitialize,

    // #[error("3008")] : "3008" for internal error
    #[error("3008 [ {err:?} ]")]
    InternalError { err: String },
}
