#!/bin/bash

export CARGO_PROFILE_RELEASE_DEBUG=false

echo 1 | sudo tee  /proc/sys/kernel/kptr_restrict
echo 4 | sudo tee /proc/sys/kernel/perf_event_paranoid
