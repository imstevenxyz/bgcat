#!/bin/sh

cargo install cargo-watch
cargo watch --quiet \
    --ignore builds \
    --ignore container \
    --ignore data \
    --ignore docs \
    --ignore .gitignore \
    --ignore .gitlab-ci.yml \
    --ignore dev.sh \
    --ignore LICENSE \
    --ignore README.adoc \
    --exec run
