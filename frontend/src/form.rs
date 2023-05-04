use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use serde_json::json;

use crate::router::Route;


#[function_component]
pub fn Form() -> Html {

    let navigator = use_navigator().unwrap();

    //name
    let name_ref = NodeRef::default();
    let name_ref_outer = name_ref.clone();

    //price
    let price_ref = NodeRef::default();
    let price_ref_outer = price_ref.clone();

    //submit form data
    let onclick = Callback::from(move |_| {
        // gloo_console::log!("Button Clicked");
        let price = price_ref.cast::<HtmlInputElement>().unwrap();
        let name = name_ref.cast::<HtmlInputElement>().unwrap();
        // gloo_console::log!(name.value());

        wasm_bindgen_futures::spawn_local(async move {
            let product = json!({
                "name": name.value(),
                "price": price.value().parse::<i32>().unwrap()
            });
    
            let client = reqwest::Client::new();
            let res = client.post("http://localhost:3000/api/products")
                .json(&product)
                .send()
                .await;
        });
        navigator.push(&Route::Home)
    });

    html! {
        <div class="container">
            <h2>{"Add a Product"} </h2>
            <div>
                <label for="name" class="">
                    {"Name"}
                    <input ref={name_ref_outer.clone()}
                        id="name"
                        class="formInput"
                        type="text"
                    />
                </label> <br /> <br />
                <label for="price" class="">
                {"Price"}
                <input ref={price_ref_outer.clone()}
                    id="price"
                    class="formInput"
                    type="number"
                />
            </label> <br /><br />
            <button {onclick} class="btn-primary">{"Add Product"} </button>
            </div>
            <hr />
        </div>
    }
}