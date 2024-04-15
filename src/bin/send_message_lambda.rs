extern crate aws_rust_lambda_function_boilerplate;

use aws_rust_lambda_function_boilerplate::common::{setup::setup_sqs_client, sqs::{find_first_queue, send, SQSMessage}};
use aws_rust_lambda_function_boilerplate::models::opt::Opt;

#[tokio::main]
async fn main() -> Result<(), aws_sdk_sqs::Error> {
    tracing_subscriber::fmt::init();
    let opt = <Opt as clap::Parser>::parse();

    let client = setup_sqs_client(opt.region, opt.verbose).await?;

    let first_queue_url = find_first_queue(&client).await?;
    let queue_url = opt.queue.unwrap_or(first_queue_url);

    let message = SQSMessage {
        body: "hello from my queue".to_owned(),
    };

    send(&client, &queue_url, &message).await?;

    Ok(())
}
