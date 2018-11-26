#[macro_use]
extern crate yew;
use yew::prelude::*;

mod components;
use self::components::modal::{BACKDROP_STYLE, MODAL_STYLE};

pub struct SettingsContainer {}

impl Component for SettingsContainer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SettingsContainer {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<SettingsContainer> for SettingsContainer {
    fn view(&self) -> Html<Self> {
        html! { </> }
    }
}

const DEFAULT_PROFILE_PIC: &str = "/cat-eating-bird-circle.png";

pub struct AppModel {
    modal_is_open: bool,
    something: u8,
}

pub enum Msg {}

impl Component for AppModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        AppModel {
            modal_is_open: true,
            something: 1,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<AppModel> for AppModel {
    fn view(&self) -> Html<Self> {
        let modal_is_open = self.modal_is_open;
        let profile_pic = "";
        match modal_is_open {
            true => html! {
                <div style={BACKDROP_STYLE},>
                    <div style={MODAL_STYLE},>
                        <div align="center",>
                            <p classname="h1",>{"Welcome to Coolcats2!"}</p>
                        </div>
                        // <SettingsContainer />
                    </div>
                </div>
            }, _ => html! {
                <div className="container",>
                    <div className="spinner transition500",/>
                    <div className="error transition500",/>
                    <div className="row first",>
                        <div className="fixed-area",>
                            <div className="col-sm-2 contentcontainer",>
                                <div className="logo",>
                                    <img
                                        src={
                                            if !profile_pic.is_empty() {profile_pic}
                                            else {DEFAULT_PROFILE_PIC}
                                        },
                                        alt="user-profile",
                                    />
                                </div>
                                <div id="displayName",>
                                    {&format!("Something: {}", self.something)}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
