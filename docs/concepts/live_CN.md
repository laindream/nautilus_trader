# 实时交易

NautilusTrader中的实时交易使交易者能够在真实的交易环境中部署其回测策略，无需任何代码更改。从回测到实时交易的这种无缝过渡是平台的核心功能，确保一致性和可靠性。但是，需要了解回测和实时交易之间的关键差异。

本指南提供了实时交易关键方面的概述。

## 配置

在操作实时交易系统时，正确配置执行引擎和策略对于确保可靠性、准确性和性能至关重要。以下是涉及实时配置的关键概念和设置的概述。

### TradingNodeConfig

实时交易系统的主要配置类是`TradingNodeConfig`，它继承自`NautilusKernelConfig`并提供实时特定的配置选项：

```python
from nautilus_trader.config import TradingNodeConfig

config = TradingNodeConfig(
    trader_id="MyTrader-001",

    # 组件配置
    cache: CacheConfig(),
    message_bus: MessageBusConfig(),
    data_engine=LiveDataEngineConfig(),
    risk_engine=LiveRiskEngineConfig(),
    exec_engine=LiveExecEngineConfig(),
    portfolio=PortfolioConfig(),

    # 客户端配置
    data_clients={
        "BINANCE": BinanceDataClientConfig(),
    },
    exec_clients={
        "BINANCE": BinanceExecClientConfig(),
    },
)
```

#### 核心TradingNodeConfig参数

| 设置                  | 默认            | 描述                                        |
|--------------------------|--------------------|----------------------------------------------------|
| `trader_id`              | "TRADER-001"       | 唯一交易者标识符（名称-标签格式）。        |
| `instance_id`            | `None`             | 可选的唯一实例标识符。               |
| `timeout_connection`     | 30.0               | 连接超时秒数。                     |
| `timeout_reconciliation` | 10.0               | 协调超时秒数。                 |
| `timeout_portfolio`      | 10.0               | 投资组合初始化超时。                  |
| `timeout_disconnection`  | 10.0               | 断开连接超时。                             |
| `timeout_post_stop`      | 5.0                | 停止后清理超时。                         |

#### 缓存数据库配置

配置与后备数据库的数据持久化：

```python
from nautilus_trader.config import CacheConfig
from nautilus_trader.config import DatabaseConfig

cache_config = CacheConfig(
    database=DatabaseConfig(
        host="localhost",
        port=6379,
        username="nautilus",
        password="pass",
        timeout=2.0,
    ),
    encoding="msgpack",  # 或"json"
    timestamps_as_iso8601=True,
    buffer_interval_ms=100,
    flush_on_start=False,
)
```

#### 消息总线配置

配置消息路由和外部流式传输：

```python
from nautilus_trader.config import MessageBusConfig
from nautilus_trader.config import DatabaseConfig

message_bus_config = MessageBusConfig(
    database=DatabaseConfig(timeout=2),
    timestamps_as_iso8601=True,
    use_instance_id=False,
    types_filter=[QuoteTick, TradeTick],  # 过滤特定消息类型
    stream_per_topic=False,
    autotrim_mins=30,  # 自动消息修剪
    heartbeat_interval_secs=1,
)
```

### 多场所配置

实时交易系统通常连接到多个场所或市场类型。以下是为Binance配置现货和期货市场的示例：

```python
config = TradingNodeConfig(
    trader_id="MultiVenue-001",

    # 不同市场类型的多个数据客户端
    data_clients={
        "BINANCE_SPOT": BinanceDataClientConfig(
            account_type=BinanceAccountType.SPOT,
            testnet=False,
        ),
        "BINANCE_FUTURES": BinanceDataClientConfig(
            account_type=BinanceAccountType.USDT_FUTURE,
            testnet=False,
        ),
    },

    # 相应的执行客户端
    exec_clients={
        "BINANCE_SPOT": BinanceExecClientConfig(
            account_type=BinanceAccountType.SPOT,
            testnet=False,
        ),
        "BINANCE_FUTURES": BinanceExecClientConfig(
            account_type=BinanceAccountType.USDT_FUTURE,
            testnet=False,
        ),
    },
)
```

### 执行引擎配置

`LiveExecEngineConfig`设置实时执行引擎，管理订单处理、执行事件以及与交易场所的协调。
以下概述了主要配置选项。

通过深思熟虑地配置这些参数，您可以确保您的交易系统高效运行，正确处理订单，并在面临潜在问题（如丢失事件或冲突数据/信息）时保持弹性。

:::info
另请参阅`LiveExecEngineConfig` [API参考](../api_reference/config_CN.md#class-liveexecengineconfig)了解更多详细信息。
:::

#### 协调

**目的**：通过恢复任何错过的事件（如订单和头寸状态更新），确保系统状态与交易场所保持一致。

| 设置                         | 默认 | 描述                                                                                        |
|---------------------------------|---------|----------------------------------------------------------------------------------------------------|
| `reconciliation`                | True    | 在启动时激活协调，将系统的内部状态与场所状态对齐。  |
| `reconciliation_lookback_mins`  | None    | 指定系统请求过去事件以协调未缓存状态的回看时间（分钟）。   |
| `reconciliation_instrument_ids` | None    | 要考虑用于协调的特定工具ID包含列表。                         |
| `filtered_client_order_ids`     | None    | 要从协调中过滤的客户端订单ID列表（当场所持有重复项时有用）。 |

:::info
另请参阅[执行协调](../concepts/execution_CN.md#execution-reconciliation)了解更多详细信息。
:::

#### 订单过滤

**目的**：管理系统应处理哪些订单事件和报告，以避免与其他交易节点的冲突和不必要的数据处理。

| 设置                            | 默认 | 描述                                                                                                |
|------------------------------------|---------|------------------------------------------------------------------------------------------------------------|
| `filter_unclaimed_external_orders` | False   | 过滤掉未声明的外部订单，以防止无关订单影响策略。            |
| `filter_position_reports`          | False   | 过滤掉头寸状态报告，当多个节点交易同一账户时有用，以避免冲突。 |

#### 传输中订单检查

**目的**：定期检查传输中订单（已提交、修改或取消但尚未确认的订单）的状态，以确保它们得到正确和及时的处理。

| 设置                       | 默认   | 描述                                                                                                                         |
|-------------------------------|-----------|-------------------------------------------------------------------------------------------------------------------------------------|
| `inflight_check_interval_ms`  | 2,000 ms  | 确定系统检查传输中订单状态的频率。                                                                 |
| `inflight_check_threshold_ms` | 5,000 ms  | 设置传输中订单触发场所状态检查的时间阈值。如果位于同一位置，请调整以避免竞争条件。 |
| `inflight_check_retries`      | 5 retries | 指定引擎尝试与场所验证传输中订单状态的重试次数，如果初始尝试失败。 |

#### 其他执行引擎选项

以下附加选项提供对执行行为的进一步控制：

| 设置                            | 默认 | 描述                                                                                                |
|------------------------------------|---------|------------------------------------------------------------------------------------------------------------|
| `generate_missing_orders`          | True    | 如果在协调期间将生成`MARKET`订单事件以对齐头寸差异。这些订单使用策略ID `INTERNAL-DIFF`。          |
| `snapshot_orders`                  | False   | 是否应在订单事件上拍摄订单快照。                                                        |
| `snapshot_positions`               | False   | 是否应在头寸事件上拍摄头寸快照。                                                  |
| `snapshot_positions_interval_secs` | None    | 启用时头寸快照之间的间隔（秒）。                                            |
| `debug`                            | False   | 启用调试模式以获得额外的执行日志。                                                        |

如果传输中订单的状态在用尽所有重试后无法协调，系统会根据其状态生成以下事件之一来解决：

- `SUBMITTED` -> `REJECTED`：如果没有收到确认，则假设提交失败。
- `PENDING_UPDATE` -> `CANCELED`：如果未解决，则将待修改视为已取消。
- `PENDING_CANCEL` -> `CANCELED`：如果场所不响应，则假设取消。

这确保交易节点即使在不可靠的条件下也能保持一致的执行状态。 