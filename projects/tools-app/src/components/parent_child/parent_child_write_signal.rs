use leptos::*;

#[component]
pub fn ParentChildWriteSignal() -> impl IntoView {
    let (counter, set_counter) = create_signal::<i32>(0);

    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px; padding: 4px; text-align: center;">
                <h3>"Parent Write Signal"</h3>
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
            <Child counter=counter set_counter=set_counter />
        </div>
    }
}

#[component]
fn Child(counter: ReadSignal<i32>, set_counter: WriteSignal<i32>) -> impl IntoView {
    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div style="border: 1px solid black; margin: 4px; padding: 4px; text-align: center;">
            <h3>"Child Write Signal"</h3>
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
    }
}
