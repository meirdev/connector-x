use std::net::IpAddr;

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use chrono_tz::Tz;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum DataType {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    Decimal32(Decimal),
    Decimal64(Decimal),
    String(String),
    FixedString(Vec<u8>),
    Date(NaiveDate),
    Date32(NaiveDate),
    Time(NaiveTime),
    Time64(NaiveTime),
    DateTime(DateTime<Utc>),
    DateTime64(DateTime<Utc>),
    Enum8(i8),
    Enum16(i16),
    UUID(Uuid),
    IPv4(IpAddr),
    IPv6(IpAddr),
    Bool(bool),
    Null,
    ArrayBool(Vec<Option<bool>>),
    ArrayString(Vec<Option<String>>),
    ArrayInt8(Vec<Option<i8>>),
    ArrayInt16(Vec<Option<i16>>),
    ArrayInt32(Vec<Option<i32>>),
    ArrayInt64(Vec<Option<i64>>),
    ArrayUInt8(Vec<Option<u8>>),
    ArrayUInt16(Vec<Option<u16>>),
    ArrayUInt32(Vec<Option<u32>>),
    ArrayUInt64(Vec<Option<u64>>),
    ArrayFloat32(Vec<Option<f32>>),
    ArrayFloat64(Vec<Option<f64>>),
    ArrayDecimal32(Vec<Option<Decimal>>),
    ArrayDecimal64(Vec<Option<Decimal>>),
}

#[derive(Clone, Copy, Debug)]
pub enum ClickHouseTypeSystem {
    Int8(bool),
    Int16(bool),
    Int32(bool),
    Int64(bool),
    UInt8(bool),
    UInt16(bool),
    UInt32(bool),
    UInt64(bool),
    Float32(bool),
    Float64(bool),
    Decimal32(bool),
    Decimal64(bool),
    String(bool),
    FixedString(bool),
    Date(bool),
    Date32(bool),
    Time(bool),
    Time64(bool),
    DateTime(bool),
    DateTime64(bool),
    Enum8(bool),
    Enum16(bool),
    UUID(bool),
    IPv4(bool),
    IPv6(bool),
    Bool(bool),
    // ClickHouse supports arrays of any type,
    // but for simplicity we only define arrays of primitive types here.
    ArrayBool(bool),
    ArrayString(bool),
    ArrayInt8(bool),
    ArrayInt16(bool),
    ArrayInt32(bool),
    ArrayInt64(bool),
    ArrayUInt8(bool),
    ArrayUInt16(bool),
    ArrayUInt32(bool),
    ArrayUInt64(bool),
    ArrayFloat32(bool),
    ArrayFloat64(bool),
    ArrayDecimal32(bool),
    ArrayDecimal64(bool),
}

#[derive(Clone, Debug, Default)]
pub struct TypeMetadata {
    /// Decimal
    pub scale: u8,
    /// Decimal, Time64, DateTime64
    pub precision: u8,
    /// DateTime, DateTime64
    pub timezone: Option<Tz>,
    /// Enum8, Enum16
    pub named_values: Option<Vec<(String, i16)>>,
    /// FixedString
    pub length: usize,
}

impl_typesystem! {
    system = ClickHouseTypeSystem,
    mappings = {
        { Int8 | Enum8 => i8 }
        { Int16 | Enum16 => i16 }
        { Int32 => i32 }
        { Int64 => i64 }
        { UInt8 => u8 }
        { UInt16 => u16 }
        { UInt32 => u32 }
        { UInt64 => u64 }
        { Float32 => f32 }
        { Float64 => f64 }
        { Decimal32 | Decimal64 => Decimal }
        { String => String }
        { FixedString => Vec<u8> }
        { Date | Date32 => NaiveDate }
        { Time | Time64 => NaiveTime }
        { DateTime | DateTime64 => DateTime<Utc> }
        { UUID => Uuid }
        { IPv4 | IPv6 => IpAddr }
        { Bool => bool }
        { ArrayBool => Vec<Option<bool>> }
        { ArrayString => Vec<Option<String>> }
        { ArrayInt8 => Vec<Option<i8>> }
        { ArrayInt16 => Vec<Option<i16>> }
        { ArrayInt32 => Vec<Option<i32>> }
        { ArrayInt64 => Vec<Option<i64>> }
        { ArrayUInt8 => Vec<Option<u8>> }
        { ArrayUInt16 => Vec<Option<u16>> }
        { ArrayUInt32 => Vec<Option<u32>> }
        { ArrayUInt64 => Vec<Option<u64>> }
        { ArrayFloat32 => Vec<Option<f32>> }
        { ArrayFloat64 => Vec<Option<f64>> }
        { ArrayDecimal32 | ArrayDecimal64 => Vec<Option<Decimal>> }
    }
}

impl ClickHouseTypeSystem {
    pub fn from_type_str_with_metadata(type_str: &str) -> (Self, TypeMetadata) {
        use ClickHouseTypeSystem::*;

        let type_str = type_str.trim();
        let mut metadata = TypeMetadata::default();

        let (type_str, nullable) = if type_str.starts_with("Nullable(") && type_str.ends_with(')') {
            let inner = &type_str[9..type_str.len() - 1];
            (inner, true)
        } else {
            (type_str, false)
        };

        if type_str.starts_with("LowCardinality(") && type_str.ends_with(')') {
            let inner = &type_str[15..type_str.len() - 1];
            return Self::from_type_str_with_metadata(inner);
        }

        if type_str.starts_with("Enum8(") {
            return (Enum8(nullable), metadata);
        }
        if type_str.starts_with("Enum16(") {
            return (Enum16(nullable), metadata);
        }

        // Extract base type and parameters
        let (base_type, params) = if let Some(idx) = type_str.find('(') {
            let base = &type_str[..idx];
            let params_str = &type_str[idx + 1..type_str.len() - 1];
            (base, Some(params_str))
        } else {
            (type_str, None)
        };

        let ts = match base_type {
            "Int8" => Int8(nullable),
            "Int16" => Int16(nullable),
            "Int32" => Int32(nullable),
            "Int64" => Int64(nullable),
            "UInt8" => UInt8(nullable),
            "UInt16" => UInt16(nullable),
            "UInt32" => UInt32(nullable),
            "UInt64" => UInt64(nullable),
            "Float32" => Float32(nullable),
            "Float64" => Float64(nullable),
            "Decimal" | "Decimal32" => {
                metadata.scale = Self::parse_decimal_scale(params);
                Decimal32(nullable)
            }
            "Decimal64" => {
                metadata.scale = Self::parse_decimal_scale(params);
                Decimal64(nullable)
            }
            "String" => String(nullable),
            "FixedString" => {
                metadata.length = params
                    .and_then(|p| p.trim().parse::<usize>().ok())
                    .unwrap_or(1);
                FixedString(nullable)
            }
            "Date" => Date(nullable),
            "Date32" => Date32(nullable),
            "Time" => Time(nullable),
            "Time64" => Time64(nullable),
            "DateTime" => {
                // DateTime can have optional timezone: DateTime('Asia/Istanbul')
                metadata.timezone = params.and_then(Self::parse_timezone);
                DateTime(nullable)
            }
            "DateTime64" => {
                // DateTime64(precision) or DateTime64(precision, 'timezone')
                if let Some(p) = params {
                    let parts: Vec<&str> = p.splitn(2, ',').collect();
                    metadata.precision = parts[0].trim().parse::<u8>().unwrap_or(3);
                    if parts.len() > 1 {
                        metadata.timezone = Self::parse_timezone(parts[1]);
                    }
                } else {
                    metadata.precision = 3;
                }
                DateTime64(nullable)
            }
            "UUID" => UUID(nullable),
            "Bool" => Bool(nullable),
            "IPv4" => IPv4(nullable),
            "IPv6" => IPv6(nullable),
            _ => String(nullable),
        };

        (ts, metadata)
    }

    pub fn from_type_str(type_str: &str) -> Self {
        Self::from_type_str_with_metadata(type_str).0
    }

    fn parse_decimal_scale(params: Option<&str>) -> u8 {
        params
            .and_then(|p| {
                let parts: Vec<&str> = p.split(',').collect();
                if parts.len() >= 2 {
                    parts[1].trim().parse::<u8>().ok()
                } else {
                    parts[0].trim().parse::<u8>().ok()
                }
            })
            .unwrap_or(0)
    }

    fn parse_timezone(s: &str) -> Option<Tz> {
        let s = s.trim();
        if s.starts_with('\'') && s.ends_with('\'') && s.len() > 2 {
            s[1..s.len() - 1].parse::<Tz>().ok()
        } else {
            None
        }
    }

    pub fn is_nullable(&self) -> bool {
        match self {
            ClickHouseTypeSystem::Int8(nullable)
            | ClickHouseTypeSystem::Int16(nullable)
            | ClickHouseTypeSystem::Int32(nullable)
            | ClickHouseTypeSystem::Int64(nullable)
            | ClickHouseTypeSystem::UInt8(nullable)
            | ClickHouseTypeSystem::UInt16(nullable)
            | ClickHouseTypeSystem::UInt32(nullable)
            | ClickHouseTypeSystem::UInt64(nullable)
            | ClickHouseTypeSystem::Float32(nullable)
            | ClickHouseTypeSystem::Float64(nullable)
            | ClickHouseTypeSystem::Decimal32(nullable)
            | ClickHouseTypeSystem::Decimal64(nullable)
            | ClickHouseTypeSystem::String(nullable)
            | ClickHouseTypeSystem::FixedString(nullable)
            | ClickHouseTypeSystem::Date(nullable)
            | ClickHouseTypeSystem::Date32(nullable)
            | ClickHouseTypeSystem::Time(nullable)
            | ClickHouseTypeSystem::Time64(nullable)
            | ClickHouseTypeSystem::DateTime(nullable)
            | ClickHouseTypeSystem::DateTime64(nullable)
            | ClickHouseTypeSystem::Enum8(nullable)
            | ClickHouseTypeSystem::Enum16(nullable)
            | ClickHouseTypeSystem::UUID(nullable)
            | ClickHouseTypeSystem::IPv4(nullable)
            | ClickHouseTypeSystem::IPv6(nullable)
            | ClickHouseTypeSystem::Bool(nullable)
            // It's not possible to have nullable arrays in ClickHouse,
            // but we keep the nullable flag for consistency with other types.
            | ClickHouseTypeSystem::ArrayBool(nullable)
            | ClickHouseTypeSystem::ArrayString(nullable)
            | ClickHouseTypeSystem::ArrayInt8(nullable)
            | ClickHouseTypeSystem::ArrayInt16(nullable)
            | ClickHouseTypeSystem::ArrayInt32(nullable)
            | ClickHouseTypeSystem::ArrayInt64(nullable)
            | ClickHouseTypeSystem::ArrayUInt8(nullable)
            | ClickHouseTypeSystem::ArrayUInt16(nullable)
            | ClickHouseTypeSystem::ArrayUInt32(nullable)
            | ClickHouseTypeSystem::ArrayUInt64(nullable)
            | ClickHouseTypeSystem::ArrayFloat32(nullable)
            | ClickHouseTypeSystem::ArrayFloat64(nullable)
            | ClickHouseTypeSystem::ArrayDecimal32(nullable)
            | ClickHouseTypeSystem::ArrayDecimal64(nullable) => *nullable,
        }
    }
}
