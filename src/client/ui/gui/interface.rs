use iced::{self, Application};

use std::{
    process,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use super::super::interface;
use crate::{network::tcp::network::TcpNetwork, ui::request::DeviceManager};

struct App;

#[derive(Clone)]
pub struct Gui {
    master_addr: String,
    device_manager_uuid: Uuid,
    device_uuid: Uuid,
    network: TcpNetwork,
}

impl interface::Interface for Gui {
    fn new(master_addr: String, listen_port: u16, file_storage: String) -> Self {
        Gui {
            master_addr,
            device_manager_uuid: Uuid::nil(),
            device_uuid: Uuid::nil(),
            network: TcpNetwork::new(listen_port, file_storage),
        }
    }
    async fn entry(&mut self) {
        let _ = App::run(iced::Settings::default());
    }
    async fn exit(&self, error_opt: Option<String>) -> ! {
        process::exit(-1);
    }
    async fn render(&self, device_manager: Arc<Mutex<DeviceManager>>) {}
    async fn register_device_fs(&self, manager_uuid: Uuid) -> Uuid {
        Uuid::nil()
    }
    async fn register_device_spec(&self, manager_uuid: Uuid) -> Uuid {
        Uuid::nil()
    }
    async fn enter_group(&mut self) {}
    async fn create_group(&mut self) {}
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (App, iced::Command<Self::Message>) {
        (App, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Xilers")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        "Hello, world!".into()
    }
}
