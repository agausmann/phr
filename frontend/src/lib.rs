pub mod api;
pub mod routes;

use routes::{Index, Racer, Racers};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Model {}

pub enum Msg {}

#[derive(Debug, Clone, Switch)]
pub enum Route {
    #[to = "/"]
    Index,
    #[to = "/racers/{id}"]
    Racer(i32),
    #[to = "/racers/"]
    Racers,
}

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
            <Router<Route, ()>
                render = Router::render(|switch: Route| {
                    match switch {
                        Route::Index => html!(<Index />),
                        Route::Racer(id) => html!(<Racer id=id />),
                        Route::Racers => html!(<Racers />),
                    }
                })
            />
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::start_app::<Model>()
}
