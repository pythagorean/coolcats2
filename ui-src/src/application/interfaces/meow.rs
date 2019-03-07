use yew::prelude::*;
use stdweb::web::Date;

use crate::utils::Dict;

pub struct Meow {
    pub post: Dict,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub post: Dict,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            post: Dict::new(),
        }
    }
}

impl Component for Meow {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Meow {
            post: props.post,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.post = props.post;
        true
    }
}

impl Meow {
    // replace https URLs and hashtags with links
    fn linkify(&self, text: &str) -> Html<Self> {
        html! {<>
            {for text.split_whitespace().map(|s| {
                if s.len() > 8
                    && s.starts_with("https://")
                {
                    html! {<>
                        <a href={s}, target="_blank",>{s}</a>
                        {' '}
                    </>}
                } else if s.len() > 1
                    && s.starts_with('#')
                    && s[1..].chars().skip_while(|c| c.is_alphabetic()).next().is_none()
                {
                    html! {<>
                        <a href={format!("/#/tag/{}", &s[1..])}, class="hashtag",>{s}</a>
                        {' '}
                    </>}
                } else {
                    html! {<>{s}{' '}</>}
                }
            })}
        </>}
    }
}

impl Renderable<Meow> for Meow {
    fn view(&self) -> Html<Self> {
        let stamp = &self.post.string("stamp");
        let message = &self.post.string("message");
        let author = &self.post.string("author");
        let address = &self.post.string("address");
        let user_handle = &self.post.string("user_handle");
        html! {<>
            <div class="meow", id={stamp},>
                <a class="meow-edit", href="#",>
                    {"edit"}
                </a>
                /*
                <a className="meow-edit" onClick={() => "openEditPost('+id+')"}>
                  edit
                </a>
                */
                <a class="user", href={format!("/#/u/{}", author)},>
                    {"@"}{user_handle}
                </a>
                {" | "}
                <a class="stamp", href={format!("/#/meow/{}", address)},>
                    { Date::from_time(stamp.parse().unwrap()).to_string() }
                </a>
                <div class="message",>{self.linkify(message)}</div>
                /*
                <FavesContainer hash={hash} />
                */
            </div>
        </>}
    }
}
