#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    description: Option<String>,
    help: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    PermissionDenied,
    FanControlDisabled,
    InvalidValue,
    FileNotFound,
    ValueTooHigh,
    ValueTooLow,
    GenericError,
}

/// Quickly construct an err::Error.
///
/// Note: the kind arguments only require you to supply the enum variant. Assume
/// that err::ErrorKind::* is already included.
///
/// Function signatures include:
///  * `kind: err::ErrorKind`
///  * `kind: err::ErrorKind, description: impl ToString`
///  * `kind: err::ErrorKind, help: impl ToString, description: impl ToString`
///  * `kind: err::ErrorKind, help: impl ToString, format: &str, format args`
///  * `kind: err::ErrorKind, format: &str, format args`
///
///
#[macro_export]
macro_rules! err {
    ($kind:expr) => {
        $crate::error::Error::without_description_help($kind)
    };
    ($kind:ident,$description:expr) => {
        $crate::error::Error::without_help($crate::error::ErrorKind::$kind, $description.to_string())
    };
    ($kind:ident,$help: expr, $description: expr) => {
        $crate::error::Error::new($crate::error::ErrorKind::$kind, $description.to_string(), $help.to_string())
    };
    ($kind:ident,$help:expr, $format:expr, $($fmt_arg:tt)*) => {
        $crate::error::Error::new($crate::error::ErrorKind::$kind, format!($format, $($fmt_arg)*), $help.to_string())
    };
    ($kind:ident,$format:expr,$($fmt_arg:tt)*) => {
        $crate::error::Error::without_help($crate::error::ErrorKind::$kind, format!($format, $($fmt_arg)*))
    };
}

#[macro_export]
/// Quickly construct a Generic Error.
macro_rules! generic_err {
    ($err:expr) => {
        $crate::err!(GenericError, "Some error occurred: {}", $err)
    };
}

pub use crate::{err, generic_err};

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind as E;
        let s = match self {
            E::ValueTooLow => "Fan speed setting too low",
            E::ValueTooHigh => "Fan speed setting too high",
            E::InvalidValue => "Fan speed setting invalid",
            E::FileNotFound => "File not found",
            E::PermissionDenied => "Permission Denied",
            E::FanControlDisabled => "Fan control disabled",
            E::GenericError => "Generic error",
        };

        write!(f, "{s}")
    }
}

impl Error {
    pub fn new(kind: ErrorKind, description: String, help: String) -> Self {
        Self {
            kind,
            description: Some(description),
            help: Some(help),
        }
    }

    pub fn without_description_help(kind: ErrorKind) -> Self {
        Self {
            kind,
            description: None,
            help: None,
        }
    }

    pub fn without_description(kind: ErrorKind, help: String) -> Self {
        Self {
            kind,
            description: None,
            help: Some(help),
        }
    }

    pub fn without_help(kind: ErrorKind, description: String) -> Self {
        Self {
            kind,
            description: Some(description),
            help: None,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn desc(&self) -> Option<&str> {
        if let Some(ref desc) = self.description {
            Some(desc.as_str())
        } else {
            None
        }
    }

    pub fn help(&self) -> Option<&str> {
        if let Some(ref help) = self.help {
            Some(help.as_str())
        } else {
            None
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind())?;
        if let Some(ref desc) = self.description {
            write!(f, ": {}", desc)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match &self.description {
            Some(s) => s.as_ref(),
            None => "",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
