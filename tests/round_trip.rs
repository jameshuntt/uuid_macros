//! One test per macro: the type or value it generates, written out and read back.

use std::collections::HashSet;

use uuid_macros::uuid::Uuid;
use uuid_macros::{
    create_uuid_id, create_uuid_id_simple, id_from_str, named_id, new_id, new_id_v5, typed_uuid, typed_uuid_2,
};

const TEXT: &str = "123e4567-e89b-12d3-a456-426614174000";

create_uuid_id!(PlanetId);
create_uuid_id_simple!(ShelfId);
typed_uuid!(RegionId, "region");
typed_uuid!(ZoneId, "zone");
typed_uuid_2!(OrderId, "order");

#[test]
fn create_uuid_id_display_fromstr_from() {
    let id = PlanetId(Uuid::new_v4());
    let text = id.to_string();
    assert_eq!(text, id.0.hyphenated().to_string());
    assert_eq!(text.parse::<PlanetId>().unwrap(), id);
    assert_eq!(PlanetId::from(Uuid::from(id)), id);
    assert!("not-a-uuid".parse::<PlanetId>().is_err());
}

#[test]
fn create_uuid_id_simple_is_a_hashable_copy_newtype() {
    let a = ShelfId(Uuid::parse_str(TEXT).unwrap());
    let b = a; // Copy
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    assert_eq!(set.len(), 1);
    assert_eq!(a.0.to_string(), TEXT);
}

#[test]
fn named_id_is_v5_of_the_label_and_round_trips() {
    let id = named_id!(PlanetId, "alpha");
    assert_eq!(id.0, Uuid::new_v5(&Uuid::NAMESPACE_OID, b"alpha"));
    assert_eq!(id.to_string().parse::<PlanetId>().unwrap(), id);
    let label = String::from("alpha");
    assert_eq!(named_id!(PlanetId, label), id); // any `.as_bytes()` label
}

#[test]
fn new_id_is_random_v4_and_round_trips() {
    let id = new_id!(PlanetId);
    assert_eq!(id.0.get_version_num(), 4);
    assert_ne!(id, new_id!(PlanetId));
    assert_eq!(id.to_string().parse::<PlanetId>().unwrap(), id);
}

#[test]
fn new_id_v5_is_v5_inside_v5_and_round_trips() {
    let id = new_id_v5!(PlanetId, "us-west", "node-1");
    let ns = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"us-west");
    assert_eq!(id.0, Uuid::new_v5(&ns, b"node-1"));
    assert_eq!(id.to_string().parse::<PlanetId>().unwrap(), id);
}

#[test]
fn id_from_str_parses_and_round_trips() {
    let id = id_from_str!(PlanetId, TEXT);
    assert_eq!(id.to_string(), TEXT);
    assert_eq!(id_from_str!(PlanetId, &id.to_string()), id);
}

#[test]
fn typed_uuid_display_fromstr_and_constructors() {
    let id = RegionId::from_str(TEXT); // the inherent, panicking parse
    assert_eq!(id.to_string(), TEXT);
    assert_eq!(id.to_string().parse::<RegionId>().unwrap(), id); // FromStr
    assert!("not-a-uuid".parse::<RegionId>().is_err());
    assert_eq!(RegionId::KIND, "region");
    assert_eq!(ZoneId::KIND, "zone");
    assert_eq!(RegionId::named("alpha").0, named_id!(PlanetId, "alpha").0);
    assert_eq!(RegionId::named_v5("us-west", "node-1").0, new_id_v5!(PlanetId, "us-west", "node-1").0);
    assert_eq!(RegionId::new_v4().get_version_num(), 4);
    assert_eq!(RegionId::from(Uuid::from(id)), id);
}

#[test]
#[should_panic(expected = "Invalid UUID string")]
fn typed_uuid_inherent_from_str_panics_on_bad_text() {
    let _ = RegionId::from_str("not-a-uuid");
}

#[cfg(feature = "serde")]
#[test]
fn typed_uuid_serde_round_trip() {
    let id = RegionId::from_str(TEXT);
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, format!("\"{TEXT}\"")); // a newtype struct is its inner value in JSON
    assert_eq!(serde_json::from_str::<RegionId>(&json).unwrap(), id);
    // the same bytes as the derive on a newtype struct would write
    assert_eq!(json, serde_json::to_string(&id.0).unwrap());
    assert!(serde_json::from_str::<RegionId>("\"not-a-uuid\"").is_err());
}

#[test]
fn typed_uuid_2_display_fromstr_order_nil() {
    let id = OrderId::new();
    assert_eq!(id.get_version_num(), 4);
    assert_eq!(id.to_string().parse::<OrderId>().unwrap(), id);
    assert!(OrderId::nil().is_nil());
    assert!(OrderId::nil() < id);
    assert_eq!(OrderId::KIND, "order");
    assert_eq!(OrderId::from(Uuid::from(id)), id);
}
