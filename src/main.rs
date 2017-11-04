extern crate catan_agent;
extern crate catan_core;
extern crate catan_render;
extern crate catan_server;

extern crate chrono;
#[macro_use]
extern crate clap;
extern crate colored;
extern crate fern;
#[macro_use]
extern crate log;
extern crate tempfile;

use clap::{App, Arg, SubCommand};
use tempfile::tempfile;
use colored::*;

use catan_agent::AgentType;

fn main() {
    let mut app = App::new("catan")
        .about("Run various catan related services")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("verbose")
                .help("Show verbose output")
                .short("v")
                .long("verbose")
                .multiple(true),
        )
        .subcommand(
            SubCommand::with_name("server")
                .about("Start catan game server")
                .arg(
                    Arg::with_name("port")
                        .help("Port number for server to listen on")
                        .short("p")
                        .long("port")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("agent")
                .about("Start catan agent")
                .arg(
                    Arg::with_name("port")
                        .help("Port number server is listening on")
                        .short("p")
                        .long("port")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("render").about(
            "Render catan board view",
        ));
    let matches = app.clone().get_matches();

    let verbosity: u64 = matches.occurrences_of("verbose");

    setup_logging(verbosity).expect("Failed to initialize logging!");

    debug!("DEBUG output enabled!");
    trace!("TRACE output enabled!");

    if let Some(matches) = matches.subcommand_matches("server") {
        info!("Current working directory: {:?}", std::env::current_dir());
        info!("Server mode is starting.");

        let port = value_t!(matches, "port", u16).unwrap_or(12_345);
        catan_server::serve(port).expect("Server failed");
    } else if let Some(matches) = matches.subcommand_matches("agent") {
        info!("Current working directory: {:?}", std::env::current_dir());
        info!("Agent mode is starting.");

        let port = value_t!(matches, "port", u16).unwrap_or(12_345);
        catan_agent::setup_agent(port, AgentType::Simple).expect("Agent failed");
    } else if let Some(_) = matches.subcommand_matches("render") {
        catan_render::start_application_view();
    } else {
        app.write_long_help(&mut std::io::stdout()).expect(
            "Failed to write help to stdout!",
        );
    }
}

fn setup_logging(verbosity: u64) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => base_config.level(log::LogLevelFilter::Info),
        1 => base_config.level(log::LogLevelFilter::Debug),
        _2_or_more => base_config.level(log::LogLevelFilter::Trace),
    };

    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            let location = record.location();

            out.finish(format_args!(
                "{}[{}][{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                format!("{}:{}", location.file(), location.line()),
                record.level(),
                message
            ))
        })
        .chain(tempfile()?);

    let stdout_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            let string_message = format!("{}", message);
            let color_message = match record.level() {
                log::LogLevel::Trace => string_message.blue(),
                log::LogLevel::Debug => string_message.green(),
                log::LogLevel::Info => string_message.normal(),
                log::LogLevel::Warn => string_message.yellow(),
                log::LogLevel::Error => string_message.red(),
            };

            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M"),
                record.target(),
                record.level(),
                color_message
            ))
        })
        .chain(std::io::stdout());

    base_config.chain(file_config).chain(stdout_config).apply()?;

    Ok(())
}
