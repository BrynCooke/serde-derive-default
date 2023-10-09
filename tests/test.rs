use serde::Deserialize;

#[test]
fn test() {
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

    let container1 = serde_yaml::from_str::<Container>("a: {}").unwrap();
    let container2 = serde_yaml::from_str::<Container>("a: {b: {}}").unwrap();
    assert_eq!(container1.a.b.c, container2.a.b.c);
}
