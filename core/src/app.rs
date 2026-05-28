use legion::{Resources, Schedule, World, systems::Builder as ScheduleBuilder};
use sdk::SimulationPlugin;
use sdk::SimulationTime;
use sdk::influence::InfluenceRegistry;

pub struct App {
    pub world: World,
    pub resources: Resources,
    pub schedule_builder: ScheduleBuilder,
    pub schedule: Option<Schedule>,
}

impl App {
    pub fn new(registry: InfluenceRegistry, sim_time: SimulationTime) -> Self {
        let world = World::default();
        let mut resources = Resources::default();

        resources.insert(registry);
        resources.insert(sim_time);

        App {
            world,
            resources,
            schedule_builder: Schedule::builder(),
            schedule: None,
        }
    }

    /// Plugins call this to register their ECS systems.
    pub fn add_plugin<P: SimulationPlugin>(&mut self, plugin: P) {
        plugin.register_systems(&mut self.schedule_builder);
    }

    /// Finalize the schedule after all plugins are registered.
    pub fn finalize_schedule(&mut self) {
        self.schedule = Some(self.schedule_builder.build());
    }

    pub fn advance_tick(&mut self) {
        if let Some(schedule) = &mut self.schedule {
            schedule.execute(&mut self.world, &mut self.resources);
            let mut sim_time = self
                .resources
                .get_mut::<SimulationTime>()
                .expect("SimulationTime missing");
            sim_time.step += 1;
        } else {
            panic!("Schedule not finalized! Call finalize_schedule() after registering plugins.");
        }
    }
}
