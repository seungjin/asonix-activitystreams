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

use crate::primitives::XsdString;

/// The rdf.langString type extends xs.string, and represents a language tagged string in RDF.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct RdfLangString {
    /// The content of the langstring
    ///
    /// Represented in json as "@value"
    #[serde(rename = "@value")]
    pub value: XsdString,

    /// The language identifier
    ///
    /// Represented in json as "@language"
    #[serde(rename = "@language")]
    pub language: XsdString,
}

impl std::fmt::Display for RdfLangString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.language, self.value)
    }
}
