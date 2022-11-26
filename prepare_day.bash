#!/bin/bash

DAY=$(($(env TZ=America/Toronto date +%-d)+1))
DAY_PADDED=$(printf "%02d" $DAY)

cp template.rs src/day${DAY_PADDED}.rs

echo "Don't forget to update src/lib.rs with the new day!"
