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

重要的是要认识到，这与Nautilus `InstrumentId`不同，后者是由符号+场所和句点分隔符组成的字符串，即`"{symbol}.{venue}"`。

Nautilus解码器将使用Databento `raw_symbol`作为Nautilus `symbol`，并使用来自Databento工具定义消息的[ISO 10383 MIC](https://www.iso20022.org/market-identifier-codes)（市场标识符代码）作为Nautilus `venue`。

Databento数据集用*数据集代码*标识，这与场所标识符不同。您可以在[这里](https://databento.com/docs/api-reference-historical/basics/datasets)阅读更多关于Databento数据集命名约定的信息。

特别值得注意的是，对于CME Globex MDP 3.0数据（`GLBX.MDP3`数据集代码），以下交易所都归在`GLBX`场所下。这些映射可以从工具的`exchange`字段确定：

- `CBCM`：XCME-XCBT跨交易所价差
- `NYUM`：XNYM-DUMX跨交易所价差
- `XCBT`：芝加哥期货交易所 (CBOT)
- `XCEC`：商品交易中心 (COMEX)
- `XCME`：芝加哥商品交易所 (CME)
- `XFXS`：CME FX Link价差
- `XNYM`：纽约商品交易所 (NYMEX)

:::info
可以在[metadata.list_publishers](https://databento.com/docs/api-reference-historical/metadata/metadata-list-publishers)端点响应的`venue`字段中找到其他场所MIC。
:::

## 时间戳

Databento数据包括各种时间戳字段，包括（但不限于）：

- `ts_event`：表示为自UNIX纪元以来纳秒数的匹配引擎接收时间戳。
- `ts_in_delta`：表示为`ts_recv`之前纳秒数的匹配引擎发送时间戳。
- `ts_recv`：表示为自UNIX纪元以来纳秒数的捕获服务器接收时间戳。
- `ts_out`：Databento发送时间戳。

Nautilus数据至少包括两个时间戳（由`Data`契约要求）：

- `ts_event`：数据事件发生时的UNIX时间戳（纳秒）。
- `ts_init`：创建数据对象时的UNIX时间戳（纳秒）。

在解码和标准化Databento到Nautilus时，我们通常将Databento `ts_recv`值分配给Nautilus `ts_event`字段，因为此时间戳更可靠和一致，并且保证每个工具单调递增。
此规则的例外是`DatabentoImbalance`和`DatabentoStatistics`数据类型，它们具有所有时间戳的字段，因为这些类型专门为适配器定义。

:::info
有关更多信息，请参阅以下Databento文档：

- [Databento标准和约定 - 时间戳](https://databento.com/docs/standards-and-conventions/common-fields-enums-types#timestamps)
- [Databento时间戳指南](https://databento.com/docs/architecture/timestamping-guide)

:::

## 数据类型

以下部分讨论Databento模式 -> Nautilus数据类型等价性和注意事项。

:::info
参见Databento [模式和数据格式](https://databento.com/docs/schemas-and-data-formats)。
:::

### 工具定义

Databento提供单一模式来涵盖所有工具类别，这些被解码为相应的Nautilus `Instrument`类型。

NautilusTrader支持以下Databento工具类别：

| Databento工具类别 | 代码 |  Nautilus工具类型    |
|----------------------------|------|------------------------------|
| 股票                      | `K`  | `Equity`                     |
| 期货                     | `F`  | `FuturesContract`            |
| 买权                       | `C`  | `OptionContract`             |
| 卖权                        | `P`  | `OptionContract`             |
| 期货价差              | `S`  | `FuturesSpread`              |
| 期权价差              | `T`  | `OptionSpread`               |
| 混合价差               | `M`  | `OptionSpread`               |
| FX现货                    | `X`  | `CurrencyPair`               |
| 债券                       | `B`  | 尚未可用            |

### MBO（按订单市场）

此模式是Databento提供的最高粒度数据，代表完整的订单簿深度。一些消息还提供交易信息，因此在解码MBO消息时，Nautilus将产生`OrderBookDelta`并可选地产生`TradeTick`。

Nautilus实时数据客户端将缓冲MBO消息，直到看到`F_LAST`标志。然后将离散的`OrderBookDeltas`容器对象传递给注册的处理程序。

订单簿快照也被缓冲到离散的`OrderBookDeltas`容器对象中，这发生在重放启动序列期间。

### MBP-1（按价格市场，顶级订单簿）

此模式仅代表顶级订单簿（报价*和*交易）。与MBO消息一样，一些消息携带交易信息，因此在解码MBP-1消息时，Nautilus将产生`QuoteTick`，如果消息是交易，*还*会产生`TradeTick`。

### OHLCV（K线聚合）

Databento K线聚合消息在K线间隔的**开盘**时加时间戳。
Nautilus解码器将标准化`ts_event`时间戳到K线的**收盘**（原始`ts_event` + K线间隔）。

### 不平衡和统计

Databento `imbalance`和`statistics`模式无法表示为内置的Nautilus数据类型，因此在Rust中定义了特定类型`DatabentoImbalance`和`DatabentoStatistics`。
通过pyo3（Rust）提供Python绑定，因此这些类型的行为与内置Nautilus数据类型略有不同，其中所有属性都是pyo3提供的对象，不直接与某些可能期望Cython提供类型的方法兼容。有pyo3 -> 传统Cython对象转换方法可用，可在API参考中找到。

以下是将pyo3 `Price`转换为Cython `Price`的一般模式：

```python
price = Price.from_raw(pyo3_price.raw, pyo3_price.precision)
```

此外，请求和订阅这些数据类型需要使用自定义数据类型的低级通用方法。以下示例订阅`AAPL.XNAS`工具（在纳斯达克交易所交易的Apple Inc）的`imbalance`模式：

```python
from nautilus_trader.adapters.databento import DATABENTO_CLIENT_ID
from nautilus_trader.adapters.databento import DatabentoImbalance
from nautilus_trader.model import DataType

instrument_id = InstrumentId.from_str("AAPL.XNAS")
self.subscribe_data(
    data_type=DataType(DatabentoImbalance, metadata={"instrument_id": instrument_id}),
    client_id=DATABENTO_CLIENT_ID,
)
```

或请求`ES.FUT`父符号（CME Globex交易所上所有活跃的E-mini S&P 500期货合约）前一天的`statistics`模式：

```python
from nautilus_trader.adapters.databento import DATABENTO_CLIENT_ID
from nautilus_trader.adapters.databento import DatabentoStatistics
from nautilus_trader.model import DataType

instrument_id = InstrumentId.from_str("ES.FUT.GLBX")
metadata = {
    "instrument_id": instrument_id,
    "start": "2024-03-06",
}
self.request_data(
    data_type=DataType(DatabentoStatistics, metadata=metadata),
    client_id=DATABENTO_CLIENT_ID,
)
```

## 性能注意事项

使用Databento DBN数据进行回测时，有两个选择：

- 将数据存储在DBN (`.dbn.zst`) 格式文件中，并在每次运行时解码为Nautilus对象
- 将DBN文件转换为Nautilus对象，然后一次性写入数据目录（存储为磁盘上的Nautilus Parquet格式）

虽然DBN -> Nautilus解码器用Rust实现并已经过优化，但通过将Nautilus对象写入数据目录（执行一次解码步骤）可以获得回测的最佳性能。

[DataFusion](https://arrow.apache.org/datafusion/)提供查询引擎后端，可有效地从磁盘加载和流式传输Nautilus Parquet数据，从而实现极高的吞吐量（比为每次回测运行即时转换DBN -> Nautilus至少快一个数量级）。

:::note
性能基准测试目前正在开发中。
:::

## 加载DBN数据

您可以使用`DatabentoDataLoader`类加载DBN文件并将记录转换为Nautilus对象。这样做有两个主要目的：

- 将转换后的数据直接传递给`BacktestEngine.add_data`进行回测。
- 将转换后的数据传递给`ParquetDataCatalog.write_data`以供后续与`BacktestNode`一起流式使用。

### DBN数据到BacktestEngine

此代码片段演示如何加载DBN数据并传递给`BacktestEngine`。
由于`BacktestEngine`需要添加工具，我们将使用`TestInstrumentProvider`提供的测试工具（您也可以传递从DBN文件解析的工具对象）。
数据是一个月的TSLA（特斯拉公司）在纳斯达克交易所的交易：

```python
# 添加工具
TSLA_NASDAQ = TestInstrumentProvider.equity(symbol="TSLA")
engine.add_instrument(TSLA_NASDAQ)

# 解码数据为传统Cython对象
loader = DatabentoDataLoader()
trades = loader.from_dbn_file(
    path=TEST_DATA_DIR / "databento" / "temp" / "tsla-xnas-20240107-20240206.trades.dbn.zst",
    instrument_id=TSLA_NASDAQ.id,
)

# 添加数据
engine.add_data(trades)
```

### DBN数据到ParquetDataCatalog

此代码片段演示如何加载DBN数据并写入`ParquetDataCatalog`。
我们为`as_legacy_cython`标志传递false值，这将确保DBN记录被解码为pyo3（Rust）对象。值得注意的是，传统Cython对象也可以传递给`write_data`，但这些需要在底层转换回pyo3对象（因此传递pyo3对象是一种优化）。

```python
# 初始化目录接口
# （将使用`NAUTILUS_PATH`环境变量作为路径）
catalog = ParquetDataCatalog.from_env()

instrument_id = InstrumentId.from_str("TSLA.XNAS")

# 解码数据为pyo3对象
loader = DatabentoDataLoader()
trades = loader.from_dbn_file(
    path=TEST_DATA_DIR / "databento" / "temp" / "tsla-xnas-20240107-20240206.trades.dbn.zst",
    instrument_id=instrument_id,
    as_legacy_cython=False,  # 这是写入目录的优化
)

# 写入数据
catalog.write_data(trades)
```

:::info
另请参阅[数据概念指南](../concepts/data.md)。
:::

## 实时客户端架构

`DatabentoDataClient`是一个包含其他Databento适配器类的Python类。
每个Databento数据集有两个`DatabentoLiveClient`：

- 一个用于MBO（订单簿增量）实时数据源
- 一个用于所有其他实时数据源

:::warning
目前有一个限制，即数据集的所有MBO（订单簿增量）订阅必须在节点启动时进行，以便能够从会话开始重放数据。如果后续订阅在启动后到达，则会记录错误（并忽略订阅）。

对于任何其他Databento模式，没有此类限制。
:::

单个`DatabentoHistoricalClient`实例在`DatabentoInstrumentProvider`和`DatabentoDataClient`之间重用，用于进行历史工具定义和数据请求。

## 配置

最常见的用例是配置实时`TradingNode`以包含Databento数据客户端。为此，向客户端配置添加`DATABENTO`部分：

```python
from nautilus_trader.adapters.databento import DATABENTO
from nautilus_trader.live.node import TradingNode

config = TradingNodeConfig(
    ...,  # 省略
    data_clients={
        DATABENTO: {
            "api_key": None,  # 'DATABENTO_API_KEY'环境变量
            "http_gateway": None,  # 覆盖默认HTTP历史网关
            "live_gateway": None,  # 覆盖默认原始TCP实时网关
            "instrument_provider": InstrumentProviderConfig(load_all=True),
            "instrument_ids": None,  # 启动时加载的Nautilus工具ID
            "parent_symbols": None,  # 启动时加载的Databento父符号
        },
    },
    ..., # 省略
)
```

然后，创建`TradingNode`并添加客户端工厂：

```python
from nautilus_trader.adapters.databento.factories import DatabentoLiveDataClientFactory
from nautilus_trader.live.node import TradingNode

# 使用配置实例化实时交易节点
node = TradingNode(config=config)

# 向节点注册客户端工厂
node.add_data_client_factory(DATABENTO, DatabentoLiveDataClientFactory)

# 最后构建节点
node.build()
```

### 配置参数

- `api_key`：Databento API密钥。如果为``None``，则将获取`DATABENTO_API_KEY`环境变量。
- `http_gateway`：历史HTTP客户端网关覆盖（对测试有用，大多数用户通常不需要）。
- `live_gateway`：原始TCP实时客户端网关覆盖（对测试有用，大多数用户通常不需要）。
- `parent_symbols`：启动时订阅工具定义的Databento父符号。这是Databento数据集键的映射 -> 到父符号序列，例如{'GLBX.MDP3', ['ES.FUT', 'ES.OPT']}（对于所有E-mini S&P 500期货和期权产品）。
- `instrument_ids`：启动时请求工具定义的工具ID。
- `timeout_initial_load`：等待工具加载的超时（秒）（每个数据集并发）。
- `mbo_subscriptions_delay`：等待MBO/L3订阅的超时（秒）（每个数据集并发）。超时后，MBO订单簿数据源将启动并从初始快照重放消息，然后是所有增量。

:::tip
我们建议使用环境变量来管理您的凭据。
::: 