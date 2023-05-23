use crate::settings::Settings;
use crate::AppState;
use bevy::prelude::*;
use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen};
use bevy_tweening::EaseFunction;
use std::process::exit;
use std::time::Duration;

pub struct SplashscreenPlugin;

impl Plugin for SplashscreenPlugin {
    fn build(&self, app: &mut App) {
        let settings = Settings::new().unwrap();
        let title = match settings.translation.get(&envmnt::get_or_panic("LANG")) {
            Some(v) => v.title.clone(),
            None => {
                eprintln!(
                    "No field matching {} in settings",
                    envmnt::get_or_panic("LANG")
                );
                exit(1);
            }
        };
        let devs = match settings.translation.get(&envmnt::get_or_panic("LANG")) {
            Some(v) => v.developers.clone(),
            None => {
                eprintln!(
                    "No field matching {} in settings",
                    envmnt::get_or_panic("LANG")
                );
                exit(1);
            }
        };

        app.add_plugin(
            SplashPlugin::new(AppState::Splash, AppState::MainMenu)
                .skipable()
                .add_screen(title_splashscreen(title, devs)),
        );
    }
}

pub fn title_splashscreen(title: String, devs: String) -> SplashScreen {
    SplashScreen {
        brands: vec![
            SplashItem {
                asset: SplashAssetType::SingleText(
                    Text::from_sections([
                        TextSection::new(
                            format!("{}\n", title),
                            TextStyle {
                                font_size: 100.,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "by\n",
                            TextStyle {
                                font_size: 24.,
                                color: Color::WHITE.with_a(0.75),
                                ..default()
                            },
                        ),
                        TextSection::new(
                            devs,
                            TextStyle {
                                font_size: 50.,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                    ])
                    .with_alignment(TextAlignment::Center),
                    "fonts/Kenney Pixel Square.ttf".to_string(),
                ),
                tint: Color::WHITE,
                size: Size::new(Val::Percent(33.3), Val::Percent(50.)),
                ease_function: EaseFunction::QuarticInOut.into(),
                duration: Duration::from_secs_f32(3.),
                is_static: false,
            },
            SplashItem {
                asset: SplashAssetType::SingleImage("branding/icon.png".to_string()),
                tint: Color::WHITE,
                size: Size::new(Val::Px(128.), Val::Px(128.)),
                ease_function: EaseFunction::QuinticInOut.into(),
                duration: Duration::from_secs_f32(3.),
                is_static: true,
            },
        ],
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    }
}
