// https://doc.rust-lang.org/rustdoc/documentation-tests.html#documenting-macros

/// Similar to `private_struct!` but public.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate born;
/// # fn main() {
/// public_struct!(
///     // pub is required before 'struct' when you use public_struct!
///     pub struct MessageBase {
///         pub text: String
///         // text: String // pub is optional in fields.
///     }
/// ); // It is lazy. Nothing is made yet.
///
/// MessageBase!(); // You have to call it to use the struct.
/// let message = MessageBase {
///     text: "First Message".into(),
/// };
/// println!("{}", &message.text);
/// # }
/// ```
#[macro_export]
macro_rules! public_struct {
    (pub struct $commonstruct:ident { $( $commonfieldpub:vis $commonfield:ident: $commonty:ty ),+ $(,)? }) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonstruct {
                    () => {
                        pub struct $commonstruct {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };
                    (#[derive($s($arg:tt)+)]) => {
                        #[derive($s($arg)+)]
                        pub struct $commonstruct {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };

                    (pub struct $name:ident { $s( $pub:vis $field:ident: $ty:ty ),+ $s(,)* }) => {
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                            $s( $pub $field: $ty ),+
                        }
                    };
                    (#[derive($s($arg:tt)+)] pub struct $name:ident { $s( $pub:vis $field:ident: $ty:ty ),+ $s(,)* }) => {
                        #[derive($s($arg)+)]
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                            $s( $pub $field: $ty ),+
                        }
                    };

                    (pub struct $name:ident) => {
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };
                    (#[derive($s($arg:tt)+)] pub struct $name:ident) => {
                        #[derive($s($arg)+)]
                        pub struct $name {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };
                }
            }
        }
    };
}
