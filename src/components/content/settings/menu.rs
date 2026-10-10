use gpui_kit::component::ThemeRegistry;
use zopra::{component, view};
use gpui_kit::*;
use gpui_kit::component::setting::*;
use gpui_kit::component::Sizable;
use crate::components::content::settings::config::Config;
use crate::components::content::settings::theme::ThemeMode;
use gpui_kit::component::Theme;
use gpui_kit::component;

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
                                app_cx.update_global::<Config, _>(|config, cx| {
                                    config.theme_mode = mode;
                                });
                                Theme::change(
                                    match mode {
                                        ThemeMode::System => gpui_kit::component::ThemeMode::Dark,
                                        ThemeMode::Light => gpui_kit::component::ThemeMode::Light,
                                        ThemeMode::Dark => gpui_kit::component::ThemeMode::Dark,
                                    },
                                    None,
                                    app_cx
                                );
                            } }}
                        />
                    </SettingItem>
                    <SettingItem title="Theme Name" description="Select the color scheme.">
                        <SettingDropdown
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
                                app_cx.update_global::<Config, _>(|config, cx| {
                                    config.theme_name = val.to_string();
                                });
                                if let Some(theme) = ThemeRegistry::global(app_cx).themes().get(&val).cloned() {
                                    Theme::change(theme.mode, None, app_cx);
                                    Theme::update(app_cx, |current| current.apply_config(&theme));
                                }
                            } }}
                        />
                    </SettingItem>
                </SettingGroup>
            </SettingPage>
        </Settings>
    }
}
