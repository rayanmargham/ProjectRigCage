use std::time::Duration;

use super::GameState;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_tweening::lens::{SpriteColorLens, TransformScaleLens};
use bevy_tweening::*;

pub struct PreloaderPlugin;

impl Plugin for PreloaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::ClockAsset).continue_to_state(GameState::Preload),
        )
        .add_loading_state(
            LoadingState::new(GameState::Preload).continue_to_state(GameState::Intro),
        )
        .add_collection_to_loading_state::<_, PreloadClockAsset>(GameState::ClockAsset)
        .add_collection_to_loading_state::<_, PreloadFunkinAssets>(GameState::Preload)
        .add_system(PreLoaderInit.in_schedule(OnEnter(GameState::Preload)))
        .add_systems(
            (TickClockTimer, Clock, take_care_of_clock).in_set(OnUpdate(GameState::Preload)),
        )
        .add_system(ByeByePreloader.in_schedule(OnExit(GameState::Preload)))
        .add_system(GetRidOfClock.in_set(OnUpdate(GameState::Intro)));
    }
}
#[derive(AssetCollection, Resource)]
pub struct PreloadFunkinAssets {
    // currently the loading time to load the song on the debug build is ABSOLUTELY UNEXPECTABLE
    // but here's the issue, the bevy_kira_plugin has no support for streaming audio
    // therefore its slow AF
    // to fix this we need to add support in the bevy kira plugin
    // however I'm really stupid and I don't want to waste my time learning audio development
    // so in summary:

    // TODO: Add support for streaming audio
    #[asset(path = "audio/freaky/freaky.ogg")]
    pub freaky: Handle<AudioSource>,
    #[asset(path = "images/alphabet.png")]
    pub alphabet: Handle<Image>
}
#[derive(AssetCollection, Resource)]
pub struct PreloadClockAsset {
    #[asset(path = "images/clock.png")]
    clock: Handle<Image>,
}
#[derive(Component)]
pub struct ClockTimer(Timer);

#[derive(Bundle)]
pub struct ClockTweens {
    scale: Animator<Transform>,
    alpha: Animator<Sprite>,
    idenifier: ClockGhost,
}
#[derive(Component)]
pub struct ClockGhost(); // used to identify the effect clock and delete it after use
pub fn PreLoaderInit(
    mut commands: Commands,
    clock_img: Res<PreloadClockAsset>,
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut window: Query<&mut Window, With<PrimaryWindow>>
) {
    let mut window = window.get_single_mut().unwrap();
    window.resolution = WindowResolution::default();
    settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);
    let tween = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_millis(1000),
        TransformScaleLens {
            start: Vec3::new(1.0, 1.0, 1.0),
            end: Vec3::new(2., 2., 2.),
        },
    );
    let tween2 = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_millis(900),
        SpriteColorLens {
            start: Color::rgba(1., 1., 1., 1.),
            end: Color::rgba(1., 1., 1., 0.),
        },
    );
    // Spawn a Basic Camera, Nothing Fancy just for UI
    commands.spawn(Camera3dBundle
    {
        camera_3d: Camera3d
        {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::None, // fixes colors looking greyish
        ..default()
    });
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                width: 1280.,
                height: 720.,
            }, // TODO: Allow this to be variable in the future.
            ..default()
        },
        camera: Camera {
            order: 1,
            
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });
    
    commands
        .spawn((
            SpriteBundle {
                texture: clock_img.clock.clone(),
                transform: Transform::from_xyz((1280.0 / 2.0) - 50.0, (720.0 / 2.0) - 50.0, 0.0)
                    .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                ..default()
            },
            ClockTimer(Timer::from_seconds(1.5, TimerMode::Repeating)),
        ))
        .with_children(|paren| {
            paren.spawn((
                SpriteBundle {
                    texture: clock_img.clock.clone(),
                    ..default()
                },
                ClockTweens {
                    alpha: Animator::new(tween2),
                    scale: Animator::new(tween),
                    idenifier: ClockGhost(),
                },
            ));
        });
    println!("{}: Preloading Assets, Please Wait...", file!());
}

pub fn TickClockTimer(mut query: Query<&mut ClockTimer, With<ClockTimer>>, time: Res<Time>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

pub fn Clock(
    mut commands: Commands,
    query: Query<(&ClockTimer, Entity), With<ClockTimer>>,
    clock_img: Res<PreloadClockAsset>,
) {
    for (tim, entit) in query.iter() {
        if tim.0.just_finished() {
            let tween = Tween::new(
                EaseFunction::SineInOut,
                Duration::from_millis(1000),
                TransformScaleLens {
                    start: Vec3::new(1.0, 1.0, 1.0),
                    end: Vec3::new(2., 2., 2.),
                },
            )
            .with_completed_event(0);
            let tween2 = Tween::new(
                EaseFunction::SineInOut,
                Duration::from_millis(900),
                SpriteColorLens {
                    start: Color::rgba(1., 1., 1., 1.),
                    end: Color::rgba(1., 1., 1., 0.),
                },
            )
            .with_completed_event(0);
            commands.entity(entit).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: clock_img.clock.clone(),
                        ..default()
                    },
                    ClockTweens {
                        alpha: Animator::new(tween2),
                        scale: Animator::new(tween),
                        idenifier: ClockGhost(),
                    },
                ));
            });
        }
    }
}

pub fn take_care_of_clock(mut commands: Commands, mut reader: EventReader<TweenCompleted>) {
    for event in reader.iter() {
        commands.entity(event.entity).despawn();
    }
}

pub fn ByeByePreloader(mut commands: Commands, query: Query<Entity, With<ClockTimer>>) {
    for entity in query.iter() {
        let tween = Tween::new(
            EaseFunction::QuinticInOut,
            Duration::from_millis(900),
            SpriteColorLens {
                start: Color::rgba(1.0, 1.0, 1.0, 1.0),
                end: Color::rgba(1.0, 1.0, 1.0, 0.0),
            },
        )
        .with_completed_event(1);
        commands.entity(entity).insert(Animator::new(tween));
    }
}

// This is stupid but IDC!
pub fn GetRidOfClock(mut commands: Commands, mut reader: EventReader<TweenCompleted>) {
    for ev in reader.iter() {
        if ev.user_data == 1 {
            commands.entity(ev.entity).despawn();
        }
    }
}
