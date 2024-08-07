use std::time::Duration;

// something
use rand::Rng;
use winit::{
    event::{ElementState, VirtualKeyCode},
    event_loop::EventLoop,
};

use crate::{
    app::KeyState::{Pressed, Released},
    render::RenderLoop,
};

#[derive(Default, PartialEq)]
pub enum KeyState {
    Pressed,
    #[default]
    Released,
}

#[derive(Default)]
struct Keys {
    a: KeyState,
    w: KeyState,
    s: KeyState,
    d: KeyState,
    space: KeyState,
}

pub struct App {
    render_loop: RenderLoop,
    square: Square,
    keys: Keys,
}

impl App {
    pub fn start(event_loop: &EventLoop<()>) -> Self {
        Self {
            render_loop: RenderLoop::new(event_loop),
            square: Square::new(),
            keys: Keys::default(),
        }
    }

    pub fn update(&mut self, duration_since_last_update: &Duration) {
        let seconds_passed = (duration_since_last_update.as_micros() as f32) / 1_000_000.0;

        self.update_movement(seconds_passed);

        self.render_loop.update(&self.square);
    }

    fn update_movement(&mut self, seconds_passed: f32) {
        if self.keys.w == Pressed && self.keys.s == Released {
            self.square.move_up(seconds_passed)
        }
        if self.keys.s == Pressed && self.keys.w == Released {
            self.square.move_down(seconds_passed)
        }
        if self.keys.a == Pressed && self.keys.d == Released {
            self.square.move_left(seconds_passed)
        }
        if self.keys.d == Pressed && self.keys.a == Released {
            self.square.move_right(seconds_passed)
        }
    }

    pub fn handle_keyboard_input(&mut self, key_code: VirtualKeyCode, state: ElementState) {
        let state = match state {
            ElementState::Pressed => Pressed,
            ElementState::Released => Released,
        };

        match key_code {
            VirtualKeyCode::Space => {
                if state == Pressed && self.keys.space == Released {
                    self.square.change_to_random_color();
                }
                self.keys.space = state;
            }
            VirtualKeyCode::W => self.keys.w = state,
            VirtualKeyCode::A => self.keys.a = state,
            VirtualKeyCode::S => self.keys.s = state,
            VirtualKeyCode::D => self.keys.d = state,
            _ => {}
        }
    }

    pub fn handle_window_resize(&mut self) {
        self.render_loop.handle_window_resize();
    }
}

pub struct Square {
    pub color: [f32; 3],
    pub position: [f32; 2],
    pub speed: f32,
}

impl Square {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            color: [1.0, 0.0, 0.0],
            position: [0.0, 0.0],
            speed: 1.3,
        }
    }

    pub fn change_to_random_color(&mut self) {
        let get_random_float = || rand::thread_rng().gen_range(0..100) as f32 / 100.0;
        self.color = [get_random_float(), get_random_float(), get_random_float()];
    }

    pub fn move_right(&mut self, seconds_passed: f32) {
        self.position[0] += seconds_passed * self.speed
    }

    pub fn move_left(&mut self, seconds_passed: f32) {
        self.position[0] -= seconds_passed * self.speed
    }

    pub fn move_up(&mut self, seconds_passed: f32) {
        self.position[1] -= seconds_passed * self.speed
    }

    pub fn move_down(&mut self, seconds_passed: f32) {
        self.position[1] += seconds_passed * self.speed
    }
}
