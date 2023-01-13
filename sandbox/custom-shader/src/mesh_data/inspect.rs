use bevy::pbr::wireframe::Wireframe;
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, Debug, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct InspectorInfo<T>
where
    T: 'static + Send + Sync + Default + Reflect + Component,
{
    pub show_wireframe: bool,
    pub outdated: bool,

    #[inspector(suffix = " read-only")]
    pub num_vertices: usize,
    #[inspector(suffix = " read-only")]
    pub num_indices: usize,

    #[reflect(ignore)]
    pub old_data: T,
}

impl<T> Default for InspectorInfo<T>
where
    T: 'static + Send + Sync + Default + Reflect + Component,
{
    fn default() -> Self {
        Self {
            show_wireframe: true,
            outdated: true,
            num_vertices: 0,
            num_indices: 0,
            old_data: T::default(),
        }
    }
}

pub trait InspectableMesh<T>
where
    T: 'static + Send + Sync + Default + Reflect + Component,
{
    fn update_info(
        meshes: ResMut<Assets<Mesh>>,
        query: Query<(&T, &mut InspectorInfo<T>, &mut Handle<Mesh>)>,
    );

    fn insert_debug_components(mut commands: Commands, query: Query<Entity, With<T>>) {
        // println!("{:?}", query);
        info!("Enabled {} debugging!", std::any::type_name::<T>());
        for entt in query.iter() {
            if let Some(mut entity) = commands.get_entity(entt) {
                entity.insert(InspectorInfo::<T>::default());
            };
        }
    }

    fn toggle_wireframe(mut commands: Commands, query: Query<(Entity, &mut InspectorInfo<T>)>) {
        // println!("{:?}", query);
        for (entt, debug_info) in query.iter() {
            if let Some(mut entity) = commands.get_entity(entt) {
                if debug_info.show_wireframe {
                    entity.insert(Wireframe);
                } else {
                    entity.remove::<Wireframe>();
                }
            };
        }
    }
}

#[derive(Component, Debug, Reflect, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct InspectablePlugin<T>
where
    T: 'static + Send + Sync + Default + Reflect + Component + InspectableMesh<T>,
{
    data: T,
}

impl<T> Plugin for InspectablePlugin<T>
where
    T: 'static + Send + Sync + Default + Reflect + Component + InspectableMesh<T>,
{
    fn build(&self, app: &mut App) {
        app.register_type::<InspectorInfo<T>>()
            .add_plugin(WireframePlugin)
            .add_startup_system_to_stage(StartupStage::PostStartup, T::insert_debug_components)
            .add_system(T::toggle_wireframe)
            .add_system(T::update_info);
    }
}
