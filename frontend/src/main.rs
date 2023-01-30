use yew::prelude::*;

mod products;
use products::Products;
mod form;
use form::Form;
// mod formshort;
// use formshort::FormShort;

#[function_component]
fn App() -> Html {


    html! {
        <div class="container">
            <h1 class="title">{"Yew ProductsApp"} </h1>
            <Form />
            <Products />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
