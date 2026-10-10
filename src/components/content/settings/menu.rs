use gpui_kit::component::ThemeRegistry;
use zopra::{component, view};
use gpui_kit::*;
use gpui_kit::component::setting::*;
use gpui_kit::component::Sizable;
use crate::components::content::settings::config::Config;
use crate::components::content::settings::theme::{ThemeMode, WindowStyle};
use gpui_kit::component::Theme;
use gpui_kit::component;
use strum::IntoEnumIterator;

fn apply_theme_state(cx: &mut App) {
    let config = cx.global::<Config>();
    match config.theme_mode {
        ThemeMode::System => Theme::sync_system_appearance(None, cx),
        _ => {
            if let Some(theme) = ThemeRegistry::global(cx).themes().get(config.theme_name.as_str()).cloned() {
                Theme::change(theme.mode, None, cx);
                Theme::update(cx, |current| current.apply_config(&theme));
            }
        }
    }
}

#[component]
pub fn settings_menu() {
    view! {
        <Settings id="syzygy-settings" with_size={component::Size::Medium}>
            <SettingPage title="Appearance" default_open={true}>
                <SettingGroup title="Theme">
                    <SettingItem title="Theme Mode" description="Select the overall application theme.">
                        <SettingDropdown
                            options={vec![
                                (SharedString::from("System"), SharedString::from("System")),
                                (SharedString::from("Light"), SharedString::from("Light")),
                                (SharedString::from("Dark"), SharedString::from("Dark")),
                            ]}
                            state={
                                match cx.global::<Config>().theme_mode {
                                    ThemeMode::System => SharedString::from("System"),
                                    ThemeMode::Light => SharedString::from("Light"),
                                    ThemeMode::Dark => SharedString::from("Dark"),
                                }
                            }
                            on_change={{ |val: SharedString, app_cx| {
                                let mode = match val.as_ref() {
                                    "Light" => ThemeMode::Light,
                                    "Dark" => ThemeMode::Dark,
                                    _ => ThemeMode::System,
                                };
                                app_cx.update_global::<Config, _>(|config, _| config.theme_mode = mode);
                                apply_theme_state(app_cx);
                            } }}

                        />
                    </SettingItem>
                    <SettingItem title="Theme Name" description="Select the color scheme.">
                        <SettingScrollableDropdown
                            options={{
                                ThemeRegistry::global(cx)
                                    .sorted_themes()
                                    .into_iter()
                                    .map(|t| (t.name.clone(), t.name.clone()))
                                    .collect::<Vec<_>>()
                            }}

                            state={
                                SharedString::from(cx.global::<Config>().theme_name.clone())
                            }
                            on_change={{ |val: SharedString, app_cx| {
                                app_cx.update_global::<Config, _>(|config, _| config.theme_name = val.to_string());
                                apply_theme_state(app_cx);
                            } }}

                        />
                    </SettingItem>
                    <SettingItem title="Window Styling" description="Background appearance of the window. Requires an app restart to apply.">
                        <SettingDropdown
                            options={{
                                WindowStyle::iter()
                                    .map(|s| (SharedString::from(s.to_string()), SharedString::from(s.to_string())))
                                    .collect::<Vec<_>>()
                            }}

                            state={
                                SharedString::from(cx.global::<Config>().window_style.to_string())
                            }
                            on_change={{ |val: SharedString, app_cx| {
                                if let Some(style) = WindowStyle::iter().find(|s| s.to_string() == val.as_ref()) {
                                    app_cx.update_global::<Config, _>(|config, _| config.window_style = style);
                                }
                            } }}

                        />
                    </SettingItem>
                </SettingGroup>
            </SettingPage>
        </Settings>
    }
}
