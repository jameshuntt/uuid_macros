//! Macros for strongly typed UUID ids.
//!
//! Each kind of id gets its own newtype around [`uuid::Uuid`], so a `RegionId` cannot be passed where a
//! `ZoneId` is expected. The macros that declare a type:
//!
//! - [`typed_uuid!`]: the full id type (`KIND`, `new_v4`, `named`, `named_v5`, `from_str`, `Display`,
//!   `FromStr`, `Deref`, `From` both ways, and `Serialize`/`Deserialize` with the `serde` feature).
//! - [`typed_uuid_2!`]: an ordered id type (`KIND`, `new`, `nil`, `Display`, `FromStr`, `Deref`, `From`).
//! - [`create_uuid_id!`]: a plain newtype with `Display`, `FromStr` and `From` both ways.
//! - [`create_uuid_id_simple!`]: the bare newtype and its derives.
//!
//! The macros that build a value of any tuple newtype around a `Uuid`:
//! [`new_id!`] (random v4), [`named_id!`] (v5 of a label), [`new_id_v5!`] (v5 of a name inside a
//! namespace) and [`id_from_str!`] (parse, panicking on a bad string).
//!
//! ```
//! uuid_macros::typed_uuid!(UserId, "user");
//!
//! let id = UserId::named("alice");
//! assert_eq!(id, UserId::named("alice"));
//! assert_eq!(UserId::KIND, "user");
//! ```
//!
//! The macros reach `uuid` through this crate (`uuid_macros::uuid`), so a crate using them does not need its
//! own `uuid` dependency.
#![warn(missing_docs)]

#[macro_use]
pub mod macros;

pub use uuid;

/// The serde this crate's `serde` feature builds against; the generated impls name it through here.
#[cfg(feature = "serde")]
#[doc(hidden)]
pub use serde;
