use futures::{lock::Mutex, StreamExt};
use std::{sync::Arc, time::Duration}; // Import the Stream trait for the `next` method

use log::{error, info};
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, QoS};
use std::net::Ipv4Addr;
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
            "ipv4",
            "battery_voltage_v",
            "battery_current_a",
            "battery_soc",
            "battery_soh",
            "battery_temp_c",
            "batterySE_temp",
            "motor_controller_temp",
            "motor_controller_status",
            "gps_millis",
            "gps_time",
            "gps_latitude",
            "gps_longitude",
            "gps_vitesse",
            "mottor_current_a",
            "motor_voltage_v",
            "motor_rpm",
            "motor_throttle",
            "motor_temp",
            "motor_error_code",
            "motor_switch_signals_status",
            "pac_emergency_stop",
            "pac_start",
            "pac_stop",
            "pac_current_a",
            "pac_voltage_v",
            "pac_system_state",
            "pac_error_flag",
            "pac_hydrogen_consumption_mgs",
            "pac_temperature_c",
            "pac_system_errors",
            "pac_fan_error",
            "pac_operation_time",
            "pac_produced_energy",
            "pac_total_operation_time",
            "pac_total_produced_energy",
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
                        // Décoder les octets en f64 (little-endian)
                        let data_name =
                            msg.topic().strip_prefix("nereides/").unwrap_or(msg.topic());
                        match data_name {
                            "ipv4" => {
                                let ip_u32 =
                                    u32::from_le_bytes(payload.try_into().unwrap_or([0; 4]));
                                let ipv4 = Ipv4Addr::from(ip_u32);
                                let value = ipv4.to_string();
                                info!("Decoded value: {}", value);
                                instance
                                    .app_handle
                                    .emit(data_name, value)
                                    .unwrap_or_else(|e| {
                                        error!("Failed to emit message: {:?}", e);
                                    });
                                // Convert the string to a float (example conversion)
                            } // Example value for "ipv4"

                            _ => {
                                if payload.len() == 8 {
                                    let array: [u8; 8] = payload.try_into().unwrap_or([0; 8]);
                                    let value = f64::from_le_bytes(array);
                                    info!("Decoded value: {}", value);
                                    instance.app_handle.emit(data_name, value).unwrap_or_else(
                                        |e| {
                                            error!("Failed to emit message: {:?}", e);
                                        },
                                    );
                                } else {
                                    error!("Payload size mismatch for f64 conversion");
                                    continue;
                                }
                            }
                        };
                    }
                }
            }
        });
    }

    pub fn get_data(&self) -> Vec<String> {
        self.data.clone()
    }
}
