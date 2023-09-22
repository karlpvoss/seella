mod util;

use seella::{
    session_from_csv, Cli, CsvModeOptions, EventsPath, OperationMode, SessionsPath, WaterfallWidth,
};
use util::test_data;

#[test]
fn basic_functionality() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = Vec::new();
    let cli: Cli = Cli {
        mode: OperationMode::Csv(CsvModeOptions {
            session_id: String::from("74ff67c0-397b-11ee-8ca4-9688db6cc0f1"),
            sessions_path: SessionsPath(test_data("cassandra_sessions.csv")),
            events_path: EventsPath(test_data("cassandra_events.csv")),
        }),
        ..Default::default()
    };

    if let OperationMode::Csv(ref options) = cli.mode {
        let session = session_from_csv(
            &options.sessions_path,
            &options.events_path,
            &options.session_id,
        )?;
        session.display(cli, &mut output)?;
    }

    assert_eq!(
        output,
        r#"Session ID: 74ff67c0-397b-11ee-8ca4-9688db6cc0f1
2023-08-13T01:48:10.172+00:00
172.17.0.2      (N/A) -> 172.17.0.2     
Request Size:  N/A
Response Size: N/A
Execute CQL3 query
"{'consistency_level': 'ONE', 'page_size': '100', 'query': 'INSERT INTO k.t (pk, t, v, s) VALUES (0, 1, ''val1'', ''static1'');', 'serial_consistency_level': 'SERIAL', 'user_timestamp': '1691891290172041'}"

   waterfall chart                                                                                        dur    node               activity                                                                                
 1 [█                                                                                                   ] 0      172.17.0.2      ├─ Parsing a statement                                                                     
 2 [█                                                                                                   ] 27     172.17.0.2      ├─ Processing a statement                                                                  
 3 [ ███                                                                                                ] 45     172.17.0.2      ├─ Creating write handler for token: -3485513579396041028 natural: {172.17.0.3} pending: {}
 4 [    ███                                                                                             ] 46     172.17.0.2      ├─ Creating write handler with live: {172.17.0.3} dead: {}                                 
 5 [       ███                                                                                          ] 49     172.17.0.2      ├─ Sending a mutation to /172.17.0.3                                                       
 6 [          █                                                                                         ] 5      172.17.0.3      ├─ Message received from /172.17.0.2                                                       
 7 [          ███                                                                                       ] 55     172.17.0.3      ├─ Sending mutation_done to /172.17.0.2                                                    
 8 [             ████                                                                                   ] 58     172.17.0.3      ├─ Mutation handling is done                                                               
 9 [                 ████████████████████                                                               ] 332    172.17.0.2      ├─ Got a response from /172.17.0.3                                                         
10 [                                     █████████████████████                                          ] 333    172.17.0.2      ├─ Delay decision due to throttling: do not delay, resuming now                            
11 [                                                          ████████████████████                      ] 339    172.17.0.2      ├─ Mutation successfully completed                                                         
12 [                                                                              ██████████████████████] 344    172.17.0.2      ├─ Done processing - preparing a result                                                    
"#.as_bytes());

    Ok(())
}

#[test]
fn more_functionality() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = Vec::new();
    let cli: Cli = Cli {
        mode: OperationMode::Csv(CsvModeOptions {
            session_id: String::from("74ff67c0-397b-11ee-8ca4-9688db6cc0f1"),
            sessions_path: SessionsPath(test_data("cassandra_sessions.csv")),
            events_path: EventsPath(test_data("cassandra_events.csv")),
        }),
        waterfall_width: WaterfallWidth(50),
        show_event_id: true,
        show_span_ids: true,
        show_thread: true,
        ..Default::default()
    };

    if let OperationMode::Csv(ref options) = cli.mode {
        let session = session_from_csv(
            &options.sessions_path,
            &options.events_path,
            &options.session_id,
        )?;
        session.display(cli, &mut output)?;
    }

    assert_eq!(
        output,
        r#"Session ID: 74ff67c0-397b-11ee-8ca4-9688db6cc0f1
2023-08-13T01:48:10.172+00:00
172.17.0.2      (N/A) -> 172.17.0.2     
Request Size:  N/A
Response Size: N/A
Execute CQL3 query
"{'consistency_level': 'ONE', 'page_size': '100', 'query': 'INSERT INTO k.t (pk, t, v, s) VALUES (0, 1, ''val1'', ''static1'');', 'serial_consistency_level': 'SERIAL', 'user_timestamp': '1691891290172041'}"

   waterfall chart                                      dur    node               activity                                                                                 event id                              span id              parent span id       thread name
 1 [█                                                 ] 0      172.17.0.2      ├─ Parsing a statement                                                                      74ff70c8-397b-11ee-8ca4-9688db6cc0f1  0                    0                    shard 0
 2 [█                                                 ] 27     172.17.0.2      ├─ Processing a statement                                                                   74ff71dc-397b-11ee-8ca4-9688db6cc0f1  0                    0                    shard 0
 3 [██                                                ] 45     172.17.0.2      ├─ Creating write handler for token: -3485513579396041028 natural: {172.17.0.3} pending: {} 74ff728a-397b-11ee-8ca4-9688db6cc0f1  0                    0                    shard 0
 4 [  █                                               ] 46     172.17.0.2      ├─ Creating write handler with live: {172.17.0.3} dead: {}                                  74ff7296-397b-11ee-8ca4-9688db6cc0f1  0                    0                    shard 0
 5 [   ██                                             ] 49     172.17.0.2      ├─ Sending a mutation to /172.17.0.3                                                        74ff72b1-397b-11ee-8ca4-9688db6cc0f1  0                    0                    shard 0
 6 [     █                                            ] 5      172.17.0.3      ├─ Message received from /172.17.0.2                                                        74ff75b2-397b-11ee-a288-20cc230d8ac0  0                    0                    shard 4
 7 [     █                                            ] 55     172.17.0.3      ├─ Sending mutation_done to /172.17.0.2                                                     74ff779d-397b-11ee-a288-20cc230d8ac0  0                    0                    shard 4
 8 [      ██                                          ] 58     172.17.0.3      ├─ Mutation handling is done                                                                74ff77c2-397b-11ee-a288-20cc230d8ac0  0                    0                    shard 4
 9 [        ██████████                                ] 332    172.17.0.2      ├─ Got a response from /172.17.0.3                                                          74ff7dc2-397b-11ee-8ca4-9688db6cc0f1  0                    0                    shard 0
10 [                  ███████████                     ] 333    172.17.0.2      ├─ Delay decision due to throttling: do not delay, resuming now                             74ff7dcb-397b-11ee-8ca4-9688db6cc0f1  0                    0                    shard 0
11 [                             ██████████           ] 339    172.17.0.2      ├─ Mutation successfully completed                                                          74ff7e09-397b-11ee-8ca4-9688db6cc0f1  0                    0                    shard 0
12 [                                       ███████████] 344    172.17.0.2      ├─ Done processing - preparing a result                                                     74ff7e3a-397b-11ee-8ca4-9688db6cc0f1  0                    0                    shard 0
"#.as_bytes());

    Ok(())
}
