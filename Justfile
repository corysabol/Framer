set shell := ["nu", "-c"]

build-server:
  cd rust
  cargo build -p frame-server

build-bindings:
  cd rust
  cargo build -p bindings
