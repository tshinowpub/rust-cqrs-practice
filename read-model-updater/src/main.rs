#![allow(unused_imports)]

use aws_lambda_events::dynamodb::EventRecord;
use aws_lambda_events::event::dynamodb::Event;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, MySqlConnection};
use tokio_stream::StreamExt;

use crate::message_dto::MessageDto;

mod message_dto;

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
#[allow(non_snake_case)]
struct Response {
    statusCode: i32,
    body: String,
}

#[derive(Serialize)]
struct Body {
    message: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Event>) -> Result<(), Error> {
    // Extract some useful information from the request
    let _response = Response {
        statusCode: 200,
        body: "Hello World!".to_string(),
    };

    dbg!("{}", &event);

    let mut stream = tokio_stream::iter(event.payload.records.iter());
    while let Some(item) = stream.next().await {
        // 非同期処理を行う
        let _ = push_to_read_model(item).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

async fn push_to_read_model(record: &EventRecord) -> anyhow::Result<()> {
    let mut connection = MySqlConnection::connect("mysql://rust:rust@localhost:3306/rust").await?;

    match record.event_name.as_str() {
        "INSERT" => {
            let dto = MessageDto::from_event(record)?;

            let query = r#"
                INSERT INTO messages (message_write_id, account_id, channel_id, message, created_at, updated_at, deleted_at)
                    VALUES (?, ?, ?, ?, ?, NULL, NULL);
            "#;

            sqlx::query(query)
                .bind(dto.message_write_id())
                .bind(dto.account_id())
                .bind(dto.channel_id())
                .bind(dto.message())
                .bind(
                    dto
                        .created_at()
                        .map(|datetime| datetime
                            .with_timezone(&chrono::Local)
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string()
                        )
                )
                .execute(&mut connection)
                .await?;

            println!("Called Lambda event: {:?}.", record.event_name)
        },
        _ => {
            println!("Called Lambda event: {:?}.", record.event_name)
        },
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use aws_lambda_events::dynamodb::Event;
    use aws_lambda_events::serde_json;
    use lambda_runtime::LambdaEvent;
    use crate::push_to_read_model;

    #[tokio::test]
    async fn test_function_push_to_read_model() {
        let data = include_bytes!("../tests/fixtures/example-dynamodb-event.json");
        let mut event: Event = serde_json::from_slice(data).expect("Cannot parse json.");

        let event_record = event.records.pop().unwrap();

        let result = push_to_read_model(&event_record).await;

        dbg!("{}", &result);

        assert!(result.is_ok())
    }
}
