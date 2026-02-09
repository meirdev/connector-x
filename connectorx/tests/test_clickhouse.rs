#![cfg(all(feature = "src_clickhouse", feature = "dst_arrow"))]

use arrow::{
    array::{Float64Array, Int64Array, StringArray},
    record_batch::RecordBatch,
};
use connectorx::{
    destinations::arrow::ArrowDestination, prelude::*, sources::clickhouse::ClickHouseSource,
    sql::CXQuery, transports::ClickHouseArrowTransport,
};
use std::env;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[test]
fn test_clickhouse() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("CLICKHOUSE_URL").unwrap();
    let rt = Arc::new(Runtime::new().unwrap());

    let queries = [
        CXQuery::naked("select * from flow2.flow2_metrics_network_1m limit 10000"),
        CXQuery::naked("select * from flow2.flow2_metrics_network_1m limit 10000 offset 10000"),
    ];

    let builder = ClickHouseSource::new(rt, &dburl).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, ClickHouseArrowTransport>::new(
        builder,
        &mut destination,
        &queries,
        Some(String::from("select * from flow2.flow2_metrics_network_1m")),
    );
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    println!("{:#?}", result);
}
