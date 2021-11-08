use human_panic::setup_panic;
use pretty_env_logger;

use dreadnought::*;

fn main() {
    pretty_env_logger::init();
    setup_panic!();

    let matches = cli::generate_cli().get_matches();

    match matches.subcommand_name().unwrap() {
        "watch" => {
            let matches = matches.subcommand_matches("watch").unwrap();
            watch::watch_command(matches).unwrap_or_else(|e| {
                close_with_error(e, 1);
            });
        }
        _ => (),
    }
}
