use born::{nested_macro, public_enum};

public_enum!(
    pub enum WebEventBase {
        PageLoad,
        PageUnload, // , here is required if you want to extend it later.
    }
);

WebEventBase!(); // You have to call it to use.

fn inspect(event: WebEventBase) {
    match event {
        WebEventBase::PageLoad => println!("page loaded"),
        WebEventBase::PageUnload => println!("page unloaded"),
    }
}

// $cargo test -- --nocapture
#[test]
fn pass_public_enum() {
    let load = WebEventBase::PageLoad;
    let unload = WebEventBase::PageUnload;

    inspect(load);
    inspect(unload);
}
