use super::*;
use cgmath::{Deg, Euler, InnerSpace, Matrix4, Quaternion, Rad, Vector3};
use winit::{
    dpi::PhysicalPosition,
    event::{
        DeviceEvent, ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode, WindowEvent,
    },
    window::Window,
};

pub const Z_NEAR: f32 = 0.001;
pub const Z_FAR: f32 = 100000.0;
pub const FOV_Y: Deg<f32> = Deg(45.0);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CameraPose {
    horizontal_rotation: Rad<f32>,
    vertical_rotation: Rad<f32>,
    position: Vector3<f32>,
}

pub struct Camera {
    pose: CameraPose,
}

#[derive(Copy, Clone, Debug)]
pub struct CameraViewProjMatrices {
    pub proj: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub rotation_only_view: Matrix4<f32>,
    pub position: Vector3<f32>,
    pub z_near: f32,
    pub z_far: f32,
}

#[cfg(test)]
mod tests {
    use cgmath::Vector4;

    use super::*;

    #[test]
    fn my_test() {
        let reverse_z_mat =
            make_perspective_matrix(0.1, 100000.0, cgmath::Deg(90.0).into(), 1.0, true);
        let reg_z_mat =
            make_perspective_matrix(0.1, 100000.0, cgmath::Deg(90.0).into(), 1.0, false);
        let pos = Vector4::new(-0.5, -0.5, -0.11, 1.0);
        let reverse_proj_pos = reverse_z_mat * pos;
        let reg_proj_pos = reg_z_mat * pos;
        let persp_div = |yo: Vector4<f32>| yo / yo.w;
        println!("{:?}", reverse_z_mat);
        println!("{:?}", reg_z_mat);
        println!("{:?}", reverse_proj_pos);
        println!("{:?}", reg_proj_pos);
        println!("{:?}", persp_div(reverse_proj_pos));
        println!("{:?}", persp_div(reg_proj_pos));
        assert_eq!(true, true);
    }
}

impl Camera {
    pub fn new(initial_position: Vector3<f32>) -> Self {
        Camera {
            pose: CameraPose {
                horizontal_rotation: Rad(0.0),
                vertical_rotation: Rad(0.0),
                position: initial_position,
            },
        }
    }

    pub fn build_view_projection_matrices(
        &self,
        window: &winit::window::Window,
    ) -> CameraViewProjMatrices {
        let proj = make_perspective_matrix(
            Z_NEAR,
            Z_FAR,
            FOV_Y.into(),
            window.inner_size().width as f32 / window.inner_size().height as f32,
            true,
        );
        let rotation_only_view = make_rotation_matrix(Quaternion::from(Euler::new(
            -self.pose.vertical_rotation,
            Rad(0.0),
            Rad(0.0),
        ))) * make_rotation_matrix(Quaternion::from(Euler::new(
            Rad(0.0),
            -self.pose.horizontal_rotation,
            Rad(0.0),
        )));
        let view = rotation_only_view * make_translation_matrix(-self.pose.position);
        let position = self.pose.position;
        CameraViewProjMatrices {
            proj,
            view,
            rotation_only_view,
            position,
            z_near: Z_NEAR,
            z_far: Z_FAR,
        }
    }

    pub fn get_direction_vector(&self) -> Vector3<f32> {
        let horizontal_scale = self.pose.vertical_rotation.0.cos();
        Vector3::new(
            (self.pose.horizontal_rotation.0 + std::f32::consts::PI).sin() * horizontal_scale,
            self.pose.vertical_rotation.0.sin(),
            (self.pose.horizontal_rotation.0 + std::f32::consts::PI).cos() * horizontal_scale,
        )
        .normalize()
    }

    // TODO: should this function really be in the camera module?
    pub fn build_cubemap_view_projection_matrices(
        position: Vector3<f32>,
        z_near: f32,
        z_far: f32,
        reverse_z: bool,
    ) -> Vec<CameraViewProjMatrices> {
        return vec![
            (Deg(90.0), Deg(0.0)),    // right
            (Deg(-90.0), Deg(0.0)),   // left
            (Deg(180.0), Deg(90.0)),  // top
            (Deg(180.0), Deg(-90.0)), // bottom
            (Deg(180.0), Deg(0.0)),   // front
            (Deg(0.0), Deg(0.0)),     // back
        ]
        .iter()
        .map(|(horizontal_rotation, vertical_rotation)| {
            let proj = make_perspective_matrix(z_near, z_far, Deg(90.0).into(), 1.0, reverse_z);
            let rotation_only_view = make_rotation_matrix(Quaternion::from(Euler::new(
                -Rad::from(*vertical_rotation),
                Rad(0.0),
                Rad(0.0),
            ))) * make_rotation_matrix(Quaternion::from(Euler::new(
                Rad(0.0),
                -Rad::from(*horizontal_rotation),
                Rad(0.0),
            )));
            let view = rotation_only_view * make_translation_matrix(-position);
            CameraViewProjMatrices {
                proj,
                view,
                rotation_only_view,
                position,
                z_near,
                z_far,
            }
        })
        .collect();
    }
}

pub struct CameraController {
    // mouse_state: MouseState,
    unprocessed_delta: Option<(f64, f64)>,
    window_focused: bool,

    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,

    target_pose: CameraPose,

    pub speed: f32,
}

impl CameraController {
    pub fn new(speed: f32, camera: &Camera) -> Self {
        Self {
            unprocessed_delta: None,
            window_focused: false,

            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,

            target_pose: camera.pose,

            speed,
        }
    }

    pub fn process_device_events(
        &mut self,
        event: &DeviceEvent,
        _window: &mut Window,
        logger: &mut Logger,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta: (d_x, d_y) } if self.window_focused => {
                self.unprocessed_delta = match self.unprocessed_delta {
                    Some((x, y)) => Some((x + d_x, y + d_y)),
                    None => Some((*d_x, *d_y)),
                };
            }
            DeviceEvent::MouseWheel { delta } if self.window_focused => {
                let scroll_amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => *y as f32,
                };
                self.speed = (self.speed - (scroll_amount * 0.1)).max(0.5).min(300.0);
                logger.log(&format!("Speed: {:?}", self.speed));
            }
            _ => {}
        };
    }

    pub fn process_window_events(
        &mut self,
        event: &WindowEvent,
        window: &mut Window,
        logger: &mut Logger,
    ) {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                    }
                    VirtualKeyCode::Space => {
                        self.is_up_pressed = is_pressed;
                    }
                    VirtualKeyCode::LControl => {
                        self.is_down_pressed = is_pressed;
                    }
                    _ => {}
                }
            }
            WindowEvent::Focused(focused) => {
                logger.log(&format!("Window focused: {:?}", focused));
                window.set_cursor_grab(*focused).unwrap_or_else(|err| {
                    logger.log(&format!(
                        "Couldn't {:?} cursor: {:?}",
                        if *focused { "grab" } else { "release" },
                        err
                    ))
                });
                window.set_cursor_visible(!*focused);
                self.window_focused = *focused;
            }
            _ => {}
        };
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: f32) {
        if let Some((d_x, d_y)) = self.unprocessed_delta {
            let mouse_sensitivity = 0.003;

            self.target_pose.horizontal_rotation += Rad(-d_x as f32 * mouse_sensitivity);
            self.target_pose.vertical_rotation = Rad((self.target_pose.vertical_rotation.0
                + Rad(-d_y as f32 * mouse_sensitivity).0)
                .min(Rad::from(Deg(90.0)).0)
                .max(Rad::from(Deg(-90.0)).0));
        }
        self.unprocessed_delta = None;

        let forward_direction = camera.get_direction_vector();
        let up_direction = Vector3::new(0.0, 1.0, 0.0);
        let right_direction = forward_direction.cross(up_direction);

        let movement_vector = {
            let mut res: Option<Vector3<f32>> = None;

            let mut add_movement = |movement: Vector3<f32>| {
                res = match res {
                    Some(res) => Some(res + movement),
                    None => Some(movement),
                }
            };

            if self.is_forward_pressed {
                add_movement(forward_direction);
            } else if self.is_backward_pressed {
                add_movement(-forward_direction);
            }

            if self.is_right_pressed {
                add_movement(right_direction);
            } else if self.is_left_pressed {
                add_movement(-right_direction);
            }

            if self.is_up_pressed {
                add_movement(up_direction);
            } else if self.is_down_pressed {
                add_movement(-up_direction);
            }

            res
        };

        if let Some(movement_vector) = movement_vector {
            self.target_pose.position += movement_vector.normalize() * self.speed * dt;
        }

        let min_y = -2.0;
        if self.target_pose.position.y < min_y {
            self.target_pose.position.y = min_y;
        }

        let ema_adjusted_dt = dt * 60.0;
        let pos_lerp_factor = (0.3 * ema_adjusted_dt).min(1.0);
        // let rot_lerp_factor = (0.5 * ema_adjusted_dt).min(1.0);
        let rot_lerp_factor = 1.0;

        camera.pose.position = camera
            .pose
            .position
            .lerp(self.target_pose.position, pos_lerp_factor);

        camera.pose.vertical_rotation = Rad(lerp_f32(
            camera.pose.vertical_rotation.0,
            self.target_pose.vertical_rotation.0,
            rot_lerp_factor,
        ));

        camera.pose.horizontal_rotation = Rad(lerp_f32(
            camera.pose.horizontal_rotation.0,
            self.target_pose.horizontal_rotation.0,
            rot_lerp_factor,
        ));
    }
}
