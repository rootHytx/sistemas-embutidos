use crate::raspberry::RaspberryService;
use crate::signatures::*;
use crate::util::{arduino_sensors};

mod raspberry;
mod droplet;
mod signatures;
mod util;
mod requests;

mod proto{
    tonic::include_proto!("embutidos");
}
#[tokio::main]
async fn main() {
    let raspberry = RaspberryService::new().await.expect("FAILURE INITIALIZING RASPBERRY SERVER");
    loop {
        
    }
}