use std::sync::Arc;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::proto::{RaspberryListenerRequest, RaspberryListenerResponse, Metadata, Signature, DropletListenerRequest, DropletListenerResponse};
use crate::signatures::*;
use chrono::prelude::*;
use sha256::digest;
use tokio::sync::RwLock;
use crate::proto::droplet_server::{Droplet, DropletServer};
use crate::proto::raspberry_server::RaspberryServer;
use crate::raspberry::RaspberryService;
use crate::requests::raspberry_request;
use crate::util::{format_url, RASPBERRY_IP, RASPBERRY_PORT, ARDUINO_ALERT, OPEN_COMMAND, to_safe, bind, DROPLET_IP, DROPLET_PORT, SafeBool, hold_for_input, ACK_R_R};

pub struct DropletService{
    id:Arc<RwLock<Vec<u8>>>,
    addr:Arc<RwLock<String>>,
}

impl DropletService{
    pub async fn new() -> Option<DropletService>{
        if let Some(addr) = bind(format!("{}:{}", DROPLET_IP, DROPLET_PORT)).expect("FAILURE BINDING ADDRESS"){
            let address = to_safe(format!("{}:{}", addr.local_addr().unwrap().ip(), addr.local_addr().unwrap().port()));
            let socket = addr.local_addr().unwrap();
            let id = to_safe(digest(Utc::now().to_string()).as_bytes().to_vec());
            let droplet = DropletService{id, addr:address};
            let server = droplet.clone().await;
            tokio::spawn(async move{
                let msg = format!("FAILURE INITIALIZING RASPBERRY SERVER {}", socket.clone());
                Server::builder()
                    .add_service(DropletServer::new(server))
                    .serve(socket).await.expect(&*msg);
            });
            return Option::from(droplet)
        };
        None
    }
    pub async fn clone(&self) -> DropletService{
        DropletService{
            id:to_safe(self.id.read().await.clone()),
            addr:to_safe(self.addr.read().await.clone()),
        }
    }
}

#[tonic::async_trait]
impl Droplet for DropletService{
    async fn droplet_listener(&self, request: Request<DropletListenerRequest>) -> Result<Response<DropletListenerResponse>, Status> {
        let info = request.get_ref().clone().request;
        let response;
        match info.as_str() {
            ARDUINO_ALERT => {
                response=ACK_R_R.to_string();
                tokio::spawn(async move { hold_for_input().await; });
            },
            _ => response = "".to_string(),
        };
        return Ok(Response::new(DropletListenerResponse{response}))
    }
}