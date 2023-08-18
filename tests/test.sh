#!/bin/bash

diff <(../target/debug/seella -w 50 -s "data/sessions.csv" -e "data/events.csv" --show-event-id --show-span-ids --show-thread "74ff67c0-397b-11ee-8ca4-9688db6cc0f1") "74ff67c0-397b-11ee-8ca4-9688db6cc0f1.snapshot"
