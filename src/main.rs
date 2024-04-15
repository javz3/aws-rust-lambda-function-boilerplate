extern crate aws_rust_lambda_function_boilerplate;

use aws_rust_lambda_function_boilerplate::common::{setup::setup_sqs_client, sqs::{find_first_queue, send, receive, SQSMessage}};
use aws_rust_lambda_function_boilerplate::models::opt::{Opt, Commands};

#[tokio::main]
async fn main() -> Result<(), aws_sdk_sqs::Error> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let opt = <Opt as clap::Parser>::parse();

    // Set up the SQS client with AWS configuration
    let client = setup_sqs_client(opt.region, opt.verbose).await?;

    // Determine the queue URL either from the command line or find the first available queue
    let queue_url = if let Some(queue) = &opt.queue {
        queue.clone()
    } else {
        find_first_queue(&client).await?
    };

    // Execute the command specified by the user
    match opt.command {
        Commands::Send => {
            // Prepare the message
            let message = SQSMessage {
                body: "Hello from my queue".to_owned(),
            };

            // Send the message
            send(&client, &queue_url, &message).await?;
            println!("Message sent to queue: {}", queue_url);
        },
        Commands::Receive => {
            // Receive messages
            receive(&client, &queue_url).await?;
            println!("Messages received from queue: {}", queue_url);
        },
    }

    Ok(())
}
