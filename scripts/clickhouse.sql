DROP TABLE IF EXISTS test_table;

CREATE TABLE IF NOT EXISTS test_table (
    test_int UInt64,
    test_str String
) ENGINE = MergeTree()
PRIMARY KEY test_int;

INSERT INTO test_table VALUES (1, 'abc');
INSERT INTO test_table VALUES (2, 'defg');
INSERT INTO test_table VALUES (3, 'hijkl');
INSERT INTO test_table VALUES (4, 'mnopqr');
INSERT INTO test_table VALUES (5, 'st');
INSERT INTO test_table VALUES (6, 'u');

DROP TABLE IF EXISTS test_types;

CREATE TABLE IF NOT EXISTS test_types (
    id UInt64,
    test_int8 Int8,
    test_int16 Int16,
    test_int32 Int32,
    test_int64 Int64,
    test_uint8 UInt8,
    test_uint16 UInt16,
    test_uint32 UInt32,
    test_uint64 UInt64,
    test_float32 Float32,
    test_float64 Float64,
    test_decimal32 Decimal32(3),
    test_decimal64 Decimal64(3),
    test_string String,
    test_fixed_string FixedString(10),
    test_date Date,
    test_date32 Date32,
    test_time Time,
    test_time64 Time64(3),
    test_datetime DateTime,
    test_datetime64 DateTime64(3),
    test_datetime64_tz DateTime64(3, 'Asia/Shanghai'),
    test_enum8 Enum8('a' = 1, 'b' = 2, 'c' = 3),
    test_enum16 Enum16('a' = 1, 'b' = 2, 'c' = 3),
    test_uuid UUID,
    test_ipv4 IPv4,
    test_ipv6 IPv6,
    test_bool Bool,
    test_nullable Nullable(Int32),
    test_low_cardinality LowCardinality(String)
) ENGINE = MergeTree()
PRIMARY KEY id;

INSERT INTO test_types VALUES (
    1,
    -1, -2, -3, -4,
    1, 2, 3, 4,
    1.23, 4.56,
    123.45, 678.90,
    'hello', 'world',
    '2026-01-01', '2026-01-02',
    '12:34:56', '12:34:56.789',
    '2026-01-01 12:34:56', '2026-01-01 12:34:56.789', '2026-01-01 12:34:56.789',
    'a', 'b',
    '550e8400-e29b-41d4-a716-446655440000',
    '192.168.0.1', '2001:0db8:85a3:0000:0000:8a2e:0370:7334',
    1,
    NULL,
    'low_cardinality_value'
);
