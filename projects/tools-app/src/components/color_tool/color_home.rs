use leptos::*;

use crate::components::color_tool::*;
use crate::repositories::color as color_repository;

#[component]
pub fn ColorToolHome() -> impl IntoView {
    let append_color = create_server_action::<color_repository::AppendColor>();
    let remove_color = create_server_action::<color_repository::RemoveColor>();

    let color_resource = create_resource(
        move || (append_color.version().get(), remove_color.version().get()),
        |_| color_repository::all_colors(),
    );

    view! {
        <div>
            <header>
                <h2>"Color Tool"</h2>
                <p>"Welcome to the color tool!"</p>
            </header>
            <div>
                <ColorList color_resource=color_resource remove_color=remove_color />
                <ColorForm append_color=append_color />
            </div>
        </div>
    }
}
