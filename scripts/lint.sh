#!/bin/sh

cargo clippy --all-targets --all-features -- \
  -A clippy::all \
  -D clippy::correctness \
  -D clippy::suspicious \
  -D clippy::perf \
  -W clippy::complexity \
  -W clippy::style
