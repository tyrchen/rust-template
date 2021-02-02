// all error related data structure

/// Application error definition
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppError {
    /// Error code, shall be 1:1 mapping with `error` crate
    #[prost(enumeration = "AppErrorCode", tag = "1")]
    pub code: i32,
    /// Error message
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// error code
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AppErrorCode {
    Ok = 0,
    UnsupportedApi = 1,
    MalformedApiResponse = 2,
    /// converted errors
    ProstDecodeError = 200,
    ProstEncodeError = 201,
}
// All API related data structure
// For Response, we shall define it with two fields, AppError and T, so that
// we could map it easily to Result<T, AppError>

/// request ping
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPing {
    #[prost(string, tag = "1")]
    pub msg: ::prost::alloc::string::String,
}
/// pong
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsePong {
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<AppError>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Pong>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pong {
    #[prost(string, tag = "1")]
    pub msg: ::prost::alloc::string::String,
}
