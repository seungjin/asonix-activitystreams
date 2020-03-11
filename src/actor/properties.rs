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

//! Namespace for properties of standard Actor types
//!
//! To use these properties in your own types, you can flatten them into your struct with serde:
//!
//! ```rust
//! use activitystreams::{
//!     actor::properties::ApActorProperties,
//!     object::properties::{ApObjectProperties, ObjectProperties},
//!     Actor, Object, PropRefs
//! };
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone, Debug, Serialize, Deserialize, PropRefs)]
//! #[serde(rename_all = "camelCase")]
//! pub struct MyActor {
//!     #[serde(rename = "type")]
//!     pub kind: String,
//!
//!     /// Define a require property for the MyActor type
//!     pub my_property: String,
//!
//!     #[serde(flatten)]
//!     #[activitystreams(Object)]
//!     pub object_props: ObjectProperties,
//!
//!     #[serde(flatten)]
//!     #[activitystreams(None)]
//!     pub ap_object_props: ApObjectProperties,
//!
//!     #[serde(flatten)]
//!     #[activitystreams(Actor)]
//!     pub actor_props: ApActorProperties,
//! }
//! #
//! # fn main() {}
//! ```

use crate::{endpoint::EndpointProperties, primitives::XsdAnyUri, properties};

properties! {
    ApActor {
        docs [
            "Define activitypub properties for the Actor type as described by the Activity Pub vocabulary."
        ],

        inbox {
            docs [
                "A reference to an [[ActivityStreams](https://www.w3.org/ns/activitystreams)]",
                "OrderedCollection comprised of all the messages received by the actor.",
                "",
                "- Range: `anyUri`",
                "- Functional: true",
            ],
            types [ XsdAnyUri ],
            functional,
            required,
        },

        outbox {
            docs [
                "An [ActivityStreams](https://www.w3.org/ns/activitystreams)] OrderedCollection comprised of",
                "all the messages produced by the actor.",
                "",
                "- Range: `anyUri`",
                "- Functional: true",
            ],
            types [ XsdAnyUri ],
            functional,
            required,
        },

        following {
            docs [
                "A link to an [[ActivityStreams](https://www.w3.org/ns/activitystreams)] collection of the",
                "actors that this actor is following.",
                "",
                "- Range: `anyUri`",
                "- Functional: true",
            ],
            types [ XsdAnyUri ],
            functional,
        },

        followers {
            docs [
                "A link to an [[ActivityStreams](https://www.w3.org/ns/activitystreams)] collection of the",
                "actors that follow this actor.",
                "",
                "- Range: `anyUri`",
                "- Functional: true",
            ],
            types [ XsdAnyUri ],
            functional,
        },

        liked {
            docs [
                "A link to an [[ActivityStreams](https://www.w3.org/ns/activitystreams)] collection of",
                "objects this actor has liked.",
                "",
                "- Range: `anyUri`",
                "- Functional: true",
            ],
            types [ XsdAnyUri ],
            functional,
        },

        streams {
            docs [
                "A list of supplementary Collections which may be of interest.",
                "",
                "- Range: `anyUri`",
                "- Functional: false",
            ],
            types [ XsdAnyUri ],
        },

        preferred_username {
            docs [
                "A short username which may be used to refer to the actor, with no uniqueness guarantees.",
                "",
                "- Range: `anyUri`",
                "- Functional: true",
            ],
            types [ XsdAnyUri ],
            functional,
        },

        endpoints {
            docs [
                "A json object which maps additional (typically server/domain-wide) endpoints which may be",
                "useful either for this actor or someone referencing this actor.",
                "",
                "This mapping may be nested inside the actor document as the value or may be a link to a",
                "JSON-LD document with these properties.",
                "",
                "- Range: `Endpoint`",
                "- Functional: true",
            ],
            types [ EndpointProperties ],
            functional,
        },
    }
}
