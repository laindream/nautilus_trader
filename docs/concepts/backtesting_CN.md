# 回测

使用NautilusTrader进行回测是一个有条理的模拟过程，使用特定系统实现复制交易活动。该系统由各种组件组成，包括内置引擎、`Cache`、[消息总线](message_bus_CN.md)、`Portfolio`、[角色](actors_CN.md)、[策略](strategies_CN.md)、[执行算法](execution_CN.md)和其他用户定义的模块。整个交易模拟基于由`BacktestEngine`处理的历史数据流。一旦这个数据流耗尽，引擎结束其操作，产生详细的结果和性能指标以进行深入分析。

重要的是要认识到NautilusTrader为设置和进行回测提供了两个不同的API级别：

- **高级API**：使用`BacktestNode`和配置对象（内部使用`BacktestEngine`）。
- **低级API**：直接使用`BacktestEngine`进行更"手动"的设置。

## 选择API级别

在以下情况下考虑使用**低级**API：

- 您的整个数据流可以在可用的机器资源（例如，RAM）内处理。
- 您不希望将数据存储为Nautilus特定的Parquet格式。
- 您有特定需求或偏好保留原始格式的原始数据（例如，CSV、二进制等）。
- 您需要对`BacktestEngine`进行细粒度控制，例如能够在相同数据集上重新运行回测，同时交换组件（例如，角色或策略）或调整参数配置。

在以下情况下考虑使用**高级**API：

- 您的数据流超过可用内存，需要分批流式传输数据。
- 您希望利用`ParquetDataCatalog`的性能和便利性，将数据存储为Nautilus特定的Parquet格式。
- 您重视传递配置对象的灵活性和功能，以定义和管理跨多个引擎同时进行的多个回测运行。

## 低级API

低级API围绕`BacktestEngine`，其中输入通过Python脚本手动初始化和添加。
实例化的`BacktestEngine`可以接受以下内容：

- `Data`对象列表，基于`ts_init`自动排序为单调顺序。
- 多个场所，手动初始化。
- 多个角色，手动初始化和添加。
- 多个执行算法，手动初始化和添加。

这种方法提供了对回测过程的详细控制，允许您手动配置每个组件。

## 高级API

高级API围绕`BacktestNode`，它协调多个`BacktestEngine`实例的管理，每个实例由`BacktestRunConfig`定义。多个配置可以捆绑到列表中，并由节点在一次运行中处理。

每个`BacktestRunConfig`对象包括以下内容：

- `BacktestDataConfig`对象列表。
- `BacktestVenueConfig`对象列表。
- `ImportableActorConfig`对象列表。
- `ImportableStrategyConfig`对象列表。
- `ImportableExecAlgorithmConfig`对象列表。
- 可选的`ImportableControllerConfig`对象。
- 可选的`BacktestEngineConfig`对象，如果未指定则使用默认配置。

## 数据

为回测提供的数据驱动执行流程。由于可以使用各种数据类型，确保您的场所配置与为回测提供的数据保持一致至关重要。数据和配置之间的不匹配可能导致执行期间的意外行为。

NautilusTrader主要设计和优化用于订单簿数据，它提供市场中每个价格级别或订单的完整表示，反映交易场所的实时行为。这确保了最高级别的执行粒度和真实性。但是，如果粒度订单簿数据不可用或不必要，平台有能力按以下详细程度的降序处理市场数据：

1. **订单簿数据/增量（L3市场按订单）**：
   - 提供全面的市场深度和详细的订单流，可见所有单个订单。

2. **订单簿数据/增量（L2市场按价格）**：
   - 提供跨所有价格级别的市场深度可见性。

3. **报价Tick（L1市场按价格）**：
   - 代表"账簿顶部"，仅捕获最佳买价和卖价及大小。

4. **交易Tick**：
   - 反映实际执行的交易，提供交易活动的精确视图。

5. **K线**：
   - 聚合交易活动 - 通常在固定时间间隔内，如1分钟、1小时或1天。

### 选择数据：成本vs准确性

对于许多交易策略，K线数据（例如，1分钟）对于回测和策略开发来说是足够的。这一点特别重要，因为与tick或订单簿数据相比，K线数据通常更容易获得且成本效益更高。

考虑到这种实际情况，Nautilus被设计为支持基于K线的回测，具有高级功能，即使在使用较低粒度数据时也能最大化模拟准确性。

:::tip
对于某些交易策略，从K线数据开始开发以验证核心交易想法是实用的。如果策略看起来有前景，但对精确执行时机更敏感（例如，需要在OHLC级别之间的特定价格成交，或使用紧密的止盈/止损级别），您可以投资更高粒度的数据以获得更准确的验证。
:::

## 场所

在为回测初始化场所时，您必须从以下选项中为执行处理指定其内部订单`book_type`：

- `L1_MBP`：第1级市场按价格（默认）。仅维护订单簿的顶层。
- `L2_MBP`：第2级市场按价格。维护订单簿深度，每个价格级别聚合单个订单。
- `L3_MBO`：第3级市场按订单。维护订单簿深度，跟踪数据提供的所有单个订单。

:::note
数据的粒度必须匹配指定的订单`book_type`。Nautilus无法从较低级别的数据（如报价、交易或K线）生成更高粒度的数据（L2或L3）。
:::

:::warning
如果您指定`L2_MBP`或`L3_MBO`作为场所的`book_type`，所有非订单簿数据（如报价、交易和K线）将在执行处理中被忽略。这可能导致订单看起来永远不会成交。我们正在积极开发改进的验证逻辑以防止配置和数据不匹配。
:::

:::warning
当提供L2或更高级别的订单簿数据时，确保`book_type`更新以反映数据的粒度。未能这样做将导致数据聚合：L2数据将减少为每个级别的单个订单，L1数据将仅反映账簿顶部级别。
:::

## 执行

### 数据和消息排序

在主回测循环中，新市场数据首先处理现有订单的执行，然后由数据引擎处理，该引擎随后将数据发送给策略。

### 基于K线的执行

K线数据提供每个时间段的市场活动摘要，具有四个关键价格（假设K线按交易聚合）：

- **开盘**：开盘价（第一笔交易）
- **最高**：交易的最高价
- **最低**：交易的最低价
- **收盘**：收盘价（最后一笔交易）

虽然这给了我们价格运动的概览，但我们失去了一些我们使用更细粒度数据会有的重要信息：

- 我们不知道市场以什么顺序达到最高价和最低价。
- 我们不能确切看到价格在时间段内何时变化。
- 我们不知道发生的交易的实际顺序。

这就是为什么Nautilus通过一个系统处理K线数据，该系统尝试在这些限制下维护最现实但保守的市场行为。在其核心，平台始终维护订单簿模拟 - 即使当您提供较少细粒度的数据（如报价、交易或K线）时（尽管模拟只会有顶级账簿）。

:::warning
当使用K线进行执行模拟时（在场所配置中默认启用`bar_execution=True`），Nautilus严格期望每个K线的时间戳（`ts_init`）代表其**收盘时间**。这确保准确的时间顺序处理，防止前瞻偏差，并将市场更新（开盘→最高→最低→收盘）与K线完成的时刻对齐。
:::

#### K线时间戳约定

如果您的数据源提供在**开盘时间**加时间戳的K线（在某些提供商中很常见），您必须在加载到Nautilus之前将它们调整为收盘时间。未能这样做可能导致不正确的订单成交、事件排序错误或不现实的回测结果。

- 使用适配器特定的配置，如`bars_timestamp_on_close=True`（例如，对于Bybit或Databento适配器）在数据摄取期间自动处理这个问题。
- 对于自定义数据，手动将时间戳按K线持续时间移动（例如，为`1-MINUTE`K线添加1分钟）。
- 始终验证您的数据的时间戳约定与小样本，以避免模拟不准确。

#### 处理K线数据

即使当您提供K线数据时，Nautilus为每个工具维护内部订单簿 - 就像真实场所会做的一样。

1. **时间处理**：
   - Nautilus有一种特定的处理K线数据*用于执行*的时间方法，这对于准确模拟至关重要。
   - K线时间戳（`ts_event`）预期代表K线的收盘时间。这种方法是最合理的，因为它代表K线完全形成且其聚合完成的时刻。
   - 初始化时间（`ts_init`）可以使用`BarDataWrangler`中的`ts_init_delta`参数控制，该参数通常应设置为K线的步长（时间框架）以纳秒为单位。
   - 平台确保所有事件基于这些时间戳以正确顺序发生，防止回测中任何前瞻偏差的可能性。

2. **价格处理**：
   - 平台将每个K线的OHLC价格转换为市场更新序列。
   - 这些更新始终遵循相同的顺序：开盘→最高→最低→收盘。
   - 如果您提供多个时间框架（如1分钟和5分钟K线），平台使用更细粒度的数据以获得最高准确性。

3. **执行**：
   - 当您下订单时，它们与模拟订单簿交互，就像它们在真实场所上一样。
   - 对于市价订单，执行发生在当前模拟市场价格加上任何配置的延迟。
   - 对于在市场中工作的限价订单，如果K线的任何价格达到或穿越您的限价（见下文），它们将执行。
   - 匹配引擎随着OHLC价格移动持续处理订单，而不是等待完整的K线。

#### OHLC价格模拟

在回测执行期间，每个K线被转换为四个价格点的序列：

1. 开盘价
2. 最高价 *（最高/最低之间的顺序是可配置的。请参见下面的`bar_adaptive_high_low_ordering`。）*
3. 最低价
4. 收盘价

该K线的交易量**平均分配**给这四个点（每个25%）。在边际情况下，如果原始K线的交易量除以4小于工具的最小`size_increment`，我们仍然为每个价格点使用最小`size_increment`以确保有效的市场活动（例如，CME集团交易所的1个合约）。

这些价格点的排序可以通过配置场所时的`bar_adaptive_high_low_ordering`参数控制。

Nautilus支持两种K线处理模式：

1. **固定排序**（`bar_adaptive_high_low_ordering=False`，默认）
   - 按固定序列处理每个K线：`开盘→最高→最低→收盘`。
   - 简单且确定性的方法。

2. **自适应排序**（`bar_adaptive_high_low_ordering=True`）
   - 使用K线结构估计可能的价格路径：
     - 如果开盘价更接近最高价：处理为`开盘→最高→最低→收盘`。
     - 如果开盘价更接近最低价：处理为`开盘→最低→最高→收盘`。
   - [研究](https://gist.github.com/stefansimik/d387e1d9ff784a8973feca0cde51e363)显示这种方法在预测正确的最高/最低序列方面达到约75-85%的准确率（与固定排序的统计50%准确率相比）。
   - 当止盈和止损水平都出现在同一K线内时，这一点特别重要——因为序列决定了哪个订单首先成交。

以下是如何为场所配置自适应K线排序，包括账户设置：

```python
from nautilus_trader.backtest.engine import BacktestEngine
from nautilus_trader.model.enums import OmsType, AccountType
from nautilus_trader.model import Money, Currency

# 初始化回测引擎
engine = BacktestEngine()

# 添加具有自适应K线排序和所需账户设置的场所
engine.add_venue(
    venue=venue,  # 您的场所标识符，例如Venue("BINANCE")
    oms_type=OmsType.NETTING,
    account_type=AccountType.CASH,
    starting_balances=[Money(10_000, Currency.from_str("USDT"))],
    bar_adaptive_high_low_ordering=True,  # 启用最高/最低K线价格的自适应排序
)
```

### 滑点和价差处理

在使用不同类型的数据进行回测时，Nautilus为滑点和价差模拟实施特定处理：

对于L2（市场按价格）或L3（市场按订单）数据，滑点通过以下方式高精度模拟：

- 根据实际订单簿级别成交订单。
- 按顺序匹配每个价格级别的可用数量。
- 维护现实的订单簿深度影响（每次订单成交）。

对于L1数据类型（例如，L1订单簿、交易、报价、K线），滑点通过以下方式处理：

**初始成交滑点**（`prob_slippage`）：

- 由`FillModel`的`prob_slippage`参数控制。
- 确定初始成交是否会在距离当前市场价格一个tick的地方发生。
- 示例：使用`prob_slippage=0.5`，市价买单有50%的机会在最佳卖价上方一个tick成交。

:::note
当使用K线数据进行回测时，请注意价格信息的降低粒度影响滑点机制。为了获得最现实的回测结果，在可用时考虑使用更高粒度的数据源，如L2或L3订单簿数据。
:::

### Fill Model

`FillModel`有助于在回测期间以简单的概率方式模拟订单队列位置和执行。它解决了一个基本挑战：*即使有完美的历史市场数据，我们也不能完全模拟订单在实时中如何与其他市场参与者互动*。

`FillModel`模拟交易中存在的两个关键方面，无论数据质量如何：

1. **限价订单的队列位置**：
   - 当多个交易者在相同价格级别下订单时，订单在队列中的位置影响它是否以及何时成交。

2. **市场影响和竞争**：
   - 当使用市价订单获取流动性时，您与其他交易者竞争可用流动性，这可能影响您的成交价格。

#### 配置和参数

```python
from nautilus_trader.backtest.models import FillModel
from nautilus_trader.backtest.engine import BacktestEngine
from nautilus_trader.backtest.engine import BacktestEngineConfig

# 创建具有所需概率的自定义成交模型
fill_model = FillModel(
    prob_fill_on_limit=0.2,    # 限价订单在价格匹配时成交的机会（应用于K线/交易/报价+L1/L2/L3订单簿）
    prob_fill_on_stop=0.95,    # [已弃用]将在未来版本中删除，使用`prob_slippage`代替
    prob_slippage=0.5,         # 1-tick滑点的机会（仅应用于K线/交易/报价+L1订单簿）
    random_seed=None,          # 可选：设置以获得可重现的结果
)

# 将成交模型添加到您的引擎配置
engine = BacktestEngine(
    config=BacktestEngineConfig(
        trader_id="TESTER-001",
        fill_model=fill_model,  # 在此注入您的自定义成交模型
    )
)
```

**prob_fill_on_limit**（默认：`1.0`）

- 目的：
  - 模拟限价订单在市场达到其价格级别时成交的概率。
- 详细信息：
  - 模拟您在给定价格级别的订单队列中的位置。
  - 适用于所有数据类型（例如，L1/L2/L3订单簿、报价、交易、K线）。
  - 每次市场价格触及您的订单价格（但不穿越它）时进行新的随机概率检查。
  - 成功概率检查时，成交整个剩余订单数量。

**示例**：

- 使用`prob_fill_on_limit=0.0`：
  - 限价买单在最佳卖价达到限价时永远不会成交。
  - 限价卖单在最佳买价达到限价时永远不会成交。
  - 这模拟在队列的最后面，永远不会到达前面。
- 使用`prob_fill_on_limit=0.5`：
  - 限价买单在最佳卖价达到限价时有50%的成交机会。
  - 限价卖单在最佳买价达到限价时有50%的成交机会。
  - 这模拟在队列的中间位置。
- 使用`prob_fill_on_limit=1.0`（默认）：
  - 限价买单在最佳卖价达到限价时总是成交。
  - 限价卖单在最佳买价达到限价时总是成交。
  - 这模拟在队列的前面，保证成交。

**prob_slippage**（默认：`0.0`）

- 目的：
  - 模拟执行市价订单时经历价格滑点的概率。
- 详细信息：
  - 仅适用于L1数据类型（例如，报价、交易、K线）。
  - 触发时，将成交价格向您的订单方向移动一个tick。
  - 影响所有市价类型订单（`MARKET`、`MARKET_TO_LIMIT`、`MARKET_IF_TOUCHED`、`STOP_MARKET`）。
  - 不用于L2/L3数据，其中订单簿深度可以确定滑点。

**示例**：

- 使用`prob_slippage=0.0`（默认）：
  - 不应用人工滑点，代表理想化场景，您总是以当前市场价格成交。
- 使用`prob_slippage=0.5`：
  - 市价买单有50%的机会在最佳卖价上方一个tick成交，50%的机会在最佳卖价成交。
  - 市价卖单有50%的机会在最佳买价下方一个tick成交，50%的机会在最佳买价成交。
- 使用`prob_slippage=1.0`：
  - 市价买单总是在最佳卖价上方一个tick成交。
  - 市价卖单总是在最佳买价下方一个tick成交。
  - 这模拟对您的订单的一致不利价格移动。

## 账户类型

当您将场所附加到引擎时——无论是实时交易还是回测——您必须通过传递`account_type`参数从三种会计模式中选择一种：

| 账户类型           | 典型用例                                         | 引擎锁定什么                                                                                              |
| ---------------------- | -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------|
| Cash                   | 现货交易（例如BTC/USDT，股票）                     | 每个待成交订单将开仓的头寸的名义价值。                                                      |
| Margin                 | 衍生品或任何允许杠杆的产品          | 每个订单的初始保证金加上持仓的维持保证金。                                          |
| Betting                | 体育博彩，做市                              | 场所要求的资本；没有杠杆。                                                                          |

添加回测场所的`CASH`账户示例：

```python
from nautilus_trader.adapters.binance import BINANCE_VENUE
from nautilus_trader.backtest.engine import BacktestEngine
from nautilus_trader.model.currencies import USDT
from nautilus_trader.model.enums import OmsType, AccountType
from nautilus_trader.model import Money, Currency

# 初始化回测引擎
engine = BacktestEngine()

# 为场所添加CASH账户
engine.add_venue(
    venue=BINANCE_VENUE,  # 创建或引用场所标识符
    oms_type=OmsType.NETTING,
    account_type=AccountType.CASH,
    starting_balances=[Money(10_000, USDT)],
)
```

### 现金账户

现金账户全额结算交易；没有杠杆，因此没有保证金概念。

### 保证金账户

*保证金账户*促进需要保证金的工具交易，如期货或杠杆产品。它跟踪账户余额，计算所需保证金，并管理杠杆以确保头寸和订单有足够的抵押品。

**关键概念**：

- **杠杆**：相对于账户权益放大交易敞口。更高的杠杆增加潜在回报和风险。
- **初始保证金**：提交订单开仓头寸所需的抵押品。
- **维持保证金**：维持持仓所需的最低抵押品。
- **锁定余额**：作为抵押品保留的资金，不可用于新订单或提取。

:::note
减仓订单在现金账户中**不**贡献于`balance_locked`，在保证金账户中也不增加初始保证金——因为它们只能减少现有敞口。
:::

### 投注账户

投注账户专门用于您下注一定金额以赢得或输掉固定支出的场所（一些预测市场、体育博彩等）。引擎只锁定场所要求的资本；杠杆和保证金不适用。

## 保证金模型

Nautilus Trader提供灵活的保证金计算模型以适应不同的场所类型和交易场景。

### 概述

不同的场所和经纪商在计算保证金要求方面有不同的方法：

- **传统经纪商**（Interactive Brokers、TD Ameritrade）：无论杠杆如何都使用固定保证金百分比。
- **加密交易所**（Binance等）：杠杆可能会降低保证金要求。
- **期货交易所**（CME、ICE）：每个合约的固定保证金金额。

### 可用模型

#### StandardMarginModel

使用固定百分比而不除以杠杆，匹配传统经纪商行为。

**公式：**

```python
# 固定百分比 - 忽略杠杆
margin = notional * instrument.margin_init
```

- 初始保证金 = `notional_value * instrument.margin_init`
- 维持保证金 = `notional_value * instrument.margin_maint`

**用例：**

- 传统经纪商（Interactive Brokers、TD Ameritrade）。
- 期货交易所（CME、ICE）。
- 具有固定保证金要求的外汇经纪商。

#### LeveragedMarginModel

将保证金要求除以杠杆。

**公式：**

```python
# 杠杆减少保证金要求
adjusted_notional = notional / leverage
margin = adjusted_notional * instrument.margin_init
```

- 初始保证金 = `(notional_value / leverage) * instrument.margin_init`
- 维持保证金 = `(notional_value / leverage) * instrument.margin_maint`

**用例：**

- 通过杠杆减少保证金的加密交易所。
- 杠杆影响保证金要求的场所。

### 使用方法

#### 程序化配置

```python
from nautilus_trader.backtest.models import LeveragedMarginModel
from nautilus_trader.backtest.models import StandardMarginModel
from nautilus_trader.test_kit.stubs.execution import TestExecStubs

# 创建账户
account = TestExecStubs.margin_account()

# 为传统经纪商设置标准模型
standard_model = StandardMarginModel()
account.set_margin_model(standard_model)

# 或为加密交易所使用杠杆模型
leveraged_model = LeveragedMarginModel()
account.set_margin_model(leveraged_model)
```

#### 回测配置

```python
from nautilus_trader.backtest.config import BacktestVenueConfig
from nautilus_trader.backtest.config import MarginModelConfig

venue_config = BacktestVenueConfig(
    name="SIM",
    oms_type="NETTING",
    account_type="MARGIN",
    starting_balances=["1_000_000 USD"],
    margin_model=MarginModelConfig(model_type="standard"),  # 选项：'standard'、'leveraged'
)
```

#### 可用模型类型

- `"leveraged"`：杠杆减少保证金（默认）。
- `"standard"`：固定百分比（传统经纪商）。
- 自定义类路径：`"my_package.my_module.MyMarginModel"`。

#### 默认行为

默认情况下，`MarginAccount`使用`LeveragedMarginModel`。

#### 真实世界示例

**EUR/USD交易场景：**

- **工具**：EUR/USD
- **数量**：100,000 EUR
- **价格**：1.10000
- **名义价值**：$110,000
- **杠杆**：50x
- **工具初始保证金**：3%

**保证金计算：**

| 模型     | 计算           | 结果  | 百分比 |
|-----------|----------------------|---------|------------|
| Standard  | $110,000 × 0.03      | $3,300  | 3.00%      |
| Leveraged | ($110,000 ÷ 50) × 0.03 | $66   | 0.06%      |

**账户余额影响：**

- **账户余额**：$10,000
- **标准模型**：无法交易（需要$3,300保证金）
- **杠杆模型**：可以交易（仅需要$66保证金）

### 真实世界场景

#### Interactive Brokers EUR/USD期货

```python
# IB需要固定保证金，无论杠杆如何
account.set_margin_model(StandardMarginModel())
margin = account.calculate_margin_init(instrument, quantity, price)
# 结果：名义价值的固定百分比
```

#### Binance加密交易

```python
# Binance可能通过杠杆减少保证金
account.set_margin_model(LeveragedMarginModel())
margin = account.calculate_margin_init(instrument, quantity, price)
# 结果：保证金被杠杆因子减少
```

### 模型选择

#### 使用默认模型

默认的`LeveragedMarginModel`开箱即用：

```python
account = TestExecStubs.margin_account()
margin = account.calculate_margin_init(instrument, quantity, price)
```

#### 使用标准模型

对于传统经纪商行为：

```python
account.set_margin_model(StandardMarginModel())
margin = account.calculate_margin_init(instrument, quantity, price)
```

### 自定义模型

您可以通过继承`MarginModel`创建自定义保证金模型。自定义模型通过`MarginModelConfig`接收配置：

```python
from nautilus_trader.backtest.models import MarginModel
from nautilus_trader.backtest.config import MarginModelConfig

class RiskAdjustedMarginModel(MarginModel):
    def __init__(self, config: MarginModelConfig):
        """使用配置参数初始化。"""
        self.risk_multiplier = Decimal(str(config.config.get("risk_multiplier", 1.0)))
        self.use_leverage = config.config.get("use_leverage", False)

    def calculate_margin_init(self, instrument, quantity, price, leverage, use_quote_for_inverse=False):
        notional = instrument.notional_value(quantity, price, use_quote_for_inverse)
        if self.use_leverage:
            adjusted_notional = notional.as_decimal() / leverage
        else:
            adjusted_notional = notional.as_decimal()
        margin = adjusted_notional * instrument.margin_init * self.risk_multiplier
        return Money(margin, instrument.quote_currency)

    def calculate_margin_maint(self, instrument, side, quantity, price, leverage, use_quote_for_inverse=False):
        return self.calculate_margin_init(instrument, quantity, price, leverage, use_quote_for_inverse)
```

#### 使用自定义模型

**程序化：**

```python
from nautilus_trader.backtest.config import MarginModelConfig
from nautilus_trader.backtest.config import MarginModelFactory

config = MarginModelConfig(
    model_type="my_package.my_module:RiskAdjustedMarginModel",
    config={"risk_multiplier": 1.5, "use_leverage": False}
)

custom_model = MarginModelFactory.create(config)
account.set_margin_model(custom_model)
```

### 高级回测API配置

使用高级回测API时，您可以在场所配置中使用`MarginModelConfig`指定保证金模型：

```python
from nautilus_trader.backtest.config import MarginModelConfig
from nautilus_trader.backtest.config import BacktestVenueConfig
from nautilus_trader.config import BacktestRunConfig

# 使用特定保证金模型配置场所
venue_config = BacktestVenueConfig(
    name="SIM",
    oms_type="NETTING",
    account_type="MARGIN",
    starting_balances=["1_000_000 USD"],
    margin_model=MarginModelConfig(
        model_type="standard"  # 使用标准模型进行传统经纪商模拟
    ),
)

# 在回测配置中使用
config = BacktestRunConfig(
    venues=[venue_config],
    # ... 其他配置
)
```

#### 配置示例

**标准模型（传统经纪商）：**

```python
margin_model=MarginModelConfig(model_type="standard")
```

**杠杆模型（默认）：**

```python
margin_model=MarginModelConfig(model_type="leveraged")  # 默认
```

**带配置的自定义模型：**

```python
margin_model=MarginModelConfig(
    model_type="my_package.my_module:CustomMarginModel",
    config={
        "risk_multiplier": 1.5,
        "use_leverage": False,
        "volatility_threshold": 0.02,
    }
)
```

保证金模型将在回测执行期间自动应用于模拟交易所。
