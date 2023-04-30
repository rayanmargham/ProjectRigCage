use std::time::Duration;

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use bevy_tweening::{*, lens::{TransformScaleLens, SpriteColorLens}};

#[derive(AssetCollection, Resource)]
pub struct PreloadFunkinAssets
{
    #[asset(path = "images/clock.png")]
    clock: Handle<Image>
}
#[derive(AssetCollection, Resource)]
pub struct PreloadClockAsset
{
    #[asset(path = "images/clock.png")]
    clock: Handle<Image>
}
#[derive(Component)]
pub struct ClockTimer(Timer);

#[derive(Bundle)]
pub struct ClockTweens
{
    scale: Animator<Transform>,
    alpha: Animator<Sprite>
}
#[derive(Component)]
pub struct DebugRot
{
    speed: f32
}
pub fn PreLoaderInit(
    mut commands: Commands,
    clock_img: Res<PreloadClockAsset>,
    window_query: Query<&Window, With<PrimaryWindow>>
)
{

    let tween = Tween::new(EaseFunction::SineInOut, Duration::from_millis(1000), TransformScaleLens {start: Vec3::new(1.0, 1.0, 1.0), end: Vec3::new(2., 2., 2.)});
    let tween2 = Tween::new(EaseFunction::SineInOut, Duration::from_millis(900), SpriteColorLens {start: Color::rgba(1., 1., 1., 1.), end: Color::rgba(1., 1., 1., 0.)});
    let window = window_query.get_single().unwrap();
    // Spawn a Basic Camera, Nothing Fancy just for UI

    commands.spawn(Camera2dBundle
    {
        projection: OrthographicProjection
        {
            scaling_mode: bevy::render::camera::ScalingMode::Fixed { width: 1280., height: 720. }, // TODO: Allow this to be variable in the future.
            ..default()
        },
        camera: Camera
        {
            order: 1,
            ..default()
        },
        camera_2d: Camera2d
        {
            
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    });
    commands.spawn((SpriteBundle
    {
        texture: clock_img.clock.clone(),
        transform: Transform::from_xyz((1280.0 / 2.0) - 50.0, (720.0 / 2.0) - 50.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5)),
        ..default()
    }, ClockTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))).with_children(|paren|
    {
        paren.spawn(
            (
                SpriteBundle
                {
                    texture: clock_img.clock.clone(),
                    ..default()
                },
                Animator::new(tween), DebugRot {speed: 1.}
            )
        );
    });
    println!("{}: Preloading Assets, Please Wait...", file!());
}

pub fn TickClockTimer(
    mut query: Query<&mut ClockTimer, With<ClockTimer>>,
    time: Res<Time>
)
{
    for mut timer in query.iter_mut()
    {
        timer.0.tick(time.delta());
    }
}

pub fn Clock(
    mut commands: Commands,
    mut query: Query<(&mut ClockTimer, Entity), With<ClockTimer>>,
    clock_img: Res<PreloadClockAsset>
)
{
    
    for (mut tim, entit) in query.iter_mut()
    {
        if tim.0.just_finished()
        {
            let tween = Tween::new(EaseMethod::Linear, Duration::from_millis(800), TransformScaleLens {start: Vec3::new(1.0, 1.0, 1.0), end: Vec3::new(2., 2., 2.)});
            let tween2 = Tween::new(EaseMethod::Linear, Duration::from_millis(2000), SpriteColorLens {start: Color::rgba_linear(1., 1., 1., 1.), end: Color::rgba_linear(1., 1., 1., 0.)});
            commands.entity(entit).with_children(|parent| 
                {
                    parent.spawn((
                        SpriteBundle
                        {
                            texture: clock_img.clock.clone(),
                            ..default()
                        },
                        Animator::new(tween),
                        DebugRot {speed: 1.}
                    ));
                }
            );
        }
    }
}
pub fn DebugRot(mut query: Query<(&mut Sprite, &DebugRot), With<DebugRot>>, time: Res<Time>)
{
    for (mut color, rot) in query.iter_mut()
    {
        let val = color.color.a();
        if val > 0.0
        {
            color.color.set_a(val - rot.speed * time.delta_seconds());
        }
    }
}