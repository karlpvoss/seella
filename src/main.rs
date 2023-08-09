use anyhow::{anyhow, bail};
use clap::Parser;
use std::{fs::File, io::BufReader};
use uuid::Uuid;

use seella::{Cli, EventRecord, Session, SessionRecord};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let session_id = Uuid::try_parse(&cli.session_id)?;

    let mut session_deserialization_errors = Vec::new();
    let session_record = csv::Reader::from_reader(BufReader::new(File::open(&cli.sessions_path)?))
        .deserialize::<SessionRecord>()
        .filter_map(|record_res| {
            record_res
                .map_err(|err| session_deserialization_errors.push(err))
                .ok()
        })
        .find(|record| record.id() == session_id)
        .ok_or(anyhow!(
            "could not find the session with id: {}",
            session_id
        ))?;

    if !session_deserialization_errors.is_empty() {
        for error in session_deserialization_errors {
            eprintln!("{}", error);
        }
        bail!(
            "errors were experienced deserializing sessions from {:?}",
            &cli.sessions_path
        );
    }

    let mut event_deserialization_errors = Vec::new();
    let event_records: Vec<EventRecord> =
        csv::Reader::from_reader(BufReader::new(File::open(&cli.events_path)?))
            .deserialize::<EventRecord>()
            .filter_map(|record_res| {
                record_res
                    .map_err(|err| event_deserialization_errors.push(err))
                    .ok()
            })
            .filter(|record| record.session_id() == session_id)
            .collect();

    if !event_deserialization_errors.is_empty() {
        for error in event_deserialization_errors {
            eprintln!("{}", error);
        }

        bail!(
            "errors were experienced deserializing events from {:?}",
            &cli.events_path
        );
    }

    let s = Session::new(session_record, event_records)?;

    println!("Session ID: {}", &s.id());
    println!("{}", &s.started_at.to_rfc3339());
    println!(
        "{:15} ({}) -> {:15}",
        &s.client, &s.username, &s.coordinator
    );
    println!("Request Size:  {}", &s.request_size);
    println!("Response Size: {}", &s.response_size);
    println!("{} {}", &s.command, &s.request);
    println!("{:?}", &s.parameters);

    let events = s.events();
    let activity_width = events
        .iter()
        .map(|e| e.activity_length())
        .max()
        .or(Some(0))
        .unwrap();
    for e in s.events() {
        println!("{}", e.display(&cli, activity_width));
    }

    Ok(())
}
