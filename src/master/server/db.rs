extern crate mongodb;
use std::future::IntoFuture;

use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client};

pub struct MongoDB {
    ip: String,
    port: u16,
}

impl MongoDB {
    pub fn new(ip: String, port: u16) -> Self {
        MongoDB { ip, port }
    }

    pub async fn connect_mongodb(&self) -> Result<Client, String> {
        let mongodb_uri = format!("mongodb://{}:{}", self.ip, self.port);
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
        log::debug!("Mongdb의 Collection을 생성합니다.");
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
