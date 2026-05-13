#![deny(warnings)]
#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Mix of required collections and an optional one to confirm only the required variants drop skip_serializing_if."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Mix of required collections and an optional one to confirm only the required variants drop skip_serializing_if.\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"optional_tags\": {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"type\": \"array\""]
#[doc = "    },"]
#[doc = "    \"required_tags\": {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"type\": \"array\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"required_tags\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MixedRequiredAndOptional {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub optional_tags: ::std::vec::Vec<::std::string::String>,
    #[serde(default)]
    pub required_tags: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for MixedRequiredAndOptional {
    fn default() -> Self {
        Self {
            optional_tags: Default::default(),
            required_tags: Default::default(),
        }
    }
}
impl MixedRequiredAndOptional {
    pub fn builder() -> builder::MixedRequiredAndOptional {
        Default::default()
    }
}
#[doc = "Required fields whose Rust types have an intrinsic default (Vec, Map). The wire contract still says required, so the generated code must emit #[serde(default)] (lenient deserialize) without skip_serializing_if (always serialize)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Required fields whose Rust types have an intrinsic default (Vec, Map). The wire contract still says required, so the generated code must emit #[serde(default)] (lenient deserialize) without skip_serializing_if (always serialize).\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"type\": \"array\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"metadata\","]
#[doc = "    \"tags\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RequiredCollections {
    #[serde(default)]
    pub metadata: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default)]
    pub tags: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for RequiredCollections {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            tags: Default::default(),
        }
    }
}
impl RequiredCollections {
    pub fn builder() -> builder::RequiredCollections {
        Default::default()
    }
}
#[doc = "Same as above but with the implicit default written out explicitly in the schema; behaviour must be identical."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Same as above but with the implicit default written out explicitly in the schema; behaviour must be identical.\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"default\": {},"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"default\": [],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"type\": \"array\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"metadata\","]
#[doc = "    \"tags\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RequiredCollectionsWithEmptyDefault {
    #[serde(default)]
    pub metadata: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default)]
    pub tags: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for RequiredCollectionsWithEmptyDefault {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            tags: Default::default(),
        }
    }
}
impl RequiredCollectionsWithEmptyDefault {
    pub fn builder() -> builder::RequiredCollectionsWithEmptyDefault {
        Default::default()
    }
}
#[doc = "`RequiredWithImplicitDefaultsRoot`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"RequiredWithImplicitDefaultsRoot\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RequiredWithImplicitDefaultsRoot(pub ::serde_json::Value);
impl ::std::ops::Deref for RequiredWithImplicitDefaultsRoot {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<RequiredWithImplicitDefaultsRoot> for ::serde_json::Value {
    fn from(value: RequiredWithImplicitDefaultsRoot) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for RequiredWithImplicitDefaultsRoot {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct MixedRequiredAndOptional {
        optional_tags:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        required_tags:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for MixedRequiredAndOptional {
        fn default() -> Self {
            Self {
                optional_tags: Ok(Default::default()),
                required_tags: Ok(Default::default()),
            }
        }
    }
    impl MixedRequiredAndOptional {
        pub fn optional_tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.optional_tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for optional_tags: {e}"));
            self
        }
        pub fn required_tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.required_tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required_tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MixedRequiredAndOptional> for super::MixedRequiredAndOptional {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MixedRequiredAndOptional,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                optional_tags: value.optional_tags?,
                required_tags: value.required_tags?,
            })
        }
    }
    impl ::std::convert::From<super::MixedRequiredAndOptional> for MixedRequiredAndOptional {
        fn from(value: super::MixedRequiredAndOptional) -> Self {
            Self {
                optional_tags: Ok(value.optional_tags),
                required_tags: Ok(value.required_tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequiredCollections {
        metadata: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        tags: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for RequiredCollections {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl RequiredCollections {
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RequiredCollections> for super::RequiredCollections {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequiredCollections,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::RequiredCollections> for RequiredCollections {
        fn from(value: super::RequiredCollections) -> Self {
            Self {
                metadata: Ok(value.metadata),
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequiredCollectionsWithEmptyDefault {
        metadata: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        tags: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for RequiredCollectionsWithEmptyDefault {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl RequiredCollectionsWithEmptyDefault {
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RequiredCollectionsWithEmptyDefault>
        for super::RequiredCollectionsWithEmptyDefault
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequiredCollectionsWithEmptyDefault,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::RequiredCollectionsWithEmptyDefault>
        for RequiredCollectionsWithEmptyDefault
    {
        fn from(value: super::RequiredCollectionsWithEmptyDefault) -> Self {
            Self {
                metadata: Ok(value.metadata),
                tags: Ok(value.tags),
            }
        }
    }
}
fn main() {}
