use akhsakov_machine::Route;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| rsx! { Router::<Route> {} });
}
