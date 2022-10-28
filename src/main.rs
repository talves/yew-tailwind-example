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
            <div class="min-h-screen">
            <div class="mx-auto h-4/5 w-auto px-4 pt-16 pb-8 sm:pt-24 lg:px-8">
            <h1 class="mx-auto max-w-5xl text-center text-6xl font-extrabold leading-[1.1] text-gray-800 dark:text-gray-900 tracking-tighter text-white sm:text-7xl lg:text-8xl xl:text-8xl">
              <span class="inline-block bg-gradient-to-r from-primary-500 to-secondary-500 bg-clip-text text-transparent">
              {"Yew & Tailwind Example"}
              </span>
            </h1>
            <div class="mx-auto mt-5 max-w-xl sm:flex sm:justify-center md:mt-8">
                <div>
                    <button onclick={link.callback(|_| Msg::AddOne)} class="pointer-events-auto ml-8 rounded-md bg-primary-400 py-2 px-3 text-[0.8125rem] font-bold leading-5 hover:text-gray hover:bg-secondary-700">{ "+1" }</button>
                    <p class="ml-8  py-2">{ self.value }</p>
                </div>
            </div>
            </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}