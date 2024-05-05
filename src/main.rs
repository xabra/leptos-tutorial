use std::f32::consts::PI;

use leptos::component;
use leptos::*;
use leptos::{create_signal, logging, view, IntoView};

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
                logging::log!("Value: {}", radius.get());
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
            on:mousemove=move |_| {
                logging::log!("Value: {}", 3);
            }
            <rect fill = "lightyellow" width="500" height="500" />
            <Circle x=250.0 y=250.0 r=radius/>
            <Star x=250.0 y=250.0 r=100.0 n=300 />
            <ArrowLine x1=30.0 y1 = 30.0 x2 = 250.0 y2 = 30.0 line_width = 1.0 head_width = 12.0 head_length = 20.0 barb_angle_deg=40.0 />
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
// Just a test component to draw a lot of lines
fn Star(x: f32, y: f32, r: f32, n: i32) -> impl IntoView {
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
#[component]
fn ArrowLine(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    line_width: f32,
    head_width: f32,
    head_length: f32,
    barb_angle_deg: f32,
) -> impl IntoView {
    let line_length = f32::sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1));
    let line_angle = (y2 - y1).atan2(x2 - x1) * 180.0 / PI;
    let head_half_width = head_width / 2.0;
    let line_half_width = line_width / 2.0;
    let barb_width = (head_width - line_width) / 2.0;
    let barb_offset = barb_width * (barb_angle_deg * PI / 180.0).sin();

    // SHADOWING...
    let x2 = x1 + line_length;
    let y2 = y1;

    let pline_str = format!(
        "{},{} {},{} {},{} {},{} {},{} {},{} {},{} {},{} {},{} {},{} {},{}",
        x1,
        y1,
        x1 + head_length,
        y1 - head_half_width,
        x1 + head_length - barb_offset,
        y1 - line_half_width,
        x2 - head_length + barb_offset,
        y2 - line_half_width,
        x2 - head_length,
        y2 - head_half_width,
        x2,
        y2,
        x2 - head_length,
        y2 + head_half_width,
        x2 - head_length + barb_offset,
        y2 + line_half_width,
        x1 + head_length - barb_offset,
        y1 + line_half_width,
        x1 + head_length,
        y1 + head_half_width,
        x1,
        y1
    );
    let transform_str = format!("rotate({} {} {})", line_angle, 0, 0);
    view! {
        <polyline points={pline_str} fill="black" stroke="none" transform={transform_str}/>
    }
}
