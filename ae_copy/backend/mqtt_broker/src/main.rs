use bytes::{BufMut, BytesMut};
use mqttbytes::v5::{ConnAck, ConnectReturnCode, ConnAckProperties, Packet, read};
use std::error::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

fn default_connack() -> ConnAckProperties {
    ConnAckProperties {
        session_expiry_interval: None,
        receive_max: None,
        max_qos: None,
        retain_available: None,
        max_packet_size: None,
        assigned_client_identifier: None,
        topic_alias_max: None,
        reason_string: None,
        user_properties: Vec::new(),
        wildcard_subscription_available: None,
        subscription_identifiers_available: None,
        shared_subscription_available: None,
        server_keep_alive: None,
        response_information: None,
        server_reference: None,
        authentication_method: None,
        authentication_data: None,
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TCP listener
    let listener = TcpListener::bind("0.0.0.0:1883").await?;
    println!("Listening on port 1883");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Client connected: {}", addr);

        tokio::spawn(async move {
            // sets up main buffer
            let mut buffer = BytesMut::with_capacity(1024);

            loop {
                let mut temp_buf = [0u8; 512]; // temp buffer for wating until all bits are here
                match socket.read(&mut temp_buf).await {
                    Ok(0) => {
                        println!("Client disconnected");
                        return;
                    }
                    Ok(n) => {
                        buffer.put(&temp_buf[..n]); // assigning packet to buffer

                        loop {
                            let length = buffer.len();
                            match read(&mut buffer, length) {
                                // reads the packet from buffer
                                Ok(packet) => {
                                    println!("MQTT packet retrieved");
                                    // println!("{:?}", packet);
                                    parser(packet, &mut socket).await;
                                    break;
                                    
                                }
                                Err(mqttbytes::Error::InsufficientBytes(_)) => {
                                    break;
                                }
                                Err(e) => {
                                    eprint!("Error parsing packet: {:?}\n", e);

                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Socket read error: {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}

async fn parser(packet: Packet, socket: &mut tokio::net::TcpStream) {
    match packet {
        Packet::Connect(connect) => {
            println!("Client ID: {}", connect.client_id);
            println!("Keep Alive: {}", connect.keep_alive);

            // send back connection acknowledgment
            let connack = ConnAck {
                session_present: false,
                code: ConnectReturnCode::Success,
                properties: Some(default_connack()),
            };

            // formatting the connack
            let mut out_buf = BytesMut::new();
            connack.write(&mut out_buf).expect("Failed CONNACK");

            // writing back to socket
            socket.write_all(&out_buf).await.expect("Failed TCP write");
            println!("ConnAck sent");
            println!();
        }

        Packet::Publish(publish) => {
            let payload = std::str::from_utf8(&publish.payload);

            println!("Topic: {}", publish.topic);
            println!("Payload: {:?}", payload);
            println!("QoS: {:?}", publish.qos);
            println!("Retain: {:?}", publish.retain);
        }
        _ => {}
    }
}