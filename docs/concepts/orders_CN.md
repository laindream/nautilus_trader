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