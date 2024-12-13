use leptos::*;
use leptos_router::ActionForm;

use crate::models::color::Color as ColorModel;
use crate::repositories::color as color_repository;

#[component]
pub fn ColorForm(
    append_color: Action<color_repository::AppendColor, Result<ColorModel, ServerFnError>>,
) -> impl IntoView {
    view! { 
        <ActionForm action=append_color>
            <label>
                "Name:"
                <input type="text" name="name" />
            </label>
            <label>
                "Hex code:"
                <input type="text" name="hex_code" />
            </label>
            <button type="submit">"Add Color"</button>
        </ActionForm> 
    }
}
