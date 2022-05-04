Fast and dirty reimplementation of this grpc bidirectional stream perfomance test: <https://github.com/dkorolev/grpc_playground>

Using rust+tonic, measuring QPS for middle 80% of requests.

## Usage
1. Install rust language <https://rustup.rs/>
2. Clone repo, start server and run perftest while server is running
```
git clone https://github.com/trueaywee/rust_tonic_grpc_bidi.git && cd rust_tonic_grpc_bidi

cargo run --bin server --release      # start server
cargo run --bin perftest --release    # run perftest
```

## TODO
Still unclear how to wrap stream in additional buffering layer, which was pretty straightforward in C++. Looks like this is flushing after every message.