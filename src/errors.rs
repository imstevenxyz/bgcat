use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use log::error;
use std::fmt;

use crate::web::responses::ErrorMessageResponse;

#[repr(u8)]
pub enum BGCErrorType {
    GEN = 0,
    WEB = 1,
    API = 2,
}

#[derive(Debug)]
pub enum BGCError<const T: u8> {
    InternalError(String),
    UserError(String),
    Timeout(String),
    Unauthorized(String),
    NotFound(String),
}

impl<const T: u8> fmt::Display for BGCError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            BGCError::InternalError(msg) => msg,
            BGCError::UserError(msg) => msg,
            BGCError::Timeout(msg) => msg,
            BGCError::Unauthorized(msg) => msg,
            BGCError::NotFound(msg) => msg,
        };
        write!(f, "{}", msg)
    }
}

impl<const T: u8> BGCError<T> {
    fn log(&self) {
        match self {
            BGCError::InternalError(msg) => error!("{}", msg),
            _ => {}
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            BGCError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BGCError::UserError(_) => StatusCode::BAD_REQUEST,
            BGCError::Timeout(_) => StatusCode::REQUEST_TIMEOUT,
            BGCError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            BGCError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

/*
    Conversion between BGCErrorType
*/

impl From<BGCError<{ BGCErrorType::GEN as u8 }>> for BGCError<{ BGCErrorType::WEB as u8 }> {
    fn from(error: BGCError<{ BGCErrorType::GEN as u8 }>) -> Self {
        from_generic(error)
    }
}

impl From<BGCError<{ BGCErrorType::GEN as u8 }>> for BGCError<{ BGCErrorType::API as u8 }> {
    fn from(error: BGCError<{ BGCErrorType::GEN as u8 }>) -> Self {
        from_generic(error)
    }
}

fn from_generic<const T: u8, const T2: u8>(error: BGCError<T>) -> BGCError<T2> {
    match error {
        BGCError::InternalError(msg) => BGCError::<T2>::InternalError(msg),
        BGCError::UserError(msg) => BGCError::<T2>::UserError(msg),
        BGCError::Timeout(msg) => BGCError::<T2>::Timeout(msg),
        BGCError::Unauthorized(msg) => BGCError::<T2>::Unauthorized(msg),
        BGCError::NotFound(msg) => BGCError::<T2>::NotFound(msg),
    }
}

/*
    actix_web ResponseError implementations
*/

impl actix_web::error::ResponseError for BGCError<{ BGCErrorType::WEB as u8 }> {
    fn error_response(&self) -> HttpResponse {
        self.log();
        match self {
            BGCError::Unauthorized(_) => HttpResponse::Found()
                .append_header(("Location", "/admin"))
                .finish(),
            _ => {
                let tera = tera::Tera::new("templates/**/*").unwrap();
                let mut tera_ctx = tera::Context::new();
                tera_ctx.insert("css_ver", "v0.1.0");
                tera_ctx.insert("status_code", &self.status_code().as_u16());
                tera_ctx.insert(
                    "status_msg",
                    &self.status_code().canonical_reason().unwrap_or("Unknown"),
                );
                let render = tera.render("error.html", &tera_ctx).unwrap();
                HttpResponse::build(self.status_code()).body(render)
            }
        }
    }
}

impl actix_web::error::ResponseError for BGCError<{ BGCErrorType::API as u8 }> {
    fn error_response(&self) -> HttpResponse {
        self.log();
        match self {
            BGCError::UserError(_) => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .json(ErrorMessageResponse {
                    message: self.to_string(),
                }),
            _ => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .json(ErrorMessageResponse {
                    message: self
                        .status_code()
                        .canonical_reason()
                        .unwrap_or("Unknown")
                        .to_string(),
                }),
        }
    }
}

/*
    Conversion from other errors
*/

impl<const T: u8> From<&str> for BGCError<T> {
    fn from(err: &str) -> BGCError<T> {
        BGCError::InternalError(err.to_string())
    }
}

impl<const T: u8> From<std::io::Error> for BGCError<T> {
    fn from(err: std::io::Error) -> BGCError<T> {
        BGCError::InternalError(err.to_string())
    }
}

impl<const T: u8> From<image::ImageError> for BGCError<T> {
    fn from(err: image::ImageError) -> BGCError<T> {
        BGCError::InternalError(err.to_string())
    }
}

impl<const T: u8> From<actix_session::SessionGetError> for BGCError<T> {
    fn from(err: actix_session::SessionGetError) -> BGCError<T> {
        BGCError::InternalError(err.to_string())
    }
}

impl<const T: u8> From<actix_web::error::ParseError> for BGCError<T> {
    fn from(err: actix_web::error::ParseError) -> BGCError<T> {
        BGCError::InternalError(err.to_string())
    }
}

impl<const T: u8> From<surrealdb::Error> for BGCError<T> {
    fn from(err: surrealdb::Error) -> BGCError<T> {
        match err {
            surrealdb::Error::Db(sub_err) => extract_db_error(sub_err),
            surrealdb::Error::Api(sub_err) => {
                BGCError::InternalError(format!("DB API: {}", sub_err.to_string()))
            }
        }
    }
}

fn extract_db_error<const T: u8>(err: surrealdb::error::Db) -> BGCError<T> {
    match err {
        surrealdb::error::Db::QueryTimedout => BGCError::Timeout(err.to_string()),
        surrealdb::error::Db::RecordExists { thing: _ } => BGCError::UserError(err.to_string()),
        _ => BGCError::InternalError(err.to_string()),
    }
}
