use std::ops::{BitAnd, BitOr};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::proto::{DropletListenerRequest, DropletListenerResponse, Metadata, RaspberryListenerRequest, RaspberryListenerResponse, Signature};
use crate::signatures::*;
use chrono::prelude::*;
use sha256::digest;
use tokio::net::TcpSocket;
use tokio::sync::RwLock;
use crate::proto::raspberry_server::{Raspberry, RaspberryServer};
use crate::requests::droplet_request;
use crate::util::{bind, RASPBERRY_IP, to_safe, format_url, DROPLET_IP, DROPLET_PORT, ARDUINO_ALERT, arduino_sensors, SafeBool, RASPBERRY_PORT, OPEN_COMMAND, NOT_OPEN_COMMAND, send_open_command};

pub struct RaspberryService{
    id:Arc<RwLock<Vec<u8>>>,
    addr:Arc<RwLock<String>>,
    block:Arc<RwLock<SafeBool>>,
}

impl RaspberryService{
    pub async fn new() -> Option<RaspberryService>{
        if let Some(addr) = bind(format!("{}:{}", RASPBERRY_IP, RASPBERRY_PORT)).expect("FAILURE BINDING ADDRESS"){
            let address = to_safe(format!("{}:{}", addr.local_addr().unwrap().ip(), addr.local_addr().unwrap().port()));
            let socket = addr.local_addr().unwrap();
            let id = to_safe(digest(Utc::now().to_string()).as_bytes().to_vec());
            let block = Arc::new(RwLock::from(SafeBool::new()));
            let raspberry = RaspberryService{id, addr:address, block};
            let server = raspberry.clone().await;
            tokio::spawn(async move{
                let msg = format!("FAILURE INITIALIZING RASPBERRY SERVER {}", socket.clone());
                Server::builder()
                    .add_service(RaspberryServer::new(server))
                    .serve(socket).await.expect(&*msg);
            });
            let arduino_server = raspberry.clone().await;
            tokio::spawn(async move {
                let id = arduino_server.id.read().await.clone();
                let addr = arduino_server.addr.read().await.clone();
                arduino_sensors(id, addr).await;
            });
            return Option::from(raspberry)
        }
        None
    }
    pub async fn clone(&self) -> RaspberryService{
        RaspberryService{
            id:to_safe(self.id.read().await.clone()),
            addr:to_safe(self.addr.read().await.clone()),
            block:Arc::new(RwLock::from(self.block.read().await.clone())),
        }
    }
    pub async fn block_switch(&self){
        self.block.write().await.switch();
    }
}
#[tonic::async_trait]
impl Raspberry for RaspberryService{
    async fn raspberry_listener(&self, request: Request<RaspberryListenerRequest>) -> Result<Response<RaspberryListenerResponse>, Status> {
        if request.get_ref().request==ARDUINO_ALERT.to_string(){
            if !self.block.read().await.clone().b && request.get_ref().request.len()>0{
                println!("REQUEST: {}", request.get_ref().request);
                droplet_request(request.get_ref().request.clone(),format_url(DROPLET_IP.to_string(), DROPLET_PORT.parse().unwrap())).await;
                self.block_switch().await;
            }
            else { println!("RASPBERRY IS BLOCKED!") };
        }
        else if request.get_ref().request==OPEN_COMMAND.to_string() && self.block.read().await.b{
            self.block.write().await.switch();
            send_open_command().await;
        }
        else if request.get_ref().request==NOT_OPEN_COMMAND.to_string() && self.block.read().await.b{
            self.block.write().await.switch();
        }
        Ok(Response::new(RaspberryListenerResponse{response:"".to_string()}))
    }
}