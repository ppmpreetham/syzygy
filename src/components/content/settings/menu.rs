use crate::components::content::settings::config::Config;
use crate::components::content::settings::theme::{ThemeMode, WindowStyle, effective_mode};
use gpui_kit::component::menu::{DropdownMenu, PopupMenuItem};
use gpui_kit::component::{ActiveTheme, Size, Theme, ThemeRegistry};
use gpui_kit::*;
use strum::IntoEnumIterator;
use zopra::ui::{SettingCustomField, SettingDropdown, SettingGroup, SettingItem, SettingPage, Settings};
use zopra::{component, view};

fn options(names: impl IntoIterator<Item = SharedString>) -> Vec<(SharedString, SharedString)> {
    names.into_iter().map(|name| (name.clone(), name)).collect()
}

#[component]
pub fn settings_menu() {
    view! {
        <Settings id="syzygy-settings" size={Size::Medium}>
            <SettingPage title="Appearance" default_open={true}>
                <SettingGroup title="Theme">
                    <SettingItem title="Theme Mode" description="Select the overall application theme.">
                        <SettingDropdown
                            options={options(["System".into(), "Light".into(), "Dark".into()])}
                            value={|cx| match cx.global::<Config>().theme_mode {
                                ThemeMode::System => "System".into(),
                                ThemeMode::Light => "Light".into(),
                                ThemeMode::Dark => "Dark".into(),
                            }}
                            on_change={|value, cx| {
                                let mode = match value.as_ref() {
                                    "Light" => ThemeMode::Light,
                                    "Dark" => ThemeMode::Dark,
                                    _ => ThemeMode::System,
                                };
                                cx.update_global::<Config, _>(|config, _| config.theme_mode = mode);
                                Theme::change(effective_mode(cx), None, cx);
                            }}
                        />
                    </SettingItem>
                    <SettingItem title="Theme Name" description="Select the color scheme. Hover over a theme to preview it.">
                        <SettingCustomField content={
                            move |_options, _window, cx: &mut App| {
                                let mode = effective_mode(cx);
                                let themes: Vec<_> = ThemeRegistry::global(cx)
                                    .sorted_themes()
                                    .into_iter()
                                    .filter(|t| t.mode == mode)
                                    .cloned()
                                    .collect();
                                let current = cx.theme().theme_name().clone();

                                view! {
                                    <button
                                        styled
                                        id="theme-menu-btn"
                                        label={current.clone()}
                                        dropdown_caret=true
                                        outline
                                        on_hover={|hovered, _window, cx| {
                                            if !*hovered {
                                                let mode = effective_mode(cx);
                                                Theme::change(mode, None, cx);
                                            }
                                        }}
                                        dropdown_menu={
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
                                        }
                                    />
                                }
                            }
                        } />
                    </SettingItem>
                    <SettingItem title="Window Styling" description="Background appearance of the window. Requires an app restart to apply.">
                        <SettingDropdown
                            options={options(WindowStyle::iter().map(|style| style.to_string().into()))}
                            value={|cx| cx.global::<Config>().window_style.to_string().into()}
                            on_change={|value, cx| {
                                if let Some(style) = WindowStyle::iter().find(|style| style.to_string() == value.as_ref()) {
                                    cx.update_global::<Config, _>(|config, _| config.window_style = style);
                                }
                            }}
                        />
                    </SettingItem>
                </SettingGroup>
            </SettingPage>
        </Settings>
    }
}
