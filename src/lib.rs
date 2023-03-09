#![warn(missing_docs)]

//! <div align="center">
//!
//! # Bevy Rewind
//!
//! A plugin for easily rewinding time in [Bevy](https://github.com/bevyengine/bevy)
//!
//! [<img alt="bevy tracking" src="https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue?style=for-the-badge" height="24">](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
//! [<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/LiamGallagher737/bevy_rewind/rust.yml?branch=main&style=for-the-badge" height="24">](https://github.com/LiamGallagher737/bevy_rewind/actions)
//! [<img alt="github" src="https://img.shields.io/badge/github-bevy_rewind-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="24">](https://github.com/LiamGallagher737/bevy_rewind)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/bevy_rewind.svg?style=for-the-badge&color=fc8d62&logo=rust" height="24">](https://crates.io/crates/bevy_rewind)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bevy_rewind-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="24">](https://docs.rs/bevy_rewind)
//!
//! </div>
//!
//! # Setup
//!
//! Add the plugin to your app
//! ```ignore
//! .add_plugin(RewindPlugin::default())
//! ```
//!
//! Config Options
//!
//! ```no_run
//! # use bevy_rewind::RewindPlugin;
//! RewindPlugin {
//!     // How many captures will take place before they start clearing,
//!     // default is 300 for 5 seconds of replay.
//!     // (60 ticks per seconds * 5 seconds)
//!     max_capture_count: 300,
//! };
//! ```
//!
//! # Usage
//!
//! To track a components value add the `RewindComponent<C>` to the entity with C being the component you want to track, in this example the entities `Transform` will be tracked for rewinding.
//!
//! ```ignore
//! # use bevy::prelude::*;
//! # use bevy_rewind::*;
//! commands.spawn((
//!     PbrBundle::default(),
//!     RewindComponent::<Transform>::default(),
//! ));
//! ```
//!
//! By default on `Transform` and `GlobalTransform` can be tracked, to add mode use the `init_rewind_component<C>` method on your app where C is the component you want to be able to track.
//!
//! ```rust
//! # use bevy::prelude::*;
//! # use bevy_rewind::*;
//! # #[derive(Component, Clone)]
//! # struct Velocity;
//! App::new()
//!     .add_plugin(RewindPlugin::default())
//!     .init_rewind_component::<Velocity>()
//!     .run();
//! ```

use bevy::prelude::*;

/// Add this to your app for the ability to rewind
pub struct RewindPlugin {
    /// The max number of captures per component before they start being cleared
    /// Default is 300 for 5 seconds of replay (60 ticks per seconds * 5 seconds)
    pub max_capture_count: usize,
}

impl Plugin for RewindPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rewind {
            max_capture_count: self.max_capture_count,
            rewinding: false,
        });
        app.init_rewind_component::<Transform>();
        app.init_rewind_component::<GlobalTransform>();
    }
}

impl Default for RewindPlugin {
    fn default() -> Self {
        Self {
            max_capture_count: 300,
        }
    }
}

#[doc(hidden)]
pub trait RewindAppExtentions {
    /// Add rewindability for a given component
    fn init_rewind_component<C: Component + Clone>(&mut self) -> &mut Self;
}

impl RewindAppExtentions for App {
    fn init_rewind_component<C: Component + Clone>(&mut self) -> &mut Self {
        self.add_systems((
            capture_components::<C>
                .run_if(not(rewinding))
                .in_schedule(CoreSchedule::FixedUpdate),
            rewind_components::<C>
                .run_if(rewinding)
                .in_schedule(CoreSchedule::FixedUpdate),
        ));
        self
    }
}

/// Control the current rewind status
#[derive(Resource)]
pub struct Rewind {
    /// The max number of captures per component before they start being cleared
    pub max_capture_count: usize,
    /// Set to true to start rewinding
    pub rewinding: bool,
}

/// Add this component to any entity to track it for rewinding, the generic value is the component you want to track
#[derive(Component, Default)]
pub struct RewindComponent<C: Component + Clone> {
    history: Vec<C>,
}

/// Run condition that returns true if rewinding is true
pub fn rewinding(rewind: Res<Rewind>) -> bool {
    rewind.rewinding
}

fn capture_components<C: Component + Clone>(
    mut query: Query<(&C, &mut RewindComponent<C>)>,
    rewind: Res<Rewind>,
) {
    for (component, mut rewind_component) in &mut query {
        rewind_component.history.push(component.clone());
        if rewind_component.history.len() > rewind.max_capture_count {
            rewind_component.history.remove(0);
        }
    }
}

fn rewind_components<C: Component + Clone>(mut query: Query<(&mut C, &mut RewindComponent<C>)>) {
    for (mut component, mut rewind) in &mut query {
        if let Some(old_value) = rewind.history.pop() {
            *component = old_value;
        }
    }
}
