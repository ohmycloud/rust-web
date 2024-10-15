use std::fmt::Formatter;
use warp::body::BodyDeserializeError;
use warp::cors::CorsForbidden;
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::{Rejection, Reply};
use tracing::{event, Level, instrument};
use argon2::Error as ArgonError;
use reqwest::Error as ReqwestError;
use reqwest_middleware::Error as MiddlewareReqwestError;

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    WrongPassword,
    ArgonLibraryError(ArgonError),
    QuestionNotFound,
    DatabaseQueryError(sqlx::Error),
    ExternalAPIError(ReqwestError),
    // In case the HTTP client(Reqwest) returns an error
    // We create a ClientError enum variant.
    ClientError(APILayerError),
    // In case the external API returns a 4xx or 5xx HTTP status code,
    // we have a ServerError variant.
    ServerError(APILayerError),
    ReqwestAPIError(ReqwestError),
    MiddlewareReqwestAPIError(MiddlewareReqwestError),
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::WrongPassword => write!(f, "Wrong password"),
            Error::ArgonLibraryError(_) => write!(f, "Cannot verifiy password"),
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::DatabaseQueryError(_) => {
                write!(f, "Query could not be executed")
            },
            Error::ExternalAPIError(err) => {
                write!(f, "Cannot execute: {}", err)
            },
            Error::ClientError(err) => {
                write!(f, "External Client error: {}", err)
            },
            Error::ServerError(err) => {
                write!(f, "External Server error: {}", err)
            },
            Error::ReqwestAPIError(err) => {
                write!(f, "External API error: {}", err)
            },
            Error::MiddlewareReqwestAPIError(err) => {
                write!(f, "External API error: {}", err)
            }
        }
    }
}

impl Reject for Error {}
impl Reject for APILayerError {}

const DUPLICATE_KEY: u32 = 23505;

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(crate::Error::DatabaseQueryError(e)) = r.find() {
        event!(Level::ERROR, "Database query error");

        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap() ==
                    DUPLICATE_KEY {
                    Ok(warp::reply::with_status(
                        "Account already exists".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        "Cannot update data".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                }
            },
            _ => {
                Ok(warp::reply::with_status(
                    "Cannot update data".to_string(),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ))
            }
        }
    } else if let Some(crate::Error::ExternalAPIError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<Error>() {
        event!(Level::ERROR, "{}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(crate::Error::ReqwestAPIError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::WrongPassword) = r.find() {
        event!(Level::ERROR, "Entered wrong password");
        Ok(warp::reply::with_status(
            "Wrong E-Mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(crate::Error::MiddlewareReqwestAPIError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
