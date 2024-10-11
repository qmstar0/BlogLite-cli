use std::fmt::{Debug, Display};

use clap::CommandFactory;

use crate::{api, Cli};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ResponseError { code: String, message: String },

    // request error
    HTTPError(reqwest::Error),

    // open file
    IO(std::io::Error),

    UnAuth,

    ServiceError,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::HTTPError(value)
    }
}

impl<T> From<api::response::Response<T>> for Error {
    fn from(value: api::response::Response<T>) -> Self {
        Error::ResponseError {
            code: value.code(),
            message: value.message(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HTTPError(e) => write!(f, "HTTP request error:{e}"),
            Error::IO(e) => write!(f, "io error: {e}"),
            Error::ResponseError { code: _, message } => write!(f, "{}", message),
            Error::UnAuth => {
                writeln!(f, "You must login first.")?;
                if let Some(login) = Cli::command().find_subcommand("login") {
                    let _ = login.to_owned().print_help();
                }

                Ok(())
            }
            Error::ServiceError => write!(
                f,
                "The api response is OK, but the response header lacks necessary data."
            ),
        }
    }
}
