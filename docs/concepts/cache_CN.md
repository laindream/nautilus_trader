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