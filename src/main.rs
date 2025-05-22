extern crate argparse;
extern crate paho_mqtt as mqtt;
mod sytadin;
use ini::Ini;
use std::process;
use sytadin::Sytadin;

use argparse::{ArgumentParser, Store, StoreTrue};

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
    // Parse the command line arguments
    let mut verbose = false;
    let mut ha_autodiscovery = false;
    let mut config_file = String::new();
    {  // limit the scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("retrieves traffic info from Sytadin and publishes it to MQTT.");
        ap.refer(&mut verbose)
            .add_option(
                &["-v", "--verbose"], 
                StoreTrue,
                "Be verbose");
        ap.refer(&mut ha_autodiscovery)
            .add_option(
                &["-a", "--autodiscover"], 
                StoreTrue,
                "Auto-configure HA to discover the Sytadin sensor");
        ap.refer(&mut config_file)
            .add_argument(
                "config_file",
                Store,
                "Configuration file for the MQTT informations.");
        ap.parse_args_or_exit();
    }

    // Get the MQTT broker info
    if config_file.is_empty() {
        println!("Unable to find configuration file");
        process::exit(1);
    }
    let conf = Ini::load_from_file(config_file).unwrap();
    let mqtt = conf.section(Some("MQTT")).unwrap();
    let hostname = mqtt.get("HOSTNAME").unwrap().to_string();
    let port = mqtt.get("PORT").unwrap().to_string();
    let username = mqtt.get("USERNAME").unwrap().to_string();
    let password = mqtt.get("PASSWORD").unwrap().to_string();
    let topic = mqtt.get("TOPIC").unwrap().to_string();

    // Get the Sytadin page
    console(verbose, "Getting traffic info from Sytadin..."); 
    let sytadin = Sytadin::new();
    let traffic_info = sytadin.get_traffic_data();
    console(verbose, format!("Got answer: {traffic_info}").as_str());
    
    // Publish the traffic info to MQTT
    console(verbose, "Publishing traffic info to MQTT...");
    publish(hostname, port, username, password, topic, traffic_info);
    console(verbose, "Publishing traffic info to MQTT...Done");
}
