use anyhow::{anyhow, Result};
use bytes::Bytes;
use spin_sdk::redis_component;
use spin_sdk::redis;
use std::str::from_utf8;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;


// The environment variable set in `spin.toml` that points to the
// address of the Redis server that the component will publish
// a message to.
const REDIS_ADDRESS_ENV: &str = "REDIS_ADDRESS";

// The environment variable set in `spin.toml` that specifies
// the Redis channels that the component will publish to.
const REDIS_OUT_CHANNEL_ENV: &str = "REDIS_OUT_CHANNEL";

#[http_component]
fn handle_hello_docker(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("pong\n")
        .build())
}


#[redis_component]
fn on_message(message: Bytes) -> Result<()> {
    let message_str = from_utf8(&message)?;
    println!("{}", message_str);
    process_message("wasm_msg", &message_str)?;
    Ok(())
}

fn process_message(key: &str, value: &str) -> Result<()> {
    // Process the string here
    // connect to redis 
    let address = std::env::var(REDIS_ADDRESS_ENV)?;
    let channel = std::env::var(REDIS_OUT_CHANNEL_ENV)?;

    let conn = redis::Connection::open(&address)?;

    // Concatenate key and value
    let payload = format!("{}:{}", key, value);
    let mut array = Vec::new();
    array.push(payload.clone()); 

    // set the value in the redis
    conn.set(&key, &value.to_owned().into_bytes())
        .map_err(|_| anyhow!("Error executing Redis set command"))?;

    // log the message in a log variable
    conn.sadd("wasm_log", &array)
        .map_err(|_| anyhow!("Error executing Redis set command"))?;

    // Convert payload to Vec<u8>
    let payload_bytes = payload.into_bytes();
    // Publish the result to the output channel
    match conn.publish(&channel, &payload_bytes) {
        Ok(()) => Ok(()),
        Err(_e) => Ok(()),
    }
}