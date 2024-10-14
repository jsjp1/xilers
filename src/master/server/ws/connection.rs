use super::websocket::WebSocket;
use crate::server;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::{borrow::Borrow, sync::Mutex};
use uuid::Uuid;

pub async fn start_connection(
    req: HttpRequest,
    stream: web::Payload,
    id: web::Path<(Uuid, Uuid)>,
    data: web::Data<Mutex<server::server::AppState>>,
) -> Result<HttpResponse, Error> {
    let (group_id, device_id) = id.into_inner();

    let data_lock = data.lock().unwrap();
    let ws_server = data_lock.ws_server.borrow();
    let ws = WebSocket::new(device_id, group_id, ws_server.clone());

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
