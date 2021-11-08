use super::pool;
use super::HttpPetition;
use std::net::TcpListener;

pub fn watch_command(matches: &clap::ArgMatches) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(matches.value_of("Address").unwrap())?;

    for stream in listener.incoming() {
        let petition = HttpPetition::from_conn(stream.unwrap()).unwrap();
        println!("{:?}", petition);
        println!("{}", petition);
        petition.response(b"HTTP/1.1 204 OK").unwrap();
    }

    Ok(())
}
