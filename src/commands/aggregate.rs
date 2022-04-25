use std::collections::{HashMap};
use std::thread;
use std::thread::{JoinHandle};
use chrono::{Duration, Utc};
use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use log::{debug, error};
use serde_json::{json};
use crate::aggregators::aggregation::{count_agg, percentile};
use crate::args::aggregate::{Aggregate};

pub fn aggregate(a: Aggregate) {
    if !a.is_valid() {
        error!("Either one aggregation must be specified")
    }

    let (stdin_send, stdin_recv) = channel::unbounded::<Option<String>>();
    let (grouped_send, grouped_recv) = channel::unbounded::<Option<Vec<String>>>();

    let input_reader_thread = get_input(stdin_send.clone());
    let aggregator_thread = aggregator(stdin_recv, grouped_send, a.period);

    while let Some(grouped) = grouped_recv.recv().unwrap() {
        let counted = a.count_values.iter()
            .map(|it| (it.clone(), count_agg(grouped.clone(), it.clone())))
            .collect::<HashMap<String, HashMap<String, usize>>>();

        let percentiles = a.percentiles.iter()
            .map(|it| (it.clone().agg_key, percentile(grouped.clone(), it.clone())))
            .collect::<HashMap<String, HashMap<String, String>>>();


        let mut j = HashMap::new();

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



fn get_input(
    stdin_send: Sender<Option<String>>
) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            let bytes = std::io::stdin().read_line(&mut buf).unwrap();
            if bytes == 0 {
                break;
            }
            stdin_send.send(Some(buf)).unwrap();
        }
        debug!("No More input");
        stdin_send.send(None).unwrap();
    })
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
