use std::{sync::mpsc, thread, time::Duration};

use actix_web::web;
use serde::{Serialize, Deserialize};
use crate::controllers::types::Payload;

#[derive(Serialize, Deserialize)]
pub struct Result {
    data: String
}
pub struct Request;
impl Request {
 pub fn handle(&self, payload: web::Json<Payload>) -> Result {

   let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // call client and wait for response
        thread::sleep(Duration::from_secs(2));

        let value = String::from(payload.content.clone());

        tx.send(value).unwrap();
    });

    let received = rx.recv().unwrap();

    return Result {
        data: received
    };

 }
}