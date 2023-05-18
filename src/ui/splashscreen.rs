use crate::AppState;
use bevy::prelude::*;
use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen};
use bevy_tweening::EaseFunction;
use std::time::Duration;

pub struct SplashscreenPlugin;

impl Plugin for SplashscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(
            SplashPlugin::new(AppState::Splash, AppState::MainMenu)
                .skipable()
                .add_screen(title_splashscreen()),
        );
    }
}

pub fn title_splashscreen() -> SplashScreen {
    SplashScreen {
        brands: vec![
            SplashItem {
                asset: SplashAssetType::SingleText(
                    Text::from_sections([
                        TextSection::new(
                            "Fighting Game\n",
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
                            "Erin, tqbed, Alyx",
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
