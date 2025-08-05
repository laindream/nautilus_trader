# 角色

:::info
我们目前正在编写这个概念指南。
:::

`Actor`作为与交易系统交互的基础组件。
它提供接收市场数据、处理事件和在交易环境中管理状态的核心功能。`Strategy`类继承自Actor并通过订单管理方法扩展其功能。

**关键功能**：

- 事件订阅和处理
- 市场数据接收
- 状态管理
- 系统交互原语

## 基本示例

就像策略一样，角色通过非常相似的模式支持配置。

```python
from nautilus_trader.config import ActorConfig
from nautilus_trader.model import InstrumentId
from nautilus_trader.model import Bar, BarType
from nautilus_trader.common.actor import Actor


class MyActorConfig(ActorConfig):
    instrument_id: InstrumentId   # 示例值："ETHUSDT-PERP.BINANCE"
    bar_type: BarType             # 示例值："ETHUSDT-PERP.BINANCE-15-MINUTE[LAST]-INTERNAL"
    lookback_period: int = 10


class MyActor(Actor):
    def __init__(self, config: MyActorConfig) -> None:
        super().__init__(config)

        # 自定义状态变量
        self.count_of_processed_bars: int = 0

    def on_start(self) -> None:
        # 订阅所有传入的K线
        self.subscribe_bars(self.config.bar_type)   # 您可以通过`self.config`直接访问配置

    def on_bar(self, bar: Bar):
        self.count_of_processed_bars += 1
```

## 数据处理和回调

在Nautilus中处理数据时，理解数据*请求/订阅*与其相应回调处理程序之间的关系很重要。系统根据数据是历史数据还是实时数据使用不同的处理程序。

### 历史vs实时数据

系统区分两种类型的数据流：

1. **历史数据**（来自*请求*）：
   - 通过`request_bars()`、`request_quote_ticks()`等方法获得。
   - 通过`on_historical_data()`处理程序处理。
   - 用于初始数据加载和历史分析。

2. **实时数据**（来自*订阅*）：
   - 通过`subscribe_bars()`、`subscribe_quote_ticks()`等方法获得。
   - 通过特定处理程序如`on_bar()`、`on_quote_tick()`等处理。
   - 用于实时数据处理。

### 回调处理程序

以下是不同数据操作如何映射到其处理程序：

| 操作                       | 类别         | 处理程序                  | 目的 |
|:--------------------------------|:-----------------|:-------------------------|:--------|
| `subscribe_data()`              | 实时&nbsp;  | `on_data()`              | 实时数据更新 |
| `subscribe_instrument()`        | 实时&nbsp;  | `on_instrument()`        | 实时工具定义更新 |
| `subscribe_instruments()`       | 实时&nbsp;  | `on_instrument()`        | 实时工具定义更新（对于场所） |
| `subscribe_order_book_deltas()` | 实时&nbsp;  | `on_order_book_deltas()` | 实时订单簿更新 |
| `subscribe_quote_ticks()`       | 实时&nbsp;  | `on_quote_tick()`        | 实时报价更新 |
| `subscribe_trade_ticks()`       | 实时&nbsp;  | `on_trade_tick()`        | 实时交易更新 |
| `subscribe_mark_prices()`       | 实时&nbsp;  | `on_mark_price()`        | 实时标记价格更新 |
| `subscribe_index_prices()`      | 实时&nbsp;  | `on_index_price()`       | 实时指数价格更新 |
| `subscribe_bars()`              | 实时&nbsp;  | `on_bar()`               | 实时K线更新 |
| `subscribe_instrument_status()` | 实时&nbsp;  | `on_instrument_status()` | 实时工具状态更新 |
| `subscribe_instrument_close()`  | 实时&nbsp;  | `on_instrument_close()`  | 实时工具收盘更新 |
| `request_data()`                | 历史       | `on_historical_data()`   | 历史数据处理 |
| `request_instrument()`          | 历史       | `on_instrument()`        | 工具定义更新 |
| `request_instruments()`         | 历史       | `on_instrument()`        | 工具定义更新 |
| `request_quote_ticks()`         | 历史       | `on_historical_data()`   | 历史报价处理 |
| `request_trade_ticks()`         | 历史       | `on_historical_data()`   | 历史交易处理 |
| `request_bars()`                | 历史       | `on_historical_data()`   | 历史K线处理 |
| `request_aggregated_bars()`     | 历史       | `on_historical_data()`   | 历史聚合K线（实时生成） |

### 示例

以下是演示历史和实时数据处理的示例： 