// main.rs
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::{config::Region, meta::PKG_VERSION, Client, Error};
use clap::Parser;

mod sqs;
use sqs::{find_first_queue, receive, send, SQSMessage};

#[derive(Debug, Parser)]
struct Opt {
    /// The AWS Region.
    #[clap(short, long)]
    region: Option<String>,

    /// Which queue to use. If not provided, uses the first queue found.
    #[clap(short, long)]
    queue: Option<String>,

    /// Whether to display additional information.
    #[clap(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt {
        region,
        queue,
        verbose,
    } = Opt::parse();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    if verbose {
        println!("SQS client version: {}", PKG_VERSION);
        println!(
            "Region:             {}",
            region_provider.region().await.unwrap().as_ref()
        );
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    let first_queue_url = find_first_queue(&client).await?;
    let queue_url = queue.unwrap_or(first_queue_url);

    let message = SQSMessage {
        body: "hello from my queue".to_owned(),
    };

    send(&client, &queue_url, &message).await?;
    receive(&client, &queue_url).await?;

    Ok(())
}
