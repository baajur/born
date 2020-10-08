use born::{
    nested_macro,
    public_struct,
};

use serde::{Serialize, Deserialize}; 

pub fn main() {
    public_struct!(
        pub struct MessageBase {
            pub text: String,
            pub read: bool,
        }
    );

    MessageBase!(
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "camelCase")] // This file is to test this.
        pub struct Message {
            pub id: i8,
        }
    );
}