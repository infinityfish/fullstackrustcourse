use yew_router::prelude::*;
use yew::prelude::*;

use crate::products::Products;
use crate::form::Form;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/addproduct")]
    AddProduct,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Products /> },
        Route::AddProduct => html! { <Form />},
        Route::About => html! { <h1>{ "About us " }</h1>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}