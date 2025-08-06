# 执行

NautilusTrader可以同时处理多个策略和场所的交易执行和订单管理（每个实例）。执行涉及多个相互作用的组件，因此理解执行消息（命令和事件）的可能流程至关重要。

主要的执行相关组件包括：

- `Strategy`
- `ExecAlgorithm`（执行算法）
- `OrderEmulator`
- `RiskEngine`
- `ExecutionEngine`或`LiveExecutionEngine`
- `ExecutionClient`或`LiveExecutionClient`

## 执行流程

`Strategy`基类继承自`Actor`，因此包含所有常见的数据相关方法。它还提供管理订单和交易执行的方法：

- `submit_order(...)`
- `submit_order_list(...)`
- `modify_order(...)`
- `cancel_order(...)`
- `cancel_orders(...)`
- `cancel_all_orders(...)`
- `close_position(...)`
- `close_all_positions(...)`
- `query_account(...)`
- `query_order(...)`

这些方法在底层创建必要的执行命令，并将它们通过消息总线发送到相关组件（点对点），以及发布任何事件（例如新订单的初始化，即`OrderInitialized`事件）。

一般执行流程如下（每个箭头表示通过消息总线的移动）：

`Strategy` -> `OrderEmulator` -> `ExecAlgorithm` -> `RiskEngine` -> `ExecutionEngine` -> `ExecutionClient`

`OrderEmulator`和`ExecAlgorithm`组件在流程中是可选的，取决于单个订单参数（如下所述）。

此图说明了Nautilus执行组件之间的消息流（命令和事件）。

```
                  ┌───────────────────┐
                  │                   │
                  │                   │
                  │                   │
          ┌───────►   OrderEmulator   ├────────────┐
          │       │                   │            │
          │       │                   │            │
          │       │                   │            │
┌─────────┴──┐    └─────▲──────┬──────┘            │
│            │          │      │           ┌───────▼────────┐   ┌─────────────────────┐   ┌─────────────────────┐
│            │          │      │           │                │   │                     │   │                     │
│            ├──────────┼──────┼───────────►                ├───►                     ├───►                     │
│  Strategy  │          │      │           │                │   │                     │   │                     │
│            │          │      │           │   RiskEngine   │   │   ExecutionEngine   │   │   ExecutionClient   │
│            ◄──────────┼──────┼───────────┤                ◄───┤                     ◄───┤                     │
│            │          │      │           │                │   │                     │   │                     │
│            │          │      │           │                │   │                     │   │                     │
└─────────┬──┘    ┌─────┴──────▼──────┐    └───────▲────────┘   └─────────────────────┘   └─────────────────────┘
          │       │                   │            │
          │       │                   │            │
          │       │                   │            │
          └───────►   ExecAlgorithm   ├────────────┘
                  │                   │
                  │                   │
                  │                   │
                  └───────────────────┘

```

## 订单管理系统（OMS）

订单管理系统（OMS）类型是指用于将订单分配给头寸并跟踪这些工具头寸的方法。
OMS类型适用于策略和场所（模拟和真实）。即使场所没有明确说明使用的方法，OMS类型也始终有效。可以使用`OmsType`枚举为组件指定OMS类型。

`OmsType`枚举有三个变体：

- `UNSPECIFIED`：OMS类型根据应用位置默认（详情如下）
- `NETTING`：头寸合并为每个工具ID的单个头寸
- `HEDGING`：支持每个工具ID的多个头寸（多头和空头）

下表描述了不同的配置组合及其适用场景。
当策略和场所OMS类型不同时，`ExecutionEngine`通过覆盖或为接收的`OrderFilled`事件分配`position_id`值来处理这种情况。
"虚拟头寸"是指在Nautilus系统内存在但在场所实际中不存在的头寸ID。

| 策略OMS                 | 场所OMS              | 描述                                                                                                                                                |
|:-----------------------------|:-----------------------|:-----------------------------------------------------------------------------------------------------------------------------------------------------------|
| `NETTING`                    | `NETTING`              | 策略使用场所的原生OMS类型，每个工具ID有单个头寸ID。                                                                 |
| `HEDGING`                    | `HEDGING`              | 策略使用场所的原生OMS类型，每个工具ID有多个头寸ID（`LONG`和`SHORT`）。                                      |
| `NETTING`                    | `HEDGING`              | 策略**覆盖**场所的原生OMS类型。场所跟踪每个工具ID的多个头寸，但Nautilus维护单个头寸ID。 |
| `HEDGING`                    | `NETTING`              | 策略**覆盖**场所的原生OMS类型。场所跟踪每个工具ID的单个头寸，但Nautilus维护多个头寸ID。 |

:::note
为策略和场所单独配置OMS类型增加了平台复杂性，但允许广泛的交易风格和偏好（见下文）。
:::

OMS配置示例：

- 大多数加密货币交易所使用`NETTING` OMS类型，表示每个市场单个头寸。交易者可能希望为策略跟踪多个"虚拟"头寸。
- 一些外汇ECN或经纪商使用`HEDGING` OMS类型，跟踪多个`LONG`和`SHORT`头寸。交易者可能只关心每个货币对的净头寸。

:::info
Nautilus尚不支持场所端对冲模式，如Binance的`BOTH`与`LONG/SHORT`，其中场所按方向净额。
建议将Binance账户配置保持为`BOTH`，以便单个头寸被净额。
:::

### OMS配置

如果策略OMS类型没有使用`oms_type`配置选项明确设置，它将默认为`UNSPECIFIED`。这意味着`ExecutionEngine`不会覆盖任何场所`position_id`，OMS类型将遵循场所的OMS类型。

:::tip
配置回测时，您可以为场所指定`oms_type`。为了提高回测准确性，建议将其与场所实际使用的OMS类型匹配。
:::

## 风险引擎

`RiskEngine`是每个Nautilus系统的核心组件，包括回测、沙盒和实时环境。
除非在`RiskEngineConfig`中特别绕过，否则每个订单命令和事件都通过`RiskEngine`。

`RiskEngine`包括几个内置的交易前风险检查，包括：

- 价格精度对工具正确
- 价格为正（除非是期权类型工具）
- 数量精度对工具正确
- 低于工具的最大名义价值
- 在工具的最大或最小数量范围内
- 仅在为订单指定`reduce_only`执行指令时减少头寸

如果任何风险检查失败，会生成`OrderDenied`事件，有效地关闭订单并防止其进一步进展。此事件包括拒绝的人类可读原因。

### 交易状态

此外，Nautilus系统的当前交易状态影响订单流。

`TradingState`枚举有三个变体：

- `ACTIVE`：系统正常运行
- `HALTED`：系统在处理状态更改之前不会处理进一步的订单命令
- `REDUCING`：系统只会处理取消或减少未平仓头寸的命令

:::info
请参阅`RiskEngineConfig` [API参考](../api_reference/config_CN.md#risk)以获取更多详细信息。
:::

## 执行算法

平台支持自定义执行算法组件，并提供一些内置算法，如时间加权平均价格（TWAP）算法。

### TWAP（时间加权平均价格）

TWAP执行算法旨在通过在指定的时间范围内均匀分布来执行订单。算法接收表示总规模和方向的主要订单，然后通过生成较小的子订单来分割它，这些子订单在时间范围内以固定间隔执行。

这有助于通过最小化任何给定时间的交易规模集中度来减少主要订单全部规模对市场的影响。

算法将立即提交第一个订单，最终提交的订单是时间范围结束时的主要订单。

使用TWAP算法作为示例（在`/examples/algorithms/twap.py`中找到），此示例演示如何直接向`BacktestEngine`初始化和注册TWAP执行算法（假设引擎已经初始化）：

```python
from nautilus_trader.examples.algorithms.twap import TWAPExecAlgorithm

# `engine`是一个初始化的BacktestEngine实例
exec_algorithm = TWAPExecAlgorithm()
engine.add_exec_algorithm(exec_algorithm)
```

对于这个特定算法，必须指定两个参数：

- `horizon_secs`
- `interval_secs`

`horizon_secs`参数确定算法执行的时间段，而`interval_secs`参数设置单个订单执行之间的时间。这些参数确定主要订单如何分割成一系列生成的订单。

```python
from decimal import Decimal
from nautilus_trader.model.data import BarType
from nautilus_trader.test_kit.providers import TestInstrumentProvider
from nautilus_trader.examples.strategies.ema_cross_twap import EMACrossTWAP, EMACrossTWAPConfig

# 配置您的策略
config = EMACrossTWAPConfig(
    instrument_id=TestInstrumentProvider.ethusdt_binance().id,
    bar_type=BarType.from_str("ETHUSDT.BINANCE-250-TICK-LAST-INTERNAL"),
    trade_size=Decimal("0.05"),
    fast_ema_period=10,
    slow_ema_period=20,
    twap_horizon_secs=10.0,   # 执行算法参数（总时间范围，秒）
    twap_interval_secs=2.5,    # 执行算法参数（订单之间的秒数）
)

# 实例化您的策略
strategy = EMACrossTWAP(config=config)
```

或者，您可以动态地按订单指定这些参数，根据实际市场条件确定它们。在这种情况下，策略配置参数可以提供给确定时间范围和间隔的执行模型。

:::info
您可以创建的执行算法参数数量没有限制。参数只需要是一个具有字符串键和原始值的字典（可以通过网络序列化的值，如int、float和string）。
:::

### 编写执行算法

要实现自定义执行算法，您必须定义一个继承自`ExecAlgorithm`的类。

执行算法是`Actor`的一种类型，因此它能够：

- 请求和订阅数据
- 访问`Cache`
- 使用`Clock`设置时间警报和/或定时器

此外，它还可以：

- 访问中央`Portfolio`
- 从接收的主要（原始）订单生成次要订单

一旦执行算法注册并且系统运行，它将通过`exec_algorithm_id`订单参数接收消息总线上发送给其`ExecAlgorithmId`的订单。
订单还可能携带`exec_algorithm_params`，这是一个`dict[str, Any]`。

:::warning
由于`exec_algorithm_params`字典的灵活性，彻底验证所有键值对以确保算法的正确操作很重要（首先确保字典不是`None`并且所有必要参数实际存在）。
:::

接收的订单将通过以下`on_order(...)`方法到达。这些接收的订单在执行算法处理时被称为"主要"（原始）订单。

```python
from nautilus_trader.model.orders.base import Order

def on_order(self, order: Order) -> None:
    # 在这里处理订单
```

当算法准备生成次要订单时，它可以使用以下方法之一：

- `spawn_market(...)`（生成`MARKET`订单）
- `spawn_market_to_limit(...)`（生成`MARKET_TO_LIMIT`订单）
- `spawn_limit(...)`（生成`LIMIT`订单）

:::note
其他订单类型将在未来版本中实现，根据需要。
:::

这些方法中的每一个都将主要（原始）`Order`作为第一个参数。主要订单数量将减少传入的`quantity`（成为生成订单的数量）。

:::warning
必须有足够的主要订单数量剩余（这是经过验证的）。
:::

一旦生成了所需数量的次要订单，并且执行例程结束，算法的意图是最终发送主要（原始）订单。

### 生成的订单

从执行算法生成的所有次要订单都将携带一个`exec_spawn_id`，它只是主要（原始）订单的`ClientOrderId`，其`client_order_id`从这个原始标识符派生，遵循以下约定：

- `exec_spawn_id`（主要订单`client_order_id`值）
- `spawn_sequence`（生成订单的序列号）

```
{exec_spawn_id}-E{spawn_sequence}
```

例如，`O-20230404-001-000-E1`（对于第一个生成的订单）

:::note
选择"主要"和"次要"/"生成"术语是为了避免与"父"和"子"应急订单术语的冲突或混淆（执行算法也可能处理应急订单）。
:::

### 管理执行算法订单

`Cache`提供几种方法来帮助管理（跟踪）执行算法的活动。调用下面的方法将返回给定查询过滤器的所有执行算法订单。

```python
def orders_for_exec_algorithm(
    self,
    exec_algorithm_id: ExecAlgorithmId,
    venue: Venue | None = None,
    instrument_id: InstrumentId | None = None,
    strategy_id: StrategyId | None = None,
    side: OrderSide = OrderSide.NO_ORDER_SIDE,
) -> list[Order]:
```

以及更具体地查询某个执行系列/生成的订单。
调用下面的方法将返回给定`exec_spawn_id`的所有订单（如果找到）。

```python
def orders_for_exec_spawn(self, exec_spawn_id: ClientOrderId) -> list[Order]:
```

:::note
这也包括主要（原始）订单。
:::
