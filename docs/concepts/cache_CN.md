# 缓存

`Cache`是一个中央内存数据库，自动存储和管理所有交易相关数据。将其视为您的交易系统的内存——存储从市场数据到订单历史再到自定义计算的所有内容。

Cache服务于多个关键目的：

1. **存储市场数据**：
   - 存储近期市场历史（例如，订单簿、报价、交易、K线）。
   - 为您的策略提供对当前和历史市场数据的访问。

2. **跟踪交易数据**：
   - 维护完整的`Order`历史和当前执行状态。
   - 跟踪所有`Position`和`Account`信息。
   - 存储`Instrument`定义和`Currency`信息。

3. **存储自定义数据**：
   - 任何用户定义的对象或数据都可以存储在`Cache`中供以后使用。
   - 实现不同策略之间的数据共享。

## Cache如何工作

**内置类型**：

- 数据在流经系统时自动添加到`Cache`中。
- 在实时环境中，更新异步发生——这意味着事件发生和它出现在`Cache`中之间可能有小的延迟。
- 所有数据在到达您的策略回调之前都流过`Cache`——请参见下面的图表：

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐     ┌───────────────────────┐
│                 │     │                 │     │                 │     │                       │
│                 │     │                 │     │                 │     │   Strategy callback:  │
│      Data       ├─────►   DataEngine    ├─────►     Cache       ├─────►                       │
│                 │     │                 │     │                 │     │   on_data(...)        │
│                 │     │                 │     │                 │     │                       │
└─────────────────┘     └─────────────────┘     └─────────────────┘     └───────────────────────┘
```

### 基本示例

在策略中，您可以通过`self.cache`访问`Cache`。这是一个典型示例：

:::note
您找到`self`的任何地方，它主要指的是`Strategy`本身。
:::

```python
def on_bar(self, bar: Bar) -> None:
    # 当前K线在参数'bar'中提供

    # 从Cache获取历史K线
    last_bar = self.cache.bar(self.bar_type, index=0)        # 最后一根K线（实际上与'bar'参数相同）
    previous_bar = self.cache.bar(self.bar_type, index=1)    # 前一根K线
    third_last_bar = self.cache.bar(self.bar_type, index=2)  # 第三根最后的K线

    # 获取当前头寸信息
    if self.last_position_opened_id is not None:
        position = self.cache.position(self.last_position_opened_id)
        if position.is_open:
            # 检查头寸详情
            current_pnl = position.unrealized_pnl

    # 获取我们工具的所有未结订单
    open_orders = self.cache.orders_open(instrument_id=self.instrument_id)
```

## 配置

`Cache`的行为和容量可以通过`CacheConfig`类配置。
您可以将此配置提供给`BacktestEngine`或`TradingNode`，具体取决于您的[环境上下文](architecture_CN.md#environment-contexts)。

这是配置`Cache`的基本示例：

```python
from nautilus_trader.config import CacheConfig, BacktestEngineConfig, TradingNodeConfig

# 用于回测
engine_config = BacktestEngineConfig(
    cache=CacheConfig(
        tick_capacity=10_000,  # 每个工具存储最后10,000个tick
        bar_capacity=5_000,    # 每个K线类型存储最后5,000根K线
    ),
)

# 用于实时交易
node_config = TradingNodeConfig(
    cache=CacheConfig(
        tick_capacity=10_000,
        bar_capacity=5_000,
    ),
)
```

:::tip
默认情况下，`Cache`为每种K线类型保留最后10,000根K线，为每个工具保留10,000个交易tick。
这些限制在内存使用和数据可用性之间提供了良好的平衡。如果您的策略需要更多历史数据，请增加它们。
:::

### 配置选项

`CacheConfig`类支持这些参数：

```python
from nautilus_trader.config import CacheConfig

cache_config = CacheConfig(
    database: DatabaseConfig | None = None,  # 持久化的数据库配置
    encoding: str = "msgpack",               # 数据编码格式（'msgpack'或'json'）
    timestamps_as_iso8601: bool = False,     # 将时间戳存储为ISO8601字符串
    buffer_interval_ms: int | None = None,   # 批量操作的缓冲间隔
    use_trader_prefix: bool = True,          # 在键中使用交易者前缀
    use_instance_id: bool = False,           # 在键中包含实例ID
    flush_on_start: bool = False,            # 启动时清除数据库
    drop_instruments_on_reset: bool = True,  # 重置时清除工具
    tick_capacity: int = 10_000,             # 每个工具存储的最大tick数
    bar_capacity: int = 10_000,              # 每种K线类型存储的最大K线数
)
```

:::note
每种K线类型维护其自己的单独容量。例如，如果您同时使用1分钟和5分钟K线，每种都将存储多达`bar_capacity`根K线。
当达到`bar_capacity`时，最旧的数据会自动从`Cache`中删除。
:::

### 数据库配置

为了在系统重启之间保持持久性，您可以配置数据库后端。

什么时候使用持久性有用？

- **长时间运行的系统**：如果您希望数据在系统重启、升级或意外故障中幸存，拥有数据库配置有助于从您离开的地方准确地继续。
- **历史洞察**：当您需要保留过去的交易数据用于详细的事后分析或审计时。
- **多节点或分布式设置**：如果多个服务或节点需要访问相同的状态，持久存储有助于确保共享和一致的数据。

```python
from nautilus_trader.config import DatabaseConfig

config = CacheConfig(
    database=DatabaseConfig(
        type="redis",      # 数据库类型
        host="localhost",  # 数据库主机
        port=6379,         # 数据库端口
        timeout=2,         # 连接超时（秒）
    ),
)
```

## 使用Cache

### 访问市场数据

`Cache`提供全面的接口来访问不同类型的市场数据，包括订单簿、报价、交易、K线。
缓存中的所有市场数据都使用反向索引存储——意味着最新数据在索引0处。

#### K线访问

```python
# 获取K线类型的所有缓存K线列表
bars = self.cache.bars(bar_type)  # 返回List[Bar]或如果未找到K线则返回空列表

# 获取最新的K线
latest_bar = self.cache.bar(bar_type)  # 返回Bar或如果不存在此类对象则返回None

# 通过索引获取特定的历史K线（0 = 最新）
second_last_bar = self.cache.bar(bar_type, index=1)  # 返回Bar或如果不存在此类对象则返回None

# 检查K线是否存在并获取计数
bar_count = self.cache.bar_count(bar_type)  # 返回指定K线类型在缓存中的K线数量
has_bars = self.cache.has_bars(bar_type)    # 返回布尔值，指示指定K线类型是否存在任何K线
```

#### 报价tick

```python
# 获取报价
quotes = self.cache.quote_ticks(instrument_id)                     # 返回List[QuoteTick]或如果未找到报价则返回空列表
latest_quote = self.cache.quote_tick(instrument_id)                # 返回QuoteTick或如果不存在此类对象则返回None
second_last_quote = self.cache.quote_tick(instrument_id, index=1)  # 返回QuoteTick或如果不存在此类对象则返回None

# 检查报价可用性
quote_count = self.cache.quote_tick_count(instrument_id)  # 返回此工具在缓存中的报价数量
has_quotes = self.cache.has_quote_ticks(instrument_id)    # 返回布尔值，指示此工具是否存在任何报价
```

#### 交易tick

```python
# 获取交易
trades = self.cache.trade_ticks(instrument_id)         # 返回List[TradeTick]或如果未找到交易则返回空列表
latest_trade = self.cache.trade_tick(instrument_id)    # 返回TradeTick或如果不存在此类对象则返回None
second_last_trade = self.cache.trade_tick(instrument_id, index=1)  # 返回TradeTick或如果不存在此类对象则返回None

# 检查交易可用性
trade_count = self.cache.trade_tick_count(instrument_id)  # 返回此工具在缓存中的交易数量
has_trades = self.cache.has_trade_ticks(instrument_id)    # 返回布尔值，指示是否存在任何交易
```

#### 订单簿

```python
# 获取当前订单簿
book = self.cache.order_book(instrument_id)  # 返回OrderBook或如果不存在此类对象则返回None

# 检查订单簿是否存在
has_book = self.cache.has_order_book(instrument_id)  # 返回布尔值，指示是否存在订单簿

# 获取订单簿更新计数
update_count = self.cache.book_update_count(instrument_id)  # 返回收到的更新数量
```

#### 价格访问

```python
from nautilus_trader.core.rust.model import PriceType

# 按类型获取当前价格；返回Price或None。
price = self.cache.price(
    instrument_id=instrument_id,
    price_type=PriceType.MID,  # 选项：BID、ASK、MID、LAST
)
```

#### K线类型

```python
from nautilus_trader.core.rust.model import PriceType, AggregationSource

# 获取工具的所有可用K线类型；返回List[BarType]。
bar_types = self.cache.bar_types(
    instrument_id=instrument_id,
    price_type=PriceType.LAST,  # 选项：BID、ASK、MID、LAST
    aggregation_source=AggregationSource.EXTERNAL,
)
```

#### 简单示例

```python
class MarketDataStrategy(Strategy):
    def on_start(self):
        # 订阅1分钟K线
        self.bar_type = BarType.from_str(f"{self.instrument_id}-1-MINUTE-LAST-EXTERNAL")  # instrument_id示例 = "EUR/USD.FXCM"
        self.subscribe_bars(self.bar_type)

    def on_bar(self, bar: Bar) -> None:
        bars = self.cache.bars(self.bar_type)[:3]
        if len(bars) < 3:   # 等待直到我们至少有3根K线
            return

        # 访问最后3根K线进行分析
        current_bar = bars[0]    # 最新K线
        prev_bar = bars[1]       # 倒数第二根K线
        prev_prev_bar = bars[2]  # 倒数第三根K线

        # 获取最新报价和交易
        latest_quote = self.cache.quote_tick(self.instrument_id)
        latest_trade = self.cache.trade_tick(self.instrument_id)

        if latest_quote is not None:
            current_spread = latest_quote.ask_price - latest_quote.bid_price
            self.log.info(f"当前价差：{current_spread}")
```

### 交易对象

`Cache`提供对系统内所有交易对象的全面访问，包括：

- 订单
- 头寸
- 账户
- 工具

#### 订单

订单可以通过多种方法访问和查询，具有按场所、策略、工具和订单方向的灵活过滤选项。

##### 基本订单访问

```python
# 通过客户端订单ID获取特定订单
order = self.cache.order(ClientOrderId("O-123"))

# 获取系统中的所有订单
orders = self.cache.orders()

# 获取按特定条件过滤的订单
orders_for_venue = self.cache.orders(venue=venue)                       # 特定场所的所有订单
orders_for_strategy = self.cache.orders(strategy_id=strategy_id)        # 特定策略的所有订单
orders_for_instrument = self.cache.orders(instrument_id=instrument_id)  # 工具的所有订单
```

##### 订单状态查询

```python
# 按当前状态获取订单
open_orders = self.cache.orders_open()          # 当前在场所活跃的订单
closed_orders = self.cache.orders_closed()      # 已完成生命周期的订单
emulated_orders = self.cache.orders_emulated()  # 系统本地模拟的订单
inflight_orders = self.cache.orders_inflight()  # 已提交（或修改）到场所但尚未确认的订单

# 检查特定订单状态
exists = self.cache.order_exists(client_order_id)            # 检查给定ID的订单是否存在于缓存中
is_open = self.cache.is_order_open(client_order_id)          # 检查订单当前是否打开
is_closed = self.cache.is_order_closed(client_order_id)      # 检查订单是否关闭
is_emulated = self.cache.is_order_emulated(client_order_id)  # 检查订单是否在本地模拟
is_inflight = self.cache.is_order_inflight(client_order_id)  # 检查订单是否已提交或修改但尚未确认
```

##### 订单统计

```python
# 获取不同状态的订单计数
open_count = self.cache.orders_open_count()          # 未结订单数量
closed_count = self.cache.orders_closed_count()      # 已结订单数量
emulated_count = self.cache.orders_emulated_count()  # 模拟订单数量
inflight_count = self.cache.orders_inflight_count()  # 在途订单数量
total_count = self.cache.orders_total_count()        # 系统中的订单总数

# 获取过滤的订单计数
buy_orders_count = self.cache.orders_open_count(side=OrderSide.BUY)  # 当前未结买单数量
venue_orders_count = self.cache.orders_total_count(venue=venue)      # 给定场所的订单总数
```

#### 头寸

`Cache`维护所有头寸的记录并提供多种查询方式。

##### 头寸访问

```python
# 通过ID获取特定头寸
position = self.cache.position(PositionId("P-123"))

# 按状态获取头寸
all_positions = self.cache.positions()            # 系统中的所有头寸
open_positions = self.cache.positions_open()      # 所有当前未结头寸
closed_positions = self.cache.positions_closed()  # 所有已结头寸

# 获取按各种条件过滤的头寸
venue_positions = self.cache.positions(venue=venue)                       # 特定场所的头寸
instrument_positions = self.cache.positions(instrument_id=instrument_id)  # 特定工具的头寸
strategy_positions = self.cache.positions(strategy_id=strategy_id)        # 特定策略的头寸
long_positions = self.cache.positions(side=PositionSide.LONG)             # 所有多头头寸
```

##### 头寸状态查询

```python
# 检查头寸状态
exists = self.cache.position_exists(position_id)        # 检查给定ID的头寸是否存在
is_open = self.cache.is_position_open(position_id)      # 检查头寸是否未结
is_closed = self.cache.is_position_closed(position_id)  # 检查头寸是否已结

# 获取头寸和订单关系
orders = self.cache.orders_for_position(position_id)       # 与特定头寸相关的所有订单
position = self.cache.position_for_order(client_order_id)  # 查找与特定订单关联的头寸
```

##### 头寸统计

```python
# 获取不同状态的头寸计数
open_count = self.cache.positions_open_count()      # 当前未结头寸数量
closed_count = self.cache.positions_closed_count()  # 已结头寸数量
total_count = self.cache.positions_total_count()    # 系统中的头寸总数

# 获取过滤的头寸计数
long_positions_count = self.cache.positions_open_count(side=PositionSide.LONG)              # 未结多头头寸数量
instrument_positions_count = self.cache.positions_total_count(instrument_id=instrument_id)  # 给定工具的头寸数量
```

#### 账户

```python
# 访问账户信息
account = self.cache.account(account_id)       # 通过ID检索账户
account = self.cache.account_for_venue(venue)  # 检索特定场所的账户
account_id = self.cache.account_id(venue)      # 检索场所的账户ID
accounts = self.cache.accounts()               # 检索缓存中的所有账户
```

#### 工具和货币

##### 工具

```python
# 获取工具信息
instrument = self.cache.instrument(instrument_id) # 通过ID检索特定工具
all_instruments = self.cache.instruments()        # 检索缓存中的所有工具

# 获取过滤的工具
venue_instruments = self.cache.instruments(venue=venue)              # 特定场所的工具
instruments_by_underlying = self.cache.instruments(underlying="ES")  # 按基础资产分组的工具

# 获取工具标识符
instrument_ids = self.cache.instrument_ids()                   # 获取所有工具ID
venue_instrument_ids = self.cache.instrument_ids(venue=venue)  # 获取特定场所的工具ID
```

##### 货币

```python
# 获取货币信息
currency = self.cache.load_currency("USD")  # 加载USD的货币数据
```

---

### 自定义数据

除了内置的市场数据和交易对象外，`Cache`还可以存储和检索自定义数据类型。
您可以保留任何您想要在系统组件（主要是角色/策略）之间共享的用户定义数据。

#### 基本存储和检索

```python
# 在策略方法内调用此代码（`self`指策略）

# 存储数据
self.cache.add(key="my_key", value=b"some binary data")

# 检索数据
stored_data = self.cache.get("my_key")  # 返回bytes或None
```

对于更复杂的用例，`Cache`可以存储继承自`nautilus_trader.core.Data`基类的自定义数据对象。

:::warning
`Cache`不设计为完整的数据库替代品。对于大型数据集或复杂查询需求，请考虑使用专用数据库系统。
:::

## 最佳实践和常见问题

### Cache vs. Portfolio使用

`Cache`和`Portfolio`组件在NautilusTrader中服务于不同但互补的目的：

**Cache**：

- 维护交易系统的历史知识和当前状态。
- 对于本地状态更改立即更新（初始化要提交的订单）
- 随着外部事件发生异步更新（订单成交）。
- 提供交易活动和市场数据的完整历史。
- 策略收到的所有数据（事件/更新）都存储在Cache中。

**Portfolio**：

- 聚合的头寸/敞口和账户信息。
- 提供当前状态而不包含历史。

**示例**：

```python
class MyStrategy(Strategy):
    def on_position_changed(self, event: PositionEvent) -> None:
        # 当您需要历史视角时使用Cache
        position_history = self.cache.position_snapshots(event.position_id)

        # 当您需要当前实时状态时使用Portfolio
        current_exposure = self.portfolio.net_exposure(event.instrument_id)
```

### Cache vs. 策略变量

在`Cache`中存储数据与策略变量之间的选择取决于您的具体需求：

**Cache存储**：

- 用于需要在策略之间共享的数据。
- 最适合需要在系统重启之间持久化的数据。
- 充当所有组件可访问的中央数据库。
- 理想用于需要在策略重置后继续存在的状态。

**策略变量**：

- 用于特定于策略的计算。
- 更适合临时值和中间结果。
- 提供更快的访问和更好的封装。
- 最适合只有您的策略需要的数据。

**示例**：

澄清如何在`Cache`中存储数据以便多个策略可以访问相同信息的示例。

```python
import pickle

class MyStrategy(Strategy):
    def on_start(self):
        # 准备您想要与其他策略共享的数据
        shared_data = {
            "last_reset": self.clock.timestamp_ns(),
            "trading_enabled": True,
            # 包含您希望其他策略读取的任何其他字段
        }

        # 使用描述性键将其存储在缓存中
        # 这样，多个策略可以调用self.cache.get("shared_strategy_info")
        # 来检索相同的数据
        self.cache.add("shared_strategy_info", pickle.dumps(shared_data))

```

另一个策略（并行运行）如何检索上述缓存数据：

```python
import pickle

class AnotherStrategy(Strategy):
    def on_start(self):
        # 从相同键加载共享数据
        data_bytes = self.cache.get("shared_strategy_info")
        if data_bytes is not None:
            shared_data = pickle.loads(data_bytes)
            self.log.info(f"检索到共享数据：{shared_data}")
```
