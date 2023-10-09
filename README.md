[![Latest Version](https://img.shields.io/crates/v/serde-derive-default.svg)](https://crates.io/crates/serde_derive_default)
# Usage

```toml
[dependencies]
serde_derive_default = "0.1"
```

```rust
#[derive(Deserialize, serde_derive_default::Default)]
struct MyStruct {
    
}
```

# Problem

When using serde defaulting users currently have to manually create a Default implementations that matches the serde field level annotations.

If you use the regular `#[derive(Default)]`, it you will get unexpected results.

For example:
```rust
#[derive(Deserialize)]
struct Container {
    a: A,
}
#[derive(Deserialize)]
struct A {
    #[serde(default)]
    b: B,
}

#[derive(Deserialize, Default)]
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
    
```
The output is:
```
serde and Default do not match, this is a bug!
```

This is because the implementation of Default disagrees with the serde defaults.


If instead `serde_serive_default::Default` is used it will use the same annotations used by serde to create the default implementation:

```rust
#[derive(Deserialize, serde_serive_default::Default)]
struct B {
    #[serde(default = "true_fn")]
    c: bool,
}
```

The output is:
```
serde and Default match!
```

Note that tha above problem only manifests when using field level annotations. If you are using container level `#[serde(default)]` then the regular `#[derive(Default)]` or a manual implementation of `Default` will work as expected.
