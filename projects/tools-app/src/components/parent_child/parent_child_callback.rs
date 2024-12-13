use ev::MouseEvent;
use leptos::*;

#[component]
pub fn ParentChildCallback() -> impl IntoView {
    let (counter, set_counter) = create_signal::<i32>(0);
    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px; padding: 4px; text-align: center;">
                <h3>"Parent Child Callback"</h3>
                <p>"Counter:" {counter}</p>

                <div>
                    <button type="button" on:click=increment_counter>
                        "Increment +1"
                    </button>
                    <button type="button" on:click=decrement_counter>
                        "Decrement -1"
                    </button>
                </div>
            </div>
            <Child counter=counter on_decrement=decrement_counter on_increment=decrement_counter />
        </div>
    }
}

#[component]
fn Child(
    counter: ReadSignal<i32>,
    #[prop(into)] on_increment: Callback<MouseEvent>,
    #[prop(into)] on_decrement: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <div style="border: 1px solid black; margin: 4px; padding: 4px; text-align: center;">
            <h3>"Child Callback"</h3>
            <p>"Counter:" {counter}</p>

            <div>
                <button type="button" on:click=on_increment>
                    "Increment +1"
                </button>
                <button type="button" on:click=on_decrement>
                    "Decrement -1"
                </button>
            </div>
        </div>
    }
}
