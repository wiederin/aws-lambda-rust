use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use reqwest;
use openssl;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    // Extract some useful information from the request
    println!("Received Event: {:?}", event);
    // Your API endpoint URL
    let api_url = "https://your-api-endpoint.com";

    // Make a request to the API endpoint
    let response = reqwest::get(api_url).await?;

    // Check response status and handle accordingly
    if response.status().is_success() {
        println!("API call successful!");
    } else {
        println!("Failed to call API: {:?}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    openssl::init(); // Initialize OpenSSL explicitly
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
