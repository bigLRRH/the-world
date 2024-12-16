use bevy::prelude::*;

// 插件实现
pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WorldResources::new()) // 插入全局资源
            .add_system(update_resources.system()); // 添加资源更新系统
    }
}
