#![feature(struct_field_attributes)] 

// See: https://www.ralphminderhoud.com/blog/simple-struct-macro/

extern crate toml;

use toml;

/// Macro that generates a `Config` and `ConfigTomlLoader`. Arguments are fields
/// of the struct and associated data.
///
/// # Usage
/// config!(my_field, String, "My defaul value".into(), "My docstring"
macro_rules! config {
    ($($element: ident, $ty: ty, $def:expr, $doc:expr); *) => {
        #[derive(Deserialize)]
        struct ConfigTomlLoader { $($element: Option<$ty>),* }

        #[derive(Deserialize, Serialize)]
        pub struct Config { 
            $(
                #[doc=$doc]
                pub $element: $ty
            ),* 
        }

        impl Config {
            pub fn default() -> Config {
                Config {
                    $($element: $def),* 
                }
            }

            pub fn from_str(s: &str) -> Config {
                let mut c = Config::default();
                let cl = ConfigTomlLoader::from_str(s).unwrap();
                $(
                    if let Some(e) = cl.$element {
                        c.$element = e;
                    };
                )*
                c
            }
        }

    }
}

config! {
    sqlite_name, String, "blotter.db".into(), "Name of sqlite database"
}

fn main() {
  let c = Config::from_str(r#""#);
}