use chrono::Duration;
use clap::{ValueEnum, Args};
use log::error;
use reqwest::blocking::{Client, Request};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;
use saitama::impls::constant_rate_orchestrator::ConstantRateOrchestratorConfig;
use saitama::impls::http_worker::HttpWorkerConfig;
use saitama::traits::config::Config;

#[derive(Args, Debug, Clone)]
pub struct Punch {
    /// Target URL
    #[clap(short, long)]
    pub url: String,

    /// Headers on the request
    #[clap(short = 'H', long)]
    headers: Vec<String>,

    /// HTTP method
    #[clap(short, long, value_enum, default_value_t = Method::Get)]
    method: Method,

    /// How many worker threads to start
    #[clap(short, long, default_value_t = 10)]
    pub thread_count: u16,

    /// How many RPS to drive. This can max out on certain devices
    #[clap(short, long)]
    pub rps: u64,

    /// How long to run the test. Used with duration_unit
    #[clap(short, long)]
    duration: u64,

    /// What unit to run with the test
    #[clap(short = 'n', long, value_enum, default_value_t = ChronoUnit::Minute)]
    duration_unit: ChronoUnit,

    /// Body of the request
    #[clap(short, long, default_value = "")]
    pub body: String,
}

impl Config for Punch {}

impl HttpWorkerConfig for Punch {
    fn create_request(&self) -> Request {
        Client::new()
            .request(self.get_method(), self.url.clone())
            .headers(self.get_header_map())
            .body(self.body.clone())
            .build()
            .expect("Failed to create request")
    }
}

impl ConstantRateOrchestratorConfig for Punch {
    fn get_duration(&self) -> Duration {
        match self.duration_unit {
            ChronoUnit::Second => Duration::seconds(self.duration as i64),
            ChronoUnit::Minute => Duration::minutes(self.duration as i64),
            ChronoUnit::Hour => Duration::hours(self.duration as i64),
        }
    }

    fn get_rps(&self) -> u64 {
        self.rps
    }

    fn get_thread_count(&self) -> u16 {
        self.thread_count
    }
}

impl Punch {
    pub fn get_method(&self) -> reqwest::Method {
        match self.method {
            Method::Options => reqwest::Method::OPTIONS,
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
            Method::Trace => reqwest::Method::TRACE,
            Method::Connect => reqwest::Method::CONNECT,
            Method::Patch => reqwest::Method::PATCH,
        }
    }

    pub fn get_header_map<'a>(&self) -> HeaderMap {
        let mut hm = HeaderMap::new();

        for header in &self.headers {
            let header = header
                .chars()
                .filter(|it| !it.is_whitespace())
                .collect::<String>();

            let split = header
                .split(":")
                .map(|it| it.to_string())
                .collect::<Vec<String>>();

            if split.len() != 2 {
                error!("There were more then 2 parts of this header {}", header)
            }
            let key = HeaderName::from_str(split.get(0).unwrap()).expect("Invalid Header name");
            let value = HeaderValue::from_str(split.get(1).unwrap()).expect("Invalid Header value");

            hm.insert(key, value);
        }

        return hm;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum ChronoUnit {
    Second,
    Minute,
    Hour,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}
