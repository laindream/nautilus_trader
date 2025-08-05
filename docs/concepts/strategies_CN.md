# 策略

NautilusTrader用户体验的核心在于编写和使用交易策略。定义策略涉及继承`Strategy`类并实现策略逻辑所需的方法。

**关键功能**：

- 所有`Actor`功能
- 订单管理

**与角色的关系**：
`Strategy`类继承自`Actor`，这意味着策略可以访问所有角色功能以及订单管理功能。

:::tip
我们建议在深入策略开发之前先查看[角色](actors_CN.md)指南。
:::

策略可以在任何[环境上下文](/concepts/architecture_CN.md#environment-contexts)中添加到Nautilus系统中，并且一旦系统启动，将开始根据其逻辑发送命令和接收事件。

使用数据摄取、事件处理和订单管理的基本构建块（我们将在下面讨论），可以实现任何类型的策略，包括方向性、动量、重新平衡、配对、做市等。

:::info
请参阅`Strategy` [API参考](../api_reference/trading_CN.md)以获取所有可用方法的完整描述。
:::

Nautilus交易策略有两个主要部分：

- 策略实现本身，通过继承`Strategy`类定义
- *可选的*策略配置，通过继承`StrategyConfig`类定义

:::tip
一旦定义了策略，相同的源代码可以用于回测和实时交易。
:::

策略的主要功能包括：

- 历史数据请求
- 实时数据订阅
- 设置时间警报或定时器
- 缓存访问
- 投资组合访问
- 创建和管理订单和头寸

## 策略实现

由于交易策略是一个继承自`Strategy`的类，您必须定义一个可以处理初始化的构造函数。至少需要初始化基类/超类：

```python
from nautilus_trader.trading.strategy import Strategy

class MyStrategy(Strategy):
    def __init__(self) -> None:
        super().__init__()  # <-- 必须调用超类来初始化策略
```

从这里，您可以根据需要实现处理程序，以基于状态转换和事件执行操作。

:::warning
不要在`__init__`构造函数中调用`clock`和`logger`等组件（这在注册之前）。
这是因为系统时钟和日志子系统尚未初始化。
:::

### 处理程序

处理程序是`Strategy`类中的方法，可以基于不同类型的事件或状态更改执行操作。
这些方法使用前缀`on_*`命名。您可以根据策略的具体目标和需求选择实现任何或所有这些处理程序方法。

为类似类型的事件设置多个处理程序的目的是提供处理粒度的灵活性。
这意味着您可以选择使用专用处理程序响应特定事件，或使用更通用的处理程序响应一系列相关事件（使用典型的switch语句逻辑）。
处理程序按从最具体到最通用的顺序依次调用。

#### 状态操作

这些处理程序由`Strategy`的生命周期状态更改触发。建议：

- 使用`on_start`方法初始化您的策略（例如，获取工具、订阅数据）
- 使用`on_stop`方法进行清理任务（例如，取消未结订单、关闭未结头寸、取消订阅数据）

```python
def on_start(self) -> None:
def on_stop(self) -> None:
def on_resume(self) -> None:
def on_reset(self) -> None:
def on_dispose(self) -> None:
def on_degrade(self) -> None:
def on_fault(self) -> None:
def on_save(self) -> dict[str, bytes]:  # 返回要保存的用户定义状态字典
def on_load(self, state: dict[str, bytes]) -> None:
```

#### 数据处理

这些处理程序接收数据更新，包括内置市场数据和自定义用户定义数据。
您可以使用这些处理程序定义接收数据对象实例时的操作。

```python
from nautilus_trader.core import Data
from nautilus_trader.model import OrderBook
from nautilus_trader.model import Bar
from nautilus_trader.model import QuoteTick
from nautilus_trader.model import TradeTick
from nautilus_trader.model import OrderBookDeltas
from nautilus_trader.model import InstrumentClose
from nautilus_trader.model import InstrumentStatus
from nautilus_trader.model.instruments import Instrument

def on_order_book_deltas(self, deltas: OrderBookDeltas) -> None:
def on_order_book(self, order_book: OrderBook) -> None:
def on_quote_tick(self, tick: QuoteTick) -> None:
def on_trade_tick(self, tick: TradeTick) -> None:
def on_bar(self, bar: Bar) -> None:
def on_instrument(self, instrument: Instrument) -> None:
def on_instrument_status(self, data: InstrumentStatus) -> None:
def on_instrument_close(self, data: InstrumentClose) -> None:
def on_historical_data(self, data: Data) -> None:
def on_data(self, data: Data) -> None:  # 自定义数据传递给此处理程序
def on_signal(self, signal: Data) -> None:  # 自定义信号传递给此处理程序
```

#### 订单管理

这些处理程序接收与订单相关的事件。
`OrderEvent`类型消息按以下顺序传递给处理程序：

1. 特定处理程序（例如，`on_order_accepted`、`on_order_rejected`等）
2. `on_order_event(...)`
3. `on_event(...)`

```python
from nautilus_trader.model.events import OrderAccepted
from nautilus_trader.model.events import OrderCanceled
from nautilus_trader.model.events import OrderCancelRejected
from nautilus_trader.model.events import OrderDenied
from nautilus_trader.model.events import OrderEmulated
from nautilus_trader.model.events import OrderEvent
from nautilus_trader.model.events import OrderExpired
from nautilus_trader.model.events import OrderFilled
from nautilus_trader.model.events import OrderInitialized
from nautilus_trader.model.events import OrderModifyRejected
from nautilus_trader.model.events import OrderPendingCancel
from nautilus_trader.model.events import OrderPendingUpdate
from nautilus_trader.model.events import OrderRejected
from nautilus_trader.model.events import OrderReleased
from nautilus_trader.model.events import OrderSubmitted
from nautilus_trader.model.events import OrderTriggered
from nautilus_trader.model.events import OrderUpdated

def on_order_initialized(self, event: OrderInitialized) -> None:
def on_order_denied(self, event: OrderDenied) -> None:
def on_order_emulated(self, event: OrderEmulated) -> None:
def on_order_released(self, event: OrderReleased) -> None:
def on_order_submitted(self, event: OrderSubmitted) -> None:
def on_order_rejected(self, event: OrderRejected) -> None:
def on_order_accepted(self, event: OrderAccepted) -> None:
def on_order_canceled(self, event: OrderCanceled) -> None:
def on_order_expired(self, event: OrderExpired) -> None:
def on_order_triggered(self, event: OrderTriggered) -> None:
def on_order_pending_update(self, event: OrderPendingUpdate) -> None:
def on_order_pending_cancel(self, event: OrderPendingCancel) -> None:
def on_order_modify_rejected(self, event: OrderModifyRejected) -> None:
def on_order_cancel_rejected(self, event: OrderCancelRejected) -> None:
def on_order_updated(self, event: OrderUpdated) -> None:
def on_order_filled(self, event: OrderFilled) -> None:
def on_order_event(self, event: OrderEvent) -> None:  # 所有订单事件消息最终都传递给此处理程序
```

#### 头寸管理

这些处理程序接收与头寸相关的事件。
`PositionEvent`类型消息按以下顺序传递给处理程序：

1. 特定处理程序（例如，`on_position_opened`、`on_position_changed`等）
2. `on_position_event(...)`
3. `on_event(...)`

```python
from nautilus_trader.model.events import PositionChanged
from nautilus_trader.model.events import PositionClosed
from nautilus_trader.model.events import PositionEvent
from nautilus_trader.model.events import PositionOpened

def on_position_opened(self, event: PositionOpened) -> None:
def on_position_changed(self, event: PositionChanged) -> None:
def on_position_closed(self, event: PositionClosed) -> None:
def on_position_event(self, event: PositionEvent) -> None:  # 所有头寸事件消息最终都传递给此处理程序
```

#### 通用事件处理

此处理程序最终将接收到达策略的所有事件消息，包括那些不存在其他特定处理程序的事件。

```python
from nautilus_trader.core.message import Event

def on_event(self, event: Event) -> None:
```

#### 处理程序示例

以下示例显示了典型的`on_start`处理程序方法实现（取自EMA交叉策略示例）。
在这里我们可以看到以下内容：

- 注册指标以接收K线更新
- 请求历史数据（以补充指标）
- 订阅实时数据

```python
def on_start(self) -> None:
    """
    策略启动时要执行的操作。
    """
    self.instrument = self.cache.instrument(self.instrument_id)
    if self.instrument is None:
        self.log.error(f"Could not find instrument for {self.instrument_id}")
        self.stop()
        return

    # 注册指标以进行更新
    self.register_indicator_for_bars(self.bar_type, self.fast_ema)
    self.register_indicator_for_bars(self.bar_type, self.slow_ema)

    # 获取历史数据
    self.request_bars(self.bar_type)

    # 订阅实时数据
    self.subscribe_bars(self.bar_type)
    self.subscribe_quote_ticks(self.instrument_id)
```

### 时钟和定时器

策略可以访问`Clock`，它提供了多种方法来创建不同的时间戳，以及设置时间警报或定时器来触发`TimeEvent`。

:::info
请参阅`Clock` [API参考](../api_reference/common_CN.md)以获取可用方法的完整列表。
:::

#### 当前时间戳

虽然有多种方法可以获取当前时间戳，这里是两种常用方法的示例：

要获取当前UTC时间戳作为tz-aware的`pd.Timestamp`：

```python
import pandas as pd


now: pd.Timestamp = self.clock.utc_now()
```

要获取当前UTC时间戳作为自UNIX纪元以来的纳秒数：

```python
unix_nanos: int = self.clock.timestamp_ns()
```

#### 时间警报

可以设置时间警报，这将导致在指定的警报时间将`TimeEvent`分派给`on_event`处理程序。在实时上下文中，这可能会延迟几微秒。

此示例设置一个时间警报，从当前时间起一分钟后触发：

```python
import pandas as pd

# 从现在起一分钟后触发TimeEvent
self.clock.set_time_alert(
    name="MyTimeAlert1",
    alert_time=self.clock.utc_now() + pd.Timedelta(minutes=1),
)
```

#### 定时器

可以设置连续定时器，它将定期生成`TimeEvent`，直到定时器到期或被取消。

此示例设置一个定时器，每分钟触发一次，立即开始：

```python
import pandas as pd

# 每分钟触发一次TimeEvent
self.clock.set_timer(
    name="MyTimer1",
    interval=pd.Timedelta(minutes=1),
)
```

### 缓存访问

可以访问交易者实例的中央`Cache`来获取数据和执行对象（订单、头寸等）。
有许多可用的方法，通常具有过滤功能，这里我们介绍一些基本用例。

#### 获取数据

以下示例显示如何从缓存中获取数据（假设分配了某个工具ID属性）：

```python
last_quote = self.cache.quote_tick(self.instrument_id)
last_trade = self.cache.trade_tick(self.instrument_id)
last_bar = self.cache.bar(bar_type)
```

#### 获取执行对象

以下示例显示如何从缓存中获取单个订单和头寸对象：

```python
order = self.cache.order(client_order_id)
position = self.cache.position(position_id)

```

:::info
请参阅`Cache` [API参考](../api_reference/cache_CN.md)以获取所有可用方法的完整描述。
:::

### 投资组合访问

可以访问交易者的中央`Portfolio`来获取账户和头寸信息。
以下显示了可用方法的一般概述。

#### 账户和头寸信息

```python
import decimal

from nautilus_trader.accounting.accounts.base import Account
from nautilus_trader.model import Venue
from nautilus_trader.model import Currency
from nautilus_trader.model import Money
from nautilus_trader.model import InstrumentId

def account(self, venue: Venue) -> Account
``` 