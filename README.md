# aws-lambda-rust

## Description
Template for deploying a rust function on AWS lambda 

## Installation
Install [cargo lambda](https://www.cargo-lambda.info/guide/getting-started.html):
`pip3 install cargo-lambda`

## Test and Deploy
To run the function locally:
`cargo lambda watch`

To invoke it locally:
`cargo lambda invoke aws-lambda-rust --data-file payload.json`

To build the binary for deployment:
`cargo lambda build --release`

Zip the generated binary found at `target/lambda/aws-lambda-rust`

