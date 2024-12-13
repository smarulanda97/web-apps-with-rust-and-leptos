use leptos::*;
use leptos_router::ActionForm;

use crate::components::error_fallback::error_fallback;
use crate::models::color::Color as ColorModel;
use crate::repositories::color as color_repository;

#[component]
fn ColorLisItem(color: ColorModel, remove_color: Action<color_repository::RemoveColor, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <li>
            <ActionForm action=remove_color>
                <input type="hidden" name="id" value=color.id />
                {color.name}
                -
                {color.hex_code}
                <button type="submit">"X"</button>
            </ActionForm>
        </li>
    }
}

#[component]
pub fn ColorList(
    color_resource: Resource<(usize, usize), Result<Vec<ColorModel>, ServerFnError>>,
    remove_color: Action<color_repository::RemoveColor, Result<(), ServerFnError>>,
) -> impl IntoView {
    let color_list_view = move|| -> Option<Result<View, _>> {
        color_resource.and_then(|colors: &Vec<ColorModel>| {
            colors.iter().map(|color| 
                view! { <ColorLisItem color=color.clone() remove_color=remove_color /> }
            ).collect_view()
        })
    };

    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=error_fallback()>
                <ul>{color_list_view}</ul>
            </ErrorBoundary>
        </Transition>
    }
}
