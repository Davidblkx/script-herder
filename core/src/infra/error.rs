use std::{fmt, io};

#[derive(Debug)]
pub enum ErrorSource {
    IO(std::io::ErrorKind),
    GIT(git2::ErrorClass, git2::ErrorCode),
    Other,
    App,
}

impl fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorSource::IO(kind) => write!(f, "IO|{:?}", kind),
            ErrorSource::Other => write!(f, "Unknown"),
            ErrorSource::App => write!(f, "App"),
            ErrorSource::GIT(cls, code) => write!(f, "GIT|{:?}:{:?}", cls, code)
        }
    }
}

#[derive(Debug)]
pub struct CoreError {
    pub source: ErrorSource,
    pub message: String,
    pub context: Option<String>,
}

impl CoreError {
    pub fn for_app(message: String) -> CoreError {
        CoreError {
            source: ErrorSource::App,
            message,
            context: None,
        }
    }

    pub fn for_err(err: Box<dyn std::error::Error>) -> CoreError {
        CoreError {
            source: ErrorSource::Other,
            message: err.to_string(),
            context: None,
        }
    }

    pub fn context(&mut self, context: String) -> &mut Self {
        self.context = Some(context);
        self
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let context_str = match &self.context {
            Some(context) => format!("[{}]", context),
            None => "".to_string(),
        };
        write!(f, "{}{}: {}", context_str, self.source, self.message)
    }
}

impl From<io::Error> for CoreError {
    fn from(error: io::Error) -> Self {
        CoreError {
            source: ErrorSource::IO(error.kind()),
            message: error.to_string(),
            context: None,
        }
    }
}

impl From<git2::Error> for CoreError {
    fn from(error: git2::Error) -> Self {
        CoreError {
            source: ErrorSource::GIT(error.class(), error.code()),
            message: error.message().to_string(),
            context: None
        }
    }
}