use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::{NonSend, Res, ResMut, Resource},
    time::Time,
};
use objc2::rc::Retained;
use objc2_core_motion::CMMotionManager;

#[derive(Resource)]
pub struct Gyroscope {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Resource)]
struct GyroscopePluginSettings {
    frequency: f64,
}

pub struct GyroscopePlugin {
    frequency: f64,
}

impl Default for GyroscopePlugin {
    fn default() -> Self {
        Self { frequency: 60.0 }
    }
}

impl Plugin for GyroscopePlugin {
    fn build(&self, app: &mut App) {
        let motion_manager = unsafe { CMMotionManager::new() };
        app.insert_non_send_resource(motion_manager);

        app.insert_resource(GyroscopePluginSettings {
            frequency: self.frequency,
        });

        app.insert_resource(Gyroscope {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });

        app.add_systems(Startup, setup_gyroscope);
        app.add_systems(Update, update_gyroscope);
    }
}

fn setup_gyroscope(
    settings: Res<GyroscopePluginSettings>,
    motion_manager: NonSend<Retained<CMMotionManager>>,
) {
    unsafe {
        motion_manager.setGyroUpdateInterval(1.0 / settings.frequency);
        motion_manager.startGyroUpdates();
    }
}

fn update_gyroscope(
    motion_manager: NonSend<Retained<CMMotionManager>>,
    mut gyroscope: ResMut<Gyroscope>,
    time: Res<Time>,
) {
    unsafe {
        if let Some(gyroscope_data) = motion_manager.gyroData() {
            let rotation_rate = gyroscope_data.rotationRate();

            gyroscope.x = -rotation_rate.y as f32 * time.delta_seconds();
            gyroscope.y = rotation_rate.z as f32 * time.delta_seconds();
            gyroscope.z = -rotation_rate.x as f32 * time.delta_seconds();
        };
    }
}
