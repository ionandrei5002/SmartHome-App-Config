use mosquitto_client::*;
use std::{thread, time};

#[derive(Clone)]
struct HelperMqtt {
    mqtt_broker:    String,
    mqtt_port:      u32,
    service_name:   Option<String>,
    connection:     Option<Mosquitto>
}

impl HelperMqtt {
    pub fn new(mqtt_broker: String, mqtt_port: u32) -> HelperMqtt {
        HelperMqtt { mqtt_broker, mqtt_port, service_name: None, connection: None }
    }
    pub fn connect(&mut self, new_service_name: String) {
        println!("\nMQTT: Connect({})\n", new_service_name);
        self.service_name = Some(new_service_name);

        if self.service_name.is_some() {
            let service = self.service_name.as_ref().unwrap();
            let ses = service.as_str();
            let mqtt = mosquitto_client::Mosquitto::new_session(
                ses,
                true
            );
            let result = mqtt.connect(
                &self.mqtt_broker.as_str(),
                self.mqtt_port
            );
            if result.is_err() {
                println!("Panic!");
            }
            self.connection = Some(mqtt);
        } else {
            println!("Panic!");
        }
    }
    pub fn subscribe(&self, topic: &String, callback: &dyn Fn(String)) {
        println!("\nMQTT: Subscribe({})\n", topic);

        let conn = self.connection.as_ref();
        if conn.is_some() {
            let sub = conn.unwrap().subscribe(topic.as_str(), 0).unwrap();
            let mut call = conn.unwrap().callbacks(());

            call.on_message(|_, msg| {
                if sub.matches(&msg) {
                    callback(String::from_utf8_lossy(msg.payload()).to_string());
                }
            });

            match conn.unwrap().loop_until_disconnect(200) {
                Ok(()) => {},
                _ => {
                    println!("Panic!");
                }
            }
        } else {
            println!("Panic!");
        }
    }
    pub fn publish(&self, topic: &String, msg: &String) {
        println!("\nMQTT: Publish({}, {})\n", topic, msg);

        let conn = self.connection.as_ref();
        if conn.is_some() {
            match conn.unwrap().publish(topic.as_str(), msg.as_bytes(), 0, false) {
                Ok(res) => {
                    println!("{}", res);
                }
                _ => {
                    println!("Panic!");
                }
            }
        } else {
            println!("Panic!");
        }
    }
    #[allow(dead_code)]
    pub fn close(&self) {
        let conn = self.connection.as_ref();
        if conn.is_some() {
            match conn.unwrap().disconnect() {
                Ok(()) => {
                    println!("Disconnected!");
                }
                _ => {
                    println!("Panic!");
                }
            }
        }
    }
}

fn print(msg: String) {
    println!("callback {}", msg);
}

fn main() {
    let service_name = String::from("core/config");
    let topic = String::from("test");

    let mut mqtt = HelperMqtt::new(String::from("localhost"), 1883);
    mqtt.connect(service_name);

    let mq = mqtt.clone();
    let tt = topic.clone();
    let sub = thread::spawn(move || {
        loop {
            mq.subscribe(&tt, &print);
        }
    });

    let pubb = thread::spawn(move || {
        loop {
            mqtt.publish(&topic, &String::from("test"));
            thread::sleep(time::Duration::from_millis(100));
        }
    });
    sub.join().ok().unwrap_or_default();
    pubb.join().ok().unwrap_or_default();

//    mqtt.close();
}
