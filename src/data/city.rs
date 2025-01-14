#[derive(Debug)]
pub struct City {
    name: String,
    coordinates: (f32, f32),
    capacity: i32
}
impl City {
    pub fn new(name: String, coordinates: (f32, f32), capacity: i32) -> City {
        Self {
            name,
            coordinates,
            capacity
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_coordinates(&self) -> (f32, f32) {
        self.coordinates
    }

    pub fn get_capacity(&self) -> i32 {
        self.capacity
    }

    pub fn build_factory(&self) {
        println!("Building a factory in {name}", name = self.get_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Импортируем все из текущего модуля

    #[test]
    fn test_city_creation() {
        let city = City::new(
            String::from("New York"),
            (40.7128, -74.0060),
            1000000,
        );
        assert_eq!(city.get_name(), "New York");
        assert_eq!(city.coordinates, (40.7128, -74.0060));
        assert_eq!(city.capacity, 1000000);
    }

    #[test]
    fn test_invalid_city() {
        let city = City::new(
            String::from("Los Angeles"),
            (34.0522, -118.2437),
            500000,
        );
        assert_ne!(city.get_name(), "New York"); // Проверяем, что имя не совпадает
    }
}
