macro_rules! gen_type {
    ($(#[$($m:tt)*])* pub struct $Name:ident { $($c:tt)* } $($rest:tt)*) => {
        gen_type!(@struct [$(#[$($m)*])*] default $Name {} $($c)*);

        impl $Name {
            gen_type!(@new {} {} $($c)*);

            pub fn custom<T>(&self, name: &str) -> Option<serde_json::Result<T>>
            where
                T: serde::de::DeserializeOwned
            {
                self.rest.get(name).map(|v| serde_json::from_value(v.clone()))
            }

            pub fn set_custom(
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
    () => {};

    (@struct [$($m:tt)*] default $Name:ident { $($c:tt)* }) => {
        $($m)*
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
        #[serde(rename_all = "snake_case")]
        pub struct $Name {
            $($c)*
            #[serde(flatten)]
            rest: crate::Rest,
        }
    };
    (@struct [$($m:tt)*] nodefault $Name:ident { $($c:tt)* }) => {
        $($m)*
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
        #[serde(rename_all = "snake_case")]
        pub struct $Name {
            $($c)*
            #[serde(flatten)]
            rest: crate::Rest,
        }
    };
    (
        @struct [$($m:tt)*] $state:ident $Name:ident { $($c:tt)* }
        $(#[$($a:tt)*])* $f:ident: $t:ty,
        $($rest:tt)*
    ) => {
        gen_type! {
            @struct [$($m)*] $state $Name {
                $($c)* $(#[$($a)*])*
                #[serde(default)]
                pub $f: $t,
            } $($rest)*
        }
    };
    (
        @struct [$($m:tt)*] $state:ident $Name:ident { $($c:tt)* }
        $(#[$($a:tt)*])* required $f:ident: $t:ty,
        $($rest:tt)*
    ) => {
        gen_type! {
            @struct [$($m)*] nodefault $Name {
                $($c)* $(#[$($a)*])*
                pub $f: $t,
            } $($rest)*
        }
    };

    (@new {$($req:ident $t:ty)*} {$($def:ident)*}) => {
        pub fn new($($req: $t,)*) -> Self {
            Self {
                $($req,)*
                $($def: Default::default(),)*
                rest: Default::default(),
            }
        }
    };
    (@new {$($r:tt)*} {$($d:tt)*} $(#[$($a:tt)*])* $f:ident: $t:ty, $($rest:tt)*) => {
        gen_type!(@new {$($r)*} {$($d)* $f} $($rest)*);
    };
    (@new {$($r:tt)*} {$($d:tt)*} $(#[$($a:tt)*])* required $f:ident: $t:ty, $($rest:tt)*) => {
        gen_type!(@new {$($r)* $f $t} {$($d)*} $($rest)*);
    };
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
#[serde(untagged)]
pub enum MaybeUnknown<K> {
    Known(K),
    Unknown(serde_json::Value)
}

impl<K: Default> Default for MaybeUnknown<K> {
    fn default() -> Self {
        MaybeUnknown::Known(K::default())
    }
}

impl<K> From<K> for MaybeUnknown<K> {
    fn from(v: K) -> MaybeUnknown<K> {
        MaybeUnknown::Known(v)
    }
}
