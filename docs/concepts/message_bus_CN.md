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
``` 