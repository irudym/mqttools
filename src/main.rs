use mqttools::{ args::Args };
use std::io::Result;
use paho_mqtt as mqtt;
use std::{ process, thread, time };
use uuid::Uuid;


fn main() -> Result<()> {
    let args = Args::parse();

    let client_id = format!("mqttools_v5-mqtt-{}", Uuid::new_v4());

    // Create the client. Use an ID for a persistent session.
    // A real system should try harder to use a unique ID.
    println!("Connect to host: {}", &args.host);
    
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .mqtt_version(mqtt::MQTT_VERSION_5)
        .server_uri(args.host)
        .client_id(client_id)
        .finalize();

    let mut client = match mqtt::Client::new(create_opts) {
        Ok(client) => client,
        Err(error) => {
            println!("Error creating the client: {:?}", error);
            process::exit(1);
        }
    };

    // Initialize the consumer before connecting
    let rx = client.start_consuming();

    // Define the set of options for the connection
    let lwt = mqtt::MessageBuilder::new()
        .topic("lwt")
        .payload("MQTTools (mqtt-v5) lost connection")
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        //.mqtt_version(mqtt::MQTT_VERSION_5)
        .clean_session(false)
        .will_message(lwt)
        .finalize();


    // Connect and wait for it to complete or fail
    if let Err(error) = client.connect(conn_opts) {
        println!("Unable to connect: {:?}", error);
        process::exit(1);
    }

    if args.publisher {
        // publish messages in the loop
        if let Some(topic) = args.topic {
            if let Some(message) = args.message {
                // Create a message and publish it
                let msg = mqtt::MessageBuilder::new()
                    .topic(topic)
                    .payload(message)
                    .qos(1)
                    .finalize();
                loop {
                    // publish a messages
                    println!("PUB: {}/{}", msg.topic(), msg.payload_str());
                    if let Err(e) = client.publish(msg.clone()) {
                        println!("Error sending message: {:?}", e);
                    }
                    thread::sleep(time::Duration::from_millis(args.interval));
                }
            } else {
                println!("Error: Message is empty");
            } 
        } else {
            println!("Error: The topic is not specified!");
        }
    } else {
        // assume that we need to subscribe
        if let Some(topic) = args.topic {
            if let Err(error) = client.subscribe(&topic, 1) {
                println!("Error: cannot subscribe to topic: {} due to error: {}", topic, error);
                process::exit(1);    
            }
            println!("Waiting messages for topic: {}", &topic);
            for message in rx.iter() {
                if let Some(message) = message {
                    println!("{}", message);
                }
            }
        }
    }

    Ok(())
}
