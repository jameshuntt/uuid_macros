#[macro_export]
macro_rules! create_uuid_id {
    ($name:ident) => {
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

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}



#[macro_export]
macro_rules! create_uuid_id_simple {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub $crate::uuid::Uuid);
    };
}


#[macro_export]
macro_rules! named_id {
    ($t:ident, $label:expr) => {
        $t($crate::uuid::Uuid::new_v5(&$crate::uuid::Uuid::NAMESPACE_OID, $label.as_bytes()))
    };
}

#[macro_export]
macro_rules! new_id {
    ($t:ident) => {
        $t($crate::uuid::Uuid::new_v4())
    };
}

#[macro_export]
macro_rules! new_id_v5 {
    ($t:ident, $namespace:expr, $name:expr) => {{
        let ns = $crate::uuid::Uuid::new_v5(&$crate::uuid::Uuid::NAMESPACE_OID, $namespace.as_bytes());
        $t($crate::uuid::Uuid::new_v5(&ns, $name.as_bytes()))
    }};
}



#[macro_export]
macro_rules! id_from_str {
    ($t:ident, $s:expr) => {
        $t($crate::uuid::Uuid::parse_str($s).expect("invalid UUID string"))
    };
}






#[macro_export]
macro_rules! typed_uuid {
    ($name:ident, $kind:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name(pub $crate::uuid::Uuid);

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
            pub fn from_str(s: &str) -> Self {
                Self($crate::uuid::Uuid::parse_str(s).expect("Invalid UUID string"))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::ops::Deref for $name {
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
    };
}

#[macro_export]
macro_rules! typed_uuid_2 {
    ($name:ident, $kind:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub $crate::uuid::Uuid);

        impl std::ops::Deref for $name {
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

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl $name {
            pub const KIND: &'static str = $kind;

            pub fn new() -> Self {
                Self($crate::uuid::Uuid::new_v4())
            }

            pub fn nil() -> Self {
                Self($crate::uuid::Uuid::nil())
            }
        }
    };
}


typed_uuid!(RegionId, "region");

#[test]
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
