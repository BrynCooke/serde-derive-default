use serde::Deserialize;
#[derive(Deserialize, serde_derive_default::Default)]
struct Test {
    #[serde(default = "default")]
    test: usize,

    test_str: String,
}

fn default() -> usize {
    1
}

fn main() {
    Test::default();
}
