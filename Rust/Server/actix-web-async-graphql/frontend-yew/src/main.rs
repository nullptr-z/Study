use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;

mod pages;
use pages::{home::Home, projects::Projects, users::Users};

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum Route {
    #[to = "/users"]
    Users,
    #[to = "/projects"]
    Projects,
    #[to = "/"]
    Home,
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        let home_cls = "nav";

        fn switch(switch: Route) -> Html {
            match switch {
                Route::Users => {
                    html! { <Users/> }
                }
                Route::Projects => {
                    html! { <Projects/> }
                }
                Route::Home => {
                    html! { <Home /> }
                }
            }
        }

        html! {
            <>
            <div class="logo-title">
                { "tide-async-graphql-mongodb / frontend-yew" }
            </div>
            <div class=home_cls>
                <Anchor route=Route::Users>
                    { "用户列表" }
                </Anchor>
                { " - " }
                <Anchor route=Route::Projects>
                    { "项目列表" }
                </Anchor>
                { " - " }
                <Anchor route=Route::Home>
                    { "主页" }
                </Anchor>
            </div>
            <main> <Router<Route> render=Router::render(switch) /></main>
            </>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

fn main() {
    yew::start_app::<App>();
}
