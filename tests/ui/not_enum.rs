use serde_repr_plus::Serialize_repr_clamp;

#[derive(Serialize_repr_clamp)]
struct SmallPrime {
    two: u8,
    three: u8,
    five: u8,
    seven: u8,
}

fn main() {}
