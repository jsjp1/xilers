use super::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::super::server::ClientGroup;

type Socket = Recipient<WsMessage>;

pub struct ClientGroupWs {
    // 각 map에 사용하는 Key는 DeviceManager의 Uuid와 Device들의 Uuid와 동일
    sessions: HashMap<Uuid, Socket>, // device: socket <- Device의 Uuid
    rooms: HashMap<Uuid, HashSet<Uuid>>, // manager: device <- DeviceManager의 Uuid
}

impl ClientGroupWs {
    pub fn new() -> Self {
        ClientGroupWs {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            log::warn!("{}에 해당하는 user가 존재하지 않습니다.", id_to);
        }
    }
}

impl Actor for ClientGroupWs {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for ClientGroupWs {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        log::info!("{}에 해당하는 user가 접속을 해제했습니다.", &msg.self_id);
        if self.sessions.remove(&msg.self_id).is_some() {
            if let Some(ws_lobby) = self.rooms.get_mut(&msg.room_id) {
                if ws_lobby.len() > 1 {
                    ws_lobby.remove(&msg.self_id);
                } else {
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<Connect> for ClientGroupWs {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        log::info!(
            "{}에 해당하는 user가 {}에 해당하는 group에 접속했습니다.",
            &msg.self_id,
            &msg.room_id
        );

        self.rooms
            .entry(msg.room_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        self.sessions.insert(msg.self_id, msg.addr);

        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|client| {
                self.send_message("", client); // room에 속한 모든 device에 manager 갱신해야한다는 정보 알림
            });
    }
}

impl Handler<ClientActorMessage> for ClientGroupWs {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .for_each(|client| {
                self.send_message(&msg.msg, client);
            });
    }
}
