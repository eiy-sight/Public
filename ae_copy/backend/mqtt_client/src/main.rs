// needs ip addr
// needs port
use dotenvy::dotenv;
use bytes::{BufMut, BytesMut};
use mqttbytes::v5::{ConnAck, ConnectReturnCode, ConnAckProperties, Packet, read};
use std::env;
use std::error::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let full_connection = format!("{}:{}", &host, &port);
    println!("listening on {}", &full_connection);
    let listener = TcpListener::bind(&full_connection).await?;

    Ok(())
}
