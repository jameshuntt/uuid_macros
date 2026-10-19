# Changelog

## 0.1.0 (2026-10-19)

First release. The macros are the ones in qweres (`crates/utils/uuid_macros`, the first commit of this
repository), each kept with its behaviour; what changed is what it took to publish them.

- Every macro and every generated item has documentation and an example that runs as a doctest
  (`#![warn(missing_docs)]`, so CI's clippy `-D warnings` keeps it that way).
- `typed_uuid!` serde: the source wrote `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`
  into the caller's code, so the *caller's* `serde` feature decided it and the caller needed its own `serde`
  dependency. Now this crate's `serde` feature decides it, and the impls go through `uuid_macros::serde`
  (`__newtype_serde!` in src/macros.rs). They write what the derive writes for a newtype struct
  (`serialize_newtype_struct`); in JSON an id is its UUID string (tests/round_trip.rs, `typed_uuid_serde_round_trip`).
- Added `FromStr` (error: `uuid::Error`) to `typed_uuid!`, `typed_uuid_2!` and `create_uuid_id!`, so text round
  trips without a panic. `typed_uuid!`'s inherent `from_str`, which panics, is kept, and `Name::from_str(s)` still
  calls it.
- The `RegionId` the source declared with `typed_uuid!` for its tests was a public type of the crate
  (`uuid_macros::macros::RegionId`); it is now declared under `#[cfg(test)]`.
- Dependencies say what the macros use: `uuid` with `v4` and `v5`; `serde` (optional) turns on `uuid/serde`.
  Edition 2024, rust-version 1.85 (the test suite passes on 1.85.1).
- Tests: the source's 12 unit tests unchanged (one gained `#[allow(unused_variables)]` for the two ids it builds
  and does not check), 10 round-trip tests (one per macro, plus the panic and serde cases), a trybuild test that two
  `typed_uuid!` kinds do not unify (tests/ui/kinds_do_not_unify.rs, E0308), and 11 doctests (one of them `compile_fail`).
