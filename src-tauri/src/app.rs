use futures::{lock::Mutex, StreamExt};
use std::{sync::Arc, time::Duration}; // Import the Stream trait for the `next` method

use log::{error, info};
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, QoS};
use tauri::{async_runtime::spawn, AppHandle, Emitter};

pub struct App {
    app_handle: AppHandle,
    client: Mutex<AsyncClient>,
    data: Vec<String>,
}

impl App {
    pub fn new(app_handle: AppHandle) -> Arc<Self> {
        let broker = "broker.hivemq.com";
        let port = 1883;
        let data = [
            "pac_temperature",
            "battery_voltage_v",
            "battery_current_a",
            "battery_soc",
            "battery_temp",
            "batterySE_temp",
            "motor_controller_temp",
            "mottor_current_a",
            "motor_voltage_v",
            "motor_rpm",
            "motor_throttle",
            "gps_long",
            "gps_lat",
            "motor_puissance_instantannée",
        ];
        let uri: String = format!("tcp://{}:{}", broker, port);
        let create_opts = CreateOptionsBuilder::new()
            .server_uri(uri)
            .client_id("rust_client")
            .finalize();

        let client = AsyncClient::new(create_opts).unwrap();

        let instance = Arc::new(Self {
            app_handle,
            client: client.into(),
            data: data.iter().map(|&s| s.to_string()).collect(),
        });
        instance.connect();
        instance
    }

    pub fn connect(self: &Arc<Self>) {
        info!("dans connect()");
        let instance = self.clone();

        spawn(async move {
            loop {
                let mut client = instance.client.lock().await;
                let mut strm = client.get_stream(25);

                let conn_opts = ConnectOptionsBuilder::new_v3()
                    .keep_alive_interval(Duration::from_secs(30))
                    .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(60))
                    .clean_session(true)
                    //.password(password)
                    // .user_name(username)
                    .finalize();
                info!("Connecting to the MQTT broker");
                if let Err(e) = client.connect(conn_opts).await {
                    error!("Error connecting to MQTT broker: {:?}", e);
                    continue;
                } else {
                    info!("Connected to the MQTT broker");
                }
                let topics = instance
                    .data
                    .iter()
                    .map(|d| format!("nereides/{}", d))
                    .collect::<Vec<_>>();
                let qos = vec![QoS::AtLeastOnce; topics.len()];

                if let Err(e) = client.subscribe_many(&topics, &qos).await {
                    error!("Error subscribing to topics {:?}: {:?}", topics, e);
                } else {
                    info!("Subscribed to topics: {:?}", topics);
                };

                info!("client is connected: {}", client.is_connected());

                while let Some(msg_opt) = strm.next().await {
                    if let Some(msg) = msg_opt {
                        let payload = msg.payload();
                        info!(
                            "Received message: topic={}, payload={:?}, qos={}",
                            msg.topic(),
                            payload,
                            msg.qos()
                        );

                        // Vérifier si le payload a la taille d'un f64 (8 octets)
                        if payload.len() == 8 {
                            match <[u8; 8]>::try_from(payload) {
                                Ok(bytes) => {
                                    // Décoder les octets en f64 (little-endian)
                                    let value = f64::from_le_bytes(bytes);
                                    info!("Decoded f64 value: {}", value);
                                    let data_name = msg
                                        .topic()
                                        .strip_prefix("nereides/")
                                        .unwrap_or(msg.topic());
                                    instance.app_handle.emit(data_name, value).unwrap_or_else(
                                        |e| {
                                            error!("Failed to emit message: {:?}", e);
                                        },
                                    );
                                }
                                Err(e) => {
                                    error!("Failed to convert payload to f64: {:?}", e);
                                }
                            }
                        } else {
                            error!(
                                "Invalid payload size: {} bytes, expected 8 for f64",
                                payload.len()
                            );
                        }
                    } else {
                        error!("Error receiving message: {:?}", msg_opt);
                    }
                }
            }
        });
    }

    pub fn get_data(&self) -> Vec<String> {
        self.data.clone()
    }
}
