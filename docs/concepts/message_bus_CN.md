# 消息总线

`MessageBus`是平台的基础部分，通过消息传递实现系统组件之间的通信。这种设计创建了一个松散耦合的架构，其中组件可以在没有直接依赖的情况下进行交互。

*消息传递模式*包括：

- 点对点
- 发布/订阅
- 请求/响应

通过`MessageBus`交换的消息分为三类：

- 数据
- 事件
- 命令

## 数据和信号发布

虽然`MessageBus`是用户通常间接交互的低级组件，`Actor`和`Strategy`类提供了基于它构建的便捷方法：

```python
def publish_data(self, data_type: DataType, data: Data) -> None:
def publish_signal(self, name: str, value, ts_event: int | None = None) -> None:
```

这些方法允许您有效地发布自定义数据和信号，而无需直接使用`MessageBus`接口。

## 直接访问

对于高级用户或专门用例，在`Actor`和`Strategy`类中可以通过`self.msgbus`引用直接访问消息总线，它提供完整的消息总线接口。

要直接发布自定义消息，您可以指定一个`str`主题和任何Python `object`作为消息载荷，例如：

```python

self.msgbus.publish("MyTopic", "MyMessage")
```

## 消息传递风格

NautilusTrader是一个**事件驱动**框架，组件通过发送和接收消息进行通信。
理解不同的消息传递风格对于构建有效的交易系统至关重要。

本指南解释了NautilusTrader中可用的三种主要消息传递模式：

| **消息传递风格**                          | **目的**                                 | **最适合**                                          |
|:---------------------------------------------|:--------------------------------------------|:------------------------------------------------------|
| **MessageBus - 发布/订阅主题** | 低级，直接访问消息总线 | 自定义事件，系统级通信             |
| **基于Actor - 发布/订阅数据**     | 结构化交易数据交换            | 交易指标，指标，需要持久化的数据 |
| **基于Actor - 发布/订阅信号**   | 轻量级通知                   | 简单警报，标志，状态更新                  |

每种方法服务于不同的目的并提供独特的优势。本指南将帮助您决定在NautilusTrader应用程序中使用哪种消息传递模式。

### MessageBus发布/订阅主题

#### 概念

`MessageBus`是NautilusTrader中所有消息的中央枢纽。它启用了一种**发布/订阅**模式，其中组件可以将事件发布到**命名主题**，其他组件可以订阅以接收这些消息。
这使组件解耦，允许它们通过消息总线间接交互。

#### 关键优势和用例

当您需要以下功能时，消息总线方法是理想的：

- 系统内的**跨组件通信**。
- **灵活性**定义任何主题并发送任何类型的载荷（任何Python对象）。
- 发布者和订阅者之间的**解耦**，他们不需要了解彼此。
- **全局覆盖**，消息可以被多个订阅者接收。
- 处理不适合预定义`Actor`模型的事件。
- 需要完全控制消息传递的高级场景。

#### 注意事项

- 您必须手动跟踪主题名称（拼写错误可能导致错过消息）。
- 您必须手动定义处理程序。

#### 快速概览代码

```python
from nautilus_trader.core.message import Event

# 定义自定义事件
class Each10thBarEvent(Event):
    TOPIC = "each_10th_bar"  # 主题名称
    def __init__(self, bar):
        self.bar = bar

# 在组件中订阅（在策略中）
self.msgbus.subscribe(Each10thBarEvent.TOPIC, self.on_each_10th_bar)

# 发布事件（在策略中）
event = Each10thBarEvent(bar)
self.msgbus.publish(Each10thBarEvent.TOPIC, event)

# 处理程序（在策略中）
def on_each_10th_bar(self, event: Each10thBarEvent):
    self.log.info(f"收到第10根K线：{event.bar}")
```

#### 完整示例

[MessageBus示例](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/backtest/example_09_messaging_with_msgbus)

### 基于Actor的发布/订阅数据

#### 概念

这种方法提供了一种在系统中的`Actor`之间交换交易特定数据的方式。
（注意：每个`Strategy`都继承自`Actor`）。它继承自`Data`，确保正确的时间戳和事件排序——这对正确的回测处理至关重要。

#### 主要优势和用例

数据发布/订阅方法在您需要以下情况时表现出色：

- **结构化交易数据的交换**，如市场数据、指标、自定义指标或期权希腊字母。
- **正确的事件排序**，通过内置时间戳（`ts_event`、`ts_init`）对回测准确性至关重要。
- **数据持久化和序列化**，通过`@customdataclass`装饰器，与NautilusTrader的数据目录系统无缝集成。
- **系统组件之间的标准化交易数据交换**。

#### 注意事项

- 需要定义一个继承自`Data`或使用`@customdataclass`的类。

#### 继承自`Data` vs. 使用`@customdataclass`

**继承自`Data`类：**

- 定义了必须由子类实现的抽象属性`ts_event`和`ts_init`。这些确保基于时间戳在回测中正确的数据排序。

**`@customdataclass`装饰器：**

- 如果`ts_event`和`ts_init`属性尚不存在，则添加它们。
- 提供序列化函数：`to_dict()`、`from_dict()`、`to_bytes()`、`to_arrow()`等。
- 启用数据持久化和外部通信。

#### 快速概览代码

```python
from nautilus_trader.core.data import Data
from nautilus_trader.model.custom import customdataclass

@customdataclass
class GreeksData(Data):
    delta: float
    gamma: float

# 发布数据（在角色/策略中）
data = GreeksData(delta=0.75, gamma=0.1, ts_event=1_630_000_000_000_000_000, ts_init=1_630_000_000_000_000_000)
self.publish_data(GreeksData, data)

# 订阅接收数据（在角色/策略中）
self.subscribe_data(GreeksData)

# 处理程序（这是具有固定名称的静态回调函数）
def on_data(self, data: Data):
    if isinstance(data, GreeksData):
        self.log.info(f"Delta: {data.delta}, Gamma: {data.gamma}")
```

#### 完整示例

[基于Actor的数据示例](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/backtest/example_10_messaging_with_actor_data)

### 基于Actor的发布/订阅信号

#### 概念

**信号**是在角色框架内发布和订阅简单通知的轻量级方式。
这是最简单的消息传递方法，不需要自定义类定义。

#### 主要优势和用例

信号消息传递方法在您需要以下情况时表现出色：

- **简单、轻量级的通知/警报**，如"RiskThresholdExceeded"或"TrendUp"。
- **快速、即时的消息传递**，无需定义自定义类。
- **广播警报或标志**作为原始数据（`int`、`float`或`str`）。
- **轻松的API集成**，通过直接的方法（`publish_signal`、`subscribe_signal`）。
- **多订阅者通信**，发布时所有订阅者都收到信号。
- **最小设置开销**，无需类定义。

#### 注意事项

- 每个信号只能包含**单个值**，类型为：`int`、`float`和`str`。这意味着不支持复杂数据结构或其他Python类型。
- 在`on_signal`处理程序中，您只能使用`signal.value`区分信号，因为信号名称在处理程序中不可访问。

#### 快速概览代码

```python
# 定义信号常量以更好地组织（可选但推荐）
import types
from nautilus_trader.core.datetime import unix_nanos_to_dt
from nautilus_trader.common.enums import LogColor

signals = types.SimpleNamespace()
signals.NEW_HIGHEST_PRICE = "NewHighestPriceReached"
signals.NEW_LOWEST_PRICE = "NewLowestPriceReached"

# 订阅信号（在角色/策略中）
self.subscribe_signal(signals.NEW_HIGHEST_PRICE)
self.subscribe_signal(signals.NEW_LOWEST_PRICE)

# 发布信号（在角色/策略中）
self.publish_signal(
    name=signals.NEW_HIGHEST_PRICE,
    value=signals.NEW_HIGHEST_PRICE,  # 为简化起见，值可以与名称相同
    ts_event=bar.ts_event,  # 来自触发事件的时间戳
)

# 处理程序（这是具有固定名称的静态回调函数）
def on_signal(self, signal):
    # 重要：我们匹配signal.value，而不是signal.name
    match signal.value:
        case signals.NEW_HIGHEST_PRICE:
            self.log.info(
                f"达到新的最高价格。| "
                f"信号值：{signal.value} | "
                f"信号时间：{unix_nanos_to_dt(signal.ts_event)}",
                color=LogColor.GREEN
            )
        case signals.NEW_LOWEST_PRICE:
            self.log.info(
                f"达到新的最低价格。| "
                f"信号值：{signal.value} | "
                f"信号时间：{unix_nanos_to_dt(signal.ts_event)}",
                color=LogColor.RED
            )
```

#### 完整示例

[基于Actor的信号示例](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/backtest/example_11_messaging_with_actor_signals)

### 总结和决策指南

以下是帮助您决定使用哪种消息传递风格的快速参考：

#### 决策指南：选择哪种风格？

| **用例**                                | **推荐方法**                                                        | **所需设置** |
|:--------------------------------------------|:--------------------------------------------------------------------------------|:-------------------|
| 自定义事件或系统级通信 | `MessageBus` + 发布/订阅主题                                                 | 主题 + 处理程序管理 |
| 结构化交易数据                     | `Actor` + 发布/订阅数据 + 如果需要序列化则可选择`@customdataclass` | 继承自`Data`的新类定义（处理程序`on_data`是预定义的） |
| 简单警报/通知                 | `Actor` + 发布/订阅信号                                                        | 只需信号名称 |

## 外部发布

`MessageBus`可以用任何为其编写了集成的数据库或消息代理技术*支持*，这使得可以外部发布消息。

:::info
Redis目前支持所有可序列化的消息，这些消息在外部发布。
支持的最低Redis版本是6.2（需要[streams](https://redis.io/docs/latest/develop/data-types/streams/)功能）。
:::

在底层，当配置了支持数据库（或任何其他兼容技术）时，
所有传出消息首先被序列化，然后通过多生产者单消费者（MPSC）通道传输到单独的线程（在Rust中实现）。
在这个单独的线程中，消息被写入其最终目的地，目前是Redis流。

这种设计主要由性能考虑驱动。通过将I/O操作卸载到单独的线程，
我们确保主线程保持非阻塞状态，可以继续其任务而不受与数据库或客户端交互的潜在耗时操作的阻碍。

### 序列化

Nautilus支持以下序列化：

- 所有Nautilus内置类型（序列化为包含可序列化原语的字典`dict[str, Any]`）。
- Python原始类型（`str`、`int`、`float`、`bool`、`bytes`）。

您可以通过`serialization`子包注册自定义类型来为其添加序列化支持。

```python
def register_serializable_type(
    cls,
    to_dict: Callable[[Any], dict[str, Any]],
    from_dict: Callable[[dict[str, Any]], Any],
):
    ...
```

- `cls`：要注册的类型。
- `to_dict`：从对象实例化原始类型字典的委托。
- `from_dict`：从原始类型字典实例化对象的委托。

## 配置

消息总线外部支持技术可以通过导入`MessageBusConfig`对象并将其传递给您的`TradingNodeConfig`来配置。以下将描述每个配置选项。

```python
...  # 省略其他配置
message_bus=MessageBusConfig(
    database=DatabaseConfig(),
    encoding="json",
    timestamps_as_iso8601=True,
    buffer_interval_ms=100,
    autotrim_mins=30,
    use_trader_prefix=True,
    use_trader_id=True,
    use_instance_id=False,
    streams_prefix="streams",
    types_filter=[QuoteTick, TradeTick],
)
...
```

### 数据库配置

必须提供`DatabaseConfig`，对于本地环回上的默认Redis设置，您可以传递`DatabaseConfig()`，这将使用默认值来匹配。

### 编码

`MessageBus`使用的内置`Serializer`目前支持两种编码：

- JSON（`json`）
- MessagePack（`msgpack`）

使用`encoding`配置选项控制消息写入编码。

:::tip
默认使用`msgpack`编码，因为它提供最优的序列化和内存性能。
当性能不是主要关注点时，我们建议使用`json`编码以获得人类可读性。
:::

### 时间戳格式

默认情况下，时间戳格式化为UNIX纪元纳秒整数。或者，您可以通过将`timestamps_as_iso8601`设置为`True`来配置ISO 8601字符串格式。

### 消息流键

消息流键对于识别单个交易者节点和在流中组织消息至关重要。
它们可以根据您的特定要求和用例进行定制。在消息总线流的上下文中，交易者键通常结构如下：

```
trader:{trader_id}:{instance_id}:{streams_prefix}
```

以下选项可用于配置消息流键：

#### 交易者前缀

如果键应该以`trader`字符串开始。

#### 交易者ID

如果键应该包含节点的交易者ID。

#### 实例ID

每个交易者节点都分配一个唯一的'实例ID'，这是一个UUIDv4。这个实例ID有助于在消息分布在多个流中时区分单个交易者。您可以通过将`use_instance_id`配置选项设置为`True`来在交易者键中包含实例ID。
这在多节点交易系统中需要跨各种流跟踪和识别交易者时特别有用。

#### 流前缀

`streams_prefix`字符串使您能够为单个交易者实例分组所有流或为多个实例组织消息。通过将字符串传递给`streams_prefix`配置选项来配置此项，确保其他前缀设置为false。

#### 每个主题的流

指示生产者是否为每个主题写入单独的流。这对于不支持监听流时的通配符主题的Redis支持特别有用。
如果设置为False，所有消息将写入同一个流。

:::info
Redis不支持通配符流主题。为了更好地与Redis兼容，建议将此选项设置为False。
:::

### 类型过滤

当消息在消息总线上发布时，如果配置并启用了消息总线的支持，它们会被序列化并写入流。为了防止高频率报价等数据充斥流，您可以从外部发布中过滤掉某些类型的消息。

要启用此过滤机制，将`type`对象列表传递给消息总线配置中的`types_filter`参数，指定应从外部发布中排除的消息类型。

```python
from nautilus_trader.config import MessageBusConfig
from nautilus_trader.data import TradeTick
from nautilus_trader.data import QuoteTick

# 创建带有类型过滤的MessageBusConfig实例
message_bus = MessageBusConfig(
    types_filter=[QuoteTick, TradeTick]
)

```

### 流自动修剪

`autotrim_mins`配置参数允许您指定消息流中自动流修剪的回顾窗口（分钟）。
自动流修剪通过删除较旧的消息来帮助管理消息流的大小，确保流在存储和性能方面保持可管理。

:::info
当前的Redis实现将维护`autotrim_mins`作为最大宽度（加上大约一分钟，因为流修剪不超过每分钟一次）。
而不是基于当前墙钟时间的最大回顾窗口。
:::

## 外部流

`TradingNode`（节点）内的消息总线被称为"内部消息总线"。
生产者节点是将消息发布到外部流的节点（参见[外部发布](#external-publishing)）。
消费者节点监听外部流以接收并在其内部消息总线上发布反序列化的消息载荷。

```
                  ┌───────────────────────────┐
                  │                           │
                  │                           │
                  │                           │
                  │      Producer Node        │
                  │                           │
                  │                           │
                  │                           │
                  │                           │
                  │                           │
                  │                           │
                  └─────────────┬─────────────┘
                                │
                                │
┌───────────────────────────────▼──────────────────────────────┐
│                                                              │
│                            Stream                            │
│                                                              │
└─────────────┬────────────────────────────────────┬───────────┘
              │                                    │
              │                                    │
┌─────────────▼───────────┐          ┌─────────────▼───────────┐
│                         │          │                         │
│                         │          │                         │
│     Consumer Node 1     │          │     Consumer Node 2     │
│                         │          │                         │
│                         │          │                         │
│                         │          │                         │
│                         │          │                         │
│                         │          │                         │
│                         │          │                         │
│                         │          │                         │
└─────────────────────────┘          └─────────────────────────┘
```

:::tip
使用表示外部流客户端的`client_id`列表设置`LiveDataEngineConfig.external_clients`。
`DataEngine`将过滤掉这些客户端的订阅命令，确保外部流为这些客户端的任何订阅提供必要的数据。
:::

### 示例配置

以下示例详细说明了一个流设置，其中生产者节点外部发布Binance数据，下游消费者节点将这些数据消息发布到其内部消息总线上。

#### 生产者节点

我们配置生产者节点的`MessageBus`发布到`"binance"`流。
设置`use_trader_id`、`use_trader_prefix`和`use_instance_id`都设置为`False`，以确保消费者节点可以注册的简单和可预测的流键。

```python
message_bus=MessageBusConfig(
    database=DatabaseConfig(timeout=2),
    use_trader_id=False,
    use_trader_prefix=False,
    use_instance_id=False,
    streams_prefix="binance",  # <---
    stream_per_topic=False,
    autotrim_mins=30,
),
```

#### 消费者节点

我们配置消费者节点的`MessageBus`从同一个`"binance"`流接收消息。
节点将监听外部流键以将这些消息发布到其内部消息总线上。
此外，我们将客户端ID `"BINANCE_EXT"`声明为外部客户端。这确保`DataEngine`不尝试向此客户端ID发送数据命令，因为我们期望这些消息从外部流发布到内部消息总线上，节点已订阅相关主题。

```python
data_engine=LiveDataEngineConfig(
    external_clients=[ClientId("BINANCE_EXT")],
),
message_bus=MessageBusConfig(
    database=DatabaseConfig(timeout=2),
    external_streams=["binance"],  # <---
),
```
