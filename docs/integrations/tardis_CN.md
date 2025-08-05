# Tardis

Tardis为加密货币市场提供细粒度数据，包括逐tick订单簿快照和更新、交易、持仓量、资金费率、期权链和主要加密交易所的清算数据。

NautilusTrader提供与Tardis API和数据格式的集成，实现无缝访问。
此适配器的功能包括：

- `TardisCSVDataLoader`：读取Tardis格式的CSV文件并将其转换为Nautilus数据，支持批量加载和内存高效流式处理。
- `TardisMachineClient`：支持从Tardis Machine WebSocket服务器进行实时流式传输和历史数据重放 - 将消息转换为Nautilus数据。
- `TardisHttpClient`：从Tardis HTTP API请求工具定义元数据，将其解析为Nautilus工具定义。
- `TardisDataClient`：提供用于从Tardis Machine WebSocket服务器订阅数据流的实时数据客户端。
- `TardisInstrumentProvider`：通过HTTP工具元数据API从Tardis提供工具定义。
- **数据管道功能**：启用从Tardis Machine重放历史数据并将其写入Nautilus Parquet格式，包括直接目录集成以简化数据管理（见下文）。

:::info
适配器需要Tardis API密钥才能正确运行。另请参阅[环境变量](#environment-variables)。
:::

## 概述

此适配器在Rust中实现，提供可选的Python绑定以便在基于Python的工作流程中使用。
它不需要任何外部Tardis客户端库依赖项。

:::info
**不需要**为`tardis`进行额外的安装步骤。
适配器的核心组件被编译为静态库，并在构建过程中自动链接。
:::

## Tardis文档

Tardis提供详尽的用户[文档](https://docs.tardis.dev/)。
我们建议将Tardis文档与此NautilusTrader集成指南结合参考。

## 支持的格式

Tardis提供*标准化*市场数据——跨所有支持交易所一致的统一格式。
这种标准化非常有价值，因为它允许单个解析器处理来自任何[Tardis支持交易所](#venues)的数据，减少开发时间和复杂性。
因此，NautilusTrader不会支持交易所原生市场数据格式，因为在此阶段为每个交易所实现单独的解析器将是低效的。

NautilusTrader支持以下标准化Tardis格式：

| Tardis格式                                                                                                               | Nautilus数据类型                                                   |
|:----------------------------------------------------------------------------------------------------------------------------|:---------------------------------------------------------------------|
| [book_change](https://docs.tardis.dev/api/tardis-machine#book_change)                                                       | `OrderBookDelta`                                                     |
| [book_snapshot_*](https://docs.tardis.dev/api/tardis-machine#book_snapshot_-number_of_levels-_-snapshot_interval-time_unit) | `OrderBookDepth10`                                                   |
| [quote](https://docs.tardis.dev/api/tardis-machine#book_snapshot_-number_of_levels-_-snapshot_interval-time_unit)           | `QuoteTick`                                                          |
| [quote_10s](https://docs.tardis.dev/api/tardis-machine#book_snapshot_-number_of_levels-_-snapshot_interval-time_unit)       | `QuoteTick`                                                          |
| [trade](https://docs.tardis.dev/api/tardis-machine#trade)                                                                   | `Trade`                                                              |
| [trade_bar_*](https://docs.tardis.dev/api/tardis-machine#trade_bar_-aggregation_interval-suffix)                            | `Bar`                                                                |
| [instrument](https://docs.tardis.dev/api/instruments-metadata-api)                                                          | `CurrencyPair`, `CryptoFuture`, `CryptoPerpetual`, `OptionContract` |
| [derivative_ticker](https://docs.tardis.dev/api/tardis-machine#derivative_ticker)                                           | *尚未支持*                                                  |
| [disconnect](https://docs.tardis.dev/api/tardis-machine#disconnect)                                                         | *不适用*                                                     |

**备注：**

- [quote](https://docs.tardis.dev/api/tardis-machine#book_snapshot_-number_of_levels-_-snapshot_interval-time_unit)是[book_snapshot_1_0ms](https://docs.tardis.dev/api/tardis-machine#book_snapshot_-number_of_levels-_-snapshot_interval-time_unit)的别名。
- [quote_10s](https://docs.tardis.dev/api/tardis-machine#book_snapshot_-number_of_levels-_-snapshot_interval-time_unit)是[book_snapshot_1_10s](https://docs.tardis.dev/api/tardis-machine#book_snapshot_-number_of_levels-_-snapshot_interval-time_unit)的别名。
- quote、quote\_10s和一级快照都被解析为`QuoteTick`。

:::info
另请参阅Tardis [标准化市场数据API](https://docs.tardis.dev/api/tardis-machine#normalized-market-data-apis)。
:::

## K线

适配器将自动将[Tardis交易K线间隔和后缀](https://docs.tardis.dev/api/tardis-machine#trade_bar_-aggregation_interval-suffix)转换为Nautilus `BarType`。
这包括以下内容：

| Tardis后缀                                                                                                | Nautilus K线聚合    |
|:-------------------------------------------------------------------------------------------------------------|:----------------------------|
| [ms](https://docs.tardis.dev/api/tardis-machine#trade_bar_-aggregation_interval-suffix) - 毫秒       | `MILLISECOND`               |
| [s](https://docs.tardis.dev/api/tardis-machine#trade_bar_-aggregation_interval-suffix) - 秒             | `SECOND`                    |
| [m](https://docs.tardis.dev/api/tardis-machine#trade_bar_-aggregation_interval-suffix) - 分钟             | `MINUTE`                    |
| [ticks](https://docs.tardis.dev/api/tardis-machine#trade_bar_-aggregation_interval-suffix) - tick数量 | `TICK`                      |
| [vol](https://docs.tardis.dev/api/tardis-machine#trade_bar_-aggregation_interval-suffix) - 成交量大小       | `VOLUME`                    |

## 符号体系和标准化

Tardis集成通过一致地标准化符号确保与NautilusTrader的加密交易所适配器无缝兼容。通常，NautilusTrader使用Tardis提供的原生交易所命名约定。但是，对于某些交易所，原始符号会调整以遵守Nautilus符号体系标准化，如下所述：

### 通用规则

- 所有符号都转换为大写。
- 对某些交易所附加带连字符的市场类型后缀（见[交易所特定标准化](#exchange-specific-normalizations)）。
- 原始交易所符号保留在Nautilus工具定义的`raw_symbol`字段中。

### 交易所特定标准化

- **Binance**：Nautilus为所有永续符号附加后缀`-PERP`。
- **Bybit**：Nautilus使用特定的产品类别后缀，包括`-SPOT`、`-LINEAR`、`-INVERSE`、`-OPTION`。
- **dYdX**：Nautilus为所有永续符号附加后缀`-PERP`。
- **Gate.io**：Nautilus为所有永续符号附加后缀`-PERP`。

每个交易所的详细符号体系文档：

- [Binance符号体系](./binance_CN.md#symbology)
- [Bybit符号体系](./bybit_CN.md#symbology) 