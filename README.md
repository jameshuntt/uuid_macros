# uuid_macros

Macros for strongly typed UUID ids: one newtype per kind of id, so a `RegionId` is never passed where a `ZoneId`
is expected.

```toml
[dependencies]
uuid_macros = "0.1"                                     # or, with Serialize/Deserialize:
# uuid_macros = { version = "0.1", features = ["serde"] }
```

```rust
uuid_macros::typed_uuid!(UserId, "user");

let id = UserId::named("alice");               // v5 of the label: the same label, the same id
assert_eq!(id, UserId::named("alice"));
assert_eq!(id.to_string().parse::<UserId>().unwrap(), id);
assert_eq!(UserId::KIND, "user");
```

## The macros

| macro | what it gives you |
|---|---|
| `typed_uuid!(Name, "kind")` | `pub struct Name(pub Uuid)`: `KIND`, `new_v4`, `named`, `named_v5`, `from_str` (panics on bad text), `Display`, `FromStr`, `Deref<Target = Uuid>`, `From` both ways; `Serialize`/`Deserialize` with the `serde` feature |
| `typed_uuid_2!(Name, "kind")` | the same newtype, also `PartialOrd`/`Ord`: `KIND`, `new`, `nil`, `Display`, `FromStr`, `Deref`, `From` both ways |
| `create_uuid_id!(Name)` | the newtype with `Display`, `FromStr` and `From` both ways |
| `create_uuid_id_simple!(Name)` | the newtype and its derives only |
| `new_id!(T)` | `T(Uuid::new_v4())` |
| `named_id!(T, label)` | `T(Uuid::new_v5(&NAMESPACE_OID, label))` |
| `new_id_v5!(T, namespace, name)` | `T(v5(v5(NAMESPACE_OID, namespace), name))` |
| `id_from_str!(T, s)` | `T(Uuid::parse_str(s))`, panicking with `invalid UUID string` |

`T` in the last four is any tuple struct whose one field is a `Uuid`. The macros reach `uuid` through
`uuid_macros::uuid`, so your crate does not need its own `uuid` dependency. The `serde` feature is this crate's:
turning it on here is what gives `typed_uuid!` types their serde impls.

## Releasing

Bump `version` in Cargo.toml and give CHANGELOG.md a dated `## <version> (YYYY-MM-DD)` entry, then push to main.
CI (`.github/workflows/ci.yml`) tests on stable and Rust 1.85, runs clippy, and publishes to crates.io
through trusted publishing; a version already on crates.io, or an entry still marked `(unreleased)`, publishes nothing.

## Ownership

Copyright © 2026 1o. All rights reserved.

Developed and published by James Hunt. The libraries are published individually, through his crates.io account
(`jameshuntt`) and his GitHub account; he intends the rights in them to go to the `1o` organization
([github.com/1o-llc](https://github.com/1o-llc)). The assignments are pending documentation. The license texts
(LICENSE-MIT, LICENSE-APACHE) are left as they are in his other crates.

## License

MIT OR Apache-2.0
