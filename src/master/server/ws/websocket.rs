use actix::{fut, ActorContext, ActorFuture, ActorFutureExt, ContextFutureSpawner, WrapFuture};
use actix::{Actor, Addr, Running, StreamHandler};
use actix::{AsyncContext, Handler, Recipient};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::lobby::ClientGroupWs;
use super::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebSocket {
    room: Uuid,
    cg_addr: Addr<ClientGroupWs>,
    hb: Instant,
    id: Uuid, // websocket 이용해 새로 접속하는 device의 uuid
}

impl WebSocket {
    pub fn new(device_id: Uuid, room: Uuid, lobby: Addr<ClientGroupWs>) -> Self {
        WebSocket {
            id: device_id,
            room,
            hb: Instant::now(),
            cg_addr: lobby,
        }
    }
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.cg_addr
            .send(Connect {
                addr: addr.recipient(),
                room_id: self.room,
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.cg_addr.do_send(Disconnect {
            self_id: self.id,
            room_id: self.room,
        });

        Running::Stop
    }
}

impl WebSocket {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // TODO: 일정시간 이상 ping 응답 없다는 에러 로깅
                log::info!("WebSocket 연결이 끊어졌습니다. {}", act.id);
                ctx.stop();
                return;
            }

            ctx.ping(b"ping");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => self.cg_addr.do_send(ClientActorMessage {
                id: self.id,
                msg: s.to_string(),
                room_id: self.room,
            }),
            Err(e) => std::panic::panic_any(e),
        }
    }
}

impl Handler<WsMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
