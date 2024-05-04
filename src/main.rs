use leptos::component;
use leptos::*;
use leptos::{create_signal, view, IntoView};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/>})
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = Signal::derive(move || count.get() * 2);
    let values = vec![10, 11, 12];
    let (name, set_name) = create_signal("Controlled".to_string());
    let (value, set_value) = create_signal("B".to_string());
    let (radius, set_radius) = create_signal(60.0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n|*n += 1);
            }
            class:red=move || count.get() % 2 == 1
        >
            "Click me: "
            {move || count.get()}
        </button>
        // Progress bar components
        <p>{"Progress Bar Components"}</p>
        <div>
            <ProgressBar progress=count max = 30/>
        </div>
        <div>
            <ProgressBar progress=double_count max = 100/>
        </div>
        // ----------------
        <p>{"Static List (vec)"}</p>
        // or we can wrap them in <li>
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect_view()}
        </ul>
        // ------------------
        <p>{"Dynamic List (for)"}</p>
        <DynamicList initial_length=5/>
        // -------- TEXT INPUT ------------
        <input type="text"
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name.set(event_target_value(&ev));
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=name
        />
        <p>"Name is: " {name}</p>
        // ------ SELECT ------------
        <select on:change=move |ev| {
            let new_value = event_target_value(&ev);
            set_value.set(new_value);
        }>
            <SelectOption value is="A"/>
            <SelectOption value is="B"/>
            <SelectOption value is="C"/>
        </select>
        // ------- SVG --------

        <svg width="500" height="300"
            viewBox="0 0 500 300"
            xmlns="http://www.w3.org/2000/svg"
            >
            <rect fill = "lightyellow" width="500" height="300" />
           //<circle stroke="red" fill="none" cx="50" cy="50" r="10" />
           // <line x1="0" y1="80" x2="100" y2="20" stroke="black" stroke-width = "1"/>
            <Circle x=30.0 y=40.0 r= radius/>
            <path d="M 100 100 L 300 100 L 200 300 z"
            fill="red" stroke="blue" stroke-width="3" />
            // <foreignObject  x="20" y="20" width="160" height="160">
            //     <input type="text"/>
            //     <p>"Name is: Adam"</p>
            // </foreignObject>
        </svg>
    }
}

#[component]
fn Circle(x: f32, y: f32, r: ReadSignal<f32>) -> impl IntoView {
    //let cx = x.to_string();
    let cx = format!("{}", x);
    let cy = y.to_string();
    let r = r.get().to_string();
    view! {
       <circle stroke="red" fill="none" cx=cx cy=cy r=r/>
    }
}
#[component]
fn ProgressBar(
    #[prop(into)] progress: Signal<i32>,
    #[prop(default = 100)] max: u16,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option
            value=is
            selected=move || value.get() == is
        >
            {is}
        </option>
    }
}

#[component]
fn DynamicList(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each= move|| {counters.get()}
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, (signal, _))| {
                                                // NOTE: in this example, we are creating the signals
                                                // in the scope of the parent. This means the memory used to
                                                // store them will not be reclaimed until the parent component
                                                // is unmounted. Here, we're removing the signal early (i.e, before
                                                // the DynamicList is unmounted), so we manually dispose of the signal
                                                // to avoid leaking memory.
                                                //
                                                // This is only necessary in an example with nested signals like this one.
                                                if counter_id == &id {
                                                    signal.dispose();
                                                }
                                                counter_id != &id
                                            })
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
