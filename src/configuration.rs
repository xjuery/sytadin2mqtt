#[cfg(not(test))]
use argparse::{ArgumentParser, Store, StoreTrue};
use log::{error};

use ini::Ini;
use std::process;


struct ProgramArguments {
    //verbose: bool,
    // ha_autodiscovery: bool,
    config_file: String,
}

pub(crate) struct Configuration {
    //pub verbose: bool,
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
            error!("Unable to find configuration file");
            process::exit(1);
        }
        let ini_conf: Ini = Self::load_ini_configuration(args.config_file.clone());

        Self::build_configuration(ini_conf)
    }

    fn build_configuration(conf: Ini) -> Configuration {
        ////////////////////////////////////////////////////////////////
        // Get the MQTT broker info
        let result = conf.section(Some("MQTT"));
        let mqtt = match result {
            Some(mqtt) => mqtt,
            None => {
                error!("MQTT section not found in the configuration file.");
                process::exit(1);
            }
        };

        ////////////////////////////////////////////////////////////////
        // Get the MQTT broker Hostname info
        let result = mqtt.get("HOSTNAME");
        let mqtt_hostname = match result {
            Some(mqtt_hostname) => mqtt_hostname,
            None => {
                error!("MQTT HOSTNAME not found in the configuration file.");
                process::exit(1);
            }
        }.parse().unwrap();

        ////////////////////////////////////////////////////////////////
        // Get the MQTT broker Port info
        let result = mqtt.get("PORT");
        let mqtt_port= match result {
            Some(mqtt_port) => mqtt_port,
            None => {
                error!("MQTT PORT not found in the configuration file.");
                process::exit(1);
            }
        }.parse().unwrap();

        ////////////////////////////////////////////////////////////////
        // Get the MQTT broker Username info
        let result = mqtt.get("USERNAME");
        let mqtt_username= match result {
            Some(mqtt_username) => mqtt_username,
            None => {
                error!("MQTT USERNAME not found in the configuration file.");
                process::exit(1);
            }
        }.parse().unwrap();

        ////////////////////////////////////////////////////////////////
        // Get the MQTT broker Password info
        let result = mqtt.get("PASSWORD");
        let mqtt_password= match result {
            Some(mqtt_password) => mqtt_password,
            None => {
                error!("MQTT PASSWORD not found in the configuration file.");
                process::exit(1);
            }
        }.parse().unwrap();

        ////////////////////////////////////////////////////////////////
        // Get the MQTT broker Topic info
        let result = mqtt.get("TOPIC");
        let mqtt_topic= match result {
            Some(mqtt_topic) => mqtt_topic,
            None => {
                error!("MQTT TOPIC not found in the configuration file.");
                process::exit(1);
            }
        }.parse().unwrap();

        ////////////////////////////////////////////////////////////////
        // Build the Configuration object
        Configuration {
            //verbose: args.verbose,
            //ha_autodiscovery,
            mqtt_hostname,
            mqtt_port,
            mqtt_username,
            mqtt_password,
            mqtt_topic,
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
            //verbose,
            // ha_autodiscovery,
            config_file,       
        }        
    }
    
    #[cfg(test)]
    fn parse_arguments() -> ProgramArguments {
        ProgramArguments {
            //verbose: true,
            // ha_autodiscovery,
            config_file: "test.conf".to_string(),
        }
    }

    #[cfg(not(test))]
    fn load_ini_configuration(config_file: String) -> Ini {
        let result = Ini::load_from_file(config_file);
        match result {
            Ok(conf) => conf,
            Err(err) => {
                error!("Unable to parse configuration file: {:?}", err);
                process::exit(1);
            }
        }
    }

    #[cfg(test)]
    fn load_ini_configuration(_config_file: String) -> Ini {
        let input = "
[MQTT]
HOSTNAME=localhost
PORT=1883
USERNAME=user
PASSWORD=password
TOPIC=mytopic
        ";
        // Ini::load_from_str(input).unwrap_or_else(|err| {
        //     console(verbose, format!("Unable to parse configuration file: {:?}", err).as_str());
        //     process::exit(1);
        // })
        let result = Ini::load_from_str(input);
        match result {
            Ok(conf) => conf,
            Err(err) => {
                error!("Unable to parse configuration file: {:?}", err);
                process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_ini_configuration() {
        let args = ProgramArguments {
            //verbose: true,
            // ha_autodiscovery: bool,
            config_file: "test.conf".to_string(),
        };

        let ini_conf: Ini = Configuration::load_ini_configuration(args.config_file.clone());
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
            //verbose: true,
            // ha_autodiscovery: bool,
            config_file: "test.conf".to_string(),
        };

        let ini_conf: Ini = Configuration::load_ini_configuration(args.config_file.clone());
        let config = Configuration::build_configuration(ini_conf);
        
        //assert_eq!(config.verbose, true);
        assert_eq!(config.mqtt_hostname, "localhost");
        assert_eq!(config.mqtt_port, "1883");
        assert_eq!(config.mqtt_username, "user");
        assert_eq!(config.mqtt_password, "password");
        assert_eq!(config.mqtt_topic, "mytopic");
    }
    
    #[test]
    fn test_new() {
        let config = Configuration::new();
        //assert_eq!(config.verbose, true);
        assert_eq!(config.mqtt_hostname, "localhost");
        assert_eq!(config.mqtt_port, "1883");
        assert_eq!(config.mqtt_username, "user");
    }
}
