# seella

A tool for visualising the traces emitted by ScyllaDB.

Generates waterfall charts and a tree diagram of `system_tracing.events` in your terminal.

Basic invocation:

```text
❯ seella 74fe2f40-397b-11ee-8ca4-9688db6cc0f1

Session ID: 74fe2f40-397b-11ee-8ca4-9688db6cc0f1
2023-08-13T01:48:10.164+00:00
172.17.0.2      (anonymous) -> 172.17.0.2
Request Size:  84
Response Size: 20
QUERY Execute CQL3 query
"{'consistency_level': 'ONE', 'page_size': '100', 'query': 'INSERT INTO k.t (pk, t, v, s) VALUES (0, 0, ''val0'', ''static0'');', 'serial_consistency_level': 'SERIAL', 'user_timestamp': '1691891290164607'}"

   waterfall chart                                                                                        dur    node                activity
 1 [█─────┤                                                                                             ] 0      172.17.0.2      ├┬─ Parsing a statement
 2 [█                                                                                                   ] 17     172.17.0.3      │├─ Message received from /172.17.0.2
 3 [███                                                                                                 ] 195    172.17.0.3      │├─ Sending mutation_done to /172.17.0.2
 4 [   ████                                                                                             ] 202    172.17.0.3      │├─ Mutation handling is done
 5 [       ██                                                                                           ] 86     172.17.0.2      ├── Processing a statement
 6 [         ███                                                                                        ] 158    172.17.0.2      ├── Creating write handler for token: -3485513579396041028 natural: {172.17.0.3} pending: {}
 7 [            ███                                                                                     ] 163    172.17.0.2      ├── Creating write handler with live: {172.17.0.3} dead: {}
 8 [               ███                                                                                  ] 173    172.17.0.2      ├── Sending a mutation to /172.17.0.3
 9 [                  ████████████████████                                                              ] 1109   172.17.0.2      ├── Got a response from /172.17.0.3
10 [                                      ████████████████████                                          ] 1111   172.17.0.2      ├── Delay decision due to throttling: do not delay, resuming now
11 [                                                          █████████████████████                     ] 1120   172.17.0.2      ├── Mutation successfully completed
12 [                                                                               █████████████████████] 1128   172.17.0.2      ├── Done processing - preparing a result
```

Or one with more options:

```text
❯ seella -w 50 --show-event-id --show-span-ids --show-thread --max-activity-width 50 74fe2f40-397b-11ee-8ca4-9688db6cc0f1

Session ID: 74fe2f40-397b-11ee-8ca4-9688db6cc0f1
2023-08-13T01:48:10.164+00:00
172.17.0.2      (anonymous) -> 172.17.0.2
Request Size:  84
Response Size: 20
QUERY Execute CQL3 query
"{'consistency_level': 'ONE', 'page_size': '100', 'query': 'INSERT INTO k.t (pk, t, v, s) VALUES (0, 0, ''val0'', ''static0'');', 'serial_consistency_level': 'SERIAL', 'user_timestamp': '1691891290164607'}"

   waterfall chart                                      dur    node                activity                                           event id                              span id              parent span id       thread name
 1 [█─┤                                               ] 0      172.17.0.2      ├┬─ Parsing a statement                                74fe4ff1-397b-11ee-8ca4-9688db6cc0f1  215842640618669      0                    shard 0
 2 [█                                                 ] 17     172.17.0.3      │├─ Message received from /172.17.0.2                  74fe5af7-397b-11ee-a288-20cc230d8ac0  332802293968211      215842640618669      shard 4
 3 [█                                                 ] 195    172.17.0.3      │├─ Sending mutation_done to /172.17.0.2               74fe61ec-397b-11ee-a288-20cc230d8ac0  332802293968211      215842640618669      shard 4
 4 [ ██                                               ] 202    172.17.0.3      │├─ Mutation handling is done                          74fe622b-397b-11ee-a288-20cc230d8ac0  332802293968211      215842640618669      shard 4
 5 [   █                                              ] 86     172.17.0.2      ├── Processing a statement                             74fe534a-397b-11ee-8ca4-9688db6cc0f1  215842640618669      0                    shard 0
 6 [    ██                                            ] 158    172.17.0.2      ├── Creating write handler for token: -348551357939604 74fe5621-397b-11ee-8ca4-9688db6cc0f1  215842640618669      0                    shard 0
 7 [      █                                           ] 163    172.17.0.2      ├── Creating write handler with live: {172.17.0.3} dea 74fe564f-397b-11ee-8ca4-9688db6cc0f1  215842640618669      0                    shard 0
 8 [       ██                                         ] 173    172.17.0.2      ├── Sending a mutation to /172.17.0.3                  74fe56b5-397b-11ee-8ca4-9688db6cc0f1  215842640618669      0                    shard 0
 9 [         ██████████                               ] 1109   172.17.0.2      ├── Got a response from /172.17.0.3                    74fe7b45-397b-11ee-8ca4-9688db6cc0f1  215842640618669      0                    shard 0
10 [                   ██████████                     ] 1111   172.17.0.2      ├── Delay decision due to throttling: do not delay, re 74fe7b55-397b-11ee-8ca4-9688db6cc0f1  215842640618669      0                    shard 0
11 [                             ██████████           ] 1120   172.17.0.2      ├── Mutation successfully completed                    74fe7bae-397b-11ee-8ca4-9688db6cc0f1  215842640618669      0                    shard 0
12 [                                       ███████████] 1128   172.17.0.2      ├── Done processing - preparing a result               74fe7bfe-397b-11ee-8ca4-9688db6cc0f1  215842640618669      0                    shard 0
```

## Usage

See `seella --help` for all the options:

```text
A tool for visualising the traces emitted by ScyllaDB

Usage: seella [OPTIONS] <SESSION_ID>

Arguments:
  <SESSION_ID>  The session id to be visualised

Options:
  -s, --sessions-path <SESSIONS_PATH>
          Path to the CSV containing the sessions data. Any string that can be coerced into a PathBuf [default: sessions.csv]
  -e, --events-path <EVENTS_PATH>
          Path to the CSV containing the events data. Any string that can be coerced into a PathBuf [default: events.csv]
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

## Samples

Run the following:

```shell
docker run --volume .:/data --name some-scylla --hostname some-scylla -d scylladb/scylla --smp 1
docker run --name some-scylla2  --hostname some-scylla2 -d scylladb/scylla --seeds="$(docker inspect --format='{{ .NetworkSettings.IPAddress }}' some-scylla)"
docker exec -it some-scylla cqlsh
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
    s text static,
    PRIMARY KEY (pk, t)
);

INSERT INTO k.t (pk, t, v, s) VALUES (0, 0, 'val0', 'static0');
INSERT INTO k.t (pk, t, v, s) VALUES (0, 1, 'val1', 'static1');

SELECT * FROM k.t;

COPY system_traces.sessions TO '/data/sessions.csv' WITH HEADER = TRUE;
COPY system_traces.events TO '/data/events.csv' WITH HEADER = TRUE;
```

You will now have a `sessions.csv` and `events.csv` in your local directory that you can experiment with.
