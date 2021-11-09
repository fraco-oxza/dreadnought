use clap::{App, AppSettings, Arg, SubCommand};

pub fn generate_cli() -> App<'static, 'static> {
    App::new("Dreadnought Server")
        .version(clap::crate_version!())
        .bin_name("dng")
        .author("Francisco Carvajal <fraco_oxza@outlook.com>")
        .about("A safe, fast and reliable web server.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(watch_cli())
}

fn watch_cli() -> App<'static, 'static> {
    SubCommand::with_name("watch")
        .about("Listen and write through a Address")
        .arg(
            Arg::with_name("Address")
                .help("Address to open the socket")
                .required(true),
        )
        .arg(
            Arg::with_name("read-timeout")
                .short("t")
                .value_name("TIME")
                .default_value("2000")
                .help("Set the timeout for read in TCP, in milliseconds")
                .takes_value(true),
        )
}
