//! ZooKeeper "For Letter Word" commands
//!
//! Provides a high-level TCP client for monitoring ZooKeeper
#[macro_use]
extern crate failure;

use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::num;
use std::str::Utf8Error;

pub struct Client {
    addr: String,
}

pub trait FourLetterWord {
    type Response;
    fn command() -> &'static str;
    fn parse_response(_: &str) -> Result<Self::Response>;
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to parse integer: {}", _0)]
    Parse(#[cause] num::ParseIntError),

    #[fail(display = "Field missing from response: {}", _0)]
    MissingField(&'static str),

    #[fail(display = "Encountered I/O error: {}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "Response wasn't valid UTF-8: {}", _0)]
    Utf8(#[cause] Utf8Error),
}

impl From<num::ParseIntError> for Error {
    fn from(val: num::ParseIntError) -> Error {
        Error::Parse(val)
    }
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Error {
        Error::Io(val)
    }
}

impl From<Utf8Error> for Error {
    fn from(val: Utf8Error) -> Error {
        Error::Utf8(val)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl Client {
    pub fn new<S: Into<String>>(addr: S) -> Self {
        Self { addr: addr.into() }
    }

    pub fn exec<F>(&self) -> Result<F::Response>
        where F: FourLetterWord
    {
        let mut stream = TcpStream::connect(&self.addr)?;
        stream.write_all(F::command().as_bytes())?;
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf)?;
        let s = ::std::str::from_utf8(&buf)?;

        F::parse_response(s)
    }
}

/// The "mntr" command
pub use mntr::Mntr;

pub mod mntr {
    use std::collections::HashMap;

    use super::{Result, Error};

    pub struct Mntr;

    /// Response to the `mntr` command
    ///
    /// The fields here are what's defined in the docs as of 2018/02/23.
    /// Additional fields are stored as strings within zk_extras.
    #[derive(Debug)]
    pub struct Response {
        pub zk_version: String,
        pub zk_avg_latency: i64,
        pub zk_max_latency: i64,
        pub zk_min_latency: i64,
        pub zk_packets_received: i64,
        pub zk_packets_sent: i64,
        pub zk_outstanding_requests: i64,
        pub zk_server_state: String,
        pub zk_znode_count: i64,
        pub zk_watch_count: i64,
        pub zk_ephemerals_count: i64,
        pub zk_approximate_data_size: i64,
        pub zk_followers: Option<i64>,
        pub zk_synced_followers: Option<i64>,
        pub zk_pending_syncs: Option<i64>,
        pub zk_open_file_descriptor_count: Option<i64>,
        pub zk_max_file_descriptor_count: Option<i64>,
        pub zk_extras: HashMap<String, String>,
    }

    impl ::FourLetterWord for Mntr {
        type Response = Response;

        fn command() -> &'static str { "mntr" }

        // XXX (jwilm) This method could be dramatically simplified if there
        //             were a serde deserializer for
        //             "value\t *key\nvalue\t *key\n..."
        fn parse_response(response: &str) -> Result<Self::Response> {
            let mut zk_version: Option<String> = None;
            let mut zk_avg_latency: Option<i64> = None;
            let mut zk_max_latency: Option<i64> = None;
            let mut zk_min_latency: Option<i64> = None;
            let mut zk_packets_received: Option<i64> = None;
            let mut zk_packets_sent: Option<i64> = None;
            let mut zk_outstanding_requests: Option<i64> = None;
            let mut zk_server_state: Option<String> = None;
            let mut zk_znode_count: Option<i64> = None;
            let mut zk_watch_count: Option<i64> = None;
            let mut zk_ephemerals_count: Option<i64> = None;
            let mut zk_approximate_data_size: Option<i64> = None;
            let mut zk_followers: Option<i64> = None;
            let mut zk_synced_followers: Option<i64> = None;
            let mut zk_pending_syncs: Option<i64> = None;
            let mut zk_open_file_descriptor_count: Option<i64> = None;
            let mut zk_max_file_descriptor_count: Option<i64> = None;
            let mut zk_extras = HashMap::new();

            let lines = response.lines();

            for line in lines {
                let mut iter = line.split('\t');
                match (iter.next().map(|s| s.trim()), iter.next().map(|s| s.trim())) {
                    (Some(key), Some(value)) => {
                        match key {
                            "zk_version" => zk_version = Some(value.into()),
                            "zk_avg_latency" => zk_avg_latency = Some(value.parse()?),
                            "zk_max_latency" => zk_max_latency = Some(value.parse()?),
                            "zk_min_latency" => zk_min_latency = Some(value.parse()?),
                            "zk_packets_received" => zk_packets_received = Some(value.parse()?),
                            "zk_packets_sent" => zk_packets_sent = Some(value.parse()?),
                            "zk_outstanding_requests" => zk_outstanding_requests = Some(value.parse()?),
                            "zk_server_state" => zk_server_state = Some(value.into()),
                            "zk_znode_count" => zk_znode_count = Some(value.parse()?),
                            "zk_watch_count" => zk_watch_count = Some(value.parse()?),
                            "zk_ephemerals_count" => zk_ephemerals_count = Some(value.parse()?),
                            "zk_approximate_data_size" => zk_approximate_data_size = Some(value.parse()?),
                            "zk_followers" => zk_followers = Some(value.parse()?),
                            "zk_synced_followers" => zk_synced_followers = Some(value.parse()?),
                            "zk_pending_syncs" => zk_pending_syncs = Some(value.parse()?),
                            "zk_open_file_descriptor_count" => zk_open_file_descriptor_count = Some(value.parse()?),
                            "zk_max_file_descriptor_count" => zk_max_file_descriptor_count = Some(value.parse()?),
                            _ => { zk_extras.insert(key.into(), value.into()); },
                        }
                    },
                    _ => break,
                }
            }

            macro_rules! error_if_none {
                ($($name:ident)*) => {
                    $(
                        match $name {
                            Some(v) => v,
                            None => return Err(Error::MissingField(stringify!($name))),
                        }
                    )*
                }
            }

            Ok(Response {
                zk_version: error_if_none!(zk_version),
                zk_avg_latency: error_if_none!(zk_avg_latency),
                zk_max_latency: error_if_none!(zk_max_latency),
                zk_min_latency: error_if_none!(zk_min_latency),
                zk_packets_received: error_if_none!(zk_packets_received),
                zk_packets_sent: error_if_none!(zk_packets_sent),
                zk_outstanding_requests: error_if_none!(zk_outstanding_requests),
                zk_server_state: error_if_none!(zk_server_state),
                zk_znode_count: error_if_none!(zk_znode_count),
                zk_watch_count: error_if_none!(zk_watch_count),
                zk_ephemerals_count: error_if_none!(zk_ephemerals_count),
                zk_approximate_data_size: error_if_none!(zk_approximate_data_size),
                zk_followers,
                zk_synced_followers,
                zk_pending_syncs,
                zk_open_file_descriptor_count,
                zk_max_file_descriptor_count,
                zk_extras
            })
        }
    }
}
