use leptos::component;
use leptos::*;
use leptos::{create_signal, view, IntoView};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/>})
}

#[component]
fn App() -> impl IntoView {
    let (radius, set_radius) = create_signal(20.0);

    view! {
        <div>
        <button
            on:click=move |_| {
                set_radius.update(|r| *r += 5.0);
            }
        >
            "Click me: "
            {move || radius.get()}
        </button>
        </div>

        // ------- SVG --------
        <svg width="500" height="500"
            viewBox="0 0 500 500"
            xmlns="http://www.w3.org/2000/svg"
            >
            <rect fill = "lightyellow" width="500" height="500" />
            <Circle x=250.0 y=250.0 r=radius/>
        </svg>
    }
}

#[component]
fn Circle(x: f32, y: f32, r: ReadSignal<f32>) -> impl IntoView {
    //let cx = x.to_string();
    let cx = format!("{}", x);
    let cy = y.to_string();
    view! {
       <circle stroke="red" fill="none" cx=cx cy=cy r={r}/>
    }
}
