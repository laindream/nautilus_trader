# Databento

NautilusTrader提供了一个适配器，用于与Databento API和[Databento二进制编码（DBN）](https://databento.com/docs/standards-and-conventions/databento-binary-encoding)格式数据集成。
由于Databento纯粹是市场数据提供商，因此没有提供执行客户端——尽管仍然可以设置带有模拟执行的沙盒环境。
也可以将Databento数据与Interactive Brokers执行匹配，或为加密货币交易计算传统资产类别信号。

此适配器的功能包括：

- 从DBN文件加载历史数据并解码为Nautilus对象，用于回测或写入数据目录。
- 请求被解码为Nautilus对象的历史数据，以支持实时交易和回测。
- 订阅被解码为Nautilus对象的实时数据源，以支持实时交易和沙盒环境。

:::tip
[Databento](https://databento.com/signup)目前为新账户注册提供125美元的免费数据积分（仅限历史数据）。

通过仔细的请求，这对于测试和评估目的是绰绰有余的。
我们建议您利用[/metadata.get_cost](https://databento.com/docs/api-reference-historical/metadata/metadata-get-cost)端点。
:::

## 概述

适配器实现以[databento-rs](https://crates.io/crates/databento) crate为依赖项，这是Databento提供的官方Rust客户端库。

:::info
**不需要**可选的额外安装`databento`，因为适配器的核心组件被编译为静态库，并在构建过程中自动链接。
:::

提供以下适配器类：

- `DatabentoDataLoader`：从文件加载Databento二进制编码（DBN）数据。
- `DatabentoInstrumentProvider`：与Databento API（HTTP）集成，提供最新或历史工具定义。
- `DatabentoHistoricalClient`：与Databento API（HTTP）集成，用于历史市场数据请求。
- `DatabentoLiveClient`：与Databento API（原始TCP）集成，用于订阅实时数据源。
- `DatabentoDataClient`：提供`LiveMarketDataClient`实现，用于实时运行交易节点。

:::info
与其他集成适配器一样，大多数用户只需为实时交易节点定义配置（下面涵盖），不需要直接使用这些较低级别的组件。
:::

## 示例

您可以在[这里](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/databento/)找到实时示例脚本。

## Databento文档

Databento为新用户提供详尽的文档，可在[Databento新用户指南](https://databento.com/docs/quickstart/new-user-guides)中找到。
我们建议您结合此NautilusTrader集成指南参考Databento文档。

## Databento二进制编码（DBN）

Databento二进制编码（DBN）是用于标准化市场数据的极快消息编码和存储格式。
[DBN规范](https://databento.com/docs/standards-and-conventions/databento-binary-encoding)包括一个简单、自描述的元数据头和一组固定的结构定义，这些定义强制执行标准化市场数据的标准化方式。

该集成提供了一个解码器，可以将DBN格式数据转换为Nautilus对象。

相同的Rust实现的Nautilus解码器用于：

- 从磁盘加载和解码DBN文件
- 实时解码历史和实时数据

## 支持的模式

NautilusTrader支持以下Databento模式：

| Databento模式 | Nautilus数据类型                |
|:-----------------|:----------------------------------|
| MBO              | `OrderBookDelta`                  |
| MBP_1            | `(QuoteTick, Option<TradeTick>)`  |
| MBP_10           | `OrderBookDepth10`                |
| BBO_1S           | `QuoteTick`                       |
| BBO_1M           | `QuoteTick`                       |
| TBBO             | `(QuoteTick, TradeTick)`          |
| TRADES           | `TradeTick`                       |
| OHLCV_1S         | `Bar`                             |
| OHLCV_1M         | `Bar`                             |
| OHLCV_1H         | `Bar`                             |
| OHLCV_1D         | `Bar`                             |
| DEFINITION       | `Instrument`（各种类型）      |
| IMBALANCE        | `DatabentoImbalance`              |
| STATISTICS       | `DatabentoStatistics`             |
| STATUS           | `InstrumentStatus`                |

:::warning
NautilusTrader不再支持Databento DBN v1模式解码。
您需要将历史DBN v1数据迁移到v2或v3进行加载。
:::

:::info
另请参阅Databento [模式和数据格式](https://databento.com/docs/schemas-and-data-formats)指南。
:::

## 工具ID和符号体系

Databento市场数据包括一个`instrument_id`字段，这是由原始来源场所或Databento在标准化过程中内部分配的整数。 