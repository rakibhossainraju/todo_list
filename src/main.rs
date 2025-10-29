mod components;
mod models;
mod storage;
mod todo_list;

use dioxus::prelude::*;
use todo_list::TodoList;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const CSS: Asset = asset!("./src/style.css");

fn main() {
    #[cfg(feature = "desktop")]
    {
        use dioxus_desktop::{wry::dpi::PhysicalSize, Config, WindowBuilder};
        const MIN_WINDOW_SIZE: (u32, u32) = (600, 700);

        let desktop_config = Config::new().with_window(
            WindowBuilder::new()
                .with_always_on_top(false)
                .with_title("Todo List")
                .with_min_inner_size(PhysicalSize::new(MIN_WINDOW_SIZE.0, MIN_WINDOW_SIZE.1))
                .with_inner_size(PhysicalSize::new(MIN_WINDOW_SIZE.0, MIN_WINDOW_SIZE.1))
                .with_resizable(true)
                .with_decorations(true),
        );
        dioxus_desktop::launch::launch_virtual_dom(VirtualDom::new(App), desktop_config);
    }

    #[cfg(feature = "web")]
    {
        dioxus::launch(App);
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: CSS }
        TodoList {}
    }
}
