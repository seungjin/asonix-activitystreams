# ActivityStreams
_A set of Traits and Types that make up the ActivityStreams and ActivityPub specifications_

- Find the code on [git.asonix.dog](https://git.asonix.dog/Aardwolf/activitystreams)
- Read the docs on [docs.rs](https://docs.rs/activitystreams)
- Join the matrix channel at [#activitypub:asonix.dog](https://matrix.to/#/!fAEcHyTUdAaKCzIKCt:asonix.dog?via=asonix.dog&via=matrix.org&via=t2bot.io)
- Hit me up on [mastodon](https://asonix.dog/@asonix)

## Usage

First, add ActivityStreams to your dependencies
```toml
[dependencies]
activitystreams = "0.7.0-alpha.6"
```

### Types

The project is laid out by Kind => Type

So to use an ActivityStreams Video, you'd write
```rust
use activitystreams::object::Video;
let video = Video::new();
```

And to use an ActivityPub profile, you'd write
```rust
use activitystreams::object::{ApObject, Profile};
let inner = Profile::new();
let profile = ApObject::new(inner);
```

There's only one kind of Link
```rust
use activitystreams::link::Mention;
let mention = Mention::new();
```

### Fields

Many fields on the provided types are wrapped in `OneOrMany<>` or have a type of `AnyBase`. This
is because the activitystreams spec is very open as to what is considered a valid structure.

For example, the Object type in ActivityStreams has a `summary` field, which can either be
represented as an `xsd:string` or an `rdf:langString`. It also states that the `summary` field
is not `functional`, meaning that any number of `xsd:string` or `rdf:langString`, or a
combination thereof, can be present. This library represents this as `Option<OneOrMany<AnyString>>`.

This resulting type is exactly specific enough to match the following valid ActivityStreams
json, without matching any invalid json.

With no summary:
```json
{}
```

With a string summary:
```json
{
    "summary": "A string"
}
```

With an rdf langstring
```json
{
    "summary": {
        "@value": "A string",
        "@language": "en"
    }
}
```

With multiple values
```json
{
    "summary": [
        {
            "@value": "A string",
            "@language": "en"
        },
        "An xsd:string this time"
    ]
}
```

It may seem like interacting with these types might get unweildy, there are some custom methods
implemented on the `OneOrMany` type depending on what's inside of it.

```rust,ignore
fn from_xsd_string<T>(&mut self, T) -> Self;
fn from_rdf_lang_string<T>(&mut self, T) -> Self;

fn as_single_xsd_string(&self) -> Option<&str>;
fn as_single_rdf_langstring(&self) -> Option<&RdfLangString>;

fn single_xsd_string(self) -> Option<String>;
fn single_rdf_lang_string(self) -> Option<RdfLangString>;

fn add_xsd_string<T>(&mut self, T) -> &mut Self;
fn add_rdf_lang_string<T>(&mut self, T) -> &mut Self;
```
These methods provide access to setting and fetching uniformly typed data, as well as deleting
the data. In the setter methods, the type parameter T is bound by
`Into<String>` or `Into<RdfLangString>`. This allows passing values to the method that
can be converted into the types, rather than requiring the caller to perform the conversion.

Types like `RdfLangString` can be found in the `primitives` module. Unless
you're building your own custom types, you shouldn't need to import them yourself. They each
implement `FromStr` for parsing and `Display` to convert back to strings, as well as `From` and
`Into` or `TryFrom` and `TryInto` for types you might expect them to (e.g.
`XsdNonNegativeInteger` implements `From<u64>` and `Into<u64>`).

### Traits

Since ActivityStreams is a heirarchical structure of data, it's represented as structs containing
other structs. This means that the `context` field, which can be present on any ActivityStreams type,
will be located in the innermost struct. In order to avoid writing code like
`ap_object.collection.object.base.context = Some(context())`, this library provides traits that are
automatically implmeneted for provided types.

For example, the `BaseExt` trait provides the following methods for `context`,
```rust,ignore
fn context(&self) -> Option<&OneOrMany<AnyBase>>;

fn set_context<T>(&mut self, context: T) -> &mut Self
where
    T: Into<AnyBase>;

fn set_many_contexts<I, T>(&mut self, items: I) -> &mut Self
where
    I: IntoIterator<Item = T>,
    T: Into<AnyBase>;

fn add_context<T>(&mut self, context: T) -> &mut Self
where
    T: Into<AnyBase>;

fn take_context(&mut self) -> Option<OneOrMany<AnyBase>>;
fn delete_context(&mut self) -> &mut Self;
```

For fields with more specific bounds, like `id`,
```rust,ignore
fn id(&self) -> Option<&XsdAnyUri>;
fn set_id(&mut self, XsdAnyUri) -> &mut Self;
fn take_id(&self) -> Option<XsdAnyUri>;
fn delete_id(&mut self) -> &mut Self;
```

The full list of extension traits that implement methods like these on types can be found in the
prelude module. By using `use activitystreams::prelude::*;` all of the methods will be
implemented for types containing their fields.

### Markers

This library provides a number of traits, such as `Object`, `Link`, `Actor`, `Activity`,
`Collection`, and `CollectionPage`. The majority of these traits exist solely to "mark" types,
meaning they don't provide value, at runtime, but exist to add constraints to generics at
compiletime.

If you want to make a function that manipulates an Activity, but not a normal object, you could
bound the function like so:

```rust
use activitystreams::{base::BaseExt, context, markers::Activity, uri};

fn manipulator<T, Kind>(mut activity: T) -> Result<(), anyhow::Error>
where
    T: Activity + BaseExt<Kind>,
{
    activity
        .set_id(uri!("https://example.com"))
        .set_context(context());
    Ok(())
}
```

### Kinds

This library has a set of unit structs that serialize and deserialize to strings. This is to
enable different ActivityPub Object types to be deserialized into different Named structs.
These can be found in `activitystreams::objects::kind`, and similar paths.

To build your own Person struct, for example, you could write
```rust
use activitystreams::actor::kind::PersonType;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MyPerson {
    // Do a rename since `type` is not a valid rust field name
    #[serde(rename = "type")]
    kind: PersonType,
}
```
And this type would only deserialize for JSON where `"type":"Person"`

## Examples

### Create

```rust
use activitystreams::{
    context,
    object::{ApObject, Video},
    prelude::*,
    uri,
};

fn main() -> Result<(), anyhow::Error> {
    let mut video = ApObject::new(Video::new());

    video
        .set_context(context())
        .set_id(uri!("https://example.com/@example/lions"))
        .set_media_type("video/webm".parse()?)
        .set_url(uri!("https://example.com/@example/lions/video.webm"))
        .set_summary("A cool video")
        .set_duration("PT4M20S".parse()?)
        .set_shares(uri!("https://example.com/@example/lions/video.webm#shares"));

    println!("Video, {:#?}", video);

    let s = serde_json::to_string(&video)?;

    println!("json, {}", s);

    let v: ApObject<Video> = serde_json::from_str(&s)?;

    println!("Video again, {:#?}", v);

    Ok(())
}
```

### Parse

```rust
use activitystreams::{activity::ActorAndObject, prelude::*};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum AcceptedTypes {
    Accept,
    Announce,
    Create,
    Delete,
    Follow,
    Reject,
    Update,
    Undo,
}

pub type AcceptedActivity = ActorAndObject<AcceptedTypes>;

pub fn handle_activity(activity: AcceptedActivity) -> Result<(), anyhow::Error> {
    println!("Actor: {:?}", activity.actor());
    println!("Object: {:?}", activity.object());

    match activity.kind() {
        Some(AcceptedTypes::Accept) => println!("Accept"),
        Some(AcceptedTypes::Announce) => println!("Announce"),
        Some(AcceptedTypes::Create) => println!("Create"),
        Some(AcceptedTypes::Delete) => println!("Delete"),
        Some(AcceptedTypes::Follow) => println!("Follow"),
        Some(AcceptedTypes::Reject) => println!("Reject"),
        Some(AcceptedTypes::Update) => println!("Update"),
        Some(AcceptedTypes::Undo) => println!("Undo"),
        None => return Err(anyhow::Error::msg("No activity type provided")),
    }

    Ok(())
}

static EXAMPLE_JSON: &str = r#"{"actor":"https://asonix.dog/users/asonix","object":"https://asonix.dog/users/asonix/posts/1","type":"Announce"}"#;

fn main() -> Result<(), anyhow::Error> {
    handle_activity(serde_json::from_str(EXAMPLE_JSON)?)
}
```

## Contributing
Feel free to open issues for anything you find an issue with. Please note that any contributed code will be licensed under the GPLv3.

## License

Copyright © 2020 Riley Trautman

ActivityStreams is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

ActivityStreams is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of ActivityStreams.

You should have received a copy of the GNU General Public License along with ActivityStreams. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
