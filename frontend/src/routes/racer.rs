use yew::prelude::*;

pub struct Racer {
    props: Props,
}

pub enum Msg {}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub id: i32,
}

impl Component for Racer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Racer { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // TODO use neq_assign (https://github.com/yewstack/yew/issues/1233)
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div />
        }
    }
}
