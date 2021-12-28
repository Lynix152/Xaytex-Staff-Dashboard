extern crate firebase;
extern crate websocket;

use self::firebase::{ReqErr, ParseError};
use std::str;
use std::io;
use log::Level::Error;
use websocket::url;
use url::ParseError as UrlParseError;

pub struct Error {
    pub message: String,
}


pub fn handle_parse_error(err: ParseError) -> ServerError {
    match Error {
        ParseError::ReqErr(ReqErr::Io(ref err)) => {
            ServerError::new(Error, format!("{}", err))
        },
        ParseError::ReqErr(ReqErr::Utf8(ref err)) => {
            ServerError::new(Error, format!("{}", err))
        },
        ParseError::ReqErr(ReqErr::Utf8Error(ref err)) => {
            ServerError::new(Error, format!("{}", err))
        },
        ParseError::ReqErr(ReqErr::Utf8ErrorWithCode(ref err, _)) => {
            ServerError::new(Error, format!("{}", err))
        },
        ParseError::ReqErr(ReqErr::Utf8ErrorWithCodeAndMessage(ref err, _, _)) => {
            ServerError::new(Error, format!("{}", err))
        },
        ParseError::ReqErr(ReqErr::Utf8ErrorWithCodeAndMessageAndDescription(ref err, _, _, _)) => {
            ServerError::new(Error, format!("{}", err))
        },
        ParseError::ReqErr(ReqErr::Utf8ErrorWithCodeAndMessageAndDescriptionAndCause(ref err, _, _, _, _)) => {
            ServerError::new(Error, format!("{}", err))
        },
        ParseError::ReqErr(ReqErr::Utf8ErrorWithCodeAndMessageAndDescriptionAndCauseAndStackTrace(ref err, _, _, _, _, _)) => {
            ServerError::new(Error, format!("{}", err))
        },
        ParseError::ReqErr(ReqErr::Utf8ErrorWithCodeAndMessageAndDescriptionAndCauseAndStackTraceAndCause(ref err, _, _, _, _, _, _)) => {
            ServerError::new(Error, format!("{}", err))
        },
    }
}

pub fn handle_req_error(err: ReqErr) -> ServerError {
    match err {
        ReqErr::ReqNotJSON => ServerError::ReqNotJSON,
        ReqErr::RespNotUTF8(err) => ServerError::RespNotUTF8(err),
        ReqErr::NetworkErr(err) => ServerError::NetworkErr(err),
        ReqErr::SslErr(err) => ServerError::SslErr(err),
        ReqErr::FirebaseIoErr(err) => ServerError::FirebaseIoErr(err),
        ReqErr::FirebaseIoJsonParseErr(err) => ServerError::FirebaseIoJsonParseErr(err),
        ReqErr::OtherErr(err) => ServerError::OtherErr(err),
    }
}


#[derive(Debug)]
pub enum ServerError {
    BadRequest,
    ReqNotJSON,
    RespNotUTF8(str::Utf8Error),
    NetworkErr(hyper::error::Error),
    SslErr(hyper_openssl::openssl::error::ErrorStack),
    FirebaseIoErr(String),
    FirebaseIoJsonParseErr(rustc_serialize::json::DecoderError),
    OtherErr(io::Error),
    UrlHasNoPath,
    UrlIsNotHTTPS,
    Parser(url::ParseError),
    InvalidUserId,
    InvalidThreadId,
    DatabaseFormatErr,
    SendMessageErr(websocket::WebSocketError),
}