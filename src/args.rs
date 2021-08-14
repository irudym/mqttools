use clap::{App, Arg};

#[derive(Debug)]
pub struct Args {
    pub topic: Option<String>,
    pub message: Option<String>,
    pub host: String,
    pub publisher: bool,
    pub interval: u64,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("mqttools")
            .author("Igor Rudym <irudym@gmail.com>")
            .about("A simple MQTT tool to publish and subscribe to messages")
            .arg(
                Arg::with_name("publish")
                    .help("Publish a message to MQTT server to a specific topic")
                    .short("p")
                    .long("publish")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("subscribe")
                    .help("Subscribe to a topic")
                    .short("s")
                    .long("subscribe")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("message")
                    .help("A message string")
                    .short("m")
                    .long("message")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("host")
                    .help("An address of MQTT server")
                    .short("h")
                    .long("host")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("interval")
                    .help("Time interval between publishing in ms, by default is 1000ms")
                    .short("i")
                    .long("interval")
                    .takes_value(true),
            )
            .get_matches();
        let mut publisher = false;
        let mut message = None;
        let mut interval: u64 = 1000;

        let topic = if matches.is_present("publish") {
            publisher = true;
            message = Some(matches.value_of("message").unwrap_or_default().to_string());
            Some(matches.value_of("publish").unwrap().to_string())
        } else {
            Some(matches.value_of("subscribe").unwrap_or_default().to_string())
        };
        if matches.is_present("interval") {
            interval = u64::from_str_radix(matches.value_of("interval").unwrap_or_default(), 10).unwrap_or_else(|val| {
                println!("Error: wrong interval number format: {}. Fallback to default 1000ms.", val);
                1000
            });
        }
        let host = matches.value_of("host").unwrap_or("127.0.0.1:1883").to_string();
        Self {
            topic,
            host,
            publisher,
            message,
            interval,
        }
    }
}
