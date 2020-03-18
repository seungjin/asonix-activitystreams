/*
 * This file is part of ActivityStreams.
 *
 * Copyright © 2020 Riley Trautman
 *
 * ActivityStreams is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Namespace for Activity types

#[cfg(feature = "kinds")]
pub mod kind;
#[cfg(feature = "types")]
pub mod properties;
#[cfg(feature = "types")]
mod types;

#[cfg(feature = "types")]
pub use self::types::{
    AMove, Accept, Add, Announce, Arrive, Block, Create, Delete, Dislike, Flag, Follow, Ignore,
    Invite, Join, Leave, Like, Listen, Offer, Question, Read, Reject, Remove, TentativeAccept,
    TentativeReject, Travel, Undo, Update, View,
};

use crate::object::Object;

/// An Activity is a subtype of `Object` that describes some form of action that may happen, is
/// currently happening, or has already happened.
///
/// The `Activity` type itself serves as an abstract base type for all types of activities. It is
/// important to note that the `Activity` type itself does not carry any specific semantics about
/// the kind of action being taken.
#[cfg_attr(feature = "derive", crate::wrapper_type)]
pub trait Activity: Object {}

/// Instances of `IntransitiveActivity` are a subtype of `Activity` representing intransitive
/// actions.
///
/// The `object` property is therefore inappropriate for these activities.
#[cfg_attr(feature = "derive", crate::wrapper_type)]
pub trait IntransitiveActivity: Activity {}
