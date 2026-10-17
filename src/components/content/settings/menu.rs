use gpui_kit::component::ActiveTheme;
use gpui_kit::component::menu::DropdownMenu;
use gpui_kit::component::ThemeRegistry;
use zopra::{component, view};
use gpui_kit::*;
use gpui_kit::component::setting::*;
use gpui_kit::component::Sizable;
use gpui_kit::component::menu::PopupMenuItem;
use crate::components::content::settings::config::Config;
use crate::components::content::settings::theme::{ThemeMode, WindowStyle, effective_mode};
use gpui_kit::component::Theme;
use gpui_kit::component;
use strum::IntoEnumIterator;

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
                                app_cx.update_global::<Config, _>(|c, _| c.theme_mode = mode);
                                let mode = effective_mode(app_cx);
                                Theme::change(mode, None, app_cx);
                            } }}
                        />
                    </SettingItem>
                    <SettingItem title="Theme Name" description="Select the color scheme. Hover over a theme to preview it.">
                        <SettingCustomField>
                            {{
                                move |_options, _window, cx: &mut App| {
                                    let mode = effective_mode(cx);
                                    let themes: Vec<_> = ThemeRegistry::global(cx)
                                        .sorted_themes()
                                        .into_iter()
                                        .filter(|t| t.mode == mode)
                                        .collect();
                                    let current = cx.theme().theme_name().clone();

                                    view! {
                                        <button
                                            styled
                                            id="theme-menu-btn"
                                            label={current.clone()}
                                            dropdown_caret=true
                                            outline
                                            on_hover={{
                                                move |hovered, _window, cx| {
                                                    if !*hovered {
                                                        let mode = effective_mode(cx);
                                                        Theme::change(mode, None, cx);
                                                    }
                                                }
                                            }}
                                            dropdown_menu={{
                                                let themes = themes.clone();
                                                let current = current.clone();
                                                move |menu, _window, _cx| {
                                                    let mut menu = menu;
                                                    for theme_cfg in &themes {
                                                        let is_checked = theme_cfg.name == current;
                                                        let theme_cfg = theme_cfg.clone();

                                                        menu = menu.item(
                                                            PopupMenuItem::element({
                                                                let name = theme_cfg.name.clone();
                                                                let theme_cfg_hover = theme_cfg.clone();
                                                                move |_window, _cx| {
                                                                    div()
                                                                        .id(SharedString::from(format!("theme-hover-{}", name)))
                                                                        .w_full()
                                                                        .child(name.clone())
                                                                        .on_hover({
                                                                            let theme_cfg = theme_cfg_hover.clone();
                                                                            move |hovered, _window, cx| {
                                                                                if *hovered {
                                                                                    Theme::update(cx, |t| t.apply_config(&theme_cfg));
                                                                                } else {
                                                                                    let mode = effective_mode(cx);
                                                                                    Theme::change(mode, None, cx);
                                                                                }
                                                                            }
                                                                        })
                                                                }
                                                            })
                                                            .checked(is_checked)
                                                            .on_click({
                                                                let theme_cfg = theme_cfg.clone();
                                                                move |_, _, cx| {
                                                                    let mode = effective_mode(cx);
                                                                    Theme::update(cx, |t| {
                                                                        if mode.is_dark() { t.dark_theme = theme_cfg.clone(); }
                                                                        else { t.light_theme = theme_cfg.clone(); }
                                                                    });
                                                                    Theme::change(mode, None, cx);
                                                                    cx.update_global::<Config, _>(|c, _| {
                                                                        if mode.is_dark() {
                                                                            c.dark_theme_name = theme_cfg.name.to_string();
                                                                        } else {
                                                                            c.light_theme_name = theme_cfg.name.to_string();
                                                                        }
                                                                    });
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

