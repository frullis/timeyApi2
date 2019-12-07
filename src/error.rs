use std::error::Error as StdError;
use std::convert::From;
use std::fmt;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::{self, Response, Responder};
use rocket::Request;
use rocket::http::ContentType;
use std::io::Cursor;
//use rocket::http::hyper::Request;
//use rocket::request::Request;


#[derive(Debug)]
pub enum Error {
    NotFound,
    InternalServerError,
    AuthError,
    UserExist,
    ProjectExist,
    ParamProjectNameMissing,
    ParamStartDateMissing,
    ParamEndDateMissing
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound => f.write_str("NotFound"),
            Error::AuthError => f.write_str("AuthError"),
            Error::UserExist => f.write_str("UserExist"),
            Error::ProjectExist => f.write_str("ProjectExist"),
            Error::ParamProjectNameMissing => f.write_str("ProjectNameMissing"),
            Error::InternalServerError => f.write_str("InternalServerError"),
            Error::ParamStartDateMissing => f.write_str("ParamStartDateMissing"),
            Error::ParamEndDateMissing => f.write_str("ParamEndDateMissing")
        }
    }
}

impl From<DieselError> for Error {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => Error::NotFound,
            _ => Error::InternalServerError,
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotFound => "Record not found",
            Error::AuthError => "Username or password is invalid",
            Error::UserExist => "Username already exist",
            Error::ProjectExist => "Project already exist",
            Error::ParamProjectNameMissing => "name param is missing",
            Error::InternalServerError => "Internal server error",
            Error::ParamStartDateMissing => "Param start_date missing",
            Error::ParamEndDateMissing => "Param end date missing",

        }
    }
}

/*
impl Responder<'static> for Error {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::Plain)
            .sized_body(Cursor::new(format!("testing"))
            //.sized_body(Cursor::new(self))
            .ok()
    }
}*/
impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        
        let mut my_response;
        match self {
        Error::AuthError => my_response = json!({"status": "error", "reason": "Username or password is invalid"}),
        Error::NotFound => my_response = json!({"status": "error", "reason": "Resource was not found."}), 
        Error::InternalServerError => my_response = json!({"status": "error", "reason": "InternalServerError"}),
        Error::UserExist => my_response = json!({"status": "error", "reason": "User already exist"}),
        Error::ProjectExist => my_response = json!({"status": "error", "reason": "Project already exist"}),
        Error::ParamProjectNameMissing => my_response = json!({"status": "error", "reason": "we missing 'name' param"}),
        Error::ParamStartDateMissing => my_response = json!({"status": "error", "reason": "we missing 'start_date' param"}),
        Error::ParamEndDateMissing => my_response = json!({"status": "error", "reason": "we missing 'end_date' param"}),
        }
        
        Response::build()
            .sized_body(Cursor::new(format!("{}", my_response.to_string())))
            //.raw_header("X-Person-Name", "")
            //.raw_header("X-Person-Age", "")
            .header(ContentType::new("application", "json"))
            .ok()
    }
}
