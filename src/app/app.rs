use std::{any::TypeId, collections::HashMap, default, f32::consts::E};

use bevy_ecs::{
    bundle::Bundle,
    schedule::{IntoSystemConfigs, Schedule, ScheduleLabel},
    world::{EntityWorldMut, World},
};

use crate::*;

#[derive(Resource)]
pub struct DeltaTime {
    pub delta_time: f64,
}

pub struct GameManager {
    pub(crate) world: World,
    pub(crate) schedules: HashMap<ScheduleTimes, Schedule>,
}

#[derive(ScheduleLabel, Default, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ScheduleTimes {
    #[default]
    Startup,
    Update,
    Draw,
    End,
}

pub use ScheduleTimes::*;

impl GameManager {
    // pub fn key_press(&mut self, key : Key, input_action : InputAction) -> ControlFlow
    // {
    //     if key == Key::Escape
    //     {
    //         return ControlFlow::Exit
    //     }

    //     ControlFlow::Continue
    // }

    // pub fn mouse_move(&mut self, x : f64, y : f64)
    // {

    // }

    // pub fn mouse_button_down(&mut self, button : MouseButton, action : InputAction)
    // {

    // }

    pub fn step(&mut self) {
        self.schedules
            .get_mut(&Update)
            .unwrap()
            .run(&mut self.world);
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut {
        self.world.spawn(bundle)
    }

    pub fn add_systems<M>(
        &mut self,
        time: &ScheduleTimes,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.schedules.get_mut(&time).unwrap().add_systems(systems);
        self
    }
}

pub struct App<T: RenderAPI> {
    backend: T,
    pub game: GameManager,
}

impl<T: RenderAPI> App<T> {
    pub fn new() -> Self {
        let mut schedules = HashMap::new();
        // TODO: Make a system that is more viable than this
        for time in vec![Startup, Update, Draw, End] {
            schedules.insert(time, Schedule::new(time));
        }
        App {
            backend: T::init_with_window(WindowOptions {
                size: (480, 480),
                title: String::from("RenderAPI Test"),
            }),
            game: GameManager {
                world: World::new(),
                schedules,
            },
        }
    }

    pub fn run(mut self) {
        self.backend.take_control(self.game);
    }
}
