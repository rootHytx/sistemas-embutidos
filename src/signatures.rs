/*use std::fmt::format;
use chrono::Utc;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private, Public, Id};
use openssl::rsa::Rsa;
use openssl::sign::{Signer, Verifier};
use sha256::{digest, TrySha256Digest};
use crate::proto::Metadata;
use crate::proto::*;
fn sign(content:String, skey: Vec<u8>) -> Vec<u8>{
    let skey = Rsa::private_key_from_pem(&*skey).unwrap();
    let skey = PKey::from_rsa(skey).unwrap();
    let mut signer = Signer::new(MessageDigest::sha256(), &skey).unwrap();
    signer.update(content.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    signature
}

fn verify(content:String, signature:Vec<u8>, pkey: Vec<u8>){
    let pkey = PKey::public_key_from_pem(&*pkey).unwrap();
    let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey).unwrap();
    verifier.update(content.as_bytes()).unwrap();
    assert!(verifier.verify(&signature).unwrap())
}
pub fn sign_droplet_listener_request(data:Metadata, request:String, skey:Vec<u8>) ->Vec<u8>{
    let content = format!("{}{}", data.to_string(), request);
    sign(content, skey)}
pub fn verify_droplet_listener_request(request:DropletListenerRequest){
    let content = format!("{}{}", request.data.clone().unwrap().to_string(), request.request);
    verify(content, request.sign.clone().unwrap().hash, request.sign.unwrap().pkey)
}
pub fn sign_droplet_listener_response(data:Metadata, response:String, skey:Vec<u8>) ->Vec<u8>{
    let content = format!("{}{}", data.to_string(), response);
    sign(content, skey)
}
pub fn verify_droplet_listener_response(response:DropletListenerResponse){
    let content = format!("{}{}", response.data.clone().unwrap().to_string(), response.response);
    verify(content, response.sign.clone().unwrap().hash, response.sign.unwrap().pkey)
}

pub fn sign_raspberry_listener_request(data:Metadata, request:String, skey:Vec<u8>) -> Vec<u8>{
    let content = format!("{}{}", data.to_string(), request);
    sign(content, skey)
}
pub fn verify_raspberry_listener_request(request:RaspberryListenerRequest){
    let content = format!("{}{}", request.data.clone().unwrap().to_string(), request.request.clone());
    verify(content, request.sign.clone().unwrap().hash, request.sign.unwrap().pkey)
}
pub fn sign_raspberry_listener_response(data:Metadata, response:String, skey:Vec<u8>) -> Vec<u8>{
    let content = format!("{}{}", data.to_string(), response);
    sign(content, skey)
}
pub fn verify_raspberry_listener_response(response:RaspberryListenerResponse){
    let content = format!("{}{}", response.data.clone().unwrap().to_string(), response.response.clone());
    verify(content, response.sign.clone().unwrap().hash, response.sign.unwrap().pkey)
}*/