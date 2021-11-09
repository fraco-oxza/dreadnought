// use super::pool;
// use super::close_with_error;
use super::HttpPetition;
use super::ERROR_STYLE;
use std::net::TcpListener;

pub fn watch_command(matches: &clap::ArgMatches) -> Result<(), std::io::Error> {
    let timeout: Option<u64> = match matches.value_of("read-timeout") {
        Some(val) => {
            let val: u64 = match val.parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(
                        "{} {}, {}",
                        ERROR_STYLE.paint("Error:"),
                        "The timeout must be a number",
                        e
                    );
                    std::process::exit(2);
                }
            };
            if val <= 0 {
                eprintln!(
                    "{} {}",
                    ERROR_STYLE.paint("Error:"),
                    "Value of timeout must be bigger than 0"
                );
                std::process::exit(1);
            }
            Some(val)
        }
        None => None,
    };

    println!(
        r#"
  ____                     _                         _     _   
 |  _ \ _ __ ___  __ _  __| |_ __   ___  _   _  __ _| |__ | |_ 
 | | | | '__/ _ \/ _` |/ _` | '_ \ / _ \| | | |/ _` | '_ \| __|
 | |_| | | |  __/ (_| | (_| | | | | (_) | |_| | (_| | | | | |_ 
 |____/|_|  \___|\__,_|\__,_|_| |_|\___/ \__,_|\__, |_| |_|\__|
                                               |___/           "#
    );

    let listener = TcpListener::bind(matches.value_of("Address").unwrap())?;

    println!(
        "\n{} {}",
        ansi_term::Style::new()
            .fg(ansi_term::Colour::Yellow)
            .paint("Dreadnought listening for connection in"),
        listener.local_addr()?
    );

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{} {}", ERROR_STYLE.paint("Error:"), e);
                continue;
            }
        };

        if let Some(t) = timeout {
            stream
                .set_read_timeout(Some(std::time::Duration::from_millis(t)))
                .unwrap();
        };
        println!(
            "\nRequest from {}: ",
            match stream.peer_addr() {
                Ok(ip) => ip,
                Err(e) => {
                    eprintln!("{} {}", ERROR_STYLE.paint("Error:"), e);
                    continue;
                }
            }
        );

        let petition = match HttpPetition::from_conn(stream) {
            Ok(pet) => pet,
            Err(e) => {
                eprintln!("  {} {}", ERROR_STYLE.paint("Error:"), e);
                continue;
            }
        };

        println!("{}", petition);

        petition
            .response(
                format!(
                    "HTTP/1.1 204 OK\r\nServer: Dreadnought/{}\r\n\r\n",
                    clap::crate_version!()
                )
                .as_bytes(),
            )
            .unwrap();
    }

    Ok(())
}
