mod data;
use data::city::City as city;
use data::factory::Factory as factory;
use data::transport::Transport as transport;
use data::factory::Product as product;
use data::transport::TransportType as transport_type;

fn main() {
    let new_york = city::new(
        String::from("New York"),
        (12.42, 123.43),
        500
    );

    let steelwork = factory::new(
        String::from("Steelwork"),
        (12.42, 123.43),
        5000,
        0.45,
        product::Steel
    );

    let truck = transport::new(
        String::from("Speedy Cargo"),
        (12.42, 123.43),
        3400,
        0.45,
        transport_type::Truck
    );

    println!("City created: {:?}", new_york);
    println!("Factory created: {:?}", steelwork);
    println!("Transport created: {:?}", truck);
}