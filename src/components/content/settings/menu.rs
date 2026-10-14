use gpui_kit::component::ThemeRegistry;
use zopra::{component, view};
use gpui_kit::*;
use gpui_kit::component::setting::*;
use gpui_kit::component::Sizable;
use gpui_kit::component::menu::{DropdownMenu, PopupMenuItem};
use crate::components::content::settings::config::Config;
use crate::components::content::settings::theme::{ThemeMode, WindowStyle};
use gpui_kit::component::Theme;
use gpui_kit::component;
use strum::IntoEnumIterator;

fn apply_theme_by_name(name: &str, cx: &mut App) {
    if let Some(theme) = ThemeRegistry::global(cx).themes().get(name).cloned() {
        Theme::change(theme.mode, None, cx);
        Theme::update(cx, |current| current.apply_config(&theme));
    }
}

fn apply_theme_state(cx: &mut App) {
    let mode = cx.global::<Config>().theme_mode;
    let theme_name = cx.global::<Config>().theme_name.clone();
    match mode {
        ThemeMode::System => Theme::sync_system_appearance(None, cx),
        _ => {
            apply_theme_by_name(&theme_name, cx);
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
                    <SettingItem title="Theme Name" description="Select the color scheme. Hover over a theme to preview it.">
                        <SettingCustomField>
                            {{
                                move |_options, _window, cx: &mut App| {
                                    let current_theme = cx.global::<Config>().theme_name.clone();
                                    let themes: Vec<SharedString> = ThemeRegistry::global(cx)
                                        .sorted_themes()
                                        .into_iter()
                                        .map(|t| t.name.clone())
                                        .collect();

                                    view! {
                                        <button
                                            styled
                                            id="theme-menu-btn"
                                            label={SharedString::from(current_theme.clone())}
                                            dropdown_caret=true
                                            outline
                                            on_hover={{
                                                move |hovered, _window, cx| {
                                                    if !*hovered {
                                                        let saved = cx.global::<Config>().theme_name.clone();
                                                        apply_theme_by_name(&saved, cx);
                                                    }
                                                }
                                            }}
                                            dropdown_menu={{
                                                let themes = themes.clone();
                                                let current_theme = current_theme.clone();
                                                move |menu, _window, _cx| {
                                                    let mut menu = menu;
                                                    for theme_name in &themes {
                                                        let name = theme_name.clone();
                                                        let is_checked = name == current_theme.as_str();

                                                        menu = menu.item(
                                                            PopupMenuItem::element({
                                                                let name = name.clone();
                                                                move |_window, _cx| {
                                                                    div()
                                                                        .id(SharedString::from(format!("theme-hover-{}", name)))
                                                                        .w_full()
                                                                        .child(name.clone())
                                                                        .on_hover({
                                                                            let name = name.clone();
                                                                            move |hovered, _window, cx| {
                                                                                if *hovered {
                                                                                    apply_theme_by_name(&name, cx);
                                                                                }
                                                                            }
                                                                        })
                                                                }
                                                            })
                                                            .checked(is_checked)
                                                            .on_click({
                                                                let name = name.clone();
                                                                move |_, _, cx| {
                                                                    cx.update_global::<Config, _>(|config, _| {
                                                                        config.theme_name = name.to_string();
                                                                    });
                                                                    apply_theme_by_name(&name, cx);
                                                                }
                                                            })
                                                        );
                                                    }
                                                    menu.scrollable(true)
                                                }
                                            }}

                                        />
                                    }
                                }
                            }}
                        </SettingCustomField>
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

