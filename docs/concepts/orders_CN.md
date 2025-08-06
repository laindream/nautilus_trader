# 订单

本指南提供了平台可用订单类型的更多详细信息，以及每种订单支持的执行指令。

订单是任何算法交易策略的基本构建块之一。
NautilusTrader支持广泛的订单类型和执行指令，从标准到高级，尽可能多地暴露交易场所的功能。这使交易者能够定义订单执行和管理的指令和应急措施，便于创建几乎任何交易策略。

## 概述

所有订单类型都源自两个基本类型：*市价*和*限价*订单。就流动性而言，它们是相反的。
*市价*订单通过以最佳可用价格立即执行来消费流动性，而*限价*订单通过在指定价格的订单簿中等待匹配来提供流动性。

平台可用的订单类型有（使用枚举值）：

- `MARKET`
- `LIMIT`
- `STOP_MARKET`
- `STOP_LIMIT`
- `MARKET_TO_LIMIT`
- `MARKET_IF_TOUCHED`
- `LIMIT_IF_TOUCHED`
- `TRAILING_STOP_MARKET`
- `TRAILING_STOP_LIMIT`

:::info
NautilusTrader为许多订单类型和执行指令提供统一的API，但并非所有场所都支持每个选项。
如果订单包含目标场所不支持的指令或选项，系统**不应**提交订单；相反，它会记录一个带有清晰解释消息的错误。
:::

### 术语

- 如果订单类型是`MARKET`或作为*可市场化*订单执行（即消费流动性），则订单是**激进的**。
- 如果订单不可市场化（即提供流动性），则订单是**被动的**。
- 如果订单在以下三种非终端状态之一中保持在本地系统边界内，则订单是**活跃本地**：
  - `INITIALIZED`
  - `EMULATED`
  - `RELEASED`
- 当订单处于以下状态之一时，订单是**传输中**：
  - `SUBMITTED`
  - `PENDING_UPDATE`
  - `PENDING_CANCEL`
- 当订单处于以下（非终端）状态之一时，订单是**开放的**：
  - `ACCEPTED`
  - `TRIGGERED`
  - `PENDING_UPDATE`
  - `PENDING_CANCEL`
  - `PARTIALLY_FILLED`
- 当订单处于以下（终端）状态之一时，订单是**关闭的**：
  - `DENIED`
  - `REJECTED`
  - `CANCELED`
  - `EXPIRED`
  - `FILLED`

## 执行指令

某些场所允许交易者指定订单如何处理和执行的条件和限制。以下是可用的不同执行指令的简要摘要。

### 有效期

订单的有效期指定订单在任何剩余数量被取消之前将保持开放或活跃的时间。

- `GTC` **（Good Till Cancel）**：订单保持活跃，直到被交易者或场所取消。
- `IOC` **（Immediate or Cancel / Fill and Kill）**：订单立即执行，任何未成交部分被取消。
- `FOK` **（Fill or Kill）**：订单立即完全执行或根本不执行。
- `GTD` **（Good Till Date）**：订单保持活跃，直到指定的到期日期和时间。
- `DAY` **（Good for session/day）**：订单保持活跃，直到当前交易会话结束。
- `AT_THE_OPEN` **（OPG）**：订单仅在交易会话开盘时活跃。
- `AT_THE_CLOSE`：订单仅在交易会话收盘时活跃。

### 到期时间

此指令与`GTD`有效期一起使用，以指定订单将到期并从场所的订单簿（或订单管理系统）中删除的时间。

### 仅发布

标记为`post_only`的订单将只参与向限价订单簿提供流动性，永远不会发起消费流动性的激进交易。此选项对于做市商或寻求将订单限制为流动性*制造者*费用层级的交易者很重要。

### 仅减少

设置为`reduce_only`的订单将只减少工具上的现有头寸，永远不会开立新头寸（如果已经平仓）。此指令的确切行为可能因场所而异。

但是，Nautilus `SimulatedExchange`中的行为典型于真实场所。

- 如果关联头寸关闭（变为平仓），订单将被取消。
- 随着关联头寸大小的减少，订单数量将减少。

### 显示数量

`display_qty`指定在限价订单簿上显示的*限价*订单的部分。这些也被称为冰山订单，因为有一个可见的部分要显示，还有更多隐藏的数量。指定显示数量为零也等同于将订单设置为`hidden`。

### 触发类型

也称为[触发方法](https://guides.interactivebrokers.com/tws/usersguidebook/configuretws/modify_the_stop_trigger_method.htm)，适用于条件触发订单，指定触发止损价格的方法。

- `DEFAULT`：场所的默认触发类型（通常是`LAST`或`BID_ASK`）。
- `LAST`：触发价格将基于最后交易价格。
- `BID_ASK`：触发价格将基于买入订单的`BID`和卖出订单的`ASK`。
- `DOUBLE_LAST`：触发价格将基于最后两个连续的`LAST`价格。
- `DOUBLE_BID_ASK`：触发价格将基于最后两个连续的`BID`或`ASK`价格（如适用）。
- `LAST_OR_BID_ASK`：触发价格将基于`LAST`或`BID`/`ASK`。
- `MID_POINT`：触发价格将基于`BID`和`ASK`之间的中点。
- `MARK`：触发价格将基于场所对该工具的标记价格。
- `INDEX`：触发价格将基于场所对该工具的指数价格。

### 触发偏移类型

适用于条件追踪止损触发订单，指定基于与*市场*（买价、卖价或最后价格，如适用）的偏移触发止损价格修改的方法。

- `DEFAULT`：场所的默认偏移类型（通常是`PRICE`）。
- `PRICE`：偏移基于价格差异。
- `BASIS_POINTS`：偏移基于以基点表示的价格百分比差异（100bp = 1%）。
- `TICKS`：偏移基于tick数量。
- `PRICE_TIER`：偏移基于场所特定的价格层级。

### 应急订单

可以在订单之间指定更高级的关系。
例如，子订单可以分配为仅在父订单被激活或成交时触发，或者订单可以链接，以便一个取消或减少另一个的数量。有关更多详细信息，请参阅[高级订单](#advanced-orders)部分。

## 订单工厂

创建新订单的最简单方法是使用内置的`OrderFactory`，它会自动附加到每个`Strategy`类。这个工厂将处理较低级别的细节——例如确保分配正确的交易者ID和策略ID，生成必要的初始化ID和时间戳，并抽象出不一定适用于正在创建的订单类型或仅在指定更高级执行指令时需要的参数。

这使工厂具有更简单的订单创建方法，所有示例都将在`Strategy`上下文中利用`OrderFactory`。

:::info
有关更多详细信息，请参阅`OrderFactory` [API参考](../api_reference/common_CN.md#class-orderfactory)。
:::

## 订单类型

以下描述了平台可用的订单类型及代码示例。
任何可选参数都将用包含默认值的注释清楚标记。

### 市价订单

*市价*订单是交易者立即以最佳可用价格交易给定数量的指令。您还可以指定几个有效期选项，并指示此订单是否仅用于减少头寸。

在以下示例中，我们在Interactive Brokers [IdealPro](https://ibkr.info/node/1708) Forex ECN上创建一个*市价*订单，使用USD购买100,000 AUD：

```python
from nautilus_trader.model.enums import OrderSide
from nautilus_trader.model.enums import TimeInForce
from nautilus_trader.model import InstrumentId
from nautilus_trader.model import Quantity
from nautilus_trader.model.orders import MarketOrder

order: MarketOrder = self.order_factory.market(
    instrument_id=InstrumentId.from_str("AUD/USD.IDEALPRO"),
    order_side=OrderSide.BUY,
    quantity=Quantity.from_int(100_000),
    time_in_force=TimeInForce.IOC,  # <-- 可选（默认GTC）
    reduce_only=False,  # <-- 可选（默认False）
    tags=["ENTRY"],  # <-- 可选（默认None）
)
```

:::info
有关更多详细信息，请参阅`MarketOrder` [API参考](../api_reference/model/orders_CN.md#class-marketorder)。
:::

### 限价订单

*限价*订单以特定价格放置在限价订单簿上，只会以该价格（或更好的价格）执行。

在以下示例中，我们在Binance期货加密交易所创建一个*限价*订单，以5000 USDT的限价卖出20个ETHUSDT-PERP永续期货合约，作为做市商。

```python
from nautilus_trader.model.enums import OrderSide
from nautilus_trader.model.enums import TimeInForce
from nautilus_trader.model import InstrumentId
from nautilus_trader.model import Price
from nautilus_trader.model import Quantity
from nautilus_trader.model.orders import LimitOrder

order: LimitOrder = self.order_factory.limit(
    instrument_id=InstrumentId.from_str("ETHUSDT-PERP.BINANCE"),
    order_side=OrderSide.SELL,
    quantity=Quantity.from_int(20),
    price=Price.from_str("5_000.00"),
    time_in_force=TimeInForce.GTC,  # <-- 可选（默认GTC）
    expire_time=None,  # <-- 可选（默认None）
    post_only=True,  # <-- 可选（默认False）
    reduce_only=False,  # <-- 可选（默认False）
    display_qty=None,  # <-- 可选（默认None，表示完全显示）
    tags=None,  # <-- 可选（默认None）
)
```

:::info
有关更多详细信息，请参阅`LimitOrder` [API参考](../api_reference/model/orders_CN.md#class-limitorder)。
:::

### 止损市价订单

*止损市价*订单是一个条件订单，一旦触发，将立即下达一个*市价*订单。此订单类型通常用作止损以限制损失，要么作为针对多头头寸的卖出订单，要么作为针对空头头寸的买入订单。

在以下示例中，我们在Binance现货/保证金交易所创建一个*止损市价*订单，以触发价格100,000 USDT卖出1个BTC，直到另行通知：

```python
from nautilus_trader.model.enums import OrderSide
from nautilus_trader.model.enums import TimeInForce
from nautilus_trader.model.enums import TriggerType
from nautilus_trader.model import InstrumentId
from nautilus_trader.model import Price
from nautilus_trader.model import Quantity
from nautilus_trader.model.orders import StopMarketOrder

order: StopMarketOrder = self.order_factory.stop_market(
    instrument_id=InstrumentId.from_str("BTCUSDT.BINANCE"),
    order_side=OrderSide.SELL,
    quantity=Quantity.from_int(1),
    trigger_price=Price.from_int(100_000),
    trigger_type=TriggerType.LAST_PRICE,  # <-- 可选（默认DEFAULT）
    time_in_force=TimeInForce.GTC,  # <-- 可选（默认GTC）
    expire_time=None,  # <-- 可选（默认None）
    reduce_only=False,  # <-- 可选（默认False）
    tags=None,  # <-- 可选（默认None）
)
```

:::info
有关更多详细信息，请参阅`StopMarketOrder` [API参考](../api_reference/model/orders_CN.md#class-stopmarketorder)。
:::

### 止损限价订单

*止损限价*订单是一个条件订单，一旦触发将立即以指定价格下达一个*限价*订单。

在以下示例中，我们在Currenex FX ECN创建一个*止损限价*订单，一旦市场达到触发价格1.30010 USD，以限价1.3000 USD购买50,000 GBP，有效期至2022年6月6日中午（UTC）：

```python
import pandas as pd
from nautilus_trader.model.enums import OrderSide
from nautilus_trader.model.enums import TimeInForce
from nautilus_trader.model.enums import TriggerType
from nautilus_trader.model import InstrumentId
from nautilus_trader.model import Price
from nautilus_trader.model import Quantity
from nautilus_trader.model.orders import StopLimitOrder

order: StopLimitOrder = self.order_factory.stop_limit(
    instrument_id=InstrumentId.from_str("GBP/USD.CURRENEX"),
    order_side=OrderSide.BUY,
    quantity=Quantity.from_int(50_000),
    price=Price.from_str("1.30000"),
    trigger_price=Price.from_str("1.30010"),
    trigger_type=TriggerType.BID_ASK,  # <-- 可选（默认DEFAULT）
    time_in_force=TimeInForce.GTD,  # <-- 可选（默认GTC）
    expire_time=pd.Timestamp("2022-06-06T12:00"),
    post_only=True,  # <-- 可选（默认False）
    reduce_only=False,  # <-- 可选（默认False）
    tags=None,  # <-- 可选（默认None）
)
```

:::info
有关更多详细信息，请参阅`StopLimitOrder` [API参考](../api_reference/model/orders_CN.md#class-stoplimitorder)。
:::

### 市价转限价订单

*市价转限价*订单作为市价订单提交，以当前最佳市场价格执行。
如果订单只是部分成交，订单的剩余部分被取消并重新提交为*限价*订单，限价等于订单成交部分的执行价格。

在以下示例中，我们在Interactive Brokers [IdealPro](https://ibkr.info/node/1708) 外汇ECN上创建一个*市价转限价*订单，使用JPY购买200,000 USD：

```python
from nautilus_trader.model.enums import OrderSide
from nautilus_trader.model.enums import TimeInForce
from nautilus_trader.model import InstrumentId
from nautilus_trader.model import Quantity
from nautilus_trader.model.orders import MarketToLimitOrder

order: MarketToLimitOrder = self.order_factory.market_to_limit(
    instrument_id=InstrumentId.from_str("USD/JPY.IDEALPRO"),
    order_side=OrderSide.BUY,
    quantity=Quantity.from_int(200_000),
    time_in_force=TimeInForce.GTC,  # <-- 可选（默认GTC）
    reduce_only=False,  # <-- 可选（默认False）
    display_qty=None,  # <-- 可选（默认None表示完全显示）
    tags=None,  # <-- 可选（默认None）
)
```

:::info
有关更多详细信息，请参阅`MarketToLimitOrder` [API参考](../api_reference/model/orders_CN.md#class-markettolimitorder)。
:::

### 触及市价订单

*触及市价*订单是一种条件订单，一旦触发将立即下达*市价*订单。这种订单类型通常用于在止损价格上进入新头寸，或为现有头寸获利，要么作为针对多头头寸的卖出订单，要么作为针对空头头寸的买入订单。

在以下示例中，我们在币安期货交易所创建一个*触及市价*订单，以触发价格10,000 USDT卖出10个ETHUSDT-PERP永续期货合约，有效期直到另行通知：

```python
from nautilus_trader.model.enums import OrderSide
from nautilus_trader.model.enums import TimeInForce
from nautilus_trader.model.enums import TriggerType
from nautilus_trader.model import InstrumentId
from nautilus_trader.model import Price
from nautilus_trader.model import Quantity
from nautilus_trader.model.orders import MarketIfTouchedOrder

order: MarketIfTouchedOrder = self.order_factory.market_if_touched(
    instrument_id=InstrumentId.from_str("ETHUSDT-PERP.BINANCE"),
    order_side=OrderSide.SELL,
    quantity=Quantity.from_int(10),
    trigger_price=Price.from_str("10_000.00"),
    trigger_type=TriggerType.LAST_PRICE,  # <-- 可选（默认DEFAULT）
    time_in_force=TimeInForce.GTC,  # <-- 可选（默认GTC）
    expire_time=None,  # <-- 可选（默认None）
    reduce_only=False,  # <-- 可选（默认False）
    tags=["ENTRY"],  # <-- 可选（默认None）
)
```

:::info
有关更多详细信息，请参阅`MarketIfTouchedOrder` [API参考](../api_reference/model/orders_CN.md#class-marketiftouchedorder)。
:::

### 触及限价订单

*触及限价*订单是一种条件订单，一旦触发将立即以指定价格下达*限价*订单。

在以下示例中，我们在币安期货交易所创建一个*触及限价*订单，以限价30,100 USDT购买5个BTCUSDT-PERP永续期货合约（一旦市场达到触发价格30,150 USDT），有效期至2022年6月6日中午（UTC）：

```python
import pandas as pd
from nautilus_trader.model.enums import OrderSide
from nautilus_trader.model.enums import TimeInForce
from nautilus_trader.model.enums import TriggerType
from nautilus_trader.model import InstrumentId
from nautilus_trader.model import Price
from nautilus_trader.model import Quantity
from nautilus_trader.model.orders import LimitIfTouchedOrder

order: LimitIfTouchedOrder = self.order_factory.limit_if_touched(
    instrument_id=InstrumentId.from_str("BTCUSDT-PERP.BINANCE"),
    order_side=OrderSide.BUY,
    quantity=Quantity.from_int(5),
    price=Price.from_str("30_100"),
    trigger_price=Price.from_str("30_150"),
    trigger_type=TriggerType.LAST_PRICE,  # <-- 可选（默认DEFAULT）
    time_in_force=TimeInForce.GTD,  # <-- 可选（默认GTC）
    expire_time=pd.Timestamp("2022-06-06T12:00"),
    post_only=True,  # <-- 可选（默认False）
    reduce_only=False,  # <-- 可选（默认False）
    tags=["TAKE_PROFIT"],  # <-- 可选（默认None）
)
```

:::info
有关更多详细信息，请参阅`LimitIfTouched` [API参考](../api_reference/model/orders_CN.md#class-limitiftouchedorder-1)。
:::

### 跟踪止损市价订单

*跟踪止损市价*订单是一种条件订单，它根据定义的市场价格固定偏移量跟踪止损触发价格。一旦触发，将立即下达*市价*订单。

在以下示例中，我们在币安期货交易所创建一个*跟踪止损市价*订单，卖出10个ETHUSD-PERP COIN_M保证金永续期货合约，在5,000 USD价格激活，然后以1%（基点）的偏移量跟踪当前最后成交价：

```python
import pandas as pd
from decimal import Decimal
from nautilus_trader.model.enums import OrderSide
from nautilus_trader.model.enums import TimeInForce
from nautilus_trader.model.enums import TriggerType
from nautilus_trader.model.enums import TrailingOffsetType
from nautilus_trader.model import InstrumentId
from nautilus_trader.model import Price
from nautilus_trader.model import Quantity
from nautilus_trader.model.orders import TrailingStopMarketOrder

order: TrailingStopMarketOrder = self.order_factory.trailing_stop_market(
    instrument_id=InstrumentId.from_str("ETHUSD-PERP.BINANCE"),
    order_side=OrderSide.SELL,
    quantity=Quantity.from_int(10),
    activation_price=Price.from_str("5_000"),
    trigger_type=TriggerType.LAST_PRICE,  # <-- 可选（默认DEFAULT）
    trailing_offset=Decimal(100),
    trailing_offset_type=TrailingOffsetType.BASIS_POINTS,
    time_in_force=TimeInForce.GTC,  # <-- 可选（默认GTC）
    expire_time=None,  # <-- 可选（默认None）
    reduce_only=True,  # <-- 可选（默认False）
    tags=["TRAILING_STOP-1"],  # <-- 可选（默认None）
)
```

:::info
有关更多详细信息，请参阅`TrailingStopMarketOrder` [API参考](../api_reference/model/orders_CN.md#class-trailingstopmarketorder-1)。
:::

### 跟踪止损限价订单

*跟踪止损限价*订单是一种条件订单，它根据定义的市场价格固定偏移量跟踪止损触发价格。一旦触发，将立即以定义的价格下达*限价*订单（该价格也会随着市场移动而更新，直到触发）。

在以下示例中，我们在Currenex外汇ECN创建一个*跟踪止损限价*订单，使用USD购买1,250,000 AUD，限价0.71000 USD，在0.72000 USD激活，然后以0.00100 USD的止损偏移量跟踪当前卖价，有效期直到另行通知：

```python
import pandas as pd
from decimal import Decimal
from nautilus_trader.model.enums import OrderSide
from nautilus_trader.model.enums import TimeInForce
from nautilus_trader.model.enums import TriggerType
from nautilus_trader.model.enums import TrailingOffsetType
from nautilus_trader.model import InstrumentId
from nautilus_trader.model import Price
from nautilus_trader.model import Quantity
from nautilus_trader.model.orders import TrailingStopLimitOrder

order: TrailingStopLimitOrder = self.order_factory.trailing_stop_limit(
    instrument_id=InstrumentId.from_str("AUD/USD.CURRENEX"),
    order_side=OrderSide.BUY,
    quantity=Quantity.from_int(1_250_000),
    price=Price.from_str("0.71000"),
    activation_price=Price.from_str("0.72000"),
    trigger_type=TriggerType.BID_ASK,  # <-- 可选（默认DEFAULT）
    limit_offset=Decimal("0.00050"),
    trailing_offset=Decimal("0.00100"),
    trailing_offset_type=TrailingOffsetType.PRICE,
    time_in_force=TimeInForce.GTC,  # <-- 可选（默认GTC）
    expire_time=None,  # <-- 可选（默认None）
    reduce_only=True,  # <-- 可选（默认False）
    tags=["TRAILING_STOP"],  # <-- 可选（默认None）
)
```

:::info
有关更多详细信息，请参阅`TrailingStopLimitOrder` [API参考](../api_reference/model/orders_CN.md#class-trailingstoplimitorder-1)。
:::

## 高级订单

以下指南应与涉及这些订单类型、列表/组合和执行指令的经纪商或场所的特定文档一起阅读（例如Interactive Brokers）。

### 订单列表

条件订单的组合或更大的订单批量可以通过共同的`order_list_id`组合到一个列表中。此列表中包含的订单可能有也可能没有彼此的条件关系，这取决于订单本身的构造方式以及它们被路由到的特定场所。

### 应急类型

- **OTO（一个触发其他）** – 一个父订单，一旦执行，自动下达一个或多个子订单。
  - *完全触发模型*：子订单**仅在父订单完全成交后**发布。在大多数零售股票/期权经纪商（如Schwab、Fidelity、TD Ameritrade）和许多现货加密货币场所（Binance、Coinbase）很常见。
  - *部分触发模型*：子订单**按每次部分成交的比例**发布。专业级平台如Interactive Brokers、大多数期货/外汇OMS和Kraken Pro使用此模型。

- **OCO（一个取消其他）** – 两个（或更多）链接的活跃订单，其中执行一个取消其余的。

- **OUO（一个更新其他）** – 两个（或更多）链接的活跃订单，其中执行一个减少其余订单的开放数量。

:::info
这些应急类型与ContingencyType FIX标签<1385> <https://www.onixs.biz/fix-dictionary/5.0.sp2/tagnum_1385.html>相关。
:::

#### 一个触发其他（OTO）

OTO订单涉及两个部分：

1. **父订单** – 立即提交到匹配引擎。
2. **子订单** – 保持*账外*，直到满足触发条件。

##### 触发模型

| 触发模型       | 何时发布子订单？                                                                                                                  |
|---------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| **完全触发**    | 当父订单的累积数量等于其原始数量时（即*完全*成交）。                                           |
| **部分触发** | 在父订单的每次部分执行时立即；子订单的数量匹配执行金额，并随着进一步成交而增加。 |

:::info
NautilusTrader的默认回测场所为OTO订单使用*部分触发模型*。
未来的更新将添加配置以选择*完全触发模型*。
:::

> **为什么区别很重要**
> *完全触发*留下风险窗口：任何部分成交的头寸在剩余数量成交之前都是活跃的，没有保护性退出。
> *部分触发*通过确保每个执行的手数立即拥有其链接的止损/限价来降低该风险，代价是创建更多的订单流量和更新。

OTO订单可以使用场所上任何支持的资产类型（例如股票入场与期权对冲、期货入场与OCO括号、加密现货入场与TP/SL）。

| 场所/适配器ID                           | 资产类别             | 子订单触发规则                      | 实际注意事项                                                   |
|----------------------------------------------|---------------------------|---------------------------------------------|-------------------------------------------------------------------|
| Binance / Binance Futures (`BINANCE`)        | 现货，永续期货   | **部分或完全** – 首次成交时触发。  | OTOCO/TP-SL子订单立即出现；监控保证金使用。      |
| Bybit Spot (`BYBIT`)                         | 现货                      | **完全** – 完成后放置子订单。   | TP-SL预设仅在限价订单完全成交后激活。 |
| Bybit Perps (`BYBIT`)                        | 永续期货         | **部分和完全** – 可配置。        | "部分头寸"模式在成交到达时调整TP-SL大小。              |
| Kraken Futures (`KRAKEN`)                    | 期货和永续           | **部分和完全** – 自动。           | 子订单数量匹配每次部分执行。                   |
| OKX (`OKX`)                                  | 现货，期货，期权    | **完全** – 附加止损等待成交。    | 头寸级TP-SL可以单独添加。                     |
| Interactive Brokers (`INTERACTIVE_BROKERS`)  | 股票，期权，外汇，期货  | **可配置** – OCA可以按比例分配。        | `OcaType 2/3`减少剩余子订单数量。                 |
| Coinbase International (`COINBASE_INTX`)     | 现货和永续              | **完全** – 执行后添加括号。    | 入场加括号不同时；头寸活跃后添加。 |
| dYdX v4 (`DYDX`)                             | 永续期货（DEX）   | 链上条件（大小精确）。            | TP-SL由预言机价格触发；部分成交不适用。      |
| Polymarket (`POLYMARKET`)                    | 预测市场（DEX）   | N/A。                                        | 高级应急完全在策略层处理。      |
| Betfair (`BETFAIR`)                          | 体育博彩            | N/A。                                        | 高级应急完全在策略层处理。      |

#### 一个取消其他（OCO）

OCO订单是一组链接的订单，其中**任何**订单的执行（完全*或部分*）触发对其他订单的尽力而为取消。
两个订单同时活跃；一旦一个开始成交，场所尝试取消剩余订单的未执行部分。

#### 一个更新其他（OUO）

OUO订单是一组链接的订单，其中一个订单的执行导致其他订单的开放数量立即*减少*。
两个订单同时活跃，每次部分执行按比例更新其对等订单的剩余数量，尽力而为。

### 括号订单

括号订单是一种高级订单类型，允许交易者同时为头寸设置止盈和止损水平。这涉及下达父订单（入场订单）和两个子订单：止盈`LIMIT`订单和止损`STOP_MARKET`订单。当父订单执行时，子订单在市场中下达。如果市场向有利方向移动，止盈订单以盈利关闭头寸，而如果市场向不利方向移动，止损订单限制损失。

可以使用[OrderFactory](../api_reference/common_CN.md#class-orderfactory)轻松创建括号订单，它支持各种订单类型、参数和指令。

:::warning
您应该了解头寸的保证金要求，因为括号头寸将消耗更多的订单保证金。
:::

## 模拟订单

### 介绍

在深入技术细节之前，了解Nautilus Trader中模拟订单的基本目的很重要。其核心是，模拟允许您使用某些订单类型，即使您的交易场所本身不支持它们。

这通过让Nautilus在本地模拟这些订单类型的行为（如`STOP_LIMIT`或`TRAILING_STOP`订单）来工作，同时仅使用简单的`MARKET`和`LIMIT`订单在场所进行实际执行。

当您创建模拟订单时，Nautilus持续跟踪特定类型的市场价格（由`emulation_trigger`参数指定），并基于您设置的订单类型和条件，当满足触发条件时，将自动提交适当的基本订单（`MARKET` / `LIMIT`）。

例如，如果您创建模拟`STOP_LIMIT`订单，Nautilus将监控市场价格，直到达到您的`stop`价格，然后自动向场所提交`LIMIT`订单。

要执行模拟，Nautilus需要知道它应该监控哪种**市场价格类型**。
默认情况下，它使用买卖价格（报价），这就是为什么您经常在示例中看到`emulation_trigger=TriggerType.DEFAULT`（这等同于使用`TriggerType.BID_ASK`）。但是，Nautilus支持各种其他价格类型，
可以指导模拟过程。

### 提交订单进行模拟

模拟订单的唯一要求是向`Order`构造函数或`OrderFactory`创建方法的`emulation_trigger`参数传递`TriggerType`。目前支持以下模拟触发类型：

- `NO_TRIGGER`：完全禁用本地模拟，订单完全提交到场所。
- `DEFAULT`：与`BID_ASK`相同。
- `BID_ASK`：使用报价触发模拟。
- `LAST`：使用交易触发模拟。

触发类型的选择决定了订单模拟的行为方式：

- 对于`STOP`订单，订单的触发价格将与指定的触发类型进行比较。
- 对于`TRAILING_STOP`订单，跟踪偏移量将基于指定的触发类型更新。
- 对于`LIMIT`订单，订单的限价将与指定的触发类型进行比较。

以下是您可以设置到`emulation_trigger`参数的所有可用值及其用途：

| 触发类型      | 描述                                                                                          | 常见用例                                                                                             |
|:------------------|:-----------------------------------------------------------------------------------------------------|:-------------------------------------------------------------------------------------------------------------|
| `NO_TRIGGER`      | 完全禁用模拟。订单直接发送到场所，无任何本地处理。 | 当您想使用场所的原生订单处理，或对于不需要模拟的简单订单类型时。 |
| `DEFAULT`         | 与`BID_ASK`相同。这是大多数模拟订单的标准选择。                             | 当您想使用"默认"类型的市场价格时的通用模拟。                    |
| `BID_ASK`         | 使用最佳买卖价格（报价）指导模拟。                                        | 止损订单、跟踪止损和其他应该对当前市场价差作出反应的订单。                |
| `LAST_PRICE`      | 使用最近交易的价格指导模拟。                                          | 应该基于实际执行的交易而不是报价触发的订单。                               |
| `DOUBLE_LAST`     | 使用两个连续的最后交易价格确认触发条件。                             | 当您希望在触发前对价格移动进行额外确认时。                                   |
| `DOUBLE_BID_ASK`  | 使用两个连续的买卖价格更新确认触发条件。                         | 当您希望在触发前对报价移动进行额外确认时。                                       |
| `LAST_OR_BID_ASK` | 在最后交易价格或买卖价格上触发。                                               | 当您希望对任何类型的价格移动更加敏感时。                                           |
| `MID_POINT`       | 使用最佳买卖价格之间的中点。                                           | 应该基于理论公允价格触发的订单。                                              |
| `MARK_PRICE`      | 使用标记价格（在衍生品市场中常见）触发。                                  | 特别适用于期货和永续合约。                                                     |
| `INDEX_PRICE`     | 使用基础指数价格触发。                                                       | 当交易跟踪指数的衍生品时。                                                                |

### 技术实现

平台使得可以在本地模拟大多数订单类型，无论交易场所是否支持该类型。订单模拟的逻辑和代码路径对于所有[环境上下文](/concepts/architecture_CN.md#environment-contexts)都是相同的，并利用通用的`OrderEmulator`组件。

:::note
每个运行实例的模拟订单数量没有限制。
:::

### 生命周期

模拟订单将经历以下阶段：

1. 由`Strategy`通过`submit_order`方法提交。
2. 发送到`RiskEngine`进行交易前风险检查（在此时可能被拒绝）。
3. 发送到`OrderEmulator`，在那里被*持有*/模拟。
4. 一旦触发，模拟订单被转换为`MARKET`或`LIMIT`订单并发布（提交到场所）。
5. 发布的订单在场所提交前进行最终风险检查。

:::note
模拟订单受到与*常规*订单相同的风险控制，可以由交易策略以正常方式修改和取消。它们在取消所有订单时也会被包括在内。
:::

:::info
模拟订单在其整个生命周期中将保留其原始客户端订单ID，使其易于通过缓存查询。
:::

#### 持有的模拟订单

对于现在由`OrderEmulator`组件*持有*的模拟订单，将发生以下情况：

- 原始`SubmitOrder`命令将被缓存。
- 模拟订单将在本地`MatchingCore`组件内处理。
- `OrderEmulator`将订阅任何需要的市场数据（如果尚未订阅）以更新匹配核心。
- 模拟订单可以被修改（由交易者）和更新（由市场），直到*发布*或取消。

#### 发布的模拟订单

一旦模拟订单基于数据到达在本地触发/匹配，将发生以下*发布*操作：

- 订单将通过额外的`OrderInitialized`事件转换为`MARKET`或`LIMIT`订单（见下表）。
- 订单的`emulation_trigger`将设置为`NONE`（任何组件将不再将其视为模拟订单）。
- 附加到原始`SubmitOrder`命令的订单将发送回`RiskEngine`进行额外检查，因为有任何修改/更新。
- 如果未被拒绝，则命令将继续到`ExecutionEngine`，并通过`ExecutionClient`正常发送到交易场所。

下表列出了哪些订单类型可以模拟，以及它们在发布提交到交易场所时转换为哪种订单类型。

### 可以模拟的订单类型

下表列出了哪些订单类型可以模拟，以及它们在发布提交到交易场所时转换为哪种订单类型。

| 模拟订单类型 | 可以模拟 | 发布类型 |
|:-------------------------|:------------|:--------------|
| `MARKET`                 |             | n/a           |
| `MARKET_TO_LIMIT`        |             | n/a           |
| `LIMIT`                  | ✓           | `MARKET`      |
| `STOP_MARKET`            | ✓           | `MARKET`      |
| `STOP_LIMIT`             | ✓           | `LIMIT`       |
| `MARKET_IF_TOUCHED`      | ✓           | `MARKET`      |
| `LIMIT_IF_TOUCHED`       | ✓           | `LIMIT`       |
| `TRAILING_STOP_MARKET`   | ✓           | `MARKET`      |
| `TRAILING_STOP_LIMIT`    | ✓           | `LIMIT`       |

### 查询

在编写交易策略时，可能需要了解系统中模拟订单的状态。有几种查询模拟状态的方法：

#### 通过缓存

以下`Cache`方法可用：

- `self.cache.orders_emulated(...)`：返回所有当前模拟的订单。
- `self.cache.is_order_emulated(...)`：检查特定订单是否被模拟。
- `self.cache.orders_emulated_count(...)`：返回模拟订单的计数。

有关其他详细信息，请参阅完整的[API参考](../../api_reference/cache_CN)。

#### 直接订单查询

您可以直接使用以下方式查询订单对象：

- `order.is_emulated`

如果其中任何一个返回`False`，则订单已从`OrderEmulator`*发布*，因此不再被视为模拟订单（或从未是模拟订单）。

:::warning
不建议持有对模拟订单的本地引用，因为当/如果模拟订单被*发布*时，订单对象将被转换。您应该依赖为此工作而设计的`Cache`。
:::

### 持久化和恢复

如果运行的系统崩溃或关闭时有活跃的模拟订单，那么它们将从任何配置的缓存数据库中重新加载到`OrderEmulator`中。这确保了跨系统重启和恢复的订单状态持久性。

### 最佳实践

在使用模拟订单时，请考虑以下最佳实践：

1. 始终使用`Cache`查询或跟踪模拟订单，而不是存储本地引用
2. 注意模拟订单在发布时转换为不同类型
3. 记住模拟订单在提交和发布时都要进行风险检查

:::note
订单模拟允许您使用高级订单类型，即使在本地不支持它们的场所上，使您的交易策略在不同场所之间更具可移植性。
:::
