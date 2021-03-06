use crate::args::punch::Punch;
use crossbeam::channel;
use log::info;
use saitama::model::request_record::RequestRecord;
use saitama::orchestrator::constant_http_orchestrator::ConstantHttpOrchestrator;
use saitama::orchestrator::orchestrator::Orchestrator;
use saitama::output::output::OutputHandler;
use saitama::output::term_output_handler::TermOutputHandler;
use saitama::worker::http_worker::HttpWorker;
use saitama::worker::worker::Worker;
use std::thread;
use std::thread::JoinHandle;

pub fn punch(punch: Punch) {
    let (work_send, work_recv) = channel::bounded::<bool>(1);
    let (output_send, output_recv) = channel::unbounded::<Option<RequestRecord>>();
    let (feedback_send, feedback_recv) = channel::unbounded();

    info!("Starting {} worker threads", punch.thread_count);

    let output_thread = thread::Builder::new()
        .name("output-thread".to_string())
        .spawn(|| TermOutputHandler::handle_output(output_recv))
        .expect("Unable to start output thread");

    let worker_threads = (0..punch.thread_count)
        .map(|i| {
            let work_recv = work_recv.clone();
            let output_send = output_send.clone();
            let feedback_send = feedback_send.clone();
            let punch = punch.clone();

            thread::Builder::new()
                .name(format!("worker-thread-{}", i))
                .spawn(move || {
                    HttpWorker::start(work_recv, output_send, feedback_send, punch.into())
                })
        })
        .map(|j| j.expect("Worker Thread failed to launch"))
        .collect::<Vec<JoinHandle<()>>>();

    info!("Starting orchestrator thread");
    let orchestrator_thread = thread::Builder::new()
        .name("orchestrator-thread".to_string())
        .spawn(move || ConstantHttpOrchestrator::start(work_send, feedback_recv, punch.into()))
        .expect("Orchestrator Thread failed to launch");

    for x in worker_threads {
        x.join().expect("Could not join worker thread")
    }
    output_send.send(None).unwrap();
    orchestrator_thread
        .join()
        .expect("Could not join orchestrator thread");

    output_thread.join().unwrap();
}
