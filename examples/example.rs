use serde::Deserialize;

#[derive(Deserialize)]
struct Container {
    a: A,
}
#[derive(Deserialize)]
struct A {
    #[serde(default)]
    b: B,
}

#[derive(Deserialize, serde_derive_default::Default)]
struct B {
    #[serde(default = "true_fn")]
    c: bool,
}

fn true_fn() -> bool {
    true
}

fn main() {
    let container1 = serde_yaml::from_str::<Container>("a: {}").unwrap();
    let container2 = serde_yaml::from_str::<Container>("a: {b: {}}").unwrap();
    if container1.a.b.c == container2.a.b.c {
        println!("serde and Default match!");
    } else {
        println!("serde and Default do not match, this is a bug!");
    }
}
