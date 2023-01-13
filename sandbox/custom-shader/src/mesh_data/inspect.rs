use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;

pub trait Inspectable<T>
where
    T: Default + Reflect + Component,
{
    type InspectorInfo;

    fn update_info(
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        query: Query<(Entity, &T, &mut Self::InspectorInfo, &mut Handle<Mesh>)>,
    ) where
        Self::InspectorInfo: Default + Reflect + Component;

    fn insert_debug_components(mut commands: Commands, query: Query<Entity, With<T>>)
    where
        Self::InspectorInfo: Default + Reflect + Component,
    {
        // println!("{:?}", query);
        info!("Enabled {} debugging!", std::any::type_name::<T>());
        for entt in query.iter() {
            if let Some(mut entity) = commands.get_entity(entt) {
                entity.insert(Self::InspectorInfo::default());
            };
        }
    }
}

#[derive(Component, Debug, Reflect, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct InspectPlugin<T>
where
    T: Inspectable<T> + Default + Reflect + Component,
{
    data: T,
}

impl<T> Plugin for InspectPlugin<T>
where
    T: Inspectable<T> + Default + Reflect + Component,
    <T as Inspectable<T>>::InspectorInfo: Default + Reflect + Component + GetTypeRegistration,
{
    fn build(&self, app: &mut App) {
        app.register_type::<<T as Inspectable<T>>::InspectorInfo>()
            .add_plugin(WireframePlugin)
            .add_startup_system_to_stage(StartupStage::PostStartup, T::insert_debug_components)
            .add_system(T::update_info);
    }
}
