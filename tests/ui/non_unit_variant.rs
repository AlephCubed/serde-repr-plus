use serde_repr_plus::Serialize_repr_clamp;

#[derive(Serialize_repr_clamp)]
#[repr(u8)]
enum SmallPrime {
    Two(u8),
    Three(u8),
    Five(u8),
    Seven(u8),
}

fn main() {}
