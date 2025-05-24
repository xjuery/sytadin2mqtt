#[cfg(not(test))]
use argparse::{ArgumentParser, Store, StoreTrue};

use ini::Ini;
use std::process;

struct ProgramArguments {
    verbose: bool,
    // ha_autodiscovery: bool,
    config_file: String,
}

pub(crate) struct Configuration {
    pub verbose: bool,
    //ha_autodiscovery: bool,
    pub mqtt_hostname: String,
    pub mqtt_port: String,
    pub mqtt_username: String,
    pub mqtt_password: String,
    pub mqtt_topic: String,
}

impl Configuration {
    pub(crate) fn new () -> Configuration {
        // Get program parameters
        let args = Self::parse_arguments();

        // Get the MQTT broker info
        if args.config_file.is_empty() {
            println!("Unable to find configuration file");
            process::exit(1);
        }
        let ini_conf: Ini = Self::load_ini_configuration(args.config_file.clone());

        Self::build_configuration(args, ini_conf)
    }

    fn build_configuration(args: ProgramArguments, conf: Ini) -> Configuration {
        let mqtt = conf.section(Some("MQTT")).unwrap_or_else(|| {
            println!("Error when accessing the MQTT section in the configuration file. Be sure it exists and is properly formatted.");
            process::exit(1);
        });

        // Build the Configuration object
        Configuration {
            verbose: args.verbose,
            //ha_autodiscovery,
            mqtt_hostname: mqtt.get("HOSTNAME").unwrap_or_else(|| {
                println!("Error when reading the configuration file. MQTT HOSTNAME is missing or malformed.");
                process::exit(1);
            }).parse().unwrap(),

            mqtt_port: mqtt.get("PORT").unwrap_or_else(|| {
                println!("Error when reading the configuration file. MQTT PORT is missing or malformed.");
                process::exit(1);
            }).parse().unwrap(),
            mqtt_username: mqtt.get("USERNAME").unwrap_or_else(|| {
                println!("Error when reading the configuration file. MQTT USERNAME is missing or malformed.");
                process::exit(1);
            }).parse().unwrap(),
            mqtt_password: mqtt.get("PASSWORD").unwrap_or_else(|| {
                println!("Error when reading the configuration file. MQTT PASSWORD is missing or malformed.");
                process::exit(1);
            }).parse().unwrap(),
            mqtt_topic: mqtt.get("TOPIC").unwrap_or_else(|| {
                println!("Error when reading the configuration file. MQTT TOPIC is missing or malformed.");
                process::exit(1);
            }).parse().unwrap(),
        }
    }

    #[cfg(not(test))]  
    fn parse_arguments() -> ProgramArguments {
        let mut verbose = false;
        let mut ha_autodiscovery = false;
        let mut config_file = String::new();
        {
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
        ProgramArguments {
            verbose,
            // ha_autodiscovery,
            config_file,       
        }        
    }
    
    #[cfg(test)]
    fn parse_arguments() -> ProgramArguments {
        ProgramArguments {
            verbose: true,
            // ha_autodiscovery,
            config_file: "test.conf".to_string(),
        }
    }

    #[cfg(not(test))]
    fn load_ini_configuration(filename: String) -> Ini {
        Ini::load_from_file(filename).unwrap_or_else(|err| {
            println!("Error loading ini configuration file: {:?}", err);
            process::exit(1);
        })
    }

    #[cfg(test)]
    fn load_ini_configuration(_filename: String) -> Ini {
        let input = "
[MQTT]
HOSTNAME=localhost
PORT=1883
USERNAME=user
PASSWORD=password
TOPIC=mytopic
        ";
        Ini::load_from_str(input).unwrap_or_else(|err| {
            println!("Error loading ini configuration file: {:?}", err);
            process::exit(1);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_ini_configuration() {
        let ini_conf: Ini = Configuration::load_ini_configuration("test.conf".to_string());
        let mqtt = ini_conf.section(Some("MQTT"));
        assert_eq!(mqtt.unwrap().get("HOSTNAME"), Some("localhost"));
        assert_eq!(mqtt.unwrap().get("PORT"), Some("1883"));
        assert_eq!(mqtt.unwrap().get("USERNAME"), Some("user"));
        assert_eq!(mqtt.unwrap().get("PASSWORD"), Some("password"));
        assert_eq!(mqtt.unwrap().get("TOPIC"), Some("mytopic"));
    }

    #[test]
    fn test_build_configuration() {
        let args = ProgramArguments {
            verbose: true,
            // ha_autodiscovery: bool,
            config_file: "test.conf".to_string(),
        };
        let ini_conf: Ini = Configuration::load_ini_configuration("test.conf".to_string());
        let config = Configuration::build_configuration(args, ini_conf);
        
        assert_eq!(config.verbose, true);
        assert_eq!(config.mqtt_hostname, "localhost");
        assert_eq!(config.mqtt_port, "1883");
        assert_eq!(config.mqtt_username, "user");
        assert_eq!(config.mqtt_password, "password");
        assert_eq!(config.mqtt_topic, "mytopic");
    }
    
    #[test]
    fn test_new() {
        let config = Configuration::new();
        assert_eq!(config.verbose, true);
        assert_eq!(config.mqtt_hostname, "localhost");
        assert_eq!(config.mqtt_port, "1883");
        assert_eq!(config.mqtt_username, "user");
    }
}
