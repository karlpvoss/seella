#!/bin/bash

cat <<EOF
# seella

A tool for visualising the traces emitted by ScyllaDB.

Generates waterfall charts and a tree diagram of \`system_tracing.events\` in your terminal.

Basic invocation:

\`\`\`text
$ seella "74ff67c0-397b-11ee-8ca4-9688db6cc0f1"

EOF

cargo run -- csv -s tests/data/sessions.csv -e tests/data/events.csv "74ff67c0-397b-11ee-8ca4-9688db6cc0f1"

cat <<EOF
\`\`\`

Or one with more options:

\`\`\`text
$ seella -w 50 --show-event-id --show-span-ids --show-thread --max-activity-width 50 "74ff67c0-397b-11ee-8ca4-9688db6cc0f1"

EOF

cargo run -- -w 50 --show-event-id --show-span-ids --show-thread --max-activity-width 50 csv -s tests/data/sessions.csv -e tests/data/events.csv "74ff67c0-397b-11ee-8ca4-9688db6cc0f1"

cat <<EOF
\`\`\`

## Usage

See \`seella --help\` for all the options:

\`\`\`text
$ seella --help
EOF

cargo run -- --help

cat <<EOF
\`\`\`

\`\`\`text
EOF

echo "$ seella csv --help"

cargo run -- csv --help

cat <<EOF
\`\`\`

\`\`\`text
EOF

echo "$ seella db --help"

cargo run -- db --help

cat <<EOF
\`\`\`

## Samples

Run the following:

\`\`\`shell
make up
make cqlsh
\`\`\`

And then within that \`cqlsh\` session:

\`\`\`sql
EOF

cat tests/data/load-db.cql

cat <<EOF

COPY system_traces.sessions TO '/data/sessions.csv' WITH HEADER = TRUE;
COPY system_traces.events TO '/data/events.csv' WITH HEADER = TRUE;
\`\`\`

You will now have a \`sessions.csv\` and \`events.csv\` in your local directory that you can experiment with.

Run \`make down\` when you're done to shut down the cluster.
EOF
