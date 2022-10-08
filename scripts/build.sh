#!/bin/bash
set -e
cd "`dirname $0`"
source flags.sh
cd ../ && cargo near build --release --embed-abi --doc --out-dir ./res
