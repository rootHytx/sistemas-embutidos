use tonic::Status;
use tonic::transport::Channel;
use crate::proto::{DropletListenerRequest, DropletListenerResponse, RaspberryListenerRequest, Metadata, Signature};
use crate::proto::droplet_client::DropletClient;
use crate::proto::raspberry_client::RaspberryClient;
use crate::util::{format_url};

pub async fn try_raspberry(url:String) -> Result<Option<RaspberryClient<Channel>>, Status>{
    if RaspberryClient::connect(url.clone()).await.is_err(){return Ok(None)};
    Ok(Option::from(RaspberryClient::connect(url.clone()).await.expect("ERROR CONNECTING TO ARDUINO")))
}
pub async fn try_droplet(url:String) -> Result<Option<DropletClient<Channel>>, Status>{
    if DropletClient::connect(url.clone()).await.is_err(){return Ok(None)};
    Ok(Option::from(DropletClient::connect(url.clone()).await.expect("ERROR CONNECTING TO ARDUINO")))
}
pub async fn droplet_request(request:String, url:String) -> String{
    if let Some(mut client) = try_droplet(url.clone()).await.expect("ERROR CONNECTING TO RASPBERRY"){
        let request = tonic::Request::new(DropletListenerRequest{request});
        let response = client.droplet_listener(request).await.expect("ERROR INITIALIZING RASPBERRY COMMUNICATION");
        return response.get_ref().clone().response;
    }
    "".to_string()
}

pub async fn raspberry_request(request:String, url:String) -> String{
    if let Some(mut client) = try_raspberry(url.clone()).await.expect("ERROR CONNECTING TO DROPLET"){
        let request = tonic::Request::new(RaspberryListenerRequest{request});
        let response = client.raspberry_listener(request).await.expect("ERROR INITIALIZING DROPLET COMMUNICATION");
        return response.get_ref().clone().response;
    }
    "".to_string()
}