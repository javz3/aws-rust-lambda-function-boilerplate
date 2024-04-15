use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::{Client, config::Region};

pub async fn setup_sqs_client(region: Option<String>, verbose: bool) -> Result<Client, aws_sdk_sqs::Error> {
    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    if verbose {
        println!("Region: {}", region_provider.region().await.unwrap().as_ref());
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Ok(Client::new(&shared_config))
}