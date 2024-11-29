# Bevy Gyroscope Plugin

> [!WARNING]  
> This plugin currently only supports iOS devices.

A Bevy plugin that provides gyroscope functionality.

## Installation

You can add this plugin to your project using cargo:

```bash
cargo add bevy_gyroscope
```

Or manually add it to your `Cargo.toml`:

```toml
[dependencies]
bevy_gyroscope = "0.1.0"  # replace with actual version
```

## Usage

```rust
use bevy::prelude::*;
use bevy_gyroscope::{GyroscopePlugin, Gyroscope};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the gyroscope plugin with default settings (60Hz update frequency)
        .add_plugins(GyroscopePlugin::default())
        .add_systems(Update, use_gyroscope_data)
        .run();
}

fn use_gyroscope_data(gyroscope: Res<Gyroscope>) {
    println!("Gyroscope: x={}, y={}, z={}", 
        gyroscope.x, 
        gyroscope.y, 
        gyroscope.z
    );
}
```

## Custom Configuration

You can customize the update frequency when adding the plugin:

```rust
.add_plugins(GyroscopePlugin { frequency: 120.0 })
```

## Resource Components

The plugin provides a `Gyroscope` resource that contains the rotation delta (change in rotation) for the current frame:

- `x`: Change in rotation around the x-axis (in radians) per frame
- `y`: Change in rotation around the y-axis (in radians) per frame
- `z`: Change in rotation around the z-axis (in radians) per frame

## License

[MIT License](LICENSE.md)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.