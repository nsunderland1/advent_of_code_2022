#!/bin/bash

export CARGO_PROFILE_RELEASE_DEBUG=true

echo 0 | sudo tee  /proc/sys/kernel/kptr_restrict
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
