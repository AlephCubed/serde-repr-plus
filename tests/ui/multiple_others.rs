use serde_repr_plus::Deserialize_repr_clamp;

#[derive(Deserialize_repr_clamp)]
#[repr(u8)]
enum MultipleOthers {
    #[serde(other)]
    A,
    #[serde(other)]
    B,
}

fn main() {}
