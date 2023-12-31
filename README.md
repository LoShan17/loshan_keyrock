KeyRock Challenge defined as per Rust L2.pdf

To run the server:
```
cargo run --bin orderbook-server
```

After the server is up and running:
```
cargo run --bin orderbook-client btcusdt 10
```
or 
```
cargo run --bin orderbook-client ethbtc 10
```

to see summaries (as defined in orderbookaggregator.proto) printed to standard output
the symbol has to be present on both exchanges or the program will exit with a stream error.

References used for several topics included below:

Rust General:
https://doc.rust-lang.org/cargo/guide/project-layout.html
https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs
https://rust-cli.github.io/book/tutorial/cli-args.html
https://tokio.rs/tokio/topics/tracing

OrderBook:
https://sanket.tech/posts/rustbook/
https://github.com/inv2004/orderbook-rs/blob/master/src/ob.rs
https://stackoverflow.com/questions/30851464/https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

Streams:
https://github.com/snapview/tokio-tungstenite/issues/137
https://docs.rs/tokio-stream/latest/tokio_stream/struct.StreamMap.html
https://docs.rs/futures/latest/futures/stream/struct.SplitStream.html

https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md
notes explaining how to take an initial snapshot of exchanges and applying the diff is the right thing to do for Binance, and since we are doing it also for Bitstamp

Grcp server/client streaming example
https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md

Tests
https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html#:~:text=The%20bodies%20of%20test%20functions,in%20the%20test%20function%20panics.