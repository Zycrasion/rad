use std::{any::TypeId, collections::HashMap, default, f32::consts::E};

use bevy_ecs::{
    bundle::Bundle,
    schedule::{IntoSystemConfigs, Schedule, ScheduleLabel},
    world::{EntityWorldMut, World},
};
use prospect_obj::Object;

use crate::*;

#[derive(Resource)]
pub struct DeltaTime {
    pub delta_time: f64,
}

pub struct GameManager {
    pub world: World,
    pub(crate) schedules: HashMap<ScheduleTimes, Schedule>,
    finished_running : bool
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
    pub fn run_startup(&mut self) {
        if let Some(mut startup) = self.schedules.remove(&Startup)
        {
            startup.run(&mut self.world);
        }
    }

    pub fn end(&mut self) {
        if let Some(mut end) = self.schedules.remove(&End)
        {
            end.run(&mut self.world);
            self.finished_running = true;
        }
    }

    pub fn step_draw(&mut self) {
        self.schedules
            .get_mut(&Draw)
            .unwrap()
            .run(&mut self.world);
    }

    pub fn step_update(&mut self) {
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

    pub fn finished_running(&self) -> bool
    {
        self.finished_running
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

        let mut game = GameManager {
            world: World::new(),
            schedules,
            finished_running : false
        };

        let backend = T::init_with_window(WindowOptions {
            size: (480, 480),
            title: String::from("RenderAPI Test"),
        });

        backend.inject_systems(&mut game);

        App {
            backend,
            game,
        }
    }

    pub fn run(mut self) {
        self.backend.take_control(self.game);
    }

    pub fn register_mesh(&mut self, builder : MeshBuilder) -> AssetHandle
    {
        self.backend.create_mesh(builder)
    }
}
