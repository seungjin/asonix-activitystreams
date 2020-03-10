/*
 * This file is part of ActivityStreams Types.
 *
 * Copyright © 2020 Riley Trautman
 *
 * ActivityStreams Types is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Types is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Types.  If not, see <http://www.gnu.org/licenses/>.
 */

use activitystreams_derive::PropRefs;
use activitystreams_traits::{Activity, Object};
use serde::{Deserialize, Serialize};

use super::{
    kind::ReadType,
    properties::{ActivityProperties, ReadProperties},
    ActivityExt,
};
use crate::object::{properties::ObjectProperties, ObjectExt};

/// Indicates that the actor has read the object.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
pub struct Read {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: ReadType,

    /// Adds all valid read properties to this struct
    #[serde(flatten)]
    #[activitystreams(None)]
    pub read_props: ReadProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[activitystreams(Object)]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[activitystreams(Activity)]
    pub activity_props: ActivityProperties,
}
