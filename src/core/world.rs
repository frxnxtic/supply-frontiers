use std::fmt;
use std::fmt::Formatter;
use std::thread::sleep;
use std::time::Duration;
use crate::data::city::City;
use crate::data::factory::Factory;
use crate::data::transport::Transport;

pub struct World {
    game_time: GameTime,
    cities: Vec<City>,
    factories: Vec<Factory>,
    transports: Vec<Transport>
}

#[derive(Debug)]
pub struct GameTime {
    current_tick: u64
}

impl World {
    pub fn new() -> Self {
        Self {
            game_time: GameTime::new(),
            cities: vec![],
            factories: vec![],
            transports: vec![]
        }
    }

    pub fn add_city(&mut self, city: City) {
        self.cities.push(city)
    }

    pub fn add_factory(&mut self, factory: Factory) {
        self.factories.push(factory)
    }

    pub fn add_transport(&mut self, transport: Transport) {
        self.transports.push(transport)
    }

    pub fn update(&mut self, dt: u64) {
        println!("Current tick: {}", self.game_time);
        self.game_time.update(dt);

        for city in &mut self.cities {
            city.consume_resources();
        }

        for factory in &mut self.factories {
            factory.produce_resources();
        }

        for transport in &mut self.transports {
            transport.move_step();
        }

        sleep(Duration::new(1, 0));
    }
}

impl GameTime {
    fn new() -> Self {
        Self {
            current_tick: 0
        }
    }
    pub fn update(&mut self, dt: u64) {
        self.current_tick += dt
    }

}

impl fmt::Display for GameTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.current_tick)
    }
}