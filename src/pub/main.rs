use std::{
    env,
    process,
    thread,
    time::Duration
};

extern crate paho_mqtt as mqtt;
use chrono::prelude::*;
use configparser::ini::Ini;
use std::error::Error;


// Reconnect to the broker when connection is lost.
fn try_reconnect(cli: &mqtt::Client) -> bool
{
    println!("Connection lost. Waiting to retry connection");
    for _ in 0..12 {
        thread::sleep(Duration::from_millis(5000));
        if cli.reconnect().is_ok() {
            println!("Successfully reconnected");
            return true;
        }
    }
    println!("Unable to reconnect after several attempts.");
    false
}

fn main() -> Result<(), Box<dyn Error>>  {
    let mut config = Ini::new();
    let conf_path = env::args().nth(1);
    let _load_config = match conf_path{
        None => println!("Pass First Argument as path/name for config file."),
        Some(x) => println!("Config File: {}",x)
    };
    let _load_config = config.load(env::args().nth(1).unwrap())?;
    
    // You can easily load a file to get a clone of the map:
    let interval_in_ms = config.get("Parameters", "interval_in_ms").unwrap();
    let client_id = config.get("Parameters", "client_id").unwrap();
    let topic = config.get("Parameters", "topic").unwrap();
    let host = config.get("Parameters", "host").unwrap();
    let qos = config.get("Parameters", "qos").unwrap();

    // Define the set of options for the create.
    // Use an ID for a persistent session.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(client_id)
        .finalize();

    // Create a client.
    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    // Define the set of options for the connection.
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    // Connect and wait for it to complete or fail.
    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    // Create a message and publish it.
    loop{
       
        // Get timestamp in epoch unix format
        let now = Utc::now();
        now.to_rfc3339_opts(SecondsFormat::Secs, true).to_string();

        // Create string contents to send as a message
        let content =  now.to_string() + &"Temperature is stable".to_string();
        
        // Prepare msg schema for mqtt client
        let msg = mqtt::Message::new(&topic, content.clone(), qos.parse().unwrap());
                
       // Publish message to topic.
        let tok = cli.publish(msg);

        match tok{
            Ok(_v) => println!("msg published on topic {} at {} ",&topic,&now),
            Err(e) => println!("Error sending message:{:?}", e),
        }

        if !cli.is_connected(){
            if try_reconnect(&cli) {
                println!("Republish to topics...");
            } else {
                break;
            }
        }
        // if let Err(e) = tok {
        //         println!("Error sending message: {:?}", e);
        //         break;
        // }

        // adding sleep
        thread::sleep_ms(interval_in_ms.parse().unwrap());
    }

    // Disconnect from the broker.
    let tok = cli.disconnect(None);
    println!("Disconnect from the broker");
    tok.unwrap();
Ok(())
}