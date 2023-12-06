use std::collections::HashMap;

macro_rules! declare_kind {
    (
    $(#[$meta:meta])*
    $vis:vis enum $name:ident {
        $($kind:ident $(,)?)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($kind,)*

        }

        impl $name {
            $vis const NAMES: &'static [&'static str] = &[$(paste![stringify!( [< $kind:snake >] )],)*];
            $vis const VALUES: &'static [$name] = &[$( $name::$kind,)*];
        }
    };
}

declare_kind! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Kind {
        Path,
        Prefix,
        Type,
    }
}

mod kind_serde {

    struct KindVisitor;

    impl<'de> serde::de::Visitor<'de> for KindVisitor {
        type Value = super::Kind;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            writeln!(formatter, "Expected one of {:?}", super::Kind::NAMES)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            super::Kind::NAMES
                .iter()
                .zip(super::Kind::VALUES)
                .find_map(|(&n, &val)| v.eq(n).then_some(val))
                .ok_or(E::unknown_variant(v, super::Kind::NAMES))
        }
    }

    impl serde::ser::Serialize for super::Kind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize(self, serializer)
        }
    }

    impl<'de> serde::de::Deserialize<'de> for super::Kind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserialize(deserializer)
        }
    }

    //#[serde(with = "module")]
    fn serialize<S: serde::ser::Serializer>(val: &super::Kind, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(
            super::Kind::NAMES
                .iter()
                .zip(super::Kind::VALUES)
                .find_map(|(&n, v)| v.eq(val).then_some(n))
                .unwrap(),
        )
    }
    fn deserialize<'de, D>(de: D) -> Result<super::Kind, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        de.deserialize_str(KindVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputFile {
    definition: HashMap<String, Definition>,
    create: HashMap<String, Vec<Create>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    headers: Vec<std::path::PathBuf>,
    sources: Vec<std::path::PathBuf>,

    replace: HashMap<String, Kind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Create {
    headers_output: std::path::PathBuf,
    sources_output: std::path::PathBuf,
    replace: HashMap<String, String>,
}
