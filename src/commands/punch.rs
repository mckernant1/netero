use crate::args::punch::Punch;
use saitama::impls::http_worker::HttpWorker;
use saitama::impls::request_record::RequestRecord;
use saitama::impls::term_output_handler::TermOutputHandler;
use saitama::runner::runner::run_constant_rate_orchestrator_load_test;

pub fn punch(punch: Punch) {
    run_constant_rate_orchestrator_load_test::<
        Punch,
        RequestRecord,
        HttpWorker,
        TermOutputHandler
    >(punch);
}
