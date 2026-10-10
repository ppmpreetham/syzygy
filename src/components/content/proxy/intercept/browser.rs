use gpui_kit::base::input;
use gpui_kit::component::Icon;
use gpui_kit::*;
use zopra::components::webview::WebView;
use zopra::hooks::{use_event, use_input};
use zopra::{WebViewController, component};

use crate::backend::proxy_config;

struct BrowserWindow {
    webview_ctrl: WebViewController,
}

impl Render for BrowserWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        zopra::view! {
          <BrowserRoot
            webview_ctrl={self.webview_ctrl.clone()}
          />
        }
    }
}

#[component]
fn browser_root(webview_ctrl: WebViewController) {
    let ctrl = webview_ctrl;
    let input_state = use_input(window, cx);

    // Handle Enter key for navigation
    use_event(
        &input_state,
        {
            let ctrl = ctrl.clone();
            let input_state = input_state.clone();
            move |event, cx| {
                if let input::InputEvent::PressEnter { .. } = event {
                    let text = input_state.read(cx).text().to_string();
                    let url = if !text.starts_with("http") && !text.starts_with("file://") {
                        format!("https://{text}")
                    } else {
                        text
                    };
                    ctrl.load_url(&url, cx);
                }
            }
        },
        cx,
    );

    zopra::view! {
        <div class="flex flex-col size-full p-2 gap-2 ">
            <div class="flex flex-row gap-2 items-center px-2 py-1 rounded shadow-lg bg-[#18181b]">
                <button
                    id="win-btn-back"
                    on_click={{ let c = ctrl.clone(); move |_, _, cx| c.back(cx) }}
                    class="bg-[#27272a] text-white px-3 py-1 rounded w-fit"
                    icon={Icon::default().path("icons/browser/caret-left.svg")}
                />
                <button
                    id="win-btn-forward"
                    on_click={{ let c = ctrl.clone(); move |_, _, cx| c.forward(cx) }}
                    class="bg-[#27272a] text-white px-3 py-1 rounded w-fit"
                    icon={Icon::default().path("icons/browser/caret-right.svg")}
                />
                <button
                    id="win-btn-reload"
                    on_click={{ let c = ctrl.clone(); move |_, _, cx| c.reload(cx) }}
                    class="bg-[#27272a] text-white px-3 py-1 rounded w-fit"
                    icon={Icon::default().path("icons/browser/refresh.svg")}
                />

                <input state={&input_state} class="text-white py-2"/>

                <button id="win-btn-go" on_click={{
                    let c = ctrl.clone();
                    let input = input_state.clone();
                    move |_, _, cx| {
                        let text = input.read(cx).text().to_string();
                        let url = if text.starts_with("http") {
                            text
                        } else {
                            format!("https://{text}")
                        };
                        c.load_url(&url, cx);
                    }
                }} class="bg-blue-600 text-white px-4 py-1 rounded">"Go"</button>
            </div>
            <div class="size-full flex-1 relative rounded overflow-hidden">
                <WebView
                    url={"https://www.google.com".to_string()}
                    controller={ctrl}
                    transparent={true}
                    devtools={true}
                    proxy={proxy_config()}
                />
            </div>
        </div>
    }
}

pub(super) fn open_browser(window: &mut Window, cx: &mut App) {
    let webview_ctrl = WebViewController::default();
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
        cx.new(|_cx| BrowserWindow { webview_ctrl })
    })
    .expect("failed to open browser window");
}
