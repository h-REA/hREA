use chrono::{DateTime, FixedOffset, Utc, TimeZone};
use serde::{self, Deserialize, Serializer, Deserializer};
use hdk::prelude::*;

pub mod localdate {
    use super::*;

    const FORMAT: &'static str = "%+";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<FixedOffset>,
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
    ) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod localdate_optional {
    use super::*;

    const FORMAT: &'static str = "%+";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &Option<DateTime<FixedOffset>>,
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
    ) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let r: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = r {
            return Ok(Some(DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?));
        }

        Ok(None)
    }

}

pub mod localdate_undefined {
    use super::*;
    use serde_maybe_undefined::MaybeUndefined;

    const FORMAT: &'static str = "%+";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &MaybeUndefined<DateTime<FixedOffset>>,
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
    ) -> Result<MaybeUndefined<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let r: MaybeUndefined<String> = MaybeUndefined::deserialize(deserializer)?;
        debug!("input date string is {:?}", r);
        match r {
            MaybeUndefined::Some(s) => Ok(MaybeUndefined::Some(DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?)),
            MaybeUndefined::None => Ok(MaybeUndefined::None),
            MaybeUndefined::Undefined => Ok(MaybeUndefined::Undefined),
        }
    }

}

pub mod utcdate {
    use super::*;

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.f";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
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
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }

}
