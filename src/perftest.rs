pub mod pb {
    tonic::include_proto!("test_bidi_stream");
}

use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Instant;
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Channel;

use pb::Req;

const STARTID: u64 = 1_000_000_000u64;

fn requests_iter() -> impl Stream<Item = Req> {
    let mut rng = StdRng::seed_from_u64(123u64);

    tokio_stream::iter(STARTID..u64::MAX).map(move |id| {
        let input = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let len = input.len() as i32;
        let i: i32 = rng.random_range(1..len);
        let c: i32 = rng.random_range(0..len - i);
        Req {
            id,
            s: input,
            i,
            c,
            n: rng.random_range(1..=50),
        }
    })
}

async fn bidi_streaming(
    client: &mut pb::rpc_bidi_stream_client::RpcBidiStreamClient<Channel>,
    num: usize,
) {
    let in_stream = requests_iter().take(num);

    let response = client.go(in_stream).await.unwrap();

    let mut resp_stream = response.into_inner();

    let mut now = Instant::now();
    let mut elapsed = 0u128;
    let indext0 = STARTID + num as u64 / 10;
    let indext1 = STARTID + num as u64 * 9 / 10;
    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        if received.id == indext0 {
            now = Instant::now()
        } else if received.id == indext1 {
            elapsed = now.elapsed().as_micros();
        }
    }
    println!(
        "\treceived {} messages in {}ms",
        indext1 - indext0,
        elapsed / 1000
    );
    println!(
        "\tQPS: {:.0}",
        1_000_000f64 * ((indext1 - indext0) as f64) / elapsed as f64
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = pb::rpc_bidi_stream_client::RpcBidiStreamClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    println!("\r\nBidirectional stream:");
    let n = 100_000usize;

    bidi_streaming(&mut client, n).await;

    Ok(())
}
