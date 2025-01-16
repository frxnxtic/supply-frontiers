#[derive(Debug)]
pub enum Product {
    Metal,
    Wood,
    Oil,
    Steel,
    Plastic,
    Glass,
    Electronics,
    Furniture,
    Clothing,
    Grain,
}

#[derive(Debug)]
pub struct Factory {
    name: String,
    coordinates: (f32, f32),
    capacity: i32,
    production_rate: f32,
    product_type: Product
}
impl Factory {
    pub fn new(name: String, coordinates: (f32, f32), capacity: i32, production_rate: f32, product_type: Product) -> Factory {
        Self {
            name,
            coordinates,
            capacity,
            production_rate,
            product_type
        }
    }


    pub fn produce(&self) -> f32 {
        self.production_rate
    }

    pub fn produce_resources(&self) {
        println!("Factory {} produced {:?}", self.name, self.product_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = Factory::new(String::from("Steelworks"), (45.0, 90.0), 1000, 50.0, Product::Metal);
        assert_eq!(factory.name, "Steelworks");
        assert_eq!(factory.coordinates, (45.0, 90.0));
        assert_eq!(factory.capacity, 1000);
        assert_eq!(factory.production_rate, 50.0);
    }

    #[test]
    fn test_production_rate() {
        let factory = Factory::new(String::from("Plastics"), (30.0, 50.0), 500, 75.0, Product::Metal);
        assert_eq!(factory.produce(), 75.0);
    }
}