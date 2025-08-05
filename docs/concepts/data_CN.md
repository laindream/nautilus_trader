# 数据

NautilusTrader 提供了一套专为交易域设计的内置数据类型。
这些数据类型包括：

- `OrderBookDelta` (L1/L2/L3)：表示最细粒度的订单簿更新。
- `OrderBookDeltas` (L1/L2/L3)：批量处理多个订单簿增量数据，以提高处理效率。
- `OrderBookDepth10`：聚合的订单簿快照（买卖双方各最多10个价位）。
- `QuoteTick`：表示最优买卖价格及其规模的顶层订单簿数据。
- `TradeTick`：交易对手方之间的单笔交易/撮合事件。
- `Bar`：OHLCV（开盘价、最高价、最低价、收盘价、成交量）柱状图/蜡烛图，使用指定的*聚合方法*进行聚合。
- `InstrumentStatus`：金融工具级别的状态事件。
- `InstrumentClose`：金融工具的收盘价。

NautilusTrader 主要设计用于处理细粒度的订单簿数据，为回测中的执行模拟提供最高的现实性。
然而，回测也可以在任何支持的市场数据类型上进行，这取决于所需的模拟精度。

## 订单簿

有一个用Rust实现的高性能订单簿可用于根据提供的数据维护订单簿状态。

每个金融工具都维护`OrderBook`实例，用于回测和实时交易，可用的订单簿类型包括：

- `L3_MBO`：**逐订单市场数据 (MBO)** 或L3数据，使用每个价位的每个订单簿事件，按订单ID键入。
- `L2_MBP`：**逐价位市场数据 (MBP)** 或L2数据，按价位聚合订单簿事件。
- `L1_MBP`：**逐价位市场数据 (MBP)** 或L1数据，也称为最优买卖报价(BBO)，仅捕获顶层更新。

:::note
顶层数据，如`QuoteTick`、`TradeTick`和`Bar`，也可用于回测，市场在`L1_MBP`订单簿类型上运行。
:::

## 金融工具

可用的金融工具定义包括：

- `Betting`：表示博彩市场中的金融工具。
- `BinaryOption`：表示通用二元期权金融工具。
- `Cfd`：表示差价合约(CFD)金融工具。
- `Commodity`：表示现货/现金市场中的商品金融工具。
- `CryptoFuture`：表示可交割期货合约金融工具，以加密资产作为标的和结算货币。
- `CryptoPerpetual`：表示加密永续期货合约金融工具（又称永续掉期）。
- `CurrencyPair`：表示现货/现金市场中的通用货币对金融工具。
- `Equity`：表示通用股票金融工具。
- `FuturesContract`：表示通用可交割期货合约金融工具。
- `FuturesSpread`：表示通用可交割期货价差金融工具。
- `Index`：表示通用指数金融工具。
- `OptionContract`：表示通用期权合约金融工具。
- `OptionSpread`：表示通用期权价差金融工具。
- `Synthetic`：表示合成金融工具，价格由组成金融工具使用公式派生。

## 柱状图和聚合

### 柱状图介绍

*柱状图*（也称为蜡烛图、蜡烛线或K线）是一种数据结构，表示特定时期内的价格和成交量信息，包括：

- 开盘价
- 最高价
- 最低价
- 收盘价
- 成交量（或以tick作为成交量代理）

这些柱状图使用*聚合方法*生成，该方法根据特定标准对数据进行分组。

### 数据聚合的目的

NautilusTrader中的数据聚合将细粒度的市场数据转换为结构化的柱状图或蜡烛图，原因如下：

- 为技术指标和策略开发提供数据。
- 因为时间聚合数据（如分钟柱状图）对于许多策略来说通常是足够的。
- 与高频L1/L2/L3市场数据相比，降低成本。

### 聚合方法

平台实现了各种聚合方法：

| 名称               | 描述                                                         | 类别     |
|:-------------------|:-------------------------------------------------------------|:---------|
| `TICK`             | 聚合一定数量的tick。                                          | 阈值     |
| `TICK_IMBALANCE`   | 聚合tick的买卖不平衡。                                        | 阈值     |
| `TICK_RUNS`        | 聚合tick的连续买卖运行。                                      | 信息     |
| `VOLUME`           | 聚合成交量。                                                 | 阈值     |
| `VOLUME_IMBALANCE` | 聚合成交量的买卖不平衡。                                      | 阈值     |
| `VOLUME_RUNS`      | 聚合买卖成交量的连续运行。                                    | 信息     |
| `VALUE`            | 聚合交易的名义价值（也称为"美元柱状图"）。                     | 阈值     |
| `VALUE_IMBALANCE`  | 聚合按名义价值交易的买卖不平衡。                              | 信息     |
| `VALUE_RUNS`       | 聚合按名义价值交易的连续买卖运行。                            | 阈值     |
| `MILLISECOND`      | 以毫秒粒度聚合时间间隔。                                      | 时间     |
| `SECOND`           | 以秒粒度聚合时间间隔。                                        | 时间     |
| `MINUTE`           | 以分钟粒度聚合时间间隔。                                      | 时间     |
| `HOUR`             | 以小时粒度聚合时间间隔。                                      | 时间     |
| `DAY`              | 以天粒度聚合时间间隔。                                        | 时间     |
| `WEEK`             | 以周粒度聚合时间间隔。                                        | 时间     |
| `MONTH`            | 以月粒度聚合时间间隔。                                        | 时间     |
| `YEAR`             | 以年粒度聚合时间间隔。                                        | 时间     |

### 聚合类型

NautilusTrader实现了三种不同的数据聚合方法：

1. **交易到柱状图聚合**：从`TradeTick`对象（已执行的交易）创建柱状图
   - 使用案例：分析执行价格的策略或直接处理交易数据时。
   - 总是在柱状图规范中使用`LAST`价格类型。

2. **报价到柱状图聚合**：从`QuoteTick`对象（买卖价格）创建柱状图
   - 使用案例：专注于买卖价差或市场深度分析的策略。
   - 在柱状图规范中使用`BID`、`ASK`或`MID`价格类型。

3. **柱状图到柱状图聚合**：从较小时间框架的`Bar`对象创建较大时间框架的`Bar`对象
   - 使用案例：将现有的较小时间框架柱状图（1分钟）重新采样为较大时间框架（5分钟、小时）。
   - 规范中总是需要`@`符号。

### 柱状图类型和组件

NautilusTrader基于以下组件定义唯一的*柱状图类型*（`BarType`类）：

- **金融工具ID**（`InstrumentId`）：指定柱状图的特定金融工具。
- **柱状图规范**（`BarSpecification`）：
  - `step`：定义每个柱状图的间隔或频率。
  - `aggregation`：指定用于数据聚合的方法（见上表）。
  - `price_type`：指示柱状图的价格基础（例如，买价、卖价、中价、最新价）。
- **聚合源**（`AggregationSource`）：指示柱状图是内部聚合（在Nautilus内）还是外部聚合（由交易所或数据提供商）。

柱状图类型也可以分类为*标准*或*复合*：

- **标准**：从细粒度市场数据生成，如报价tick或交易tick。
- **复合**：通过子采样从更高粒度的柱状图类型派生（如5分钟柱状图从1分钟柱状图聚合）。

### 聚合源

柱状图数据聚合可以是*内部*或*外部*：

- `INTERNAL`：柱状图在本地Nautilus系统边界内聚合。
- `EXTERNAL`：柱状图在本地Nautilus系统边界外聚合（通常由交易所或数据提供商）。

对于柱状图到柱状图聚合，目标柱状图类型总是`INTERNAL`（因为您在NautilusTrader内进行聚合），
但源柱状图可以是`INTERNAL`或`EXTERNAL`，即您可以聚合外部提供的柱状图或已经聚合的内部柱状图。

### 使用字符串语法定义柱状图类型

#### 标准柱状图

您可以使用以下约定从字符串定义标准柱状图类型：

`{instrument_id}-{step}-{aggregation}-{price_type}-{INTERNAL | EXTERNAL}`

例如，要为纳斯达克(XNAS)上的AAPL交易（最新价）定义一个5分钟间隔的`BarType`，
由Nautilus本地从交易聚合：

```python
bar_type = BarType.from_str("AAPL.XNAS-5-MINUTE-LAST-INTERNAL")
```

#### 复合柱状图

复合柱状图通过将更高粒度的柱状图聚合到所需的柱状图类型中派生。要定义复合柱状图，
使用此约定：

`{instrument_id}-{step}-{aggregation}-{price_type}-INTERNAL@{step}-{aggregation}-{INTERNAL | EXTERNAL}`

**注意**：

- 派生的柱状图类型必须使用`INTERNAL`聚合源（因为这是柱状图聚合的方式）。
- 采样的柱状图类型必须具有比派生柱状图类型更高的粒度。
- 采样的金融工具ID被推断为与派生柱状图类型匹配。
- 复合柱状图可以*从*`INTERNAL`或`EXTERNAL`聚合源聚合。

例如，要为纳斯达克(XNAS)上的AAPL交易（最新价）定义一个5分钟间隔的`BarType`，
由Nautilus本地聚合，从外部聚合的1分钟间隔柱状图：

```python
bar_type = BarType.from_str("AAPL.XNAS-5-MINUTE-LAST-INTERNAL@1-MINUTE-EXTERNAL")
```

### 聚合语法示例

`BarType`字符串格式编码目标柱状图类型和（可选）源数据类型：

```
{instrument_id}-{step}-{aggregation}-{price_type}-{source}@{step}-{aggregation}-{source}
```

`@`符号后的部分是可选的，仅用于柱状图到柱状图聚合：

- **没有`@`**：从`TradeTick`对象（当price_type为`LAST`时）或`QuoteTick`对象（当price_type为`BID`、`ASK`或`MID`时）聚合。
- **有`@`**：从现有`Bar`对象聚合（指定源柱状图类型）。

#### 交易到柱状图示例

```python
def on_start(self) -> None:
    # 定义从TradeTick对象聚合的柱状图类型
    # 使用price_type=LAST表示TradeTick数据作为源
    bar_type = BarType.from_str("6EH4.XCME-50-VOLUME-LAST-INTERNAL")

    # 请求历史数据（将在on_historical_data处理程序中接收柱状图）
    self.request_bars(bar_type)

    # 订阅实时数据（将在on_bar处理程序中接收柱状图）
    self.subscribe_bars(bar_type)
```

#### 报价到柱状图示例

```python
def on_start(self) -> None:
    # 从ASK价格创建1分钟柱状图（在QuoteTick对象中）
    bar_type_ask = BarType.from_str("6EH4.XCME-1-MINUTE-ASK-INTERNAL")

    # 从BID价格创建1分钟柱状图（在QuoteTick对象中）
    bar_type_bid = BarType.from_str("6EH4.XCME-1-MINUTE-BID-INTERNAL")

    # 从MID价格创建1分钟柱状图（QuoteTick对象中ASK和BID价格的中间价）
    bar_type_mid = BarType.from_str("6EH4.XCME-1-MINUTE-MID-INTERNAL")

    # 请求历史数据并订阅实时数据
    self.request_bars(bar_type_ask)    # 历史柱状图在on_historical_data中处理
    self.subscribe_bars(bar_type_ask)  # 实时柱状图在on_bar中处理
```

#### 柱状图到柱状图示例

```python
def on_start(self) -> None:
    # 从1分钟柱状图创建5分钟柱状图（Bar对象）
    # 格式：目标柱状图类型@源柱状图类型
    # 注意：价格类型（LAST）只在左侧目标端需要，源端不需要
    bar_type = BarType.from_str("6EH4.XCME-5-MINUTE-LAST-INTERNAL@1-MINUTE-EXTERNAL")

    # 请求历史数据（在on_historical_data(...)处理程序中处理）
    self.request_bars(bar_type)

    # 订阅实时更新（在on_bar(...)处理程序中处理）
    self.subscribe_bars(bar_type)
```

#### 高级柱状图到柱状图示例

您可以创建复杂的聚合链，从已经聚合的柱状图进行聚合：

```python
# 首先从TradeTick对象创建1分钟柱状图（LAST表示TradeTick源）
primary_bar_type = BarType.from_str("6EH4.XCME-1-MINUTE-LAST-INTERNAL")

# 然后从1分钟柱状图创建5分钟柱状图
# 注意@1-MINUTE-INTERNAL部分标识源柱状图
intermediate_bar_type = BarType.from_str("6EH4.XCME-5-MINUTE-LAST-INTERNAL@1-MINUTE-INTERNAL")

# 然后从5分钟柱状图创建小时柱状图
# 注意@5-MINUTE-INTERNAL部分标识源柱状图
hourly_bar_type = BarType.from_str("6EH4.XCME-1-HOUR-LAST-INTERNAL@5-MINUTE-INTERNAL")
```

### 处理柱状图：请求与订阅

NautilusTrader为处理柱状图提供了两种不同的操作：

- **`request_bars()`**：获取由`on_historical_data()`处理程序处理的历史数据。
- **`subscribe_bars()`**：建立由`on_bar()`处理程序处理的实时数据流。

这些方法在典型工作流程中协同工作：

1. 首先，`request_bars()`加载历史数据以初始化指标或策略的过去市场行为状态。
2. 然后，`subscribe_bars()`确保策略在实时形成新柱状图时继续接收。

在`on_start()`中的使用示例：

```python
def on_start(self) -> None:
    # 定义柱状图类型
    bar_type = BarType.from_str("6EH4.XCME-5-MINUTE-LAST-INTERNAL")

    # 请求历史数据以初始化指标
    # 这些柱状图将传递给策略中的on_historical_data(...)处理程序
    self.request_bars(bar_type)

    # 订阅实时更新
    # 新柱状图将传递给策略中的on_bar(...)处理程序
    self.subscribe_bars(bar_type)

    # 注册指标以接收柱状图更新（它们将自动更新）
    self.register_indicator_for_bars(bar_type, self.my_indicator)
```

策略中接收数据所需的处理程序：

```python
def on_historical_data(self, data):
    # 从request_bars()处理历史柱状图批次
    # 注意：使用register_indicator_for_bars注册的指标
    # 会自动更新历史数据
    pass

def on_bar(self, bar):
    # 从subscribe_bars()实时处理单个柱状图
    # 注册此柱状图类型的指标将自动更新，并在调用此处理程序之前更新
    pass
```

### 使用聚合的历史数据请求

在为回测或初始化指标请求历史柱状图时，您可以使用`request_bars()`方法，它支持直接请求和聚合：

```python
# 请求原始1分钟柱状图（从TradeTick对象聚合，如LAST价格类型所示）
self.request_bars(BarType.from_str("6EH4.XCME-1-MINUTE-LAST-EXTERNAL"))

# 请求从1分钟柱状图聚合的5分钟柱状图
self.request_bars(BarType.from_str("6EH4.XCME-5-MINUTE-LAST-INTERNAL@1-MINUTE-EXTERNAL"))
```

如果需要历史聚合柱状图，您可以使用专门的请求`request_aggregated_bars()`方法：

```python
# 请求从历史交易tick聚合的柱状图
self.request_aggregated_bars([BarType.from_str("6EH4.XCME-100-VOLUME-LAST-INTERNAL")])

# 请求从其他柱状图聚合的柱状图
self.request_aggregated_bars([BarType.from_str("6EH4.XCME-5-MINUTE-LAST-INTERNAL@1-MINUTE-EXTERNAL")])
```

### 常见陷阱

**在请求数据之前注册指标**：确保在请求历史数据之前注册指标，以便它们正确更新。

```python
# 正确顺序
self.register_indicator_for_bars(bar_type, self.ema)
self.request_bars(bar_type)

# 错误顺序
self.request_bars(bar_type)  # 指标不会接收历史数据
self.register_indicator_for_bars(bar_type, self.ema)
```

## 时间戳

平台使用两个基本的时间戳字段，它们出现在许多对象中，包括市场数据、订单和事件。
这些时间戳有不同的目的，帮助在整个系统中维护精确的时间信息：

- `ts_event`：UNIX时间戳（纳秒），表示事件实际发生的时间。
- `ts_init`：UNIX时间戳（纳秒），表示Nautilus创建表示该事件的内部对象的时间。

### 示例

| **事件类型**     | **`ts_event`**                                        | **`ts_init`** |
| -----------------| ------------------------------------------------------| --------------|
| `TradeTick`      | 交易在交易所发生的时间。                               | Nautilus接收交易数据的时间。 |
| `QuoteTick`      | 报价在交易所发生的时间。                               | Nautilus接收报价数据的时间。 |
| `OrderBookDelta` | 订单簿更新在交易所发生的时间。                         | Nautilus接收订单簿更新的时间。 |
| `Bar`            | 柱状图收盘的时间（精确的分钟/小时）。                  | Nautilus生成（内部柱状图）或接收柱状图数据（外部柱状图）的时间。 |
| `OrderFilled`    | 订单在交易所成交的时间。                               | Nautilus接收并处理成交确认的时间。 |
| `OrderCanceled`  | 取消在交易所处理的时间。                               | Nautilus接收并处理取消确认的时间。 |
| `NewsEvent`      | 新闻发布的时间。                                       | 事件对象在Nautilus中创建（如果是内部事件）或接收（如果是外部事件）的时间。 |
| 自定义事件       | 事件条件实际发生的时间。                               | 事件对象在Nautilus中创建（如果是内部事件）或接收（如果是外部事件）的时间。 |

:::note
`ts_init`字段表示比事件"接收时间"更一般的概念。
它表示对象（如数据点或命令）在Nautilus内初始化的时间戳。
这种区别很重要，因为`ts_init`不仅限于"接收的事件"——它适用于系统中的任何内部初始化过程。

例如，`ts_init`字段也用于命令，其中接收的概念不适用。
这种更广泛的定义确保了在系统中各种对象类型的初始化时间戳的一致处理。
:::

### 延迟分析

双时间戳系统在平台内启用延迟分析：

- 延迟可以计算为`ts_init - ts_event`。
- 这个差异表示总系统延迟，包括网络传输时间、处理开销和任何排队延迟。
- 重要的是要记住，生成这些时间戳的时钟很可能不同步。

### 环境特定行为

#### 回测环境

- 数据按`ts_init`使用稳定排序进行排序。
- 此行为确保确定性处理顺序并模拟现实的系统行为，包括延迟。

#### 实时交易环境

- 数据在到达时处理，确保最小延迟并允许实时决策。
  - `ts_init`字段记录数据实时被Nautilus接收的确切时刻。
  - `ts_event`反映事件外部发生的时间，使外部事件时间和系统接收之间的准确比较成为可能。
- 我们可以使用`ts_init`和`ts_event`之间的差异来检测网络或处理延迟。

### 其他注意事项和考虑

- 对于来自外部源的数据，`ts_init`总是等于或晚于`ts_event`。
- 对于在Nautilus内创建的数据，`ts_init`和`ts_event`可以相同，因为对象在事件发生的同时初始化。
- 并非每个具有`ts_init`字段的类型都必须有`ts_event`字段。这反映了以下情况：
  - 对象的初始化与事件本身同时发生。
  - 外部事件时间的概念不适用。

#### 持久化数据

`ts_init`字段指示消息最初接收的时间。

## 数据流

平台通过在所有系统[环境上下文](/concepts/architecture.md#environment-contexts)
（例如，`backtest`、`sandbox`、`live`）中流经相同路径来确保一致性。数据主要通过`MessageBus`传输到`DataEngine`，
然后分发给订阅或注册的处理程序。

对于需要更多灵活性的用户，平台还支持创建自定义数据类型。
有关如何实现用户定义数据类型的详细信息，请参见下面的[自定义数据](#custom-data)部分。

## 加载数据

NautilusTrader为三个主要用例提供数据加载和转换：

- 为`BacktestEngine`提供数据以运行回测。
- 通过`ParquetDataCatalog.write_data(...)`持久化Nautilus特定的Parquet格式到数据目录，以便稍后与`BacktestNode`一起使用。
- 用于研究目的（确保研究和回测之间的数据一致性）。

无论目标如何，过程都保持相同：将各种外部数据格式转换为Nautilus数据结构。

为了实现这一点，需要两个主要组件：

- 一种DataLoader类型（通常特定于每个原始源/格式），可以读取数据并返回具有所需Nautilus对象正确架构的`pd.DataFrame`。
- 一种DataWrangler类型（特定于每种数据类型），接受此`pd.DataFrame`并返回Nautilus对象的`list[Data]`。

### 数据加载器

数据加载器组件通常特定于原始源/格式和每个集成。例如，Binance订单簿数据以其原始CSV文件形式存储，
格式与[Databento Binary Encoding (DBN)](https://databento.com/docs/knowledge-base/new-users/dbn-encoding/getting-started-with-dbn)文件完全不同。

### 数据整理器

数据整理器针对特定的Nautilus数据类型实现，可以在`nautilus_trader.persistence.wranglers`模块中找到。
目前存在：

- `OrderBookDeltaDataWrangler`
- `OrderBookDepth10DataWrangler`
- `QuoteTickDataWrangler`
- `TradeTickDataWrangler`
- `BarDataWrangler`

:::warning
有许多**DataWrangler v2**组件，它们将接受通常具有不同固定宽度Nautilus Arrow v2架构的`pd.DataFrame`，
并输出仅与当前正在开发的新版本Nautilus核心兼容的PyO3 Nautilus对象。

**这些PyO3提供的数据对象与当前使用传统Cython对象的地方不兼容（例如，直接添加到`BacktestEngine`）。**
:::

### 转换管道

**流程流程**：

1. 原始数据（例如CSV）输入到管道中。
2. DataLoader处理原始数据并将其转换为`pd.DataFrame`。
3. DataWrangler进一步处理`pd.DataFrame`以生成Nautilus对象列表。
4. Nautilus `list[Data]`是数据加载过程的输出。

以下图表说明了原始数据如何转换为Nautilus数据结构：

```
  ┌──────────┐    ┌──────────────────────┐                  ┌──────────────────────┐
  │          │    │                      │                  │                      │
  │          │    │                      │                  │                      │
  │ 原始数据 │    │                      │  `pd.DataFrame`  │                      │
  │ (CSV)    ├───►│      DataLoader      ├─────────────────►│     DataWrangler     ├───► Nautilus `list[Data]`
  │          │    │                      │                  │                      │
  │          │    │                      │                  │                      │
  │          │    │                      │                  │                      │
  └──────────┘    └──────────────────────┘                  └──────────────────────┘

```

具体来说，这将涉及：

- `BinanceOrderBookDeltaDataLoader.load(...)`从磁盘读取Binance提供的CSV文件，并返回`pd.DataFrame`。
- `OrderBookDeltaDataWrangler.process(...)`接受`pd.DataFrame`并返回`list[OrderBookDelta]`。

以下示例显示如何在Python中完成上述操作：

```python
from nautilus_trader import TEST_DATA_DIR
from nautilus_trader.adapters.binance.loaders import BinanceOrderBookDeltaDataLoader
from nautilus_trader.persistence.wranglers import OrderBookDeltaDataWrangler
from nautilus_trader.test_kit.providers import TestInstrumentProvider


# 加载原始数据
data_path = TEST_DATA_DIR / "binance" / "btcusdt-depth-snap.csv"
df = BinanceOrderBookDeltaDataLoader.load(data_path)

# 设置整理器
instrument = TestInstrumentProvider.btcusdt_binance()
wrangler = OrderBookDeltaDataWrangler(instrument)

# 处理为`OrderBookDelta` Nautilus对象列表
deltas = wrangler.process(df)
```

## 数据目录

数据目录是Nautilus数据的中央存储，以[Parquet](https://parquet.apache.org)文件格式持久化。它作为回测和实时交易场景的主要数据管理系统，为市场数据提供高效的存储、检索和流式传输功能。

### 概述和架构

NautilusTrader数据目录构建在结合Rust性能和Python灵活性的双后端架构上：

**核心组件：**

- **ParquetDataCatalog**：数据操作的主要Python接口。
- **Rust后端**：用于核心数据类型（OrderBookDelta、QuoteTick、TradeTick、Bar、MarkPriceUpdate）的高性能查询引擎。
- **PyArrow后端**：用于自定义数据类型和高级过滤的灵活回退。
- **fsspec集成**：支持本地和云存储（S3、GCS、Azure等）。

**主要优势：**

- **性能**：Rust后端为核心市场数据类型提供优化的查询性能。
- **灵活性**：PyArrow后端处理自定义数据类型和复杂过滤场景。
- **可扩展性**：高效压缩和列式存储降低存储成本并提高I/O性能。
- **云原生**：通过fsspec内置支持云存储提供商。
- **无依赖**：不需要外部数据库或服务的自包含解决方案。

**存储格式优势：**

- 与CSV/JSON/HDF5相比，具有更优的压缩比和读取性能。
- 列式存储使高效过滤和聚合成为可能。
- 支持数据模型更改的架构演进。
- 跨语言兼容性（Python、Rust、Java、C++等）。

用于Parquet格式的Arrow架构主要在核心`persistence` Rust crate中单一源化，
`/serialization/arrow/schema.py`模块中有一些传统架构可用。

:::note
当前计划是最终淘汰Python架构模块，以便所有架构在Rust核心中单一源化，以保持一致性和性能。
:::

### 初始化

数据目录可以从`NAUTILUS_PATH`环境变量初始化，或通过显式传入路径类对象。

以下示例显示如何初始化一个数据目录，其中给定路径已有预存在的数据写入磁盘。

```python
from pathlib import Path
from nautilus_trader.persistence.catalog import ParquetDataCatalog


CATALOG_PATH = Path.cwd() / "catalog"

# 创建新的目录实例
catalog = ParquetDataCatalog(CATALOG_PATH)

# 替代方案：基于环境的初始化
catalog = ParquetDataCatalog.from_env()  # 使用NAUTILUS_PATH环境变量
```

### 文件系统协议和存储选项

目录通过fsspec集成支持多种文件系统协议，可在本地和云存储系统之间无缝操作。

#### 支持的文件系统协议

**本地文件系统 (`file`)：**

```python
catalog = ParquetDataCatalog(
    path="/path/to/catalog",
    fs_protocol="file",  # 默认协议
)
```

**Amazon S3 (`s3`)：**

```python
catalog = ParquetDataCatalog(
    path="s3://my-bucket/nautilus-data/",
    fs_protocol="s3",
    fs_storage_options={
        "key": "your-access-key-id",
        "secret": "your-secret-access-key",
        "region": "us-east-1",
        "endpoint_url": "https://s3.amazonaws.com",  # 可选自定义端点
    }
)
```

**Google Cloud Storage (`gcs`)：**

```python
catalog = ParquetDataCatalog(
    path="gcs://my-bucket/nautilus-data/",
    fs_protocol="gcs",
    fs_storage_options={
        "project": "my-project-id",
        "token": "/path/to/service-account.json",  # 或"cloud"用于默认凭据
    }
)
```

**Azure Blob Storage (`abfs`)：**

```python
catalog = ParquetDataCatalog(
    path="abfs://container@account.dfs.core.windows.net/nautilus-data/",
    fs_protocol="abfs",
    fs_storage_options={
        "account_name": "your-storage-account",
        "account_key": "your-account-key",
        # 或使用SAS令牌："sas_token": "your-sas-token"
    }
)
```

#### 基于URI的初始化

为了方便起见，您可以使用自动解析协议和存储选项的URI字符串：

```python
# 本地文件系统
catalog = ParquetDataCatalog.from_uri("/path/to/catalog")

# S3存储桶
catalog = ParquetDataCatalog.from_uri("s3://my-bucket/nautilus-data/")

# 带存储选项
catalog = ParquetDataCatalog.from_uri(
    "s3://my-bucket/nautilus-data/",
    storage_options={
        "region": "us-east-1",
        "access_key_id": "your-key",
        "secret_access_key": "your-secret"
    }
)
```

### 写入数据

使用`write_data()`方法在目录中存储数据。支持所有Nautilus内置`Data`对象，任何继承自`Data`的数据都可以写入。

```python
# 写入数据对象列表
catalog.write_data(quote_ticks)

# 使用自定义时间戳范围写入
catalog.write_data(
    trade_ticks,
    start=1704067200000000000,  # 可选开始时间戳覆盖（UNIX纳秒）
    end=1704153600000000000,    # 可选结束时间戳覆盖（UNIX纳秒）
)

# 跳过重叠数据的分离检查
catalog.write_data(bars, skip_disjoint_check=True)
```

### 文件命名和数据组织

目录根据写入数据的时间戳范围自动生成文件名。文件使用模式`{start_timestamp}_{end_timestamp}.parquet`命名，其中时间戳为ISO格式。

数据按数据类型和金融工具ID组织在目录中：

```
catalog/
├── data/
│   ├── quote_ticks/
│   │   └── eurusd.sim/
│   │       └── 20240101T000000000000000_20240101T235959999999999.parquet
│   └── trade_ticks/
│       └── btcusd.binance/
│           └── 20240101T000000000000000_20240101T235959999999999.parquet
```

**Rust后端数据类型（增强性能）：**

以下数据类型使用优化的Rust实现：

- `OrderBookDelta`。
- `OrderBookDeltas`。
- `OrderBookDepth10`。
- `QuoteTick`。
- `TradeTick`。
- `Bar`。
- `MarkPriceUpdate`。

:::warning
默认情况下，与现有文件重叠的数据将导致断言错误，以维护数据完整性。在需要时使用`write_data()`中的`skip_disjoint_check=True`来绕过此检查。
:::

### 读取数据

使用`query()`方法从目录读取数据：

```python
from nautilus_trader.model import QuoteTick, TradeTick

# 查询特定金融工具和时间范围的报价tick
quotes = catalog.query(
    data_cls=QuoteTick,
    identifiers=["EUR/USD.SIM"],
    start="2024-01-01T00:00:00Z",
    end="2024-01-02T00:00:00Z"
)

# 使用过滤查询交易tick
trades = catalog.query(
    data_cls=TradeTick,
    identifiers=["BTC/USD.BINANCE"],
    start="2024-01-01",
    end="2024-01-02",
    where="price > 50000"
)
```

### BacktestDataConfig - 回测的数据规范

`BacktestDataConfig`类是在回测开始前指定数据需求的主要机制。它定义了应该从目录加载哪些数据以及在回测执行期间如何过滤和处理这些数据。

#### 核心参数

**必需参数：**

- `catalog_path`：数据目录目录的路径。
- `data_cls`：数据类型类（例如，QuoteTick、TradeTick、OrderBookDelta、Bar）。

**可选参数：**

- `catalog_fs_protocol`：文件系统协议（'file'、's3'、'gcs'等）。
- `catalog_fs_storage_options`：存储特定选项（凭据、区域等）。
- `instrument_id`：要加载数据的特定金融工具。
- `instrument_ids`：金融工具列表（替代单个instrument_id）。
- `start_time`：数据过滤的开始时间（ISO字符串或UNIX纳秒）。
- `end_time`：数据过滤的结束时间（ISO字符串或UNIX纳秒）。
- `filter_expr`：附加PyArrow过滤表达式。
- `client_id`：自定义数据类型的客户端ID。
- `metadata`：数据查询的附加元数据。
- `bar_spec`：柱状图数据的柱状图规范（例如，"1-MINUTE-LAST"）。
- `bar_types`：柱状图类型列表（替代bar_spec）。

#### 基本使用示例

**加载报价Tick：**

```python
from nautilus_trader.config import BacktestDataConfig
from nautilus_trader.model import QuoteTick, InstrumentId

data_config = BacktestDataConfig(
    catalog_path="/path/to/catalog",
    data_cls=QuoteTick,
    instrument_id=InstrumentId.from_str("EUR/USD.SIM"),
    start_time="2024-01-01T00:00:00Z",
    end_time="2024-01-02T00:00:00Z",
)
```

**加载多个金融工具：**

```python
data_config = BacktestDataConfig(
    catalog_path="/path/to/catalog",
    data_cls=TradeTick,
    instrument_ids=["BTC/USD.BINANCE", "ETH/USD.BINANCE"],
    start_time="2024-01-01T00:00:00Z",
    end_time="2024-01-02T00:00:00Z",
)
```

**加载柱状图数据：**

```python
data_config = BacktestDataConfig(
    catalog_path="/path/to/catalog",
    data_cls=Bar,
    instrument_id=InstrumentId.from_str("AAPL.NASDAQ"),
    bar_spec="5-MINUTE-LAST",
    start_time="2024-01-01",
    end_time="2024-01-31",
)
```

#### 高级配置示例

**带自定义过滤的云存储：**

```python
data_config = BacktestDataConfig(
    catalog_path="s3://my-bucket/nautilus-data/",
    catalog_fs_protocol="s3",
    catalog_fs_storage_options={
        "key": "your-access-key",
        "secret": "your-secret-key",
        "region": "us-east-1"
    },
    data_cls=OrderBookDelta,
    instrument_id=InstrumentId.from_str("BTC/USD.COINBASE"),
    start_time="2024-01-01T09:30:00Z",
    end_time="2024-01-01T16:00:00Z",
    filter_expr="side == 'BUY'",  # 仅买方增量
)
```

**带客户端ID的自定义数据：**

```python
data_config = BacktestDataConfig(
    catalog_path="/path/to/catalog",
    data_cls="my_package.data.NewsEventData",
    client_id="NewsClient",
    metadata={"source": "reuters", "category": "earnings"},
    start_time="2024-01-01",
    end_time="2024-01-31",
)
```

#### 与BacktestRunConfig集成

`BacktestDataConfig`对象通过`BacktestRunConfig`集成到回测框架中：

```python
from nautilus_trader.config import BacktestRunConfig, BacktestVenueConfig

# 定义多个数据配置
data_configs = [
    BacktestDataConfig(
        catalog_path="/path/to/catalog",
        data_cls=QuoteTick,
        instrument_id="EUR/USD.SIM",
        start_time="2024-01-01",
        end_time="2024-01-02",
    ),
    BacktestDataConfig(
        catalog_path="/path/to/catalog",
        data_cls=TradeTick,
        instrument_id="EUR/USD.SIM",
        start_time="2024-01-01",
        end_time="2024-01-02",
    ),
]

# 创建回测运行配置
run_config = BacktestRunConfig(
    venues=[BacktestVenueConfig(name="SIM", oms_type="HEDGING")],
    data=data_configs,  # 数据配置列表
    start="2024-01-01T00:00:00Z",
    end="2024-01-02T00:00:00Z",
)
```

#### 数据加载过程

当回测运行时，`BacktestNode`处理每个`BacktestDataConfig`：

1. **目录加载**：从配置创建`ParquetDataCatalog`实例。
2. **查询构建**：从配置属性构建查询参数。
3. **数据检索**：使用适当的后端执行目录查询。
4. **金融工具加载**：如果需要，加载金融工具定义。
5. **引擎集成**：将数据添加到具有适当排序的回测引擎。

系统自动处理：

- 金融工具ID解析和验证。
- 数据类型验证和转换。
- 大型数据集的内存高效流式传输。
- 错误处理和日志记录。

### DataCatalogConfig - 即时数据加载

`DataCatalogConfig`类为即时数据加载场景提供配置，特别适用于可能金融工具数量庞大的回测，
与预先指定回测数据的`BacktestDataConfig`不同，`DataCatalogConfig`在运行时启用灵活的目录访问。
这样定义的目录也可用于请求历史数据。

#### 核心参数

**必需参数：**

- `path`：数据目录目录的路径。

**可选参数：**

- `fs_protocol`：文件系统协议（'file'、's3'、'gcs'、'azure'等）。
- `fs_storage_options`：协议特定的存储选项。
- `name`：目录配置的可选名称标识符。

#### 基本使用示例

**本地目录配置：**

```python
from nautilus_trader.persistence.config import DataCatalogConfig

catalog_config = DataCatalogConfig(
    path="/path/to/catalog",
    fs_protocol="file",
    name="local_market_data"
)

# 转换为目录实例
catalog = catalog_config.as_catalog()
```

**云存储配置：**

```python
catalog_config = DataCatalogConfig(
    path="s3://my-bucket/market-data/",
    fs_protocol="s3",
    fs_storage_options={
        "key": "your-access-key",
        "secret": "your-secret-key",
        "region": "us-west-2",
        "endpoint_url": "https://s3.us-west-2.amazonaws.com"
    },
    name="cloud_market_data"
)
```

#### 与实时交易集成

`DataCatalogConfig`通常用于实时交易配置中以访问历史数据：

```python
from nautilus_trader.live.config import TradingNodeConfig
from nautilus_trader.persistence.config import DataCatalogConfig

# 为实时系统配置目录
catalog_config = DataCatalogConfig(
    path="/data/nautilus/catalog",
    fs_protocol="file",
    name="historical_data"
)

# 在交易节点配置中使用
node_config = TradingNodeConfig(
    # ... 其他配置
    catalog=catalog_config,  # 启用历史数据访问
)
```

#### 流式配置

要在实时交易或回测期间将数据流式传输到目录，请使用`StreamingConfig`：

```python
from nautilus_trader.persistence.config import StreamingConfig, RotationMode
import pandas as pd

streaming_config = StreamingConfig(
    catalog_path="/path/to/streaming/catalog",
    fs_protocol="file",
    flush_interval_ms=1000,  # 每秒刷新
    replace_existing=False,
    rotation_mode=RotationMode.DAILY,
    rotation_interval=pd.Timedelta(hours=1),
    max_file_size=1024 * 1024 * 100,  # 100MB最大文件大小
)
```

#### 使用案例

**历史数据分析：**

- 在实时交易期间加载历史数据用于策略计算。
- 访问金融工具查找的参考数据。
- 检索过去的性能指标。

**动态数据加载：**

- 根据运行时条件加载数据。
- 实现自定义数据加载策略。
- 支持多个目录源。

**研究和开发：**

- 在Jupyter笔记本中进行交互式数据探索。
- 临时分析和回测。
- 数据质量验证和监控。

### 查询系统和双后端架构

目录的查询系统利用复杂的双后端架构，根据数据类型和查询参数自动选择最优查询引擎。

#### 后端选择逻辑

**Rust后端（高性能）：**

- **支持类型**：OrderBookDelta、OrderBookDeltas、OrderBookDepth10、QuoteTick、TradeTick、Bar、MarkPriceUpdate。
- **条件**：当`files`参数为None（自动文件发现）时使用。
- **优势**：优化性能、内存效率、原生Arrow集成。

**PyArrow后端（灵活）：**

- **支持类型**：所有数据类型包括自定义数据类。
- **条件**：用于自定义数据类型或指定`files`参数时。
- **优势**：高级过滤、自定义数据支持、复杂查询表达式。

#### 查询方法和参数

**核心查询参数：**

```python
catalog.query(
    data_cls=QuoteTick,                    # 要查询的数据类型
    identifiers=["EUR/USD.SIM"],           # 金融工具标识符
    start="2024-01-01T00:00:00Z",         # 开始时间（支持各种格式）
    end="2024-01-02T00:00:00Z",           # 结束时间
    where="bid > 1.1000",                 # PyArrow过滤表达式
    files=None,                           # 特定文件（强制PyArrow后端）
)
```

**时间格式支持：**

- ISO 8601字符串：`"2024-01-01T00:00:00Z"`。
- UNIX纳秒：`1704067200000000000`（或ISO格式：`"2024-01-01T00:00:00Z"`）。
- Pandas时间戳：`pd.Timestamp("2024-01-01", tz="UTC")`。
- Python datetime对象（建议使用时区感知）。

**高级过滤示例：**

```python
# 复杂PyArrow表达式
catalog.query(
    data_cls=TradeTick,
    identifiers=["BTC/USD.BINANCE"],
    where="price > 50000 AND size > 1.0",
    start="2024-01-01",
    end="2024-01-02",
)

# 带元数据过滤的多个金融工具
catalog.query(
    data_cls=Bar,
    identifiers=["AAPL.NASDAQ", "MSFT.NASDAQ"],
    where="volume > 1000000",
    metadata={"bar_type": "1-MINUTE-LAST"},
)
```

### 目录操作

目录提供几种操作函数来维护和组织数据文件。这些操作有助于优化存储、提高查询性能并确保数据完整性。

#### 重置文件名

重置parquet文件名以匹配其实际内容时间戳。这确保基于文件名的过滤正常工作。

**重置目录中的所有文件：**

```python
# 重置目录中的所有parquet文件
catalog.reset_catalog_file_names()
```

**重置特定数据类型：**

```python
# 重置所有报价tick文件的文件名
catalog.reset_data_file_names(QuoteTick)

# 重置特定金融工具交易文件的文件名
catalog.reset_data_file_names(TradeTick, "BTC/USD.BINANCE")
```

#### 整合目录

将多个小parquet文件合并为更大的文件，以提高查询性能并减少存储开销。

**整合整个目录：**

```python
# 整合目录中的所有文件
catalog.consolidate_catalog()

# 在特定时间范围内整合文件
catalog.consolidate_catalog(
    start="2024-01-01T00:00:00Z",
    end="2024-01-02T00:00:00Z",
    ensure_contiguous_files=True
)
```

**整合特定数据类型：**

```python
# 整合所有报价tick文件
catalog.consolidate_data(QuoteTick)

# 整合特定金融工具的文件
catalog.consolidate_data(
    TradeTick,
    identifier="BTC/USD.BINANCE",
    start="2024-01-01",
    end="2024-01-31"
)
```

#### 按周期整合目录

将数据文件拆分为固定时间周期，以实现标准化文件组织。

**按周期整合整个目录：**

```python
import pandas as pd

# 按1天周期整合所有文件
catalog.consolidate_catalog_by_period(
    period=pd.Timedelta(days=1)
)

# 在时间范围内按1小时周期整合
catalog.consolidate_catalog_by_period(
    period=pd.Timedelta(hours=1),
    start="2024-01-01T00:00:00Z",
    end="2024-01-02T00:00:00Z"
)
```

**按周期整合特定数据：**

```python
# 按4小时周期整合报价数据
catalog.consolidate_data_by_period(
    data_cls=QuoteTick,
    period=pd.Timedelta(hours=4)
)

# 按30分钟周期整合特定金融工具
catalog.consolidate_data_by_period(
    data_cls=TradeTick,
    identifier="EUR/USD.SIM",
    period=pd.Timedelta(minutes=30),
    start="2024-01-01",
    end="2024-01-31"
)
```

#### 删除数据范围

删除特定数据类型和金融工具指定时间范围内的数据。此操作永久删除数据并智能处理文件交集。

**删除整个目录范围：**

```python
# 删除整个目录中时间范围内的所有数据
catalog.delete_catalog_range(
    start="2024-01-01T00:00:00Z",
    end="2024-01-02T00:00:00Z"
)

# 删除从开始到特定时间的所有数据
catalog.delete_catalog_range(end="2024-01-01T00:00:00Z")
```

**删除特定数据类型：**

```python
# 删除特定金融工具的所有报价tick数据
catalog.delete_data_range(
    data_cls=QuoteTick,
    identifier="BTC/USD.BINANCE"
)

# 删除特定时间范围内的交易数据
catalog.delete_data_range(
    data_cls=TradeTick,
    identifier="EUR/USD.SIM",
    start="2024-01-01T00:00:00Z",
    end="2024-01-31T23:59:59Z"
)
```

:::warning
删除操作永久删除数据且无法撤销。部分重叠删除范围的文件将被拆分以保留范围外的数据。
:::

### Feather流式传输和转换

目录支持在回测期间将数据流式传输到临时feather文件，然后可以转换为永久parquet格式以进行高效查询。

**示例：期权希腊字母流式传输**

```python
from option_trader.greeks import GreeksData
from nautilus_trader.persistence.config import StreamingConfig

# 1. 为自定义数据配置流式传输
streaming = StreamingConfig(
    catalog_path=catalog.path,
    include_types=[GreeksData],
    flush_interval_ms=1000,
)

# 2. 启用流式传输运行回测
engine_config = BacktestEngineConfig(streaming=streaming)
results = node.run()

# 3. 将流式数据转换为永久目录
catalog.convert_stream_to_data(
    results[0].instance_id,
    GreeksData,
)

# 4. 查询转换的数据
greeks_data = catalog.query(
    data_cls=GreeksData,
    start="2024-01-01",
    end="2024-01-31",
    where="delta > 0.5",
)
```

### 目录摘要

NautilusTrader数据目录提供全面的市场数据管理：

**核心功能：**

- **双后端**：Rust性能 + Python灵活性。
- **多协议**：本地、S3、GCS、Azure存储。
- **流式传输**：Feather → Parquet转换管道。
- **操作**：重置文件名、整合数据、基于周期的组织。

**主要使用案例：**

- **回测**：通过BacktestDataConfig预配置数据加载。
- **实时交易**：通过DataCatalogConfig按需数据访问。
- **维护**：文件整合和组织操作。
- **研究**：交互式查询和分析。

## 数据迁移

NautilusTrader定义了在`nautilus_model` crate中指定的内部数据格式。
这些模型被序列化为Arrow记录批次并写入Parquet文件。
使用这些Nautilus格式的Parquet文件时，Nautilus回测最为高效。

然而，在[精度模式](../getting_started/installation.md#precision-mode)和架构更改之间迁移数据模型可能具有挑战性。
本指南解释如何使用我们的实用工具处理数据迁移。

### 迁移工具

`nautilus_persistence` crate提供两个关键实用程序：

#### `to_json`

将Parquet文件转换为JSON，同时保留元数据：

- 创建两个文件：

  - `<input>.json`：包含反序列化的数据
  - `<input>.metadata.json`：包含架构元数据和行组配置

- 从文件名自动检测数据类型：

  - `OrderBookDelta`（包含"deltas"或"order_book_delta"）
  - `QuoteTick`（包含"quotes"或"quote_tick"）
  - `TradeTick`（包含"trades"或"trade_tick"）
  - `Bar`（包含"bars"）

#### `to_parquet`

将JSON转换回Parquet格式：

- 读取数据JSON和元数据JSON文件。
- 从原始元数据保留行组大小。
- 使用ZSTD压缩。
- 创建`<input>.parquet`。

### 迁移过程

以下迁移示例都使用交易数据（您也可以以相同方式迁移其他数据类型）。
所有命令应从`persistence` crate目录的根目录运行。

#### 从标准精度（64位）迁移到高精度（128位）

此示例描述了您想要从标准精度架构迁移到高精度架构的场景。

:::note
如果您正在从使用`Int64`和`UInt64` Arrow数据类型进行价格和大小的目录迁移，
请确保在编译写入初始JSON的代码**之前**检出提交[e284162](https://github.com/nautechsystems/nautilus_trader/commit/e284162cf27a3222115aeb5d10d599c8cf09cf50)。
:::

**1. 从标准精度Parquet转换为JSON**：

```bash
cargo run --bin to_json trades.parquet
```

这将创建`trades.json`和`trades.metadata.json`文件。

**2. 从JSON转换为高精度Parquet**：

添加`--features high-precision`标志以将数据写为高精度（128位）架构Parquet。

```bash
cargo run --features high-precision --bin to_parquet trades.json
```

这将创建具有高精度架构数据的`trades.parquet`文件。

#### 迁移架构更改

此示例描述了您想要从一个架构版本迁移到另一个架构版本的场景。

**1. 从旧架构Parquet转换为JSON**：

如果源数据使用高精度（128位）架构，添加`--features high-precision`标志。

```bash
cargo run --bin to_json trades.parquet
```

这将创建`trades.json`和`trades.metadata.json`文件。

**2. 切换到新架构版本**：

```bash
git checkout <new-version>
```

**3. 从JSON转换回新架构Parquet**：

```bash
cargo run --features high-precision --bin to_parquet trades.json
```

这将创建具有新架构的`trades.parquet`文件。

### 最佳实践

- 始终先用小数据集测试迁移。
- 维护原始文件的备份。
- 迁移后验证数据完整性。
- 在应用到生产数据之前在暂存环境中执行迁移。

## 自定义数据

由于Nautilus设计的模块化特性，可以设置具有非常灵活数据流的系统，包括自定义用户定义的数据类型。本指南涵盖了此功能的一些可能用例。

可以在Nautilus系统内创建自定义数据类型。首先您需要通过继承`Data`来定义您的数据。

:::info
由于`Data`不保存状态，严格来说不需要调用`super().__init__()`。
:::

```python
from nautilus_trader.core import Data


class MyDataPoint(Data):
    """
    这是用户定义数据类的示例，继承自基类`Data`。

    此类中的字段`label`、`x`、`y`和`z`是任意用户数据的示例。
    """

    def __init__(
        self,
        label: str,
        x: int,
        y: int,
        z: int,
        ts_event: int,
        ts_init: int,
    ) -> None:
        self.label = label
        self.x = x
        self.y = y
        self.z = z
        self._ts_event = ts_event
        self._ts_init = ts_init

    @property
    def ts_event(self) -> int:
        """
        数据事件发生时的UNIX时间戳（纳秒）。

        Returns
        -------
        int

        """
        return self._ts_event

    @property
    def ts_init(self) -> int:
        """
        对象初始化时的UNIX时间戳（纳秒）。

        Returns
        -------
        int

        """
        return self._ts_init

```

`Data`抽象基类在系统中作为一个契约，要求所有数据类型都必须有两个属性：`ts_event`和`ts_init`。这些分别代表事件发生时和对象初始化时的UNIX纳秒时间戳。

推荐的方法是将`ts_event`和`ts_init`分配给后备字段，然后实现每个字段的`@property`，如上所示（为了完整性，文档字符串从`Data`基类复制而来）。

:::info
这些时间戳使Nautilus能够使用单调递增的`ts_init` UNIX纳秒为回测正确排序数据流。
:::

现在我们可以使用这种数据类型进行回测和实时交易。例如，我们可以创建一个能够解析并创建这种类型对象的适配器，并将它们发送回`DataEngine`供订阅者使用。

您可以在参与者/策略中使用消息总线发布自定义数据类型，方法如下：

```python
self.publish_data(
    DataType(MyDataPoint, metadata={"some_optional_category": 1}),
    MyDataPoint(...),
)
```

`metadata`字典可选地添加更细粒度的信息，用于与消息总线一起发布数据的主题名称。

额外的元数据信息也可以传递给`BacktestDataConfig`配置对象，以便丰富和描述在回测上下文中使用的自定义数据对象：

```python
from nautilus_trader.config import BacktestDataConfig

data_config = BacktestDataConfig(
    catalog_path=str(catalog.path),
    data_cls=MyDataPoint,
    metadata={"some_optional_category": 1},
)
```

您可以在参与者/策略中订阅自定义数据类型，方法如下：

```python
self.subscribe_data(
    data_type=DataType(MyDataPoint,
    metadata={"some_optional_category": 1}),
    client_id=ClientId("MY_ADAPTER"),
)
```

`client_id`提供标识符，将数据订阅路由到特定客户端。

这将导致您的参与者/策略将接收到的`MyDataPoint`对象传递给您的`on_data`方法。您需要检查类型，因为此方法作为所有自定义数据的灵活处理程序。

```python
def on_data(self, data: Data) -> None:
    # 首先检查数据类型
    if isinstance(data, MyDataPoint):
        # 对数据进行某些操作
```

### 发布和接收信号数据

以下是从参与者或策略使用`MessageBus`发布和接收信号数据的示例。
信号是一个自动生成的自定义数据，由名称标识，仅包含基本类型的一个值（str、float、int、bool或bytes）。

```python
self.publish_signal("signal_name", value, ts_event)
self.subscribe_signal("signal_name")

def on_signal(self, signal):
    print("Signal", data)
```

### 期权希腊字母示例

此示例演示如何为期权希腊字母创建自定义数据类型，特别是delta。
通过遵循这些步骤，您可以创建自定义数据类型，订阅它们，发布它们，并将它们存储在`Cache`或`ParquetDataCatalog`中以便高效检索。

```python
import msgspec
from nautilus_trader.core import Data
from nautilus_trader.core.datetime import unix_nanos_to_iso8601
from nautilus_trader.model import DataType
from nautilus_trader.serialization.base import register_serializable_type
from nautilus_trader.serialization.arrow.serializer import register_arrow
import pyarrow as pa

from nautilus_trader.model import InstrumentId
from nautilus_trader.core.datetime import dt_to_unix_nanos, unix_nanos_to_dt, format_iso8601


class GreeksData(Data):
    def __init__(
        self, instrument_id: InstrumentId = InstrumentId.from_str("ES.GLBX"),
        ts_event: int = 0,
        ts_init: int = 0,
        delta: float = 0.0,
    ) -> None:
        self.instrument_id = instrument_id
        self._ts_event = ts_event
        self._ts_init = ts_init
        self.delta = delta

    def __repr__(self):
        return (f"GreeksData(ts_init={unix_nanos_to_iso8601(self._ts_init)}, instrument_id={self.instrument_id}, delta={self.delta:.2f})")

    @property
    def ts_event(self):
        return self._ts_event

    @property
    def ts_init(self):
        return self._ts_init

    def to_dict(self):
        return {
            "instrument_id": self.instrument_id.value,
            "ts_event": self._ts_event,
            "ts_init": self._ts_init,
            "delta": self.delta,
        }

    @classmethod
    def from_dict(cls, data: dict):
        return GreeksData(InstrumentId.from_str(data["instrument_id"]), data["ts_event"], data["ts_init"], data["delta"])

    def to_bytes(self):
        return msgspec.msgpack.encode(self.to_dict())

    @classmethod
    def from_bytes(cls, data: bytes):
        return cls.from_dict(msgspec.msgpack.decode(data))

    def to_catalog(self):
        return pa.RecordBatch.from_pylist([self.to_dict()], schema=GreeksData.schema())

    @classmethod
    def from_catalog(cls, table: pa.Table):
        return [GreeksData.from_dict(d) for d in table.to_pylist()]

    @classmethod
    def schema(cls):
        return pa.schema(
            {
                "instrument_id": pa.string(),
                "ts_event": pa.int64(),
                "ts_init": pa.int64(),
                "delta": pa.float64(),
            }
        )
```

#### 发布和接收数据

以下是从参与者或策略使用`MessageBus`发布和接收数据的示例：

```python
register_serializable_type(GreeksData, GreeksData.to_dict, GreeksData.from_dict)

def publish_greeks(self, greeks_data: GreeksData):
    self.publish_data(DataType(GreeksData), greeks_data)

def subscribe_to_greeks(self):
    self.subscribe_data(DataType(GreeksData))

def on_data(self, data):
    if isinstance(GreeksData):
        print("Data", data)
```

#### 使用缓存写入和读取数据

以下是从参与者或策略使用`Cache`写入和读取数据的示例：

```python
def greeks_key(instrument_id: InstrumentId):
    return f"{instrument_id}_GREEKS"

def cache_greeks(self, greeks_data: GreeksData):
    self.cache.add(greeks_key(greeks_data.instrument_id), greeks_data.to_bytes())

def greeks_from_cache(self, instrument_id: InstrumentId):
    return GreeksData.from_bytes(self.cache.get(greeks_key(instrument_id)))
```

#### 使用目录写入和读取数据

要将自定义数据流式传输到feather文件或将其写入目录中的parquet文件（需要使用`register_arrow`）：

```python
register_arrow(GreeksData, GreeksData.schema(), GreeksData.to_catalog, GreeksData.from_catalog)

from nautilus_trader.persistence.catalog import ParquetDataCatalog
catalog = ParquetDataCatalog('.')

catalog.write_data([GreeksData()])
```

### 自动创建自定义数据类

`@customdataclass`装饰器能够创建一个自定义数据类，并为上述所有功能提供默认实现。

如果需要，每个方法也可以被覆盖。以下是其使用示例：

```python
from nautilus_trader.model.custom import customdataclass


@customdataclass
class GreeksTestData(Data):
    instrument_id: InstrumentId = InstrumentId.from_str("ES.GLBX")
    delta: float = 0.0


GreeksTestData(
    instrument_id=InstrumentId.from_str("CL.GLBX"),
    delta=1000.0,
    ts_event=1,
    ts_init=2,
)
```

#### 自定义数据类型存根

为了增强开发便利性并改善IDE中的代码建议，您可以为自定义数据类型创建`.pyi`存根文件，
其中包含正确的构造函数签名以及属性的类型提示。
当构造函数在运行时动态生成时，这尤其有用，因为它允许IDE识别并为类的方法和属性提供建议。

例如，如果您在`greeks.py`中定义了自定义数据类，您可以创建相应的`greeks.pyi`文件，
其中包含以下构造函数签名：

```python
from nautilus_trader.core import Data
from nautilus_trader.model import InstrumentId


class GreeksData(Data):
    instrument_id: InstrumentId
    delta: float

    def __init__(
        self,
        ts_event: int = 0,
        ts_init: int = 0,
        instrument_id: InstrumentId = InstrumentId.from_str("ES.GLBX"),
        delta: float = 0.0,
  ) -> GreeksData: ...
```
