use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod api;

pub struct Model {}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div />
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::start_app::<Model>()
}
