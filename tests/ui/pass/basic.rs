use serde::Deserialize;
#[derive(Deserialize, serde_derive_default::Default)]
struct Test {
    #[serde(default = "default")]
    field_1: usize,

    #[serde(rename = "field2", default = "default", flatten)]
    field_2: usize,

    test_str: String,
}

fn default() -> usize {
    1
}

fn main() {
    Test::default();
}
