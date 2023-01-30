use yew::prelude::*;
use web_sys::HtmlInputElement;
use serde_json::json;

#[function_component]
pub fn FormShort() -> Html {

    //name field
    let name_ref = NodeRef::default();
    let name_ref_outer = name_ref.clone();

    //price field
    let price_ref = NodeRef::default();
    let price_ref_outer = price_ref.clone();
    

    //submit form data
    let onclick = Callback::from( move |_| {
        let price = price_ref.cast::<HtmlInputElement>().unwrap();
        let name = name_ref.cast::<HtmlInputElement>().unwrap();
        
        gloo_console::log!("Name: ", &name.value(), "\nprice: ", &price.value() );


        //https://docs.rs/reqwest/latest/reqwest/
        wasm_bindgen_futures::spawn_local(async move {

           let product = json!({
                "name": name.value(),
                "price": price.value().parse::<i32>().unwrap()
            });  
            let client = reqwest::Client::new();
            let _res = client.post("http://localhost:3000/api/products")
                .json(&product)
                .send()
                .await;
        })      
    });

    html! {
        <>
                <label for="name" class="font20">
                    { "My name is:" }
                    <input ref={name_ref_outer.clone()}
                        id="name"
                        type="text"    
                        class="formInput"         
                    />
                </label>  <br /> <br />

                <label for="price" class="font20">
                { "My price:" }
                <input ref={price_ref_outer.clone() }
                    id="price"
                    type="number"
                    class="formInput"
                />
            </label>  <br /> <br />

            <button {onclick} class="btn-primary font16">{ "Add Contact" }</button>

           
        </>
        
    }
}