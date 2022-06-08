use crate::aggregators::aggregation::{count_agg, percentile};
use crate::args::aggregate::Aggregate;
use chrono::{Duration, Utc};
use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use log::{debug, error};
use mckernant1_tools::crossbeam::stdin_reader;
use serde_json::json;
use std::collections::{BTreeMap, HashMap};
use std::process::exit;
use std::thread;
use std::thread::JoinHandle;

pub fn aggregate(a: Aggregate) {
    if !a.is_valid() {
        error!("Either one aggregation must be specified");
        exit(1);
    }

    let (stdin_send, stdin_recv) = channel::unbounded::<Option<String>>();
    let (grouped_send, grouped_recv) = channel::unbounded::<Option<Vec<String>>>();

    let input_reader_thread = stdin_reader::read_stdin(stdin_send.clone());
    let aggregator_thread = aggregator(stdin_recv, grouped_send, a.period);

    while let Some(grouped) = grouped_recv.recv().unwrap() {
        let counted = a
            .count_values
            .iter()
            .map(|it| (it.clone(), count_agg(grouped.clone(), it.clone())))
            .collect::<HashMap<String, HashMap<String, usize>>>();

        let percentiles = a
            .percentiles
            .iter()
            .map(|it| (it.clone().agg_key, percentile(grouped.clone(), it.clone())))
            .collect::<HashMap<String, HashMap<String, String>>>();

        let mut j = BTreeMap::new();

        if !counted.is_empty() {
            j.insert("counts".to_string(), json!(counted));
        }

        if !percentiles.is_empty() {
            j.insert("percentiles".to_string(), json!(percentiles));
        }

        println!("{}", serde_json::to_string(&j).unwrap())
    }

    debug!("Trying to end");
    input_reader_thread.join().unwrap();
    aggregator_thread.join().unwrap();
}

fn aggregator(
    stdin_recv: Receiver<Option<String>>,
    grouped_send: Sender<Option<Vec<String>>>,
    period: u64,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let duration = Duration::seconds(period as i64);
        let mut should_quit = false;
        loop {
            if should_quit {
                break;
            }
            let start = Utc::now();
            let mut inputs_in_period = vec![];
            while Utc::now() < start + duration {
                match stdin_recv.recv().unwrap() {
                    Some(i) => inputs_in_period.push(i),
                    None => {
                        should_quit = true;
                        break;
                    }
                }
            }
            grouped_send.send(Some(inputs_in_period)).unwrap();
        }
        debug!("Aggregator Done");
        grouped_send.send(None).unwrap();
    })
}
