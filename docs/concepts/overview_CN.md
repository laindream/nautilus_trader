# 概述

## 介绍

NautilusTrader是一个开源、高性能、生产级的算法交易平台，为量化交易者提供在历史数据上使用事件驱动引擎回测自动化交易策略组合的能力，同时也能在不修改代码的情况下实时部署相同的策略。

该平台是*AI优先*的，旨在在高性能且健壮的Python原生环境中开发和部署算法交易策略。这有助于解决保持Python研究/回测环境与生产实时交易环境一致性的平衡挑战。

NautilusTrader的设计、架构和实现理念在最高级别上优先考虑软件正确性和安全性，旨在支持Python原生、关键任务的交易系统回测和实时部署工作负载。

该平台也是通用的和资产类别无关的——任何REST API或WebSocket流都可以通过模块化适配器集成。它支持跨广泛资产类别和工具类型的高频交易，包括外汇、股票、期货、期权、加密货币和博彩，实现多个场所的无缝同时操作。

## 功能特性

- **快速**：核心用Rust编写，使用[tokio](https://crates.io/crates/tokio)进行异步网络处理。
- **可靠**：Rust驱动的类型和线程安全，可选的Redis支持的状态持久化。
- **便携**：操作系统独立，在Linux、macOS和Windows上运行。使用Docker部署。
- **灵活**：模块化适配器意味着任何REST API或WebSocket流都可以集成。
- **高级**：时间有效性`IOC`、`FOK`、`GTC`、`GTD`、`DAY`、`AT_THE_OPEN`、`AT_THE_CLOSE`，高级订单类型和条件触发器。执行指令`post-only`、`reduce-only`和冰山订单。应急订单包括`OCO`、`OUO`、`OTO`。
- **可定制**：添加用户定义的自定义组件，或利用[缓存](cache_CN.md)和[消息总线](message_bus_CN.md)从头组装整个系统。
- **回测**：使用历史报价tick、交易tick、K线、订单簿和自定义数据，以纳秒分辨率同时运行多个场所、工具和策略。
- **实时**：在回测和实时部署之间使用相同的策略实现。
- **多场所**：多场所功能促进做市和统计套利策略。
- **AI训练**：回测引擎足够快，可用于训练AI交易代理（RL/ES）。

![Nautilus](https://github.com/nautechsystems/nautilus_trader/blob/develop/assets/nautilus-art.png?raw=true "nautilus")
> *nautilus - 来自古希腊语'sailor'（水手）和naus'ship'（船）。*
>
> *鹦鹉螺贝壳由模块化的腔室组成，其增长因子近似于对数螺旋。
> 这个想法是可以将其转化为设计和架构的美学。*

## 为什么选择NautilusTrader？

- **高性能事件驱动Python**：原生二进制核心组件。
- **回测和实时交易之间的一致性**：相同的策略代码。
- **降低操作风险**：增强的风险管理功能、逻辑准确性和类型安全。
- **高度可扩展**：消息总线、自定义组件和角色、自定义数据、自定义适配器。

传统上，交易策略研究和回测可能使用Python中的向量化方法进行，然后需要使用C++、C#、Java或其他静态类型语言以更事件驱动的方式重新实现策略。这里的原因是向量化回测代码无法表达实时交易的细粒度时间和事件依赖复杂性，编译语言由于其固有的更高性能和类型安全性被证明更适合。

NautilusTrader的一个关键优势是，现在规避了这个重新实现步骤——因为平台的关键核心组件全部用[Rust](https://www.rust-lang.org/)或[Cython](https://cython.org/)编写。这意味着我们为工作使用了正确的工具，系统编程语言编译高性能二进制文件，CPython C扩展模块能够提供Python原生环境，适合专业量化交易者和交易公司。

## 用例

此软件包有三个主要用例：

- 在历史数据上回测交易系统（`backtest`）。
- 使用实时数据和虚拟执行模拟交易系统（`sandbox`）。
- 在真实或模拟账户上实时部署交易系统（`live`）。

项目的代码库为实现实现上述功能的系统的软件层提供了框架。您将在各自命名的子包中找到默认的`backtest`和`live`系统实现。可以使用沙盒适配器构建`sandbox`环境。

:::note

- 所有示例都将使用这些默认系统实现。
- 我们认为交易策略是端到端交易系统的子组件，这些系统包括应用和基础设施层。

:::

## 分布式

该平台被设计为易于集成到更大的分布式系统中。为了促进这一点，几乎所有配置和域对象都可以使用JSON、MessagePack或Apache Arrow（Feather）进行序列化，以便在网络上通信。

## 通用核心

通用系统核心被所有节点[环境上下文](/concepts/architecture_CN.md#environment-contexts)（`backtest`、`sandbox`和`live`）使用。用户定义的`Actor`、`Strategy`和`ExecAlgorithm`组件在这些环境上下文中一致管理。

## 回测

回测可以通过首先直接或通过更高级别的`BacktestNode`和`ParquetDataCatalog`使数据对`BacktestEngine`可用，然后以纳秒分辨率通过系统运行数据来实现。

## 实时交易

`TradingNode`可以从多个数据和执行客户端摄取数据和事件。
实时部署可以使用演示/模拟交易账户或真实账户。

对于实时交易，`TradingNode`可以从多个数据和执行客户端摄取数据和事件。该平台支持演示/模拟交易账户和真实账户。通过在单个[事件循环](https://docs.python.org/3/library/asyncio-eventloop.html)上异步运行可以实现高性能，有可能通过利用[uvloop](https://github.com/MagicStack/uvloop)实现进一步提升性能（适用于Linux和macOS）。

## 域模型

该平台具有全面的交易域模型，包括各种值类型，如`Price`和`Quantity`，以及更复杂的实体，如`Order`和`Position`对象，用于聚合多个事件以确定状态。

## 时间戳

平台内的所有时间戳都以UTC纳秒精度记录。

时间戳字符串遵循ISO 8601（RFC 3339）格式，具有9位数字（纳秒）或3位数字（毫秒）的十进制精度，（但主要是纳秒）始终保持所有数字，包括尾随零。这些可以在日志消息和对象的调试/显示输出中看到。

时间戳字符串包括：

- 始终存在的完整日期组件：`YYYY-MM-DD`。
- 日期和时间组件之间的`T`分隔符。
- 始终纳秒精度（9位小数）或某些情况下的毫秒精度（3位小数），如GTD到期时间。
- 始终由`Z`后缀指定的UTC时区。

示例：`2024-01-05T15:30:45.123456789Z`

有关完整规范，请参阅[RFC 3339：互联网上的日期和时间](https://datatracker.ietf.org/doc/html/rfc3339)。

## UUID

该平台使用通用唯一标识符（UUID）版本4（RFC 4122）作为唯一标识符。我们的高性能实现利用`uuid` crate在从字符串解析时进行正确性验证，确保输入UUID符合规范。

有效的UUID v4包括：

- 以5组显示的32个十六进制数字。
- 由连字符分隔的组：`8-4-4-4-12`格式。
- 版本4指定（由以"4"开头的第三组表示）。
- RFC 4122变体指定（由以"8"、"9"、"a"或"b"开头的第四组表示）。

示例：`2d89666b-1a1e-4a75-b193-4eb3b454c757`

有关完整规范，请参阅[RFC 4122：通用唯一标识符（UUID）URN命名空间](https://datatracker.ietf.org/doc/html/rfc4122)。

## 数据类型

以下市场数据类型可以历史请求，当场所/数据提供商提供且在集成适配器中实现时，也可以订阅为实时流。

- `OrderBookDelta`（L1/L2/L3）
- `OrderBookDeltas`（容器类型）
- `OrderBookDepth10`（每边固定10级深度）
- `QuoteTick`
- `TradeTick`
- `Bar`
- `Instrument`
- `InstrumentStatus`
- `InstrumentClose`

以下`PriceType`选项可用于K线聚合：

- `BID`
- `ASK`
- `MID`
- `LAST`

## K线聚合

以下`BarAggregation`方法可用：

- `MILLISECOND`
- `SECOND`
- `MINUTE`
- `HOUR`
- `DAY`
- `WEEK`
- `MONTH`
- `YEAR`
- `TICK`
- `VOLUME`
- `VALUE`（又称美元K线）
- `TICK_IMBALANCE`
- `TICK_RUNS`
- `VOLUME_IMBALANCE`
- `VOLUME_RUNS`
- `VALUE_IMBALANCE`
- `VALUE_RUNS`

价格类型和K线聚合可以通过`BarSpecification`以任何方式与步长>=1组合。这实现了最大灵活性，现在允许为实时交易聚合替代K线。

## 账户类型

以下账户类型适用于实时和回测环境：

- `Cash`单一货币（基础货币）
- `Cash`多货币
- `Margin`单一货币（基础货币）
- `Margin`多货币
- `Betting`单一货币

## 订单类型

以下订单类型可用（在场所可能的情况下）：

- `MARKET`
- `LIMIT`
- `STOP_MARKET`
- `STOP_LIMIT`
- `MARKET_TO_LIMIT`
- `MARKET_IF_TOUCHED`
- `LIMIT_IF_TOUCHED`
- `TRAILING_STOP_MARKET`
- `TRAILING_STOP_LIMIT`

## 值类型

以下值类型由128位或64位原始整数值支持，具体取决于编译期间使用的[精度模式](../getting_started/installation_CN.md#precision-mode)。

- `Price`
- `Quantity`
- `Money`

### 高精度模式（128位）

当`high-precision`功能标志**启用**时（默认），值使用规范：

| 类型         | 原始支持 | 最大精度 | 最小值           | 最大值          |
|:-------------|:------------|:--------------|:--------------------|:-------------------|
| `Price`      | `i128`      | 16            | -17,014,118,346,046 | 17,014,118,346,046 |
| `Money`      | `i128`      | 16            | -17,014,118,346,046 | 17,014,118,346,046 |
| `Quantity`   | `u128`      | 16            | 0                   | 34,028,236,692,093 |

### 标准精度模式（64位）

当`high-precision`功能标志**禁用**时，值使用规范：

| 类型         | 原始支持 | 最大精度 | 最小值           | 最大值          |
|:-------------|:------------|:--------------|:--------------------|:-------------------|
| `Price`      | `i64`       | 9             | -9,223,372,036      | 9,223,372,036      |
| `Money`      | `i64`       | 9             | -9,223,372,036      | 9,223,372,036      |
| `Quantity`   | `u64`       | 9             | 0                   | 18,446,744,073     |
