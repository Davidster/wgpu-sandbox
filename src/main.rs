mod animation;
mod ball;
mod camera;
mod camera_controller;
mod game;
mod game_state;
mod gameloop;
mod gltf_conv;
mod helpers;
mod light;
mod logger;
mod mesh;
mod renderer;
mod scene;
mod skinning;
mod texture;
mod time_tracker;
mod transform;

use animation::*;
use ball::*;
use camera::*;
use camera_controller::*;
use game::*;
use game_state::*;
use gltf_conv::*;
use helpers::*;
use light::*;
use logger::*;
use mesh::*;
use renderer::*;
use scene::*;
use skinning::*;
use texture::*;
use time_tracker::*;
use transform::*;

use cgmath::prelude::*;

async fn start() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = {
        let window = winit::window::WindowBuilder::new()
            // .with_inner_size(winit::dpi::LogicalSize::new(1000.0f32, 1000.0f32))
            // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
            .with_inner_size(winit::dpi::PhysicalSize::new(1920.0, 1080.0))
            .with_title("David's window name")
            // .with_visible(false)
            .build(&event_loop)
            .expect("Failed to create window");

        Some(window)
        // for selecting video modes!
        // if cfg!(target_os = "macos") {
        //     Some(window)
        // } else {
        //     let monitor = window.current_monitor().unwrap();
        //     let video_modes: Vec<winit::monitor::VideoMode> = monitor.video_modes().collect();
        //     let video_mode_labels: Vec<String> = video_modes
        //         .iter()
        //         .map(|video_mode| format!("{:}", video_mode))
        //         .collect();
        //     // println!("{:}", video_modes[0]);

        //     let selected_video_mode_index = dialoguer::Select::new()
        //         .items(&video_mode_labels)
        //         .default(0)
        //         .interact_opt()
        //         .expect("Dialoguer failed");

        //     match selected_video_mode_index {
        //         Some(selected_video_mode_index) => {
        //             window.set_fullscreen(Some(winit::window::Fullscreen::Exclusive(
        //                 video_modes[selected_video_mode_index].clone(),
        //             )));
        //             Some(window)
        //         }
        //         None => {
        //             println!("No video mode selected");
        //             None
        //         }
        //     }
        // }
    };
    if let Some(window) = window {
        let renderer_state_result = RendererState::new(&window).await;
        match renderer_state_result {
            Ok(renderer_state) => {
                let game_state = GameState::init();
                gameloop::run(window, event_loop, game_state, renderer_state);
            }
            Err(err) => {
                eprintln!(
                    "Error creating renderer state: {}\n{}",
                    err,
                    err.backtrace()
                )
            }
        }
    }
}

fn main() {
    env_logger::init();
    pollster::block_on(start());
}
