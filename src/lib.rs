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

//! ActivityStreams
//!
//! A set of Traits and Types that make up the ActivityStreams  and ActivityPub specifications
//!
//! ## Usage
//!
//! First, add ActivityStreams to your dependencies
//! ```toml
//! activitystreams = "0.5.0"
//! ```
//!
//! ### Types
//!
//! The project is laid out by Kind => Type
//!
//! So to use an ActivityStreams Video, you'd write
//! ```rust
//! use activitystreams::object::Video;
//! ```
//!
//! And to use an ActivityStreams profile, you'd write
//! ```rust
//! use activitystreams::object::Profile;
//! ```
//!
//! ### Properties
//!
//! Each concrete type implements `AsRef<>` for each of their properties fields. A basic
//! ActivityStreams object will implement `AsRef<ObjectProperties>`.
//!
//! The Properties types can be found near the kind they're associated with. `ObjectProperties` and
//! `ApObjectProperties` are located in `activitystreams::object::properties`.
//!
//! The Properties types are generated by the `properties` macro, which attempts to create fields
//! that represent exactly the bounds of the ActivityStreams and ActivityPub specifications.
//!
//! For example, the Object type in ActivityStreams has a `summary` field, which can either be
//! represented as an `xsd:string` or an `rdf:langString`. It also states that the `summary` field
//! is not `functional`, meaning that any number of `xsd:string` or `rdf:langString`, or a
//! combination thereof, can be present. To represent this, the `properties` macro generates a
//! couple `enum` types.
//!
//! First, it generates `ObjectPropertiesSummaryTermEnum`, which is a "terminating" enum.
//! "terminating" in this context means it is the smallest unit of the type. This enum has two
//! variants, named after the types they contain, `XsdString(...)` and `RdfLangString(...)`.
//!
//! Next, it generates `ObjectPropertiesSummaryEnum`, which contains two variants, `Term(...)` and
//! `Array(...)`. The `Term` variant contains an `ObjectPropertiesSummaryTermEnum`, and the `Array`
//! variant contains a `Vec<ObjectPropertiesSummaryTermEnum>`.
//!
//! Finally, when declaring the field, it generates `summary: Option<ObjectPropertiesSummaryEnum>`,
//! since `summary` is not a required field.
//!
//! This resulting type is exactly specific enough to match the following valid ActivityStreams
//! json, without matching any invalid json.
//!
//! With no summary:
//! ```json
//! {}
//! ```
//!
//! With a sring summary:
//! ```json
//! {
//!     "summary": "A string"
//! }
//! ```
//!
//! With an rdf langstring
//! ```json
//! {
//!     "summary": {
//!         "@value": "A string",
//!         "@language": "en"
//!     }
//! }
//! ```
//!
//! With multiple values
//! ```json
//! {
//!     "summary": [
//!         {
//!             "@value": "A string",
//!             "@language": "en"
//!         },
//!         "An xsd:string this time"
//!     ]
//! }
//! ```
//!
//! It may seem like interacting with these types might get unweildy, so the `properties` macro
//! also generates methods for interacting with each field.
//!
//! ```ignore
//! fn set_summary_xsd_string<T>(&mut self, T) -> Result<...>;
//! fn set_summary_rdf_lang_string<T>(&mut self, T) -> Result<...>;
//! fn set_many_summary_xsd_strings<T>(&mut self, Vec<T>) -> Result<...>;
//! fn set_many_summary_rdf_lang_strings<T>(&mut self, Vec<T>) -> Result<...>;
//!
//! fn delete_summary(&mut self) -> &mut Self;
//!
//! fn get_summary_xsd_string(&self) -> Option<XsdString>;
//! fn get_summary_rdf_lang_string(&self) -> Option<RdfLangString>;
//! fn get_many_summary_xsd_strings(&self) -> Option<Vec<&XsdString>>;
//! fn get_many_summary_rdf_lang_strings(&self) -> Option<Vec<&RdfLangString>>;
//! ```
//! These methods provide access to setting and fetching uniformly typed data, as well as deleting
//! the data. In the setter methods, the type parameter T is bound by
//! `TryInto<XsdString>` or `TryInto<RdfLangString>`. This allows passing values to the method that
//! can be converted into the types, rather than requiring the caller to perform the conversion.
//!
//! Types like `XsdString` and `RdfLangString` can be found in the `primitives` module. Unless
//! you're building your own custom types, you shouldn't need to import them yourself. They each
//! implement `FromStr` for parsing and `Display` to convert back to strings, as well as `From` and
//! `Into` or `TryFrom` and `TryInto` for types you might expect them to (e.g.
//! `XsdNonNegativeInteger` implements `From<u64>` and `Into<u64>`).
//!
//! For some fields, like `id`, there is only one valid type. methods generated for fields like
//! these will leave out the type name from the function name.
//!
//! ```ignore
//! fn set_id<T>(&mut self, T) -> Result<...>;
//! fn delete_id(&mut self) -> &mut Self;
//! fn get_id(&self) -> Option<XsdAnyUri>;
//! ```
//!
//! ### Traits
//!
//! This library provides a number of traits, such as `Object`, `Link`, `Actor`, `Activity`,
//! `Collection`, and `CollectionPage`. The majority of these traits exist solely to "mark" types,
//! meaning they don't provide value, at runtime, but exist to add constraints to generics at
//! compiletime.
//!
//! If you want to make a function that manipulates an Activity, but not a normal object, you could
//! bound the function like so:
//! ```ignore
//! fn my_manipulator<T>(some_activity: T) -> Result<&mut ObjectProperties, SomeErrorType>
//! where
//!     T: Activity + AsMut<ObjectProperties>,
//! {
//!     some_activity.as_mut().set_whatever_tbh()
//! }
//! ```
//!
//! ### Kinds
//!
//! This library has a set of unit structs that serialize and deserialize to strings. This is to
//! enable different ActivityPub Object types to be deserialized into different Named structs.
//! These can be found in `activitystreams::objects::kind`, and similar paths.
//!
//! To build your own Person struct, for example, you could write
//! ```ignore
//! use activitystreams::actor::kind::PersonType;
//!
//! #[derive(serde::Deserialize, serde::Serialize)]
//! pub struct MyPerson {
//!     // Do a rename since `type` is not a valid rust field name
//!     #[serde(rename = "type")]
//!     kind: PersonType,
//! }
//! ```
//! And this type would only deserialize for JSON where `"type":"Person"`
//!
//! ### Extensions
//!
//! In some cases, like when dealing with ActivityPub, it is neccessary to extend the
//! ActivityStreams specification. For this purpose, two traits and a type have been introduced.
//!
//! ```ignore
//! use activitystreams::ext::{Ext, Extensible, Extension};
//! ```
//!
//! The `Ext` type is a simple record containing first, the ActivityStreams type, and second, the
//! extension to that type.
//!
//! There are two provided extensions in the ActivityStreams library.
//! - ApObjectProperties, extra properties for all ActivityStreams objects in the ActivityPub spec
//! - ApActorProperties, extra properties specifically for Actors in the ActivityPub spec
//!
//! To use an object with its default extensions, the object's `full()` associated function may be
//! invoked.
//! ```rust
//! # use activitystreams::object::Video;
//! let video_with_extensions = Video::full();
//! ```
//!
//! ### Features
//! There are a number of features that can be disabled in this crate. By default, everything is
//! enabled.
//!
//! ```toml
//! activitystreams = { version = "0.5.0", default-features = "false", features = ["derive"] }
//! ```
//!
//! | feature    | what you get                                              |
//! | ---------- | --------------------------------------------------------- |
//! | none       | Just the Marker Traits                                    |
//! | derive     | Marker Traits + derive macros from activitystreams-derive |
//! | kinds      | Marker Traits + derive macros + Kind UnitStructs          |
//! | primitives | Marker Traits + Primitive values                          |
//! | types      | Everything, this is the default                           |
//!
//! ## Examples
//!
//! ### Basic
//!
//! ```rust
//! use activitystreams::object::{Video, properties::ObjectProperties};
//! use anyhow::Error;
//!
//! // We perform configuration in a dedicated function to specify which Properties type we want to
//! // perform the operations on.
//! fn configure_video(mut v: impl AsMut<ObjectProperties>) -> Result<(), Error> {
//!     v.as_mut()
//!         .set_context_xsd_any_uri("https://www.w3.org/ns/activitystreams")?
//!         .set_id("https://example.com/@example/lions")?
//!         .set_url_xsd_any_uri("https://example.com/@example/lions/video.webm")?
//!         .set_name_xsd_string("My Cool Video")?
//!         .set_summary_xsd_string("A video about some cool lions")?
//!         .set_media_type("video/webm")?
//!         .set_duration("PT4M20S")?;
//!
//!     Ok(())
//! }
//!
//! fn main() -> Result<(), Error> {
//!     let mut v = Video::default();
//!
//!     configure_video(&mut v)?;
//!
//!     println!("Video, {:#?}", v);
//!
//!     let s = serde_json::to_string(&v)?;
//!
//!     println!("json, {}", s);
//!
//!     let v: Video = serde_json::from_str(&s)?;
//!
//!     println!("Video again, {:#?}", v);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Intermediate
//!
//! ```rust
//! use activitystreams::{
//!     context,
//!     actor::{Actor, ActorBox},
//!     ext::Ext,
//!     object::{
//!         properties::{
//!             ObjectProperties,
//!             ProfileProperties
//!         },
//!         Profile,
//!         Object,
//!         ObjectBox,
//!     },
//!     primitives::XsdAnyUri,
//!     Base, BaseBox, PropRefs,
//! };
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
//! #[serde(rename_all = "camelCase")]
//! #[prop_refs(Object)]
//! #[prop_refs(Actor)]
//! pub struct Persona {
//!     #[serde(rename = "@context")]
//!     context: XsdAnyUri,
//!
//!     #[serde(rename = "type")]
//!     kind: String,
//! }
//!
//! fn main() -> Result<(), anyhow::Error> {
//!     let mut profile = Profile::full();
//!
//!     let pprops: &mut ProfileProperties = profile.as_mut();
//!
//!     pprops.set_describes_object_box(Persona {
//!         context: context(),
//!         kind: "Persona".to_owned(),
//!     })?;
//!
//!     let oprops: &mut ObjectProperties = profile.as_mut();
//!     oprops.set_context_xsd_any_uri(context())?;
//!
//!     let profile_string = serde_json::to_string(&profile)?;
//!
//!     let profile: Profile = serde_json::from_str(&profile_string)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Advanced
//!
//! ```rust
//! use activitystreams::{
//!     properties,
//!     ext::Ext,
//!     link::{
//!         properties::LinkProperties,
//!         Link, LinkBox, Mention,
//!     },
//!     Base, BaseBox, PropRefs,
//!     UnitString,
//! };
//! use serde::{Deserialize, Serialize};
//!
//! /// Using the UnitString derive macro
//! ///
//! /// This macro implements Serialize and Deserialize for the given type, making this type
//! /// represent the string "MyLink" in JSON.
//! #[derive(Clone, Debug, Default, UnitString)]
//! #[unit_string(MyLink)]
//! pub struct MyKind;
//!
//! properties! {
//!     My {
//!         docs [ "Defining our own properties struct called MyProperties" ],
//!
//!         required_key {
//!             docs [
//!                 "Our own required key field",
//!                 "",
//!                 "'types' defines the range of values that can be stored in required_key",
//!                 "",
//!                 "'functional' means there is at most one value for required_key",
//!                 "'required' means there is at least one value for required_key",
//!             ],
//!             types [ String ],
//!             functional,
//!             required,
//!         },
//!     }
//! }
//!
//! #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
//! #[serde(transparent)]
//! pub struct MyLinkProps(pub LinkProperties);
//!
//! /// Using the Properties derive macro
//! ///
//! /// This macro generates getters and setters for the associated fields.
//! #[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
//! #[serde(rename_all = "camelCase")]
//! #[prop_refs(Link)]
//! pub struct My {
//!     /// Use the UnitString MyKind to enforce the type of the object by "MyLink"
//!     pub kind: MyKind,
//!
//!     /// Derive AsRef/AsMut for My -> MyProperties
//!     #[prop_refs]
//!     pub my_properties: MyProperties,
//!
//!     /// Derive AsRef/AsMut/Link for My -> MyLinkProperties
//!     #[prop_refs]
//!     pub link_properties: MyLinkProps,
//! }
//!
//! fn main() -> Result<(), anyhow::Error> {
//!     let mut my_link = My::default();
//!
//!     let lprops: &mut MyProperties = my_link.as_mut();
//!     lprops.set_required_key("Hey")?;
//!
//!     let my_link_string = serde_json::to_string(&my_link)?;
//!
//!     let my_link: My = serde_json::from_str(&my_link_string)?;
//!
//!     Ok(())
//! }
//! ```

pub mod activity;
pub mod actor;
pub mod collection;
#[cfg(feature = "types")]
pub mod endpoint;
pub mod ext;
pub mod link;
pub mod object;
#[cfg(feature = "primitives")]
pub mod primitives;

pub use self::{
    activity::{Activity, IntransitiveActivity},
    actor::Actor,
    collection::{Collection, CollectionPage},
    link::Link,
    object::Object,
};

#[cfg_attr(feature = "types", wrapper_type)]
/// The lowermost trait of the trait structure
///
/// Base exists solely so Object and Link can have impls that don't potentially conflict
pub trait Base: std::fmt::Debug {}

#[cfg(feature = "primitives")]
/// The context associated with all of the Activity Streams types defined in the crate.
pub fn context() -> crate::primitives::XsdAnyUri {
    "https://www.w3.org/ns/activitystreams".parse().unwrap()
}

#[cfg(feature = "primitives")]
/// The 'security' extension used by some implementations
pub fn security() -> crate::primitives::XsdAnyUri {
    "https://w3id.org/security/v1".parse().unwrap()
}

#[cfg(feature = "primitives")]
/// The 'public' actor, doesn't denote a real actor but describes a publicly available object.
pub fn public() -> crate::primitives::XsdAnyUri {
    "https://www.w3.org/ns/activitystreams#Public"
        .parse()
        .unwrap()
}

#[cfg(feature = "derive")]
pub use activitystreams_derive::{properties, wrapper_type, Extensible, PropRefs, UnitString};
