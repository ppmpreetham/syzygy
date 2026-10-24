use gpui_kit::base::input;
use gpui_kit::component::Icon;
use gpui_kit::*;
use zopra::components::webview::WebView;
use zopra::component;
use zopra::hooks::{use_event, use_input, use_webview};

use crate::backend::proxy_config;

struct BrowserWindow;

impl Render for BrowserWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        browser_root(window, cx)
    }
}

fn to_url(text: String) -> String {
    if text.starts_with("http") || text.starts_with("file://") {
        text
    } else {
        format!("https://{text}")
    }
}

#[component]
fn browser_root() {
    let webview = use_webview();
    let input_state = use_input();

    // Handle Enter key for navigation
    use_event(&input_state, |event, cx| {
        if let input::InputEvent::PressEnter { .. } = event {
            webview.load_url(&to_url(input_state.read(cx).text().to_string()), cx);
        }
    });

    zopra::view! {
        <div class="flex flex-col size-full p-2 gap-2 ">
            <div class="flex flex-row gap-2 items-center px-2 py-1 rounded shadow-lg bg-[#18181b]">
                <button
                    id="win-btn-back"
                    on_click={|_, _, cx| webview.back(cx)}
                    class="bg-[#27272a] text-white px-3 py-1 rounded w-fit"
                    icon={Icon::default().path("icons/browser/caret-left.svg")}
                />
                <button
                    id="win-btn-forward"
                    on_click={|_, _, cx| webview.forward(cx)}
                    class="bg-[#27272a] text-white px-3 py-1 rounded w-fit"
                    icon={Icon::default().path("icons/browser/caret-right.svg")}
                />
                <button
                    id="win-btn-reload"
                    on_click={|_, _, cx| webview.reload(cx)}
                    class="bg-[#27272a] text-white px-3 py-1 rounded w-fit"
                    icon={Icon::default().path("icons/browser/refresh.svg")}
                />

                <input state={&input_state} class="text-white py-2"/>

                <button id="win-btn-go" on_click={|_, _, cx| {
                    webview.load_url(&to_url(input_state.read(cx).text().to_string()), cx);
                }} class="bg-blue-600 text-white px-4 py-1 rounded">"Go"</button>
            </div>
            <div class="size-full flex-1 relative rounded overflow-hidden">
                <WebView
                    url="https://www.google.com"
                    controller={webview}
                    transparent={true}
                    devtools={true}
                    proxy={proxy_config()}
                />
            </div>
        </div>
    }
}

pub(super) fn open_browser(window: &mut Window, cx: &mut App) {
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(1000.), px(800.)),
            cx,
        ))),
        window_background: WindowBackgroundAppearance::Blurred,
        ..Default::default()
    };

    cx.open_window(options, |window, cx| {
        window.activate_window();
        cx.new(|_cx| BrowserWindow)
    })
    .expect("failed to open browser window");
}
