/*
 * This file is part of ActivityStreams Traits.
 *
 * Copyright © 2018 Riley Trautman
 *
 * ActivityStreams Traits is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Traits is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Traits.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Traits for Activity Streams
//!
//! These traits don't provide any functionality other than anotations for types created in other
//! projects. See the `activitystreams-types` crate for examples of how these traits could be used.
//!
//! ## Examples
//!
//! ```rust
//! use activitystreams_traits::{Object, Actor};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone, Debug, Default, Deserialize, Serialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct Persona {
//!     #[serde(rename = "@context")]
//!     context: serde_json::Value,
//!
//!     #[serde(rename = "type")]
//!     kind: String,
//! }
//!
//! #[typetag::serde]
//! impl Object for Persona {}
//! #[typetag::serde]
//! impl Actor for Persona {}
//!
//! # fn main() {}
//! ```

mod activity;
mod actor;
mod collection;
mod link;
mod object;

pub use self::{
    activity::{Activity, IntransitiveActivity},
    actor::Actor,
    collection::{Collection, CollectionPage},
    link::Link,
    object::Object,
};
