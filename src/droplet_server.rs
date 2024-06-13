use crate::signatures::*;
use proto::droplet_server::*;
use proto::raspberry_server::*;
use requests::*;
use crate::droplet::DropletService;

mod raspberry;
mod requests;
mod droplet;
mod signatures;
mod util;
mod proto{
    tonic::include_proto!("embutidos");
}

#[tokio::main]
async fn main() {
    let droplet = DropletService::new().await;
    loop {

    }
}