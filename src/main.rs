use clap::Parser;
use log::debug;
use netero::args::args::{Cli, Commands};
use netero::commands::{aggregate, plot, punch};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

fn main() {
    let args: Cli = Cli::parse();
    let command = args.command;
    let log_level = args.verbose.log_level_filter();
    CombinedLogger::init(vec![TermLogger::new(
        log_level,
        ConfigBuilder::new().set_time_to_local(true).build(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )])
    .expect("Logger failed to instantiate");

    debug!("Got Command {:?}", command);

    match command {
        Commands::Punch(p) => punch::punch(p),
        Commands::Aggregate(a) => aggregate::aggregate(a),
        Commands::Plot(p) => plot::plot(p),
    };
}
