#![unstable]
//! HTTP Versions enum
//!
//! Instead of relying on typo-prone Strings, use expected HTTP versions as
//! the `HttpVersion` enum.
use std::fmt;

use self::HttpVersion::{Http09, Http10, Http11, Http20};

/// Represents a version of the HTTP spec.
#[deriving(PartialEq, PartialOrd)]
#[unstable]
pub enum HttpVersion {
    /// `HTTP/0.9`
    #[unstable]
    Http09,
    /// `HTTP/1.0`
    #[unstable]
    Http10,
    /// `HTTP/1.1`
    #[unstable]
    Http11,
    /// `HTTP/2.0`
    #[unstable]
    Http20
}

impl fmt::Show for HttpVersion {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Http09 => "HTTP/0.9",
            Http10 => "HTTP/1.0",
            Http11 => "HTTP/1.1",
            Http20 => "HTTP/2.0",
        }.fmt(fmt)
    }
}
