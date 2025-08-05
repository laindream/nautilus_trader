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