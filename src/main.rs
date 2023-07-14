use loshan_keyrock::exchanges::{
    binance_json_to_levels, bitstamp_json_to_levels, get_all_streams, get_binance_snapshot,
    get_bitstamp_snapshot, ParsedUpdate,
};
use loshan_keyrock::orderbookaggregator::{Level, Summary};

use anyhow::{Context, Result};
use futures::{SinkExt, StreamExt}; //, TryFutureExt};
use serde_json;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tonic::{transport::Server, Status};

// maybe at some point worth renaming this server and adding a client to consume the grpc stream

#[tokio::main]
async fn main() -> Result<()> {
    // careful with binance, apparently btcusd is not btcusd but the correct ticker is btcusdt
    let symbol = "btcusdt".to_string();

    // This works
    // let ws_read_stream = get_bitstamp_stream(&symbol).await.context("Error in getting bistamp stream").unwrap();

    // let ws_read_stream = get_binance_stream(&symbol).await.context("Error in getting bistamp stream").unwrap();
    let mut stream_map = get_all_streams(symbol).await.unwrap();
    while let Some((key, message)) = stream_map.next().await {
        let message = message.map_err(|_| Status::internal("Failed to get message"))?;

        let message_value: serde_json::Value =
            serde_json::from_slice(&message.into_data()).expect("can't parse");
        println!("UPDATE RECEIVED");
        println!("{}", key);
        println!("{}", message_value);
        println!("{}", message_value["asks"]);
        println!("{}", message_value["bids"]);

        let parsed_update = match key {
            "BINANCE" => binance_json_to_levels(message_value)
                .expect("error in binance json value to updates"),
            "BITSTAMP" => {
                let subscription_event = &message_value["event"];

                // replace the below with match and "data" in the second branch
                if subscription_event
                    .as_str()
                    .context("can't parse event field")?
                    == "bts:subscription_succeeded"
                {
                    println!("received subscription confirmation message with no data, continue");
                    continue;
                } else {
                    bitstamp_json_to_levels(&message_value)
                        .expect("error in bitstamp json value to updates")
                }
            }
            _ => panic!("not implemented exchange"),
        };
        println!("and this is the prsed update");
        println!("{:?}", parsed_update);
    }

    // let read_future = stream_map.for_each(|message| async {
    //     println!("receiving...");
    //     let unwrapped_message = message.unwrap();
    //      //let data = unwrapped_message.into_data();
    //      let msg_str = unwrapped_message.into_text().unwrap();
    //      // tokio::io::stdout().write(&data).await.unwrap();
    //      println!("{}", msg_str);
    //      println!("received...");
    // });

    // read_future.await;

    Ok(())
}

// // Working single queries snapshots
// #[tokio::main]
// async fn main() {
//     let symbol = "ethbtc".to_string();
//     let bitsamp_string_snapshot = get_bitstamp_snapshot(&symbol).await;

//     let binance_string_snapshot = get_binance_snapshot(&symbol).await;

//     println!("{}", &bitsamp_string_snapshot.expect("bitsamp snapshot returned error")[..10000]);
//     println!("{}", "JUST printed bitstamp".to_string());
//     println!("{}", binance_string_snapshot.expect("binance snapshot returned error"));
//     println!("{}", "JUST printed binance".to_string());
// }
