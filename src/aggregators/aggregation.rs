use crate::args::aggregate::Percentiles;
use itertools::Itertools;
use serde_json::Value;
use statrs::statistics::{Data, OrderStatistics};
use std::collections::HashMap;

pub fn count_agg(v: Vec<String>, key_to_count: String) -> HashMap<String, usize> {
    v.iter()
        .map(|it| serde_json::from_str::<Value>(it.as_str()).unwrap())
        .map(|it| it.get(key_to_count.clone()).unwrap().clone())
        .map(|it| values_to_keys(it))
        .into_group_map_by(|it| it.clone())
        .into_iter()
        .map(|(key, value)| (key, value.len()))
        .collect::<HashMap<String, usize>>()
}

pub fn percentile(v: Vec<String>, percentile: Percentiles) -> HashMap<String, String> {
    let values: Vec<f64> = v
        .iter()
        .map(|it| serde_json::from_str::<Value>(it.as_str()).unwrap())
        .map(|it| it.get(percentile.agg_key.clone()).unwrap().clone())
        .map(|it| it.as_f64().unwrap())
        .collect::<Vec<f64>>();
    let data = Data::new(values);

    percentile
        .percentiles
        .iter()
        .map(|&it| (it, data.clone().quantile(it / 100.0)))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<String, String>>()
}

fn values_to_keys(v: Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.to_string(),
        Value::Array(_) => panic!("Cannot aggregate arrays"),
        Value::Object(_) => panic!("Cannot aggregate objects"),
    }
}
