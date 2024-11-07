extern crate mongodb;
use std::future::IntoFuture;

use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client};

#[derive(Clone, Debug)]
pub struct MongoDB;

impl MongoDB {
    pub async fn connect_mongodb(db_ip: String, db_port: u16) -> Result<Client, String> {
        log::info!("Mongodb에 연결합니다.");

        let mongodb_uri = format!("mongodb://{}:{}", db_ip, db_port);
        let _client_options = match ClientOptions::parse(mongodb_uri).await {
            Ok(client_options) => client_options,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let _client = Client::with_options(_client_options);
        match _client {
            Ok(client) => Ok(client),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
        log::debug!(
            "Mongdb의 Collection을 생성합니다. {}: {}",
            db_name,
            coll_name
        );

        let _db = client.database(db_name);
        let res = _db.create_collection(coll_name).await;

        match res {
            Ok(_) => {}
            Err(e) => {
                log::warn!("{:?}", e);
            }
        }
    }

    pub async fn insert_document(
        client: &Client,
        db_name: &str,
        coll_name: &str,
        doc: Document,
    ) -> Result<(), String> {
        let _db = client.database(db_name);
        let _coll = _db.collection(coll_name);

        match _coll.insert_one(doc).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
