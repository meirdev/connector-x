
import polars as pl

query = 'select * from flow2.flow2_metrics_network_1m limit 10000'
uri = "clickhouse://default:Rdas6510!@172.17.1.165:8123/flow2"

df = pl.read_database_uri(query=query, uri=uri, engine="connectorx")

print(df)

# import connectorx as cx

# df = cx.read_sql(conn=uri, query=query, return_type="arrow")
# print(df)

