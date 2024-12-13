use ev::MouseEvent;
use leptos::*;

#[component]
pub fn ParentChildClosure() -> impl IntoView {
    let (counter, set_counter) = create_signal::<i32>(0);
    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px; padding: 4px; text-align: center;">
                <h3>"Parent Child Closure"</h3>
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
            <Child counter=counter on_decrement=decrement_counter on_increment=increment_counter />
        </div>
    }
}

#[component]
fn Child<I, D>(counter: ReadSignal<i32>, on_increment: I, on_decrement: D) -> impl IntoView
where
    I: Fn(MouseEvent) + 'static,
    D: Fn(MouseEvent) + 'static,
{
    view! {
        <div style="border: 1px solid black; margin: 4px; padding: 4px; text-align: center;">
            <h3>"Child Closure"</h3>
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
