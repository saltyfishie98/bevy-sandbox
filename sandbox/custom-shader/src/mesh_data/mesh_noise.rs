use bevy::prelude::*;

use super::ProceduralShape;

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct MeshNoise<T>
where
    T: ProceduralShape + Send + Sync + Reflect + Default + 'static,
{
    mesh_data: T,
}
