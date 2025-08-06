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

- [Binance符号体系](./binance_CN.md#符号体系)
- [Bybit符号体系](./bybit_CN.md#符号体系)
- [dYdX符号体系](./dydx_CN.md#符号体系)

## 交易所

Tardis上的一些交易所被分为多个场所。
下表概述了Nautilus场所与相应Tardis交易所之间的映射，以及Tardis支持的交易所：

| Nautilus场所          | Tardis交易所                                    |
|:------------------------|:------------------------------------------------------|
| `ASCENDEX`              | `ascendex`                                            |
| `BINANCE`               | `binance`, `binance-dex`, `binance-european-options`, `binance-futures`, `binance-jersey`, `binance-options` |
| `BINANCE_DELIVERY`      | `binance-delivery` (*币本位合约*)        |
| `BINANCE_US`            | `binance-us`                                          |
| `BITFINEX`              | `bitfinex`, `bitfinex-derivatives`                    |
| `BITFLYER`              | `bitflyer`                                            |
| `BITGET`                | `bitget`, `bitget-futures`                            |
| `BITMEX`                | `bitmex`                                              |
| `BITNOMIAL`             | `bitnomial`                                           |
| `BITSTAMP`              | `bitstamp`                                            |
| `BLOCKCHAIN_COM`        | `blockchain-com`                                      |
| `BYBIT`                 | `bybit`, `bybit-options`, `bybit-spot`                |
| `COINBASE`              | `coinbase`                                            |
| `COINBASE_INTX`         | `coinbase-international`                              |
| `COINFLEX`              | `coinflex` (*历史研究用*)                |
| `CRYPTO_COM`            | `crypto-com`, `crypto-com-derivatives`                |
| `CRYPTOFACILITIES`      | `cryptofacilities`                                    |
| `DELTA`                 | `delta`                                               |
| `DERIBIT`               | `deribit`                                             |
| `DYDX`                  | `dydx`                                                |
| `DYDX_V4`               | `dydx-v4`                                             |
| `FTX`                   | `ftx`, `ftx-us` (*历史研究*)               |
| `GATE_IO`               | `gate-io`, `gate-io-futures`                          |
| `GEMINI`                | `gemini`                                              |
| `HITBTC`                | `hitbtc`                                              |
| `HUOBI`                 | `huobi`, `huobi-dm`, `huobi-dm-linear-swap`, `huobi-dm-options` |
| `HUOBI_DELIVERY`        | `huobi-dm-swap`                                       |
| `HYPERLIQUID`           | `hyperliquid`                                         |
| `KRAKEN`                | `kraken`                                              |
| `KUCOIN`                | `kucoin`, `kucoin-futures`                            |
| `MANGO`                 | `mango`                                               |
| `OKCOIN`                | `okcoin`                                              |
| `OKEX`                  | `okex`, `okex-futures`, `okex-options`, `okex-spreads`, `okex-swap` |
| `PHEMEX`                | `phemex`                                              |
| `POLONIEX`              | `poloniex`                                            |
| `SERUM`                 | `serum` (*历史研究*)                       |
| `STAR_ATLAS`            | `star-atlas`                                          |
| `UPBIT`                 | `upbit`                                               |
| `WOO_X`                 | `woo-x`                                               |

## 环境变量

Tardis和NautilusTrader使用以下环境变量。

- `TM_API_KEY`：Tardis Machine的API密钥。
- `TARDIS_API_KEY`：NautilusTrader Tardis客户端的API密钥。
- `TARDIS_MACHINE_WS_URL`（可选）：NautilusTrader中`TardisMachineClient`的WebSocket URL。
- `TARDIS_BASE_URL`（可选）：NautilusTrader中`TardisHttpClient`的基础URL。
- `NAUTILUS_CATALOG_PATH`（可选）：在Nautilus目录中写入重放数据的根目录。

## 运行Tardis Machine历史重放

[Tardis Machine Server](https://docs.tardis.dev/api/tardis-machine)是一个可本地运行的服务器，具有内置数据缓存，通过HTTP和WebSocket API提供tick级历史和综合实时加密货币市场数据。

您可以执行完整的Tardis Machine WebSocket历史数据重放，并以Nautilus Parquet格式输出结果，使用Python或Rust。由于该功能在Rust中实现，无论从Python还是Rust运行，性能都是一致的，让您可以根据首选的工作流程进行选择。

端到端的`run_tardis_machine_replay`数据管道功能利用指定的[配置](#configuration)执行以下步骤：

- 连接到Tardis Machine服务器。
- 从[Tardis工具元数据](https://docs.tardis.dev/api/instruments-metadata-api) HTTP API请求并解析所有必要的工具定义。
- 从Tardis Machine服务器流式传输指定时间范围内所有请求的工具和数据类型。
- 对于每个工具、数据类型和日期（UTC），生成Nautilus格式的`.parquet`文件。
- 断开与Tardis Machine服务器的连接，并终止程序。

:::note
您可以在没有API密钥的情况下请求每月第一天的数据。对于所有其他日期，需要Tardis Machine API密钥。
:::

此过程针对直接输出到Nautilus Parquet数据目录进行了优化。
确保将`NAUTILUS_CATALOG_PATH`环境变量设置为根`/catalog/`目录。
然后Parquet文件将在`/catalog/data/`下的相应子目录中按数据类型和工具组织。

如果配置文件中未指定`output_path`且未设置`NAUTILUS_CATALOG_PATH`环境变量，系统将默认使用当前工作目录。

### 流程

首先，确保`tardis-machine` docker容器正在运行。使用以下命令：

```bash
docker run -p 8000:8000 -p 8001:8001 -e "TM_API_KEY=YOUR_API_KEY" -d tardisdev/tardis-machine
```

此命令启动`tardis-machine`服务器，没有持久的本地缓存，这可能会影响性能。
为了提高性能，请考虑使用持久卷运行服务器。有关详细信息，请参阅[Tardis Docker文档](https://docs.tardis.dev/api/tardis-machine#docker)。

### 配置

接下来，确保您有一个可用的配置JSON文件。

**配置JSON格式**

| 字段               | 类型              | 描述                                                                         | 默认值                                                                                               |
|:--------------------|:------------------|:------------------------------------------------------------------------------------|:------------------------------------------------------------------------------------------------------|
| `tardis_ws_url`     | string（可选） | Tardis Machine WebSocket URL。                                                   | 如果为`null`，则将使用`TARDIS_MACHINE_WS_URL`环境变量。                                          |
| `normalize_symbols` | bool（可选）   | 是否应应用Nautilus [符号标准化](#symbology-and-normalization)。 | 如果为`null`，则默认为`true`。                                                                |
| `output_path`       | string（可选） | 写入Nautilus Parquet数据的输出目录路径。                        | 如果为`null`，则将使用`NAUTILUS_CATALOG_PATH`环境变量，否则使用当前工作目录。 |
| `options`           | JSON[]            | [ReplayNormalizedRequestOptions](https://docs.tardis.dev/api/tardis-machine#replay-normalized-options)对象数组。                                                                 |

示例配置文件`example_config.json`可在[这里](https://github.com/nautechsystems/nautilus_trader/blob/develop/crates/adapters/tardis/bin/example_config.json)找到：

```json
{
  "tardis_ws_url": "ws://localhost:8001",
  "output_path": null,
  "options": [
    {
      "exchange": "bitmex",
      "symbols": [
        "xbtusd",
        "ethusd"
      ],
      "data_types": [
        "trade"
      ],
      "from": "2019-10-01",
      "to": "2019-10-02"
    }
  ]
}
```

### Python重放

要在Python中运行重放，创建类似以下的脚本：

```python
import asyncio

from nautilus_trader.core import nautilus_pyo3


async def run():
    config_filepath = Path("YOUR_CONFIG_FILEPATH")
    await nautilus_pyo3.run_tardis_machine_replay(str(config_filepath.resolve()))


if __name__ == "__main__":
    asyncio.run(run())
```

### Rust重放

要在Rust中运行重放，创建类似以下的二进制文件：

```rust
use std::{env, path::PathBuf};

use nautilus_adapters::tardis::replay::run_tardis_machine_replay_from_config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config_filepath = PathBuf::from("YOUR_CONFIG_FILEPATH");
    run_tardis_machine_replay_from_config(&config_filepath).await;
}
```

确保通过导出以下环境变量启用Rust日志记录：

```bash
export RUST_LOG=debug
```

可以在[这里](https://github.com/nautechsystems/nautilus_trader/blob/develop/crates/adapters/tardis/bin/example_replay.rs)找到一个工作示例二进制文件。

这也可以使用cargo运行：

```bash
cargo run --bin tardis-replay <path_to_your_config>
```

## 加载Tardis CSV数据

可以使用Python或Rust加载Tardis格式的CSV数据。加载器从磁盘读取CSV文本数据并将其解析为Nautilus数据。由于加载器在Rust中实现，无论您从Python还是Rust运行，性能都保持一致，让您可以根据首选的工作流程进行选择。

您还可以可选地为`load_*`函数/方法指定`limit`参数来控制加载的最大行数。

:::note
由于精度要求，加载混合工具CSV文件很困难，不建议使用。请改用单一工具CSV文件（见下文）。
:::

### 在Python中加载CSV数据

您可以使用`TardisCSVDataLoader`在Python中加载Tardis格式的CSV数据。
加载数据时，您可以可选地指定工具ID，但必须指定价格精度和大小精度。
提供工具ID可以提高加载性能，而指定精度是必需的，因为它们无法仅从文本数据推断。

要加载数据，创建类似以下的脚本：

```python
from nautilus_trader.adapters.tardis import TardisCSVDataLoader
from nautilus_trader.model import InstrumentId


instrument_id = InstrumentId.from_str("BTC-PERPETUAL.DERIBIT")
loader = TardisCSVDataLoader(
    price_precision=1,
    size_precision=0,
    instrument_id=instrument_id,
)

filepath = Path("YOUR_CSV_DATA_PATH")
limit = None

deltas = loader.load_deltas(filepath, limit)
```

### 在Rust中加载CSV数据

您可以使用[这里](https://github.com/nautechsystems/nautilus_trader/blob/develop/crates/adapters/tardis/src/csv/mod.rs)找到的加载函数在Rust中加载Tardis格式的CSV数据。
加载数据时，您可以可选地指定工具ID，但必须指定价格精度和大小精度。
提供工具ID可以提高加载性能，而指定精度是必需的，因为它们无法仅从文本数据推断。

有关完整示例，请参阅[这里的示例二进制文件](https://github.com/nautechsystems/nautilus_trader/blob/develop/crates/adapters/tardis/bin/example_csv.rs)。

要加载数据，您可以使用类似以下的代码：

```rust
use std::path::Path;

use nautilus_adapters::tardis;
use nautilus_model::identifiers::InstrumentId;

#[tokio::main]
async fn main() {
    // 您必须指定精度和CSV文件路径
    let price_precision = 1;
    let size_precision = 0;
    let filepath = Path::new("YOUR_CSV_DATA_PATH");

    // 可选地指定工具ID和/或限制
    let instrument_id = InstrumentId::from("BTC-PERPETUAL.DERIBIT");
    let limit = None;

    // 根据您的工作流程考虑传播任何解析错误
    let _deltas = tardis::csv::load_deltas(
        filepath,
        price_precision,
        size_precision,
        Some(instrument_id),
        limit,
    )
    .unwrap();
}
```

## 流式处理Tardis CSV数据

为了内存高效地处理大型CSV文件，Tardis集成提供流式传输功能，以可配置的块加载和处理数据，而不是一次将整个文件加载到内存中。这对于处理多GB的CSV文件而不耗尽系统内存特别有用。

流式传输功能适用于所有支持的Tardis数据类型：

- 订单簿增量（`stream_deltas`）。
- 报价tick（`stream_quotes`）。
- 交易tick（`stream_trades`）。
- 订单簿深度快照（`stream_depth10`）。

### 在Python中流式处理CSV数据

`TardisCSVDataLoader`提供作为迭代器产生数据块的流式方法。每个方法接受一个`chunk_size`参数，该参数控制每个块从CSV文件读取多少条记录：

```python
from nautilus_trader.adapters.tardis import TardisCSVDataLoader
from nautilus_trader.model import InstrumentId

instrument_id = InstrumentId.from_str("BTC-PERPETUAL.DERIBIT")
loader = TardisCSVDataLoader(
    price_precision=1,
    size_precision=0,
    instrument_id=instrument_id,
)

filepath = Path("large_trades_file.csv")
chunk_size = 100_000  # 每个块处理100,000条记录（默认）

# 以块的形式流式处理交易tick
for chunk in loader.stream_trades(filepath, chunk_size):
    print(f"Processing chunk with {len(chunk)} trades")
    # 处理每个块 - 只有此块在内存中
    for trade in chunk:
        # 您的处理逻辑在这里
        pass
```

### 流式处理订单簿数据

对于订单簿数据，增量和深度快照都可以流式处理：

```python
# 流式处理订单簿增量
for chunk in loader.stream_deltas(filepath):
    print(f"Processing {len(chunk)} deltas")
    # 处理增量块

# 流式处理depth10快照（指定级别：5或25）
for chunk in loader.stream_depth10(filepath, levels=5):
    print(f"Processing {len(chunk)} depth snapshots")
    # 处理深度块
```

### 流式处理报价数据

报价数据可以类似地流式处理：

```python
# 流式处理报价tick
for chunk in loader.stream_quotes(filepath):
    print(f"Processing {len(chunk)} quotes")
    # 处理报价块
```

### 内存效率优势

流式处理方法提供显著的内存效率优势：

- **受控内存使用**：一次只有一个块加载到内存中。
- **可扩展处理**：可以处理大于可用RAM的文件。
- **可配置块大小**：根据系统的内存和性能要求调整`chunk_size`（默认100,000）。

:::warning
当使用精度推断的流式处理（不提供显式精度）时，推断的精度可能与批量加载整个文件不同。
这是因为精度推断在块边界内工作，不同的块可能包含具有不同精度要求的值。
为了获得确定性的精度行为，在调用流式方法时提供显式的`price_precision`和`size_precision`参数。
:::

### 在Rust中流式处理CSV数据

底层流式处理功能在Rust中实现，可以直接使用：

```rust
use std::path::Path;
use nautilus_adapters::tardis::csv::{stream_trades, stream_deltas};
use nautilus_model::identifiers::InstrumentId;

#[tokio::main]
async fn main() {
    let filepath = Path::new("large_trades_file.csv");
    let chunk_size = 100_000;
    let price_precision = Some(1);
    let size_precision = Some(0);
    let instrument_id = Some(InstrumentId::from("BTC-PERPETUAL.DERIBIT"));

    // 以块的形式流式处理交易
    let stream = stream_trades(
        filepath,
        chunk_size,
        price_precision,
        size_precision,
        instrument_id,
    ).unwrap();

    for chunk_result in stream {
        match chunk_result {
            Ok(chunk) => {
                println!("Processing chunk with {} trades", chunk.len());
                // 处理块
            }
            Err(e) => {
                eprintln!("Error processing chunk: {}", e);
                break;
            }
        }
    }
}
```

## 请求工具定义

您可以使用`TardisHttpClient`在Python和Rust中请求工具定义。
此客户端与[Tardis工具元数据API](https://docs.tardis.dev/api/instruments-metadata-api)交互，请求工具元数据并将其解析为Nautilus工具。

`TardisHttpClient`构造函数接受`api_key`、`base_url`和`timeout_secs`的可选参数（默认为60秒）。

客户端提供方法来检索特定的`instrument`或特定交易所上可用的所有`instruments`。
确保在引用[Tardis支持的交易所](https://api.tardis.dev/v1/exchanges)时使用Tardis的小写短横线格式约定。

:::note
访问工具元数据API需要Tardis API密钥。
:::

### 在Python中请求工具

要在Python中请求工具定义，创建类似以下的脚本：

```python
import asyncio

from nautilus_trader.core import nautilus_pyo3


async def run():
    http_client = nautilus_pyo3.TardisHttpClient()

    instrument = await http_client.instrument("bitmex", "xbtusd")
    print(f"Received: {instrument}")

    instruments = await http_client.instruments("bitmex")
    print(f"Received: {len(instruments)} instruments")


if __name__ == "__main__":
    asyncio.run(run())
```

### 在Rust中请求工具

要在Rust中请求工具定义，使用类似以下的代码。
有关完整示例，请参阅[这里的示例二进制文件](https://github.com/nautechsystems/nautilus_trader/blob/develop/crates/adapters/tardis/bin/example_http.rs)。

```rust
use nautilus_adapters::tardis::{enums::Exchange, http::client::TardisHttpClient};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let client = TardisHttpClient::new(None, None, None).unwrap();

    // Nautilus工具定义
    let resp = client.instruments(Exchange::Bitmex).await;
    println!("Received: {resp:?}");

    let resp = client.instrument(Exchange::Bitmex, "ETHUSDT").await;
    println!("Received: {resp:?}");
}
```

## 工具提供商

`TardisInstrumentProvider`通过HTTP工具元数据API从Tardis请求并解析工具定义。
由于有多个[Tardis支持的交易所](#venues)，当加载所有工具时，
您必须使用`InstrumentProviderConfig`过滤所需的场所：

```python
from nautilus_trader.config import InstrumentProviderConfig

# 见支持的场所 https://nautilustrader.io/docs/nightly/integrations/tardis#venues
venues = {"BINANCE", "BYBIT"}
filters = {"venues": frozenset(venues)}
instrument_provider_config = InstrumentProviderConfig(load_all=True, filters=filters)
```

您也可以以通常的方式加载特定的工具定义：

```python
from nautilus_trader.config import InstrumentProviderConfig

instrument_ids = [
    InstrumentId.from_str("BTCUSDT-PERP.BINANCE"),  # 将使用'binance-futures'交易所
    InstrumentId.from_str("BTCUSDT.BINANCE"),  # 将使用'binance'交易所
]
instrument_provider_config = InstrumentProviderConfig(load_ids=instrument_ids)
```

:::note
所有订阅都必须在缓存中有可用的工具。
为简单起见，建议为您打算订阅的场所加载所有工具。
:::

## 实时数据客户端

`TardisDataClient`能够将Tardis Machine与运行中的NautilusTrader系统集成。
它支持订阅以下数据类型：

- `OrderBookDelta`（来自Tardis的L2粒度，包括所有变化或完整深度快照）
- `OrderBookDepth10`（来自Tardis的L2粒度，提供最多10级的快照）
- `QuoteTick`
- `TradeTick`
- `Bar`（具有[Tardis支持的K线聚合](#bars)的交易K线）

### 数据WebSocket

主要的`TardisMachineClient`数据WebSocket管理在初始连接阶段接收的所有流订阅，
直到`ws_connection_delay_secs`指定的持续时间。对于在此期间之后进行的任何其他订阅，
将创建一个新的`TardisMachineClient`。如果在启动时提供订阅，此方法通过允许主WebSocket在单个流中处理可能数百个订阅来优化性能。

当使用`ws_connection_delay_secs`设置初始订阅延迟时，取消订阅这些流中的任何一个实际上不会从Tardis Machine流中删除订阅，因为Tardis不支持选择性取消订阅。但是，组件仍将按预期取消订阅消息总线发布。

在任何初始延迟之后进行的所有订阅将正常运行，在请求时完全取消订阅Tardis Machine流。

:::tip
如果您预期频繁订阅和取消订阅数据，建议将`ws_connection_delay_secs`设置为零。这将为每个初始订阅创建一个新客户端，允许它们在取消订阅时单独关闭。
:::

## 限制和注意事项

目前已知以下限制和注意事项：

- 不支持历史数据请求，因为每个请求都需要从Tardis Machine进行至少一天的重放，可能还需要过滤器。这种方法既不实用也不高效。
