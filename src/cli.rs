use clap::{crate_authors, Arg, Command};

pub fn make_cli_instance() -> Command {
    Command::new("vadims_flow")
        .version(env!("CARGO_PKG_VERSION"))
        .author(crate_authors!(",\n"))
        .about("A web server application")
        .subcommand(
            Command::new("serve")
                .about("Start the web server")
                .arg(
                    Arg::new("bind")
                        .short('b')
                        .long("bind")
                        .help("Host address to bind to")
                        .default_value("0.0.0.0"),
                )
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .help("Port to listen on")
                        .default_value("8080"),
                ),
        )
        .subcommand(Command::new("status").about("Check server status"))
        .arg_required_else_help(true)
}
