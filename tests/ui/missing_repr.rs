use serde_repr_plus::Serialize_repr_clamp;

#[derive(Serialize_repr_clamp)]
enum SmallPrime {
    Two = 2,
    Three = 3,
    Five = 5,
    Seven = 7,
}

fn main() {}
