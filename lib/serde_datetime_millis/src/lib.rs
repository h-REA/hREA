use paste::paste;
use chrono::{DateTime, FixedOffset, Utc, TimeZone};
use serde::{self, Deserialize, Serializer, Deserializer};

macro_rules! serde_dt_serializer {
    ( $prefix:ident, $dest_type:ident, $fmt:literal, $parser:expr ) => {
        paste! {

            pub mod $prefix {
                use super::*;

                const FORMAT: &'static str = $fmt;

                // The signature of a serialize_with function must follow the pattern:
                //
                //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
                //    where
                //        S: Serializer
                //
                // although it may also be generic over the input types T.
                pub fn serialize<S>(
                    date: &DateTime<$dest_type>,
                    serializer: S,
                ) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let s = format!("{}", date.format(FORMAT));
                    serializer.serialize_str(&s)
                }

                // The signature of a deserialize_with function must follow the pattern:
                //
                //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
                //    where
                //        D: Deserializer<'de>
                //
                // although it may also be generic over the output types T.
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> Result<DateTime<$dest_type>, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    $parser(s)
                }
            }

            pub mod [<$prefix _optional>] {
                use super::*;

                const FORMAT: &'static str = $fmt;

                // The signature of a serialize_with function must follow the pattern:
                //
                //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
                //    where
                //        S: Serializer
                //
                // although it may also be generic over the input types T.
                pub fn serialize<S>(
                    date: &Option<DateTime<$dest_type>>,
                    serializer: S,
                ) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    if date.is_none() { return serializer.serialize_none() }
                    let s = format!("{}", date.unwrap().format(FORMAT));
                    serializer.serialize_str(&s)
                }

                // The signature of a deserialize_with function must follow the pattern:
                //
                //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
                //    where
                //        D: Deserializer<'de>
                //
                // although it may also be generic over the output types T.
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> Result<Option<DateTime<$dest_type>>, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let r: Option<String> = Option::deserialize(deserializer)?;
                    if let Some(s) = r {
                        return Ok(Some($parser(s)?));
                    }

                    Ok(None)
                }

            }

            pub mod [<$prefix _undefined>] {
                use super::*;
                use serde_maybe_undefined::MaybeUndefined;

                const FORMAT: &'static str = $fmt;

                // The signature of a serialize_with function must follow the pattern:
                //
                //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
                //    where
                //        S: Serializer
                //
                // although it may also be generic over the input types T.
                pub fn serialize<S>(
                    date: &MaybeUndefined<DateTime<$dest_type>>,
                    serializer: S,
                ) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    if date.is_undefined() { return serializer.serialize_unit() }
                    if date.is_none() { return serializer.serialize_none() }
                    let s = format!("{}", date.clone().unwrap().format(FORMAT));
                    serializer.serialize_str(&s)
                }

                // The signature of a deserialize_with function must follow the pattern:
                //
                //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
                //    where
                //        D: Deserializer<'de>
                //
                // although it may also be generic over the output types T.
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> Result<MaybeUndefined<DateTime<$dest_type>>, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let r: MaybeUndefined<String> = MaybeUndefined::deserialize(deserializer)?;
                    match r {
                        MaybeUndefined::Some(s) => Ok(MaybeUndefined::Some($parser(s)?)),
                        MaybeUndefined::None => Ok(MaybeUndefined::None),
                        MaybeUndefined::Undefined => Ok(MaybeUndefined::Undefined),
                    }
                }

            }

        }
    }
}

serde_dt_serializer!(
    localdate,
    FixedOffset, "%+",
    |s: String| { DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom) }
);

serde_dt_serializer!(
    utcdate,
    Utc, "%Y-%m-%dT%H:%M:%S%.f",
    |s: String| { Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom) }
);
