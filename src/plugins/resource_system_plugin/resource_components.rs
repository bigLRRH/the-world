use bevy::prelude::*;

// 资源组件
#[derive(Component)]
pub struct Population(pub u32); // 人口

#[derive(Component)]
pub struct Money(pub f32); // 资金

#[derive(Component)]
pub struct MineralResources(pub u32); // 矿产资源

#[derive(Component)]
pub struct CO2Emissions(pub f32); // 二氧化碳排放

#[derive(Component)]
pub struct Temperature(pub f32); // 温度

#[derive(Component)]
pub struct WaterResources(pub f32); // 水资源
