use clap::Args;
use regex::Regex;
use std::fmt::Error;
use std::hash::{Hash, Hasher};
use std::num::ParseFloatError;
use std::str::FromStr;

/// Aggregates streaming data into
#[derive(Args, Debug, Clone)]
pub struct Aggregate {
    /// Period in seconds
    #[clap(short, long, default_value_t = 1)]
    pub period: u64,

    /// Counts the different values for a json key
    #[clap(short, long)]
    pub count_values: Vec<String>,

    /// get the percentiles of a given json key. Formatted 0,50,90,99:latency
    #[clap(short = 'P', long)]
    pub percentiles: Vec<Percentiles>,
}

impl Aggregate {
    pub fn is_valid(&self) -> bool {
        !self.count_values.is_empty() || !self.percentiles.is_empty()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Percentiles {
    pub agg_key: String,
    pub percentiles: Vec<f64>,
}

impl FromStr for Percentiles {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new("(?P<percentiles>.*):(?P<agg_key>.*)").unwrap();
        let c = r.captures(s).ok_or(Error)?;

        let percentiles = c.name("percentiles").ok_or(Error)?.as_str().to_string();
        let agg_key = c.name("agg_key").ok_or(Error)?.as_str().to_string();

        let percentiles = percentiles
            .split(",")
            .map(|it| it.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
            .map_err(|_| Error)?;

        return Ok(Percentiles {
            agg_key,
            percentiles,
        });
    }
}

impl Eq for Percentiles {}

impl Hash for Percentiles {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.agg_key.hash(state)
    }
}
