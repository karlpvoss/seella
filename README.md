# seella

A tool for visualising the traces emitted by ScyllaDB.

Generates waterfall charts and a tree diagram of `system_tracing.events` in your terminal.

Basic invocation:

```text
$ seella "74ff67c0-397b-11ee-8ca4-9688db6cc0f1"

Session ID: 74ff67c0-397b-11ee-8ca4-9688db6cc0f1
2023-08-13T01:48:10.172+00:00
172.17.0.2      (anonymous) -> 172.17.0.2     
Request Size:  84
Response Size: 20
Execute CQL3 query
"{'consistency_level': 'ONE', 'page_size': '100', 'query': 'INSERT INTO k.t (pk, t, v, s) VALUES (0, 1, ''val1'', ''static1'');', 'serial_consistency_level': 'SERIAL', 'user_timestamp': '1691891290172041'}"

   waterfall chart                                                                                        dur    node                activity                                                                                
 1 [█─────┤                                                                                             ] 0      172.17.0.2      ├┬─ Parsing a statement                                                                     
 2 [█                                                                                                   ] 5      172.17.0.3      │├─ Message received from /172.17.0.2                                                       
 3 [███                                                                                                 ] 55     172.17.0.3      │├─ Sending mutation_done to /172.17.0.2                                                    
 4 [   ████                                                                                             ] 58     172.17.0.3      │├─ Mutation handling is done                                                               
 5 [       █                                                                                            ] 27     172.17.0.2      ├── Processing a statement                                                                  
 6 [        ███                                                                                         ] 45     172.17.0.2      ├── Creating write handler for token: -3485513579396041028 natural: {172.17.0.3} pending: {}
 7 [           ███                                                                                      ] 46     172.17.0.2      ├── Creating write handler with live: {172.17.0.3} dead: {}                                 
 8 [              ███                                                                                   ] 49     172.17.0.2      ├── Sending a mutation to /172.17.0.3                                                       
 9 [                 ████████████████████                                                               ] 332    172.17.0.2      ├── Got a response from /172.17.0.3                                                         
10 [                                     █████████████████████                                          ] 333    172.17.0.2      ├── Delay decision due to throttling: do not delay, resuming now                            
11 [                                                          ████████████████████                      ] 339    172.17.0.2      ├── Mutation successfully completed                                                         
12 [                                                                              ██████████████████████] 344    172.17.0.2      ├── Done processing - preparing a result                                                    
```

Or one with more options:

```text
$ seella -w 50 --show-event-id --show-span-ids --show-thread --max-activity-width 50 "74ff67c0-397b-11ee-8ca4-9688db6cc0f1"

Session ID: 74ff67c0-397b-11ee-8ca4-9688db6cc0f1
2023-08-13T01:48:10.172+00:00
172.17.0.2      (anonymous) -> 172.17.0.2     
Request Size:  84
Response Size: 20
Execute CQL3 query
"{'consistency_level': 'ONE', 'page_size': '100', 'query': 'INSERT INTO k.t (pk, t, v, s) VALUES (0, 1, ''val1'', ''static1'');', 'serial_consistency_level': 'SERIAL', 'user_timestamp': '1691891290172041'}"

   waterfall chart                                      dur    node                activity                                           event id                              span id              parent span id       thread name
 1 [█─┤                                               ] 0      172.17.0.2      ├┬─ Parsing a statement                                74ff70c8-397b-11ee-8ca4-9688db6cc0f1  153249663699531      0                    shard 0
 2 [█                                                 ] 5      172.17.0.3      │├─ Message received from /172.17.0.2                  74ff75b2-397b-11ee-a288-20cc230d8ac0  343569500103777      153249663699531      shard 4
 3 [█                                                 ] 55     172.17.0.3      │├─ Sending mutation_done to /172.17.0.2               74ff779d-397b-11ee-a288-20cc230d8ac0  343569500103777      153249663699531      shard 4
 4 [ ██                                               ] 58     172.17.0.3      │├─ Mutation handling is done                          74ff77c2-397b-11ee-a288-20cc230d8ac0  343569500103777      153249663699531      shard 4
 5 [   █                                              ] 27     172.17.0.2      ├── Processing a statement                             74ff71dc-397b-11ee-8ca4-9688db6cc0f1  153249663699531      0                    shard 0
 6 [    █                                             ] 45     172.17.0.2      ├── Creating write handler for token: -348551357939604 74ff728a-397b-11ee-8ca4-9688db6cc0f1  153249663699531      0                    shard 0
 7 [     ██                                           ] 46     172.17.0.2      ├── Creating write handler with live: {172.17.0.3} dea 74ff7296-397b-11ee-8ca4-9688db6cc0f1  153249663699531      0                    shard 0
 8 [       █                                          ] 49     172.17.0.2      ├── Sending a mutation to /172.17.0.3                  74ff72b1-397b-11ee-8ca4-9688db6cc0f1  153249663699531      0                    shard 0
 9 [        ██████████                                ] 332    172.17.0.2      ├── Got a response from /172.17.0.3                    74ff7dc2-397b-11ee-8ca4-9688db6cc0f1  153249663699531      0                    shard 0
10 [                  ███████████                     ] 333    172.17.0.2      ├── Delay decision due to throttling: do not delay, re 74ff7dcb-397b-11ee-8ca4-9688db6cc0f1  153249663699531      0                    shard 0
11 [                             ██████████           ] 339    172.17.0.2      ├── Mutation successfully completed                    74ff7e09-397b-11ee-8ca4-9688db6cc0f1  153249663699531      0                    shard 0
12 [                                       ███████████] 344    172.17.0.2      ├── Done processing - preparing a result               74ff7e3a-397b-11ee-8ca4-9688db6cc0f1  153249663699531      0                    shard 0
```

## Usage

See `seella --help` for all the options:

```text
$ seella --help
A tool for visualising the traces emitted by ScyllaDB

Usage: seella [OPTIONS] <COMMAND>

Commands:
  csv   Use a pair of CSVs as a data source
  db    Use a live database as a data source
  help  Print this message or the help of the given subcommand(s)

Options:
  -w, --waterfall-width <WATERFALL_WIDTH>
          The width of the waterfall chart [default: 100]
  -d, --duration-format <DURATION_FORMAT>
          Whether to generate span durations in milliseconds or microseconds [default: micros] [possible values: millis, micros]
      --min-duration-width <MIN_DURATION_WIDTH>
          Minimum print width for the duration field, remaining will be filled with spaces [default: 6]
      --max-activity-width <MAX_ACTIVITY_WIDTH>
          Maximum print width for the activity field, remaining will be truncated [default: 300]
      --show-event-id
          Whether to show the event uuid
      --show-span-ids
          Whether to show the span ids
      --show-thread
          Whether to show the thread name
  -h, --help
          Print help
  -V, --version
          Print version
```

```text
$ seella csv --help
Use a pair of CSVs as a data source

Usage: seella csv [OPTIONS] <SESSION_ID>

Arguments:
  <SESSION_ID>  The session id to be visualised

Options:
  -s, --sessions-path <SESSIONS_PATH>  Path to the CSV containing the sessions data. Any string that can be coerced into a PathBuf [default: sessions.csv]
  -e, --events-path <EVENTS_PATH>      Path to the CSV containing the events data. Any string that can be coerced into a PathBuf [default: events.csv]
  -h, --help                           Print help
```

```text
$ seella db --help
Use a live database as a data source

Usage: seella db [OPTIONS] <SESSION_ID>

Arguments:
  <SESSION_ID>  The session id to be visualised

Options:
  -a, --addr <ADDR>  Socket Address (IP address and port) for the database connection. See [std::net::SocketAddr::from_str] [default: 127.0.0.1:9042]
  -h, --help         Print help
```

## Samples

Run the following:

```shell
make up
make cqlsh
```

And then within that `cqlsh` session:

```sql
TRACING ON;

CREATE KEYSPACE k 
WITH REPLICATION = { 
    'class' : 'NetworkTopologyStrategy', 
    'datacenter1' : 1 
};

CREATE TABLE k.t (
    pk int,
    t int,
    v text,
    s text,
    PRIMARY KEY (pk, t)
);

INSERT INTO k.t (pk, t, v, s) VALUES (0, 0, 'val0', 'static0');

COPY system_traces.sessions TO '/data/sessions.csv' WITH HEADER = TRUE;
COPY system_traces.events TO '/data/events.csv' WITH HEADER = TRUE;
```

You will now have a `sessions.csv` and `events.csv` in your local directory that you can experiment with.

Run `make down` when you're done to shut down the cluster.
