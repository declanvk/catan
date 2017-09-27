extern crate rand;

mod catan;

use catan::board::{InternalCoord, ResourceType};

fn main() {
    let tile_type = ResourceType::Ore;
    println!("{:?}", tile_type);

    let center = InternalCoord::new(0, 0, 0);
    let a = InternalCoord::new(0, 1, -1);
    let b = InternalCoord::new(1, 0, -1);
    let c = InternalCoord::new(-1, 1, 0);
    let d = InternalCoord::new(1, -1, 0);
    let e = InternalCoord::new(-1, 0, 1);
    let f = InternalCoord::new(0, -1, 1);

    let not_neighbor = InternalCoord::new(-2, 1, 1);
    assert!(!center.adjacent(&not_neighbor));

    let neighbors = vec![a, b, c, d, e, f];

    for neighbor in neighbors {
        println!("{}", center.distance(&neighbor));
        assert!(center.adjacent(&neighbor));
    }
}
