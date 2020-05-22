[trybuild]: https://github.com/dtolnay/trybuild
[macrotest]: https://github.com/eupn/macrotest

[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg
[doc]: https://docs.rs/doc-comment/

Publish(Struct, Enum)
=============

[![Build Status](https://travis-ci.org/steadylearner/born.svg?branch=master)](https://travis-ci.org/steadylearner/born)
[![Image Crate](https://img.shields.io/crates/v/doc-comment.svg)](https://crates.io/crates/doc-comment)
[![Image Doc](https://img.shields.io/badge/rust-documentation-blue.svg)](https://docs.rs/doc-comment/
)
[![Dependency Status](https://david-dm.org/dwyl/esta.svg)](https://david-dm.org/dwyl/esta)

It provides functional macros to reuse fields from [Struct](https://doc.rust-lang.org/std/keyword.struct.html) and [Enum](https://doc.rust-lang.org/std/keyword.enum.html) definition.

```toml
[dependencies]
born = "0.0.0"
```

<br>

## Examples

### Struct

Say you build a demo web server to send private messages.

```rust
use born::{
    nested_macro,
    public_struct,
};

// Define your base public struct here.
public_struct!(
    // pub is required here before struct
    pub struct MessageBase {
        pub text: String
        // pub text: String // , is not required for the struct definition.
    }
);

MessageBase!(
    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        pub read: bool,
        // read: bool, // pub is optional.
    }
);

impl Message {
    fn update_text(&mut self, new_message: String) {
        self.text = new_message
    }
    fn read(&mut self) {
        if self.read == false {
            self.read = true;
        }
    }
}

MessageBase!(
    #[derive(Debug, Clone, PartialEq)]
    pub struct MessageCreateRequest
);

MessageBase!(
    // #[derive(Debug, Clone, PartialEq)]
    pub struct MessageUpdateRequest
);

fn main() {
    let message_create_request = MessageCreateRequest {
        text: "I am Steadylearner and 'born' is the crate name.".into(),
    };

    let mut message = Message {
        text: message_create_request.text,
        read: false,
    };
    println!("{:#?}", &message);

    assert_eq!(message, message.clone());

    let message_update_request = MessageUpdateRequest {
        text: "Reuse fields with macros from 'born'.".into(),
    };

    message.update_text(message_update_request.text);
    println!("{:#?}", &message);

    message.read();
    println!("{:#?}", &message);
}
```

### Enum

[Compare it with the code example from the Rust documenation for Enum.](https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum.html)

```rust
use born::{
    nested_macro,
    private_enum,
};

// Define your base private enum here.
private_enum!(
    enum WebEventBase {
        PageLoad,
        PageUnload, // , here is required if you want to extend it.
    }
);

WebEventBase!(
    // #[derive(Debug, Clone, PartialEq)]
    enum WebEvent {
        KeyPress(char),
        Click { x: i64, y: i64 },
        Paste(String),
    }
);

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
```

### Rename

You can rename the struct and reuse the same fields. You can do the same with enum.

```rust
use born::{
    nested_macro,
    public_struct,
};

public_struct!(
    pub struct UserBase {
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub password: String,
    }
);

UserBase!(
    pub struct User {
        pub id: i64,
    }
);

UserBase!(
    pub struct NewUser
);

// It is equal to code it manually.

// pub struct User {
//     pub id: i64,
//     pub first_name: String,
//     pub last_name: String,
//     pub email: String,
//     pub password: String,
// }

// pub struct NewUser {
//     pub first_name: String,
//     pub last_name: String,
//     pub email: String,
//     pub password: String,
// }
```

<br>

## Details

- Each struct and enum created from the macros are completely unrelevant except they have the same fields you define.

- When you use `private_struct!` and `private_enum!`, you can't use pub keyword in it and others use them. [It wouldn't be logical if a private struct or private enum can have public fields.](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public)

- `nested_macro!` is required to use the other macros from this crate. It is used to make a macro that creates other macros.

```rust
macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)+ }
        __nested_macro!($);
    }
}
```

<br>

## Comparison with attribute macro

- [You can reuse the fields with attribute macros also.](https://github.com/steadylearner/Rust-Full-Stack/tree/master/macro/attribute) But, you need some dependencies.

- [If you want more, please read the official documenation about procedural macros.](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)

<br>

## How to test it

```console
$git clone git@github.com:steadylearner/born.git && cargo test pass
```

1. `$cargo test pass` to run passing tests.
2. `$cargo test fail` to run failing tests. You need to install [trybuild] first.

If you want to see how macros from this package expand, use `$cargo test macros`. You need to install [rustfmt](https://github.com/rust-lang/rustfmt) and [cargo-expand](https://github.com/dtolnay/cargo-expand) to use it.

```console
$rustup component add rustfmt && cargo install cargo-expand
```

[macrotest] is based on [trybuild]. They are not that compatible to test with a single command.

It makes the test to redownload the dependendencies and recompile everytime. For that reason, there are commands to test them separately.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
