///! Example of echarts formatter functions using wasm/charming

#[cfg(feature = "hydrate")]
use charming::{
    component::{Axis, Title},
    element::{AxisPointer, AxisPointerType, AxisType, Formatter, Label, Tooltip, Trigger},
    renderer::wasm_renderer::WasmRenderer,
    series::Line,
    Chart,
};
#[cfg(feature = "hydrate")]
use js_sys::Function;

use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-wasm-formatter-functions.css"/>

        // ECharts inject
        <Script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js" />

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Chart/>
    }
}

#[component]
fn Chart() -> impl IntoView {
    // Render on client Only, Feature flag is needed becouse our dependencies are also gated.
    #[cfg(feature = "hydrate")]
    spawn_local(async move {
        let tooltip = Formatter::Function(js_sys::Function::new_with_args(
            "arg1, arg2, arg3",
            r#"
                let value = arg1;
                console.log(value);
                return `${value[0].data} Charming`;
            "#,
        ));

        let chart = Chart::new()
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
            )
            .tooltip(
                Tooltip::new()
                    .formatter(tooltip.into())
                    .trigger(Trigger::Axis)
                    .axis_pointer(
                        AxisPointer::new().type_(AxisPointerType::Shadow).label(
                            Label::new()
                                .background_color("#ccc")
                                .border_color("#aaa")
                                .border_width(1)
                                .shadow_blur(0)
                                .shadow_offset_x(0)
                                .shadow_offset_y(0)
                                .color("#222"),
                        ),
                    ),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

        // Charming, Requires we set a width & Height
        let renderer = WasmRenderer::new(600, 600);
        let result = renderer.render("main", &chart);
    });

    // Chart Container, it's rendered on both server and client.
    view! {<div id="main" class="chart"></div>}
}
