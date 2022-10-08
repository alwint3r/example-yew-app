use web_sys::HtmlInputElement;
use yew::{NodeRef, Component, html, Properties, Children};


struct MyApp {
}

impl Component for MyApp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <Container>
                <h1>{"Hello Yew"}</h1>
                <p>{"The following input element will focus upon the first render"}</p>
                <MyComponent />
            </Container>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct ContainerProps {
    children: Children,
}

struct Container {
}

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div>
                { ctx.props().children.clone() }
            </div>
        }
    }
}


struct MyComponent {
    node_ref: NodeRef,
}

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! (
            <input ref={self.node_ref.clone()} type="text" />
        )
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                input.focus().unwrap();
            }
        }
    }
}

fn main() {
    yew::start_app::<MyApp>();
}
