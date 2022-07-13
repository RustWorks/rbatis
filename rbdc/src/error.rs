use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    E(String),
    Io(std::io::Error),
}

impl Error {
    #[allow(dead_code)]
    #[inline]
    pub fn protocol(err: impl Display) -> Self {
        Error::E(err.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::E(e) => std::fmt::Display::fmt(&e, f),
            Error::Io(e) => f.write_str(&e.to_string()),
        }
    }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::E(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(arg: std::io::Error) -> Self {
        Error::E(arg.to_string())
    }
}

#[cfg(all(feature = "_tls-native-tls"))]
impl From<native_tls::Error> for Error {
    fn from(e: native_tls::Error) -> Self {
        Error::E(e.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::E(e.to_string())
    }
}
// Format an error message as a `Protocol` error
#[macro_export]
macro_rules! err_protocol {
    ($expr:expr) => {
        $crate::Error::E($expr.into())
    };

    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::E(format!($fmt, $($arg)*))
    };
}
