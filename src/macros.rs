//! The macros. Every one is exported at the crate root (`uuid_macros::typed_uuid!`).

/// Declares `pub struct $name(pub Uuid)` with `Display`, `FromStr` and `From` both ways.
///
/// Derives `Debug, Clone, Copy, PartialEq, Eq, Hash`. `Display` writes the hyphenated UUID; `FromStr` parses
/// any form [`uuid::Uuid::parse_str`] accepts.
///
/// ```
/// use uuid_macros::{create_uuid_id, uuid::Uuid};
///
/// create_uuid_id!(PlanetId);
///
/// let raw = Uuid::new_v4();
/// let id = PlanetId::from(raw);
/// let text = id.to_string();
/// assert_eq!(text.parse::<PlanetId>().unwrap(), id);
/// assert_eq!(Uuid::from(id), raw);
/// ```
#[macro_export]
macro_rules! create_uuid_id {
    ($name:ident) => {
        #[doc = concat!("`", stringify!($name), "`: a UUID newtype declared by `uuid_macros::create_uuid_id!`.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub $crate::uuid::Uuid);

        impl From<$crate::uuid::Uuid> for $name {
            fn from(value: $crate::uuid::Uuid) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $crate::uuid::Uuid {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::uuid::Error;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                $crate::uuid::Uuid::parse_str(s).map(Self)
            }
        }
    };
}

/// Declares `pub struct $name(pub Uuid)` with its derives and nothing else.
///
/// Derives `Debug, Clone, Copy, PartialEq, Eq, Hash`. Build values with [`new_id!`](crate::new_id!), [`named_id!`](crate::named_id!),
/// [`new_id_v5!`](crate::new_id_v5!) or [`id_from_str!`](crate::id_from_str!).
///
/// ```
/// use uuid_macros::{create_uuid_id_simple, named_id};
///
/// create_uuid_id_simple!(ShelfId);
///
/// let a = named_id!(ShelfId, "top");
/// assert_eq!(a, named_id!(ShelfId, "top"));
/// assert_eq!(a.0.get_version_num(), 5);
/// ```
#[macro_export]
macro_rules! create_uuid_id_simple {
    ($name:ident) => {
        #[doc = concat!("`", stringify!($name), "`: a UUID newtype declared by `uuid_macros::create_uuid_id_simple!`.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub $crate::uuid::Uuid);
    };
}

/// Builds `$t(Uuid::new_v5(&NAMESPACE_OID, label))`: the same label always gives the same id.
///
/// `$t` is any tuple struct whose one field is a `Uuid`; `$label` is anything with `.as_bytes()`.
///
/// ```
/// use uuid_macros::{named_id, uuid::Uuid};
///
/// struct RegionId(Uuid);
///
/// let a = named_id!(RegionId, "alpha");
/// assert_eq!(a.0, named_id!(RegionId, "alpha").0);
/// assert_eq!(a.0, Uuid::new_v5(&Uuid::NAMESPACE_OID, b"alpha"));
/// ```
#[macro_export]
macro_rules! named_id {
    ($t:ident, $label:expr) => {
        $t($crate::uuid::Uuid::new_v5(&$crate::uuid::Uuid::NAMESPACE_OID, $label.as_bytes()))
    };
}

/// Builds `$t(Uuid::new_v4())`: a random id.
///
/// ```
/// use uuid_macros::{new_id, uuid::Uuid};
///
/// struct ZoneId(Uuid);
///
/// let a = new_id!(ZoneId);
/// assert_ne!(a.0, new_id!(ZoneId).0);
/// assert_eq!(a.0.get_version_num(), 4);
/// ```
#[macro_export]
macro_rules! new_id {
    ($t:ident) => {
        $t($crate::uuid::Uuid::new_v4())
    };
}

/// Builds a two-level name-based id: `v5(v5(NAMESPACE_OID, namespace), name)`.
///
/// The same namespace and name always give the same id; the same name in another namespace gives another.
///
/// ```
/// use uuid_macros::{new_id_v5, uuid::Uuid};
///
/// struct NodeId(Uuid);
///
/// let a = new_id_v5!(NodeId, "us-west", "node-1");
/// assert_eq!(a.0, new_id_v5!(NodeId, "us-west", "node-1").0);
/// assert_ne!(a.0, new_id_v5!(NodeId, "us-east", "node-1").0);
///
/// let ns = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"us-west");
/// assert_eq!(a.0, Uuid::new_v5(&ns, b"node-1"));
/// ```
#[macro_export]
macro_rules! new_id_v5 {
    ($t:ident, $namespace:expr, $name:expr) => {{
        let ns = $crate::uuid::Uuid::new_v5(&$crate::uuid::Uuid::NAMESPACE_OID, $namespace.as_bytes());
        $t($crate::uuid::Uuid::new_v5(&ns, $name.as_bytes()))
    }};
}

/// Builds `$t(Uuid::parse_str(s))`, panicking with `invalid UUID string` when `s` is not a UUID.
///
/// For text that may be bad, parse with [`uuid::Uuid::parse_str`] (or `FromStr` on the types
/// [`typed_uuid!`](crate::typed_uuid!), [`typed_uuid_2!`](crate::typed_uuid_2!) and [`create_uuid_id!`](crate::create_uuid_id!) declare) and handle the error.
///
/// ```
/// use uuid_macros::{id_from_str, uuid::Uuid};
///
/// struct RegionId(Uuid);
///
/// let id = id_from_str!(RegionId, "123e4567-e89b-12d3-a456-426614174000");
/// assert_eq!(id.0.to_string(), "123e4567-e89b-12d3-a456-426614174000");
/// ```
///
/// ```should_panic
/// # use uuid_macros::{id_from_str, uuid::Uuid};
/// # struct RegionId(Uuid);
/// let _ = id_from_str!(RegionId, "not-a-uuid"); // panics: invalid UUID string
/// ```
#[macro_export]
macro_rules! id_from_str {
    ($t:ident, $s:expr) => {
        $t($crate::uuid::Uuid::parse_str($s).expect("invalid UUID string"))
    };
}

/// Declares the full id type for one kind of id.
///
/// `typed_uuid!(RegionId, "region")` declares `pub struct RegionId(pub Uuid)` with:
///
/// - derives `Debug, Clone, Copy, PartialEq, Eq, Hash`;
/// - `RegionId::KIND`, the label (`"region"`);
/// - `new_v4()` (random), `named(label)` (v5 of the label in `NAMESPACE_OID`), `named_v5(namespace, name)`
///   (the two-level id [`new_id_v5!`](crate::new_id_v5!) builds) and `from_str(s)`, which panics on a bad string;
/// - `Display` (hyphenated), `FromStr` (`s.parse::<RegionId>()`, returning the error instead of panicking),
///   `Deref<Target = Uuid>`, and `From` both ways;
/// - with this crate's `serde` feature, `Serialize` and `Deserialize`, written the way
///   `#[derive(Serialize, Deserialize)]` writes a newtype struct around the UUID.
///
/// `RegionId::from_str(s)` calls the inherent, panicking parse (it is kept as it was); `s.parse()` uses
/// `FromStr`.
///
/// ```
/// use uuid_macros::{typed_uuid, uuid::Uuid};
///
/// typed_uuid!(RegionId, "region");
///
/// let id = RegionId::from_str("123e4567-e89b-12d3-a456-426614174000");
/// assert_eq!(RegionId::KIND, "region");
/// assert_eq!(id.to_string(), "123e4567-e89b-12d3-a456-426614174000");
/// assert_eq!(id.to_string().parse::<RegionId>().unwrap(), id);
/// assert_eq!(RegionId::named("alpha"), RegionId::named("alpha"));
/// assert_eq!(id.get_version_num(), 1); // through Deref to the Uuid
/// let raw: Uuid = id.into();
/// assert_eq!(RegionId::from(raw), id);
/// ```
///
/// Two kinds of id are two types; one is not accepted where the other is expected:
///
/// ```compile_fail,E0308
/// uuid_macros::typed_uuid!(RegionId, "region");
/// uuid_macros::typed_uuid!(ZoneId, "zone");
///
/// fn takes_region(_: RegionId) {}
/// takes_region(ZoneId::new_v4());
/// ```
#[macro_export]
macro_rules! typed_uuid {
    ($name:ident, $kind:literal) => {
        #[doc = concat!("A `", $kind, "` id: a UUID newtype declared by `uuid_macros::typed_uuid!`.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub $crate::uuid::Uuid);

        #[allow(clippy::should_implement_trait)]
        impl $name {
            /// Returns the kind label used in macro.
            pub const KIND: &'static str = $kind;

            /// Generate a random new ID
            pub fn new_v4() -> Self {
                Self($crate::uuid::Uuid::new_v4())
            }

            /// Generate a deterministic ID from a label
            pub fn named(label: &str) -> Self {
                Self($crate::uuid::Uuid::new_v5(&$crate::uuid::Uuid::NAMESPACE_OID, label.as_bytes()))
            }

            /// Generate a nested deterministic ID from namespace + name
            pub fn named_v5(namespace: &str, name: &str) -> Self {
                let ns = $crate::uuid::Uuid::new_v5(&$crate::uuid::Uuid::NAMESPACE_OID, namespace.as_bytes());
                Self($crate::uuid::Uuid::new_v5(&ns, name.as_bytes()))
            }

            /// Parse from a string
            ///
            /// Panics with `Invalid UUID string` when `s` is not a UUID; `s.parse()` returns the error instead.
            pub fn from_str(s: &str) -> Self {
                Self($crate::uuid::Uuid::parse_str(s).expect("Invalid UUID string"))
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::uuid::Error;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                $crate::uuid::Uuid::parse_str(s).map(Self)
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $crate::uuid::Uuid;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$crate::uuid::Uuid> for $name {
            fn from(value: $crate::uuid::Uuid) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $crate::uuid::Uuid {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        $crate::__newtype_serde!($name, $crate::uuid::Uuid);
    };
}

/// Declares an ordered id type with `new` and `nil`.
///
/// `typed_uuid_2!(OrderId, "order")` declares `pub struct OrderId(pub Uuid)` with derives
/// `Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash`, `OrderId::KIND`, `new()` (random v4),
/// `nil()`, `Display`, `FromStr`, `Deref<Target = Uuid>` and `From` both ways. It has no serde impls.
///
/// ```
/// use uuid_macros::typed_uuid_2;
///
/// typed_uuid_2!(OrderId, "order");
///
/// let id = OrderId::new();
/// assert_eq!(OrderId::KIND, "order");
/// assert!(OrderId::nil() < id);
/// assert!(OrderId::nil().is_nil());
/// assert_eq!(id.to_string().parse::<OrderId>().unwrap(), id);
/// ```
#[macro_export]
macro_rules! typed_uuid_2 {
    ($name:ident, $kind:literal) => {
        #[doc = concat!("A `", $kind, "` id: a UUID newtype declared by `uuid_macros::typed_uuid_2!`.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub $crate::uuid::Uuid);

        impl ::std::ops::Deref for $name {
            type Target = $crate::uuid::Uuid;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$crate::uuid::Uuid> for $name {
            fn from(val: $crate::uuid::Uuid) -> Self {
                Self(val)
            }
        }

        impl From<$name> for $crate::uuid::Uuid {
            fn from(val: $name) -> Self {
                val.0
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::uuid::Error;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                $crate::uuid::Uuid::parse_str(s).map(Self)
            }
        }

        #[allow(clippy::new_without_default)]
        impl $name {
            /// The kind label given to the macro.
            pub const KIND: &'static str = $kind;

            /// A random (v4) id.
            pub fn new() -> Self {
                Self($crate::uuid::Uuid::new_v4())
            }

            /// The all-zero id.
            pub fn nil() -> Self {
                Self($crate::uuid::Uuid::nil())
            }
        }
    };
}

/// `Serialize`/`Deserialize` for `struct $name(pub $inner)`, as `#[derive]` writes them for a newtype struct
/// (`serialize_newtype_struct` / `deserialize_newtype_struct`). Decided by this crate's `serde` feature, not the
/// calling crate's.
#[cfg(feature = "serde")]
#[doc(hidden)]
#[macro_export]
macro_rules! __newtype_serde {
    ($name:ident, $inner:ty) => {
        impl $crate::serde::Serialize for $name {
            fn serialize<S: $crate::serde::Serializer>(
                &self,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error> {
                serializer.serialize_newtype_struct(stringify!($name), &self.0)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $name {
            fn deserialize<D: $crate::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> ::std::result::Result<Self, D::Error> {
                struct Visitor;
                impl<'de> $crate::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;
                    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(concat!("tuple struct ", stringify!($name)))
                    }
                    fn visit_newtype_struct<E: $crate::serde::Deserializer<'de>>(
                        self,
                        inner: E,
                    ) -> ::std::result::Result<$name, E::Error> {
                        <$inner as $crate::serde::Deserialize>::deserialize(inner).map($name)
                    }
                    fn visit_seq<A: $crate::serde::de::SeqAccess<'de>>(
                        self,
                        mut seq: A,
                    ) -> ::std::result::Result<$name, A::Error> {
                        match seq.next_element::<$inner>()? {
                            Some(inner) => Ok($name(inner)),
                            None => Err($crate::serde::de::Error::invalid_length(0, &self)),
                        }
                    }
                }
                deserializer.deserialize_newtype_struct(stringify!($name), Visitor)
            }
        }
    };
}

#[cfg(not(feature = "serde"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __newtype_serde {
    ($name:ident, $inner:ty) => {};
}

#[cfg(test)]
typed_uuid!(RegionId, "region");

#[test]
#[allow(unused_variables)]
fn test_typed_uuid_macros() {
    let id1 = RegionId::new_v4();
    let id2 = RegionId::named("alpha");
    let id3 = RegionId::named_v5("group", "name");
    let id4 = RegionId::from_str("123e4567-e89b-12d3-a456-426614174000");

    assert_eq!(RegionId::KIND, "region");
    assert_eq!(format!("{}", id4), "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(id4.to_string(), id4.0.to_string());

    // Deref should work
    let uuid: &uuid::Uuid = &id1;
    assert_eq!(uuid.to_string(), id1.to_string());
}


#[cfg(test)]
mod tests {
    use uuid::Uuid;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct RegionId(Uuid);
    struct ZoneId(Uuid);
    struct NodeId(Uuid);

    #[test]
    fn test_named_id() {
        let a = named_id!(RegionId, "alpha");
        let b = named_id!(RegionId, "alpha");
        let c = named_id!(RegionId, "beta");
        assert_eq!(a.0, b.0);
        assert_ne!(a.0, c.0);
    }

    #[test]
    fn test_new_id() {
        let a = new_id!(ZoneId);
        let b = new_id!(ZoneId);
        assert_ne!(a.0, b.0);
    }

    #[test]
    fn test_new_id_v5() {
        let a = new_id_v5!(NodeId, "group", "x");
        let b = new_id_v5!(NodeId, "group", "x");
        let c = new_id_v5!(NodeId, "group", "y");
        assert_eq!(a.0, b.0);
        assert_ne!(a.0, c.0);
    }

    #[test]
    fn test_id_from_str() {
        let s = "123e4567-e89b-12d3-a456-426614174000";
        let parsed = Uuid::parse_str(s).unwrap();
        let wrapped = id_from_str!(RegionId, s);
        assert_eq!(wrapped.0, parsed);
    }
}


#[cfg(test)]
mod tests_two {
    use uuid::Uuid;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RegionId(pub Uuid);
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ZoneId(pub Uuid);
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NodeId(pub Uuid);

    #[test]
    fn test_named_id_determinism() {
        let id1 = named_id!(RegionId, "north-america");
        let id2 = named_id!(RegionId, "north-america");
        let id3 = named_id!(RegionId, "europe");

        assert_eq!(id1.0, id2.0, "IDs with same label should match");
        assert_ne!(id1.0, id3.0, "IDs with different labels should differ");
    }

    #[test]
    fn test_new_id_uniqueness() {
        let id1 = new_id!(ZoneId);
        let id2 = new_id!(ZoneId);

        assert_ne!(id1.0, id2.0, "Random IDs should not be equal");
    }

    #[test]
    fn test_new_id_v5_hierarchy() {
        let id1 = new_id_v5!(NodeId, "us-west", "node-1");
        let id2 = new_id_v5!(NodeId, "us-west", "node-1");
        let id3 = new_id_v5!(NodeId, "us-west", "node-2");
        let id4 = new_id_v5!(NodeId, "us-east", "node-1");

        assert_eq!(id1.0, id2.0, "Hierarchical deterministic IDs should match");
        assert_ne!(id1.0, id3.0, "Different sub-names should yield different IDs");
        assert_ne!(id1.0, id4.0, "Different namespaces should yield different IDs");
    }

    #[test]
    fn test_id_from_str_valid() {
        let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
        let expected = Uuid::parse_str(uuid_str).unwrap();
        let id = id_from_str!(RegionId, uuid_str);

        assert_eq!(id.0, expected, "Parsed UUID should match expected");
    }

    #[test]
    #[should_panic(expected = "invalid UUID string")]
    fn test_id_from_str_invalid() {
        let _ = id_from_str!(RegionId, "not-a-uuid");
    }

    #[test]
    fn test_create_uuid_id_macro() {
        create_uuid_id!(PlanetId);

        let raw = uuid::Uuid::new_v4();
        let pid = PlanetId(raw);

        assert_eq!(pid.0, raw);
        assert_eq!(pid.to_string(), raw.to_string());

        let from_impl: PlanetId = raw.into();
        let back: uuid::Uuid = from_impl.into();
        assert_eq!(back, raw);
    }

}

#[test]
fn test_display_output() {
    let id = named_id!(RegionId, "north");
    assert!(id.to_string().contains('-'), "ID should format as UUID");
}
