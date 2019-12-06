use mosquitto_client::*;
use std::ops::Add;

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
    pub fn Connect(&mut self, newServiceName: String) {
        println!("\nMQTT: Connect({})\n", newServiceName);
        self.service_name = Some(newServiceName);

        let ses = self.service_name.as_ref().unwrap().as_str();
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

    }
    pub fn Subcribe(&self, topic: &String, callback: &dyn Fn(String)) {
        println!("\nMQTT: Subscribe({})\n", topic);

        let sub = self.connection.as_ref().unwrap().subscribe(topic.as_str(), 0).unwrap();
        let mut call = self.connection.as_ref().unwrap().callbacks(());

        call.on_message(|_, msg| {
            if sub.matches(&msg) {
                callback(String::from_utf8_lossy(msg.payload()).to_string());
            }
        });
    }
    pub fn Publish(&self, msg: &String) {
        let topic = String::from(self.service_name.as_ref().unwrap().as_str()).add("/read");
        self.PublishOn(&topic, &msg);
    }
    pub fn PublishOn(&self, topic: &String, msg: &String) {
        println!("\nMQTT: PublishOn({}, {})\n", topic, msg);
        self.connection.as_ref().unwrap().publish(topic.as_str(), msg.as_bytes(), 0, false);
    }
    pub fn close(&self) {
        self.connection.as_ref().unwrap().disconnect();
    }
}

fn print(msg: String) {
    println!("callback {}", msg);
}

fn main() {
    let serviceName = String::from("core/config");
    let topic = String::from("test");

    let mut mqtt = HelperMqtt::new(String::from("localhost"), 1883);
    mqtt.Connect(serviceName);

    mqtt.PublishOn(&topic, &String::from("test"));

    mqtt.Subcribe(&topic, &print);

    mqtt.close();
}
