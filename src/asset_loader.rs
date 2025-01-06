use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: SceneRoot,
    pub spaceship: SceneRoot,
    pub missiles: SceneRoot,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid: SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("Asteroid.glb"))),
        spaceship: SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("Spaceship.glb")),
        ),
        missiles: SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("Missiles.glb"))),
    }
}
