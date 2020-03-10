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

/// A string type that conforms to the xsd:string specification.
///
/// TODO: Escape `<` and `&` when converting
///
/// The type xsd:string represents a character string that may contain any Unicode character
/// allowed by XML. Certain characters, namely the "less than" symbol (<) and the ampersand (&),
/// must be escaped (using the entities &lt; and &amp;, respectively) when used in strings in XML
/// instances.
///
/// The xsd:string type has a whiteSpace facet of preserve, which means that all whitespace
/// characters (spaces, tabs, carriage returns, and line feeds) are preserved by the processor.
/// This is in contrast to two types derived from it: normalizedString, and token.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct XsdString(String);

impl From<String> for XsdString {
    fn from(s: String) -> Self {
        XsdString(s)
    }
}

impl From<&str> for XsdString {
    fn from(s: &str) -> Self {
        XsdString(s.to_owned())
    }
}

impl From<&mut str> for XsdString {
    fn from(s: &mut str) -> Self {
        XsdString(s.to_owned())
    }
}

impl From<XsdString> for String {
    fn from(s: XsdString) -> Self {
        s.0
    }
}

impl AsRef<str> for XsdString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for XsdString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsMut<str> for XsdString {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl AsMut<String> for XsdString {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl std::fmt::Display for XsdString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
