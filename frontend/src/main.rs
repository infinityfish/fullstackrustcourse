use yew::prelude::*;
use yew_router::prelude::*;


mod products;
use products::Products;
mod form;
use form::Form;

mod router;
use router::{Route, switch};

#[function_component]
fn App() -> Html {

    html! {
        <BrowserRouter>
            <div class="container">
                <h1 class="title">{"YewAxum ProductsApp"} </h1>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
