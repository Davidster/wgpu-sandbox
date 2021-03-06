use super::*;

use cgmath::Vector3;

#[derive(Clone, Debug)]
pub struct PointLightComponent {
    pub node_id: GameNodeId,
    pub color: Vector3<f32>,
    pub intensity: f32,
}

#[derive(Clone, Debug)]
pub struct DirectionalLightComponent {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub color: Vector3<f32>,
    pub intensity: f32,
}
