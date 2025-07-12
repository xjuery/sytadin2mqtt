extern crate argparse;
extern crate paho_mqtt as mqtt;

use log::{LevelFilter, error, info};
use log4rs;
use std::path::Path;

mod configuration;
pub mod sytadin;

use crate::configuration::Configuration;
use log4rs::Config;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use std::process;

fn configure_logging() {
    if Path::new("log4rs.yml").exists() {
        log4rs::init_file("log4rs.yml", Default::default()).unwrap();
        info!("Found log4rs.yml. Logging enabled");
    } else {
        info!("Can't find log4rs.yml. Using default logging options");
        let stdout = ConsoleAppender::builder().build();

        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Info))
            .unwrap();

        log4rs::init_config(config).unwrap();
    }
}

fn publish(
    hostname: String,
    port: String,
    username: String,
    password: String,
    topic: String,
    data: String,
) {
    let connection_options = mqtt::ConnectOptionsBuilder::new()
        .clean_session(true)
        .user_name(username)
        .password(password)
        .finalize();

    let mqtt_client = match mqtt::Client::new(format!("tcp://{}:{}", hostname, port)) {
        Ok(client) => {
            info!("Client created");
            client
        }
        Err(e) => {
            error!("Error creating the client: {:?}", e);
            process::exit(1);
        }
    };

    mqtt_client
        .connect(connection_options)
        .expect("Failed to connect to broker");

    let msg = mqtt::Message::new(topic, data, 0);

    let _result = match mqtt_client.publish(msg) {
        Ok(_) => {
            info!("Message published");
        }
        Err(e) => {
            error!("Error sending message: {:?}", e);
            process::exit(1);
        }
    };
}

fn main() {
    //Configure logging options
    configure_logging();

    //Configure the app
    let config = Configuration::new();

    // Get the Sytadin page
    //console(config.verbose, "Getting traffic info from Sytadin...");
    info!("Getting traffic info from Sytadin...");
    let traffic_info = sytadin::get_traffic_data();
    //console(config.verbose, format!("\tGot answer: {traffic_info}").as_str());
    info!("Got answer: {traffic_info}");

    // Publish the traffic info to MQTT
    info!("Publishing traffic info to MQTT...");
    publish(
        config.mqtt_hostname,
        config.mqtt_port,
        config.mqtt_username,
        config.mqtt_password,
        config.mqtt_topic,
        traffic_info,
    );
    info!("Publishing traffic info to MQTT...Done");
}
