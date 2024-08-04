extern crate mongodb;
use std::future::IntoFuture;

use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client};

pub struct MongoDB {
    ip: String,
    port: String,
}

impl MongoDB {
    pub fn new(ip: String, port: String) -> Self {
        MongoDB { ip, port }
    }

    pub async fn connect_mongodb(&self) -> Option<Client> {
        let mongodb_uri = format!("mongodb://{}:{}", self.ip, self.port);
        let _client_options = ClientOptions::parse(mongodb_uri)
            .await
            .expect("option이 잘못되었습니다.");

        let _client = Client::with_options(_client_options);
        match _client {
            Ok(client) => Some(client),
            Err(e) => {
                log::error!("{}", e);
                None
            }
        }
    }

    pub async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
        log::debug!("Mongdb의 Collection을 생성합니다.");
        let _db = client.database(db_name);
        _db.create_collection(coll_name)
            .await
            .expect("Mongodb collection 생성 도중 문제가 발생했습니다.");
    }

    pub async fn insert_document(client: &Client, db_name: &str, coll_name: &str, doc: Document) {
        let _db = client.database(db_name);
        let _coll = _db.collection(coll_name);

        _coll
            .insert_one(doc)
            .await
            .expect("Mongodb insert를 하는 과정에서 문제가 발생했습니다.");
    }
}
