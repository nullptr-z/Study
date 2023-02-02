use yew::prelude::*;

pub struct Home {
  link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Home {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { link }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {}
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let home_cls = "home";
    html! {
      <div  class=classes!(home_cls)>
        {"主页"}
      </div>
    }
  }
}
