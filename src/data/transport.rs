#[derive(Debug)]
pub enum TransportType {
    Truck,
    Train,
    Ship,
    Airplane,
    Drone,
    Bicycle,
    Car,
    Bus,
    Helicopter,
    Rocket,
}

#[derive(Debug)]
pub struct Transport {
    name: String,
    coordinates: (f32, f32),
    capacity: i32,
    speed: f32,
    transport_type: TransportType
}

impl Transport {
    pub fn new(name: String, coordinates: (f32, f32), capacity: i32, speed: f32, transport_type: TransportType) -> Self {
        Self {
            name,
            coordinates,
            capacity,
            speed,
            transport_type,
        }
    }

    pub fn move_to(&mut self, new_coordinates: (f32, f32)) {
        self.coordinates = new_coordinates;
    }

    pub fn move_step(&mut self) {
        println!("Transport {} updated it's location", self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_creation() {
        let transport = Transport::new(
            String::from("Cargo Express"),
            (10.0, 20.0),
            1000,
            60.0,
            TransportType::Truck,
        );
        assert_eq!(transport.name, "Cargo Express");
        assert_eq!(transport.coordinates, (10.0, 20.0));
        assert_eq!(transport.capacity, 1000);
        assert_eq!(transport.speed, 60.0);
    }

    #[test]
    fn test_transport_movement() {
        let mut transport = Transport::new(
            String::from("Ship A"),
            (0.0, 0.0),
            2000,
            30.0,
            TransportType::Ship,
        );
        transport.move_to((100.0, 200.0));
        assert_eq!(transport.coordinates, (100.0, 200.0));
    }
}