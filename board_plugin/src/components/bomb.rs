use bevy::prelude::Component;

#[cfg(feature = "debug")]
use bevy::reflect::Reflect;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};


#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Component)]
pub struct Bomb;
