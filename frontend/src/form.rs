

use yew::prelude::*;
use web_sys::HtmlInputElement;

#[function_component]
pub fn Form() -> Html {

    //name
    let name_ref = NodeRef::default();
    let name_ref_outer = name_ref.clone();

    //price
    let price_ref = NodeRef::default();
    let price_ref_outer = price_ref.clone();

    //submit form data
    let onclick = Callback::from( move |_| {
        let name = name_ref.cast::<HtmlInputElement>().unwrap();
        let price = price_ref.cast::<HtmlInputElement>().unwrap();

        gloo_console::log!(name.value());


    });


    html! {
        <div class="container">
            <h2>{"Add a Product"} </h2>
            <div>
                <label for="name" class="">
                    {"Name"}
                    <input ref={name_ref_outer.clone()}
                        id="name"
                        class=""
                        type="text"
                    />
                </label> <br /> <br />
                <label for="price" class="">
                {"Price"}
                <input ref={price_ref_outer.clone()}
                    id="price"
                    class=""
                    type="number"
                />
            </label> <br /><br />
            <button {onclick} class="">{"Add Product"} </button>
            </div>
            <hr />
        </div>
    }
}