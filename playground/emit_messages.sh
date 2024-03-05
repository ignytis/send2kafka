#!/usr/bin/env bash

# This example sends 5 messages to topic 'test' with simple X-Key header and payload

seq 1 5 | xargs -I % -n 1 -P 3 curl -i -X POST "http://localhost:8080/topics/test" -H X-Key:msg_% -d '{"test": 123, "th": %}'