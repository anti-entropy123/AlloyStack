#!/bin/bash

mkdir -p target/debug/ && mkdir -p target/release/

ls common_service | xargs -n 1 printf "./scripts/build_common_mpk.sh %s $1\n" | bash