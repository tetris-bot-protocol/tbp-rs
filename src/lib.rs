macro_rules! gen_type {
    ($(#[$($m:tt)*])* $v:vis struct $Name:ident { $($c:tt)* } $($rest:tt)*) => {
        $(#[$($m)*])*
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
        #[serde(rename_all = "snake_case")]
        $v struct $Name {
            $($c)*
            #[serde(flatten)]
            rest: crate::Rest,
        }

        impl $Name {
            gen_type!(@trivnew $(#[$($m)*])*);
            gen_type!(@fields $($c)*);

            pub fn custom<T>(&self, name: &str) -> Option<serde_json::Result<T>>
            where
                T: serde::de::DeserializeOwned
            {
                self.rest.get(name).map(|v| serde_json::from_value(v.clone()))
            }

            pub fn with_custom(
                &mut self,
                name: impl Into<String>,
                value: &impl serde::Serialize
            ) -> &mut Self {
                self.rest.insert(name.into(), serde_json::to_value(value).unwrap());
                self
            }
        }

        gen_type!($($rest)*);
    };
    (impl $p:path { $($c:tt)* } $($rest:tt)*) => {
        impl $p { $($c)* }
        gen_type!($($rest)*);
    };
    () => {};

    (@fields $(#[$($r:tt)*])? $name:ident: $t:ty, $($rest:tt)*) => {
        pub fn $name(&self) -> &$t {
            &self.$name
        }
        paste::paste! {
            pub fn [<with_ $name>](&mut self, $name: $t) -> &mut Self {
                self.$name = $name;
                self
            }
        }

        gen_type!(@fields $($rest)*);
    };
    (@fields) => {};

    (@trivnew #[derive(Default)] $($_:tt)*) => {
        pub fn new() -> Self {
            Self::default()
        }
    };
    (@trivnew #[$($_:tt)*] $($rest:tt)*) => {
        gen_type!(@trivnew $($rest)*);
    };
    (@trivnew) => {};
}

pub mod bot_msg;
pub mod data;
pub mod frontend_msg;

pub use bot_msg::BotMessage;
pub use frontend_msg::FrontendMessage;

pub mod move_info;
pub mod randomizer;

type Rest = std::collections::HashMap<String, serde_json::Value>;

serde_big_array::big_array!(BigArray; 40);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Feature {
    Unknown(String),
}
