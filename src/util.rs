use std::error::Error;
use std::{io, thread};
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpSocket;
use tokio::sync::RwLock;
use crate::proto::Metadata;
use crate::requests::raspberry_request;
use tokio_serial::{SerialPortBuilderExt, DataBits, FlowControl, Parity, StopBits};


pub const ARDUINO_SENSORS_COM_PORT: &str = "/dev/ttyACM1";
pub const ARDUINO_SERVO_COM_PORT: &str = "/dev/ttyACM0";
pub const ARDUINOS_BAUD_RATE: u32 = 9600;
pub const RASPBERRY_IP: &str = "192.168.1.231";
pub const RASPBERRY_PORT: &str = "50005";
pub const DROPLET_IP: &str = "192.168.1.82";
pub const DROPLET_PORT: &str = "55555";
pub const ARDUINO_ALERT: &str = "proximity_alert";
pub const OPEN_COMMAND: &str = "open";
pub const SERVO_SWITCH_COMMAND: &str = "SWITCH";
pub const NOT_OPEN_COMMAND: &str = "no_open";
pub const ACK_R_R: &str = "ACK_RASPBERRY_REQUEST";
pub const ACK_D_R: &str = "ACK_DROPLET_REQUEST";

pub fn to_safe<T>(b:T) -> Arc<RwLock<T>>{ Arc::from(RwLock::from(b)) }
pub fn format_url(ip:String,port:String) -> String{format!("http://{}:{}",ip,port)}

pub struct SafeBool{
    pub b:bool,
}
impl SafeBool{
    pub fn new() -> SafeBool{SafeBool{b:false}}
    pub fn switch(&mut self){
        if self.b==false{self.b=true;}
        else{self.b=false;};
    }
    pub fn clone(&self) -> SafeBool{SafeBool{b:self.b.clone()}}
}

pub fn bind(destination:String) -> Result<Option<TcpSocket>, Box<dyn Error>>{
    let socket = TcpSocket::new_v4();
    if socket.is_ok(){
        let res=socket.unwrap();
        res.set_reuseaddr(true).unwrap(); // allow to reuse the addr both for connect and listen
        res.set_reuseport(true).unwrap(); // same for the port
        res.bind(destination.parse().unwrap()).unwrap();
        return Ok(Option::from(res))
    }
    Ok(None)
}

pub async fn arduino_sensors(id:Vec<u8>, addr:String){
    let mut port = tokio_serial::new(ARDUINO_SENSORS_COM_PORT, ARDUINOS_BAUD_RATE)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(1000))
        .open_native_async().unwrap();
    let mut buffer = vec![0; 1024]; // Increased buffer size to handle larger messages
    let mut read_data = Vec::new(); // Vector to store the complete message

    loop {
        match port.read(&mut buffer).await {
            Ok(bytes) => {
                read_data.extend_from_slice(&buffer[..bytes]);

                if let Some(pos) = read_data.iter().position(|&x| x == b'\n') {
                    let line = read_data.split_off(pos + 1);
                    let message = String::from_utf8_lossy(&read_data);
                    raspberry_request(String::from(ARDUINO_ALERT), format!("http://{}", addr)).await;
                    read_data = line;
                }
            },
            Err(e) => {
                eprintln!("Error reading from serial port: {:?}", e);
                break;
            },
        }
    }
}

pub async fn send_open_command(){
    let mut port = tokio_serial::new(ARDUINO_SERVO_COM_PORT, ARDUINOS_BAUD_RATE)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(1000))
        .open_native_async().unwrap();
    port.write_all(SERVO_SWITCH_COMMAND.as_bytes()).expect("FAILURE WRITING BYTES TO MOTOR");
    thread::sleep(Duration::new(5,0));
    port.write_all(SERVO_SWITCH_COMMAND.as_bytes()).expect("FAILURE WRITING BYTES TO MOTOR");
}
pub async fn hold_for_input(){
    let mut t = String::new();
    loop{
        println!("OPEN GATE? (Y/Yes, N/No)");
        io::stdin()
            .read_line(&mut t)
            .expect("Failed to read line");
        match t.trim().to_ascii_lowercase().as_str() {
            "yes" | "y" => {raspberry_request(OPEN_COMMAND.to_string(), format_url(RASPBERRY_IP.to_string(), RASPBERRY_PORT.to_string())).await;return},
            "no" | "n" => {raspberry_request(NOT_OPEN_COMMAND.to_string(), format_url(RASPBERRY_IP.to_string(), RASPBERRY_PORT.to_string())).await;return},
            _ => println!("PLEASE INPUT A VALID INPUT"),
        }
    };
}