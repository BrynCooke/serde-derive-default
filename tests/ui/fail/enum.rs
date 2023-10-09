use serde::Deserialize;
#[derive(Deserialize, serde_derive_default::Default)]
enum Test {
    Test(usize),

    TestStr(String),
}

fn default() -> usize {
    1
}

fn main() {}
