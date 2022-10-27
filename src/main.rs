use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)} class="pointer-events-auto ml-8 rounded-md bg-primary-400 py-2 px-3 text-[0.8125rem] font-bold leading-5 hover:text-gray hover:bg-secondary-700">{ "+1" }</button>
                <p class="ml-8  py-2">{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}