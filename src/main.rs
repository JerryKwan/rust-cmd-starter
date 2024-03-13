mod cli;
mod config;
mod logger;
use tracing::{debug, error, info, warn};
fn main() {
    let _ = logger::setup_logging_and_tracing("main.log");
    let app = cli::CliCommand::build_cli();
    let matches = app.get_matches();
    debug!("{:?}", matches);
    if let Some(level) = matches.get_one::<String>("level") {
        debug!("Log level: {}", level);
    }
    let sys_config = config::Config::load("config.json").unwrap();
    info!("config = {:?}", sys_config);
    if let Ok(command) = cli::CliCommand::parse_cli_args(matches) {
        debug!("{:?}", command);
        command.execute();
    } else {
        error!("Failed to parse command line arguments");
    }
}
