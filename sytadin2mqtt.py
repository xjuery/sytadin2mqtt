import configparser
from bs4 import BeautifulSoup
import urllib3
import paho.mqtt.publish as publish
import json
import os

def mqtt_publish(broker_address, broker_port, client_name, user, pwd, topic, value):
        publish.single(
                topic,
                payload=json.dumps(value),
                qos=0,
                hostname=broker_address,
                port=broker_port,
                client_id=client_name,
                auth={"username": user, "password": pwd}
        )

def get_level(divBarometre):
        divNiveau = divBarometre.contents[1]
        return divNiveau.contents[1]["alt"]

def get_trend(divBarometre):
        divTendance = divBarometre.contents[3]
        return divTendance.contents[1]["alt"]

def get_value(divBarometre):
        divValeur = divBarometre.contents[7]
        return divValeur.string.replace(" km", "", 1).replace("Valeur : ", "", 1).strip()

def get_trafficjam_info(html):
        soup = BeautifulSoup(html, "html.parser")
        divs = soup.find_all("div", {"class", "barometre"})
        for child in divs:
                if child.contents[1].name == "h4" and child.contents[1].string == "Cumul de bouchon":
                        barometerDiv = child.contents[3]
                        return {'level': get_level(barometerDiv), 'trend': get_trend(barometerDiv), 'value': get_value(barometerDiv)}

if __name__ == "__main__":
        config = configparser.ConfigParser()
        config.read(os.path.dirname(os.path.abspath(__file__)) + '/config.ini')

        mqtt_hostname = config['MQTT']['hostname']
        mqtt_port = int(config['MQTT']['port'])
        mqtt_username = config['MQTT']['username']
        mqtt_password = config['MQTT']['password']
        mqtt_topic = config['MQTT']['topic']

        http = urllib3.PoolManager()
        html = http.request('GET', "http://www.sytadin.fr/sys/barometres_de_la_circulation.jsp.html").data

        value = get_trafficjam_info(html)
        mqtt_publish(mqtt_hostname, mqtt_port, "sytadin-client", mqtt_username, mqtt_password, mqtt_topic, value)



