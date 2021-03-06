mod animation;
mod audio;
mod ball;
mod buffer;
mod camera;
mod character;
mod game;
mod game_state;
mod gameloop;
mod gltf_loader;
mod helpers;
mod light;
mod logger;
mod mesh;
mod physics;
mod physics_ball;
mod player_controller;
mod renderer;
mod revolver;
mod scene;
mod skinning;
mod texture;
mod time_tracker;
mod transform;

use animation::*;
use audio::*;
use ball::*;
use buffer::*;
use camera::*;
use character::*;
use game::*;
use game_state::*;
use gltf_loader::*;
use helpers::*;
use light::*;
use logger::*;
use mesh::*;
use physics::*;
use physics_ball::*;
use player_controller::*;
use rapier3d::prelude::*;
use renderer::*;
use revolver::*;
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
        let mut logger = Logger::new();
        let mut base_render_state = BaseRendererState::new(&window).await;

        let run_result = async {
            let (game_scene, render_buffers) = init_scene(&mut base_render_state, &mut logger)?;
            let mut renderer_state =
                RendererState::new(render_buffers, base_render_state, &mut logger).await?;
            let game_state = init_game_state(game_scene, &mut renderer_state, &mut logger)?;
            gameloop::run(window, event_loop, game_state, renderer_state, logger); // this will block while the game is running
            anyhow::Ok(())
        }
        .await;

        if let Err(err) = run_result {
            eprintln!(
                "Error setting up game / render state: {}\n{}",
                err,
                err.backtrace()
            )
        }
    }
}

fn main() {
    env_logger::init();
    pollster::block_on(start());
}
