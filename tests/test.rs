use serde::Deserialize;

#[test]
fn test() {
    #[derive(Deserialize, Eq, PartialEq, Debug)]
    struct Container {
        a: A,
    }
    #[derive(Deserialize, Eq, PartialEq, Debug)]
    struct A {
        #[serde(default)]
        b: B,
    }

    #[derive(Deserialize, serde_derive_default::Default, Eq, PartialEq, Debug)]
    struct B {
        #[serde(default = "true_fn")]
        c: bool,

        #[serde(rename = "e", default = "true_fn")]
        d: bool,
    }

    fn true_fn() -> bool {
        true
    }

    let container1 = serde_yaml::from_str::<Container>("a: {}").unwrap();
    let container2 = serde_yaml::from_str::<Container>("a: {b: {}}").unwrap();
    assert_eq!(container1, container2);
}
