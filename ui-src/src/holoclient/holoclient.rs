use yew::prelude::*;

use serde::Deserialize;

use crate::application::ToApplication;

use super::{
    websocket::{
        WebSocketService,
        WebSocketStatus,
    },
    ws_rpc::{
        self,
        WsRpc,
    },
};

const HOLOCHAIN_SERVER: &str = "ws://localhost:8888";

pub struct Holoclient {
    websocket: Option<WebSocketService>,
    link: ComponentLink<Holoclient>,
    callback: Option<Callback<ToApplication>>,
    rpc_id: u32,
}

#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: String,
}

pub enum WsAction {
    Connect,
    Initialize,
    Call(WsRpc),
    Lost,
}

pub enum Msg {
    Callback(ToApplication),
    WsAction(WsAction),
    WsReady(String),
}

impl From<ToApplication> for Msg {
    fn from(msg: ToApplication) -> Self {
        Msg::Callback(msg)
    }
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

#[derive(PartialEq, Clone)]
pub struct Params {
    rpc: ws_rpc::Call,
    redux: String,
    meta: String,
}

impl From<(&str, &str)> for Params {
    fn from(args: (&str, &str)) -> Self {
        let rpc: ws_rpc::Call = args.0.into();
        let redux = args.1.into();
        Params {
            rpc,
            redux,
            meta: String::new(),
        }
    }
}

impl From<(&[&str], &[(&str, &str)], &str)> for Params {
    fn from(args: (&[&str], &[(&str, &str)], &str)) -> Self {
        let rpc: ws_rpc::Call = (args.0, args.1).into();
        let redux = args.2.into();
        Params {
            rpc,
            redux,
            meta: String::new(),
        }
    }
}

impl From<(&[&str], &[(&str, &str)], &str, &str)> for Params {
    fn from(args: (&[&str], &[(&str, &str)], &str, &str)) -> Self {
        let rpc: ws_rpc::Call = (args.0, args.1).into();
        let redux = args.2.into();
        let meta = args.3.into();
        Params {
            rpc,
            redux,
            meta,
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Params::new()
    }
}

impl Params {
    pub fn new() -> Self {
        Params {
            rpc: ws_rpc::Call::new(),
            redux: String::new(),
            meta: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.rpc.clear();
        self.redux.clear();
        self.meta.clear();
    }

    pub fn has_method(&self) -> bool {
        self.rpc.has_method()
    }

    pub fn has_redux(&self) -> bool {
        !self.redux.is_empty()
    }
}

pub enum ToHoloclient {
    Call(Params),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub params: Params,
    pub callback: Option<Callback<ToApplication>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            params: Params::new(),
            callback: None,
        }
    }
}

impl Component for Holoclient {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut holoclient = Holoclient {
            websocket: None,
            link,
            callback: props.callback,
            rpc_id: 0,
        };
        holoclient.update(WsAction::Connect.into());
        holoclient
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Callback(msg) => {
                if let Some(ref mut callback) = self.callback {
                    callback.emit(msg);
                }
            }

            Msg::WsReady(response) => {
                let response = &json::parse(&response).unwrap();
                let result = response["result"].to_string();
                let id = response["id"].to_string();
                let v: Vec<&str> = id.split(',').collect();
                let redux = v[0].to_string();
                let meta = if v.len() > 1 {
                    v[1]
                } else {
                    ""
                }
                .to_string();
                self.update(ToApplication::Redux(result, redux, meta).into());
            }

            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    if self.websocket.is_some() {
                        return false;
                    }
                    let callback = self.link.send_back(Msg::WsReady);
                    let notification = self.link.send_back(|status| match status {
                        WebSocketStatus::Opened => WsAction::Initialize.into(),
                        WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                    });
                    let service = WebSocketService::new(HOLOCHAIN_SERVER, callback, notification);
                    self.websocket = Some(service);
                }

                WsAction::Initialize => {
                    self.update(Msg::Callback(ToApplication::Initialize));
                }

                WsAction::Call(rpc) => {
                    self.websocket.as_mut().unwrap().send(&rpc.json());
                }

                WsAction::Lost => {
                    self.websocket = None;
                }
            },
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let call = props.params;
        if call.has_method() {
            let rpc: WsRpc = if call.has_redux() {
                let rpc_id = format!("{},{}", call.redux, call.meta);
                (call.rpc, rpc_id).into()
            } else {
                self.rpc_id += 1;
                (call.rpc, self.rpc_id).into()
            };
            self.update(WsAction::Call(rpc).into());
        }
        false
    }
}

impl Renderable<Holoclient> for Holoclient {
    fn view(&self) -> Html<Self> {
        html! { <></> }
    }
}
