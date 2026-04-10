use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home,

    #[route("/about")]
    About,

    #[route("/user/:id")]
    User { id: u32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn Layout(children: Element) -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        {children}
    }
}
#[component]
fn Home() -> Element {
    rsx! {
        Layout{
            div {
                "Welcome to the home page!"
                Link { to: Route::About, "Go to About Page" }
            }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! { "This is the about page." }
}

#[component]
fn User(id: u32) -> Element {
    rsx! { "User page for user with id: {id}" }
}
