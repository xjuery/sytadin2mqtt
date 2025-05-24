extern crate argparse;
extern crate paho_mqtt as mqtt;

mod configuration;
pub mod sytadin;

use std::process;

use crate::configuration::Configuration;

fn console(verbosity: bool, message: &str) {
    if verbosity {
        println!("{}", message);
    }
}
fn publish(
    hostname: String, port: String, 
    username: String, password: String,
    topic: String,
    data: String,
) {
    let client = mqtt::Client::new(format!("tcp://{}:{}", hostname, port)).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let connection_options = mqtt::ConnectOptionsBuilder::new()
        .clean_session(true)
        .user_name(username)
        .password(password)
        .finalize();

    client
        .connect(connection_options)
        .expect("Failed to connect to broker");
    let msg = mqtt::Message::new(topic, data, 0);
    if let Err(e) = client.publish(msg) {
        println!("Error sending message: {:?}", e);
        process::exit(1);
    }
}

fn main() {    
    let config = Configuration::new();

    // Get the Sytadin page
    console(config.verbose, "Getting traffic info from Sytadin..."); 
    let traffic_info = sytadin::get_traffic_data();
    console(config.verbose, format!("Got answer: {traffic_info}").as_str());
    
    // Publish the traffic info to MQTT
    console(config.verbose, "Publishing traffic info to MQTT...");
    publish(
        config.mqtt_hostname,
        config.mqtt_port,
        config.mqtt_username,
        config.mqtt_password,
        config.mqtt_topic,
        traffic_info
    );
    console(config.verbose, "Publishing traffic info to MQTT...Done");
}
