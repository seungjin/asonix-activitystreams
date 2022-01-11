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

/// The type xsd:dateTime represents a specific date and time in the format
/// CCYY-MM-DDThh:mm:ss.sss, which is a concatenation of the date and time forms, separated by a
/// literal letter "T".
///
/// All of the same rules that apply to the date and time types are applicable
/// to xsd:dateTime as well.
///
/// An optional time zone expression may be added at the end of the value. The letter Z is used to
/// indicate Coordinated Universal Time (UTC). All other time zones are represented by their
/// difference from Coordinated Universal Time in the format +hh:mm, or -hh:mm. These values may
/// range from -14:00 to 14:00. For example, US Eastern Standard Time, which is five hours behind
/// UTC, is represented as -05:00. If no time zone value is present, it is considered unknown; it
/// is not assumed to be UTC.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct XsdDateTime(pub time::OffsetDateTime);

impl XsdDateTime {
    /// Create a XsdDateTime from a time::OffsetDateTime
    pub fn new(d: time::OffsetDateTime) -> Self {
        XsdDateTime(d)
    }

    /// Extract the time::OffsetDateTime from XsdDateTime
    pub fn into_inner(self) -> time::OffsetDateTime {
        self.0
    }

    /// Borrow the underlying `time::OffsetDateTime`
    pub fn as_datetime(&self) -> &time::OffsetDateTime {
        self.as_ref()
    }

    /// Mutably borrow the underlying `time::OffsetDateTime`
    pub fn as_datetime_mut(&mut self) -> &mut time::OffsetDateTime {
        self.as_mut()
    }
}

impl From<time::OffsetDateTime> for XsdDateTime {
    fn from(d: time::OffsetDateTime) -> Self {
        XsdDateTime(d)
    }
}

impl From<XsdDateTime> for time::OffsetDateTime {
    fn from(d: XsdDateTime) -> Self {
        d.0
    }
}

impl AsRef<time::OffsetDateTime> for XsdDateTime {
    fn as_ref(&self) -> &time::OffsetDateTime {
        &self.0
    }
}

impl AsMut<time::OffsetDateTime> for XsdDateTime {
    fn as_mut(&mut self) -> &mut time::OffsetDateTime {
        &mut self.0
    }
}

impl std::convert::TryFrom<String> for XsdDateTime {
    type Error = time::error::Parse;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdDateTime {
    type Error = time::error::Parse;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdDateTime {
    type Error = time::error::Parse;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdDateTime {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(XsdDateTime(time::OffsetDateTime::parse(
            s,
            &time::format_description::well_known::Rfc3339,
        )?))
    }
}

impl std::fmt::Display for XsdDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self
            .0
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(|_| std::fmt::Error)?;
        std::fmt::Display::fmt(&s, f)
    }
}

impl serde::ser::Serialize for XsdDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for XsdDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
