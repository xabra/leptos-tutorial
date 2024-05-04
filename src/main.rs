use std::f32::consts::PI;

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
            <Star x=250.0 y=250.0 r=100.0 n=300 />
        </svg>
    }
}

#[component]
fn Circle(x: f32, y: f32, r: ReadSignal<f32>) -> impl IntoView {
    view! {
       <circle stroke="red" fill="none" cx={x} cy={y} r={r}/>
    }
}

#[component]
fn Star(x: f32, y: f32, r: f32, n: i32) -> impl IntoView {
    let p = x + r;
    let lines = 0..n;
    {
        lines
            .into_iter()
            .map(|i| {
                let theta = 2.0 * PI * (i as f32) / (n as f32);
                let dx = r * theta.cos();
                let dy = r * theta.sin();
                view! { <line stroke="blue" x1={x} y1={y} x2={x+dx} y2={y+dy} />}
            })
            .collect_view()
    }
}
