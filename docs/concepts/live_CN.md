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

#### 未结订单检查

**目的**：定期验证未结订单的状态是否与场所匹配，如果发现差异则触发协调。

| 设置                    | 默认值 | 描述                                                                                                                          |
|----------------------------|---------|--------------------------------------------------------------------------------------------------------------------------------------|
| `open_check_interval_secs` | None    | 确定在场所检查未结订单的频率（以秒为单位）。建议：5-10秒，考虑API速率限制。 |
| `open_check_open_only`     | True    | 启用时，检查期间仅请求未结订单；如果禁用，则获取完整订单历史（资源密集型）。         |

#### 订单簿审计

**目的**：确保*自有订单*簿的内部表示与场所的公共订单簿匹配。

| 设置                         | 默认值 | 描述                                                                                                                                         |
|---------------------------------|---------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| `own_books_audit_interval_secs` | None    | 设置对自有订单簿与公共订单簿进行审计的间隔（以秒为单位）。验证同步并记录不一致的错误。 |

#### 内存管理

**目的**：定期从内存缓存中清除已关闭的订单、已关闭的头寸和账户事件，以优化资源使用和在扩展/高频交易操作期间的性能。

| 设置                                | 默认值 | 描述                                                                                                                             |
|----------------------------------------|---------|-----------------------------------------------------------------------------------------------------------------------------------------|
| `purge_closed_orders_interval_mins`    | None    | 设置从内存中清除已关闭订单的频率（以分钟为单位）。建议：10-15分钟。不影响数据库记录。    |
| `purge_closed_orders_buffer_mins`      | None    | 指定订单在清除前必须关闭多长时间（以分钟为单位）。建议：60分钟以确保进程完成。    |
| `purge_closed_positions_interval_mins` | None    | 设置从内存中清除已关闭头寸的频率（以分钟为单位）。建议：10-15分钟。不影响数据库记录。 |
| `purge_closed_positions_buffer_mins`   | None    | 指定头寸在清除前必须关闭多长时间（以分钟为单位）。建议：60分钟以确保进程完成。  |
| `purge_account_events_interval_mins`   | None    | 设置从内存中清除账户事件的频率（以分钟为单位）。建议：10-15分钟。不影响数据库记录。   |
| `purge_account_events_lookback_mins`   | None    | 指定账户事件在清除前必须发生多长时间（以分钟为单位）。建议：60分钟。                            |
| `purge_from_database`                  | False   | 如果启用，清除操作还将从后备数据库（Redis/PostgreSQL）删除数据，而不仅仅是内存。**谨慎使用**。 |

通过适当配置这些内存管理设置，您可以防止在长时间运行/高频交易会话期间内存使用无限增长，同时确保最近关闭的订单、关闭的头寸和账户事件在内存中保持可用，以供可能需要它们的任何正在进行的操作使用。

#### 队列管理

**目的**：处理订单事件的内部缓冲，以确保平滑处理并防止系统资源过载。

| 设置 | 默认值  | 描述                                                                                          |
|---------|----------|------------------------------------------------------------------------------------------------------|
| `qsize` | 100,000  | 设置内部队列缓冲区的大小，管理引擎内的数据流。                |

### 策略配置

`StrategyConfig`类概述了交易策略的配置，确保每个策略以正确的参数运行并有效管理订单。
以下概述了主要配置选项。

:::info
另请参阅`StrategyConfig` [API参考](../api_reference/config_CN.md#class-strategyconfig)以获取更多详细信息。
:::

#### 指令管理

**目的**：控制策略的执行指令处理行为。

| 设置                       | 默认值 | 描述                                                                                       |
|----------------------------|--------|-----------------------------------------------------------------------------------------------|
| `manage_gtd_expiry`        | True   | 如果为True，策略将管理GTD（Good Till Date）订单到期。如果为False，场所管理GTD到期。 |
| `manage_contingent_orders` | True   | 如果为True，策略将管理应急订单。如果为False，预期场所将管理应急订单。   |

#### 订单管理系统（OMS）

**目的**：确定策略使用的订单管理系统类型。

| 设置        | 默认值        | 描述                                                                                     |
|-------------|---------------|-----------------------------------------------------------------------------------------------|
| `oms_type`  | UNSPECIFIED   | 指定策略使用的OMS类型。选项：NETTING、HEDGING、UNSPECIFIED。 |

#### 外部订单声明

**目的**：控制策略如何与外部创建的订单交互。

| 设置                     | 默认值 | 描述                                                                                                       |
|--------------------------|--------|------------------------------------------------------------------------------------------------------------|
| `external_order_claims`  | []     | 策略声明管理的外部订单列表。格式：InstrumentId、StrategyId或标签模式。  |

#### 数据订阅

**目的**：启用策略的自动数据订阅。

| 设置           | 默认值 | 描述                                                                                  |
|----------------|--------|---------------------------------------------------------------------------------------|
| `subscribe_*`  | 变化   | 各种自动订阅选项（order_book_deltas、quote_ticks、trade_ticks等）。 |

## 账户和头寸协调

在实时交易环境中，保持Nautilus缓存状态与交易场所实际状态之间的同步是至关重要的。场所状态可能由于多种原因而发生差异：

- **外部订单活动**：在Nautilus之外创建的订单或头寸
- **系统中断**：网络问题或系统重启期间丢失的更新
- **手动干预**：直接通过经纪商界面或另一个系统进行的交易
- **部分成交**：可能以不同方式处理的部分订单成交

### 账户协调

账户协调确保Nautilus的账户状态（余额、保证金、已实现PnL）与场所报告的实际值匹配。

#### 配置

账户协调通过`LiveExecEngineConfig`配置：

```python
live_exec_config = LiveExecEngineConfig(
    reconciliation=True,                      # 启用协调
    reconciliation_lookback_mins=1440,        # 查看过去24小时
    snapshot_orders=True,                     # 包含快照中的订单
    snapshot_positions=True,                  # 包含快照中的头寸
)
```

#### 协调过程

1. **初始快照**：系统启动时，请求账户、订单和头寸的完整快照
2. **定期检查**：根据配置的间隔验证状态
3. **差异检测**：比较缓存状态与场所报告的状态
4. **自动修正**：必要时更新内部状态以匹配场所

#### 常见协调场景

**余额差异**：

- 外部存款或提款
- 费用或利息调整
- 其他系统的交易活动

**头寸差异**：

- 外部开仓或平仓
- 企业行动（股票分割、股息）
- 手动头寸调整

### 头寸协调

头寸协调确保Nautilus跟踪的头寸与场所实际头寸匹配。

#### 协调策略

**完全协调**：

- 将所有Nautilus头寸与场所头寸进行比较
- 创建或调整头寸以匹配场所状态
- 适用于具有外部活动的账户

**增量协调**：

- 仅验证最近的变更
- 更高效，但假设大部分状态是同步的
- 适用于主要由Nautilus管理的账户

#### 处理差异

当检测到头寸差异时，系统可以：

1. **记录差异**：记录差异用于审计但不自动修正
2. **修正头寸**：调整内部头寸状态以匹配场所
3. **创建外部头寸事件**：生成事件以解释差异

### 外部订单处理

Nautilus可以管理不是通过系统创建的订单，使您能够监控和控制所有交易活动。

#### 外部订单声明

策略可以声明管理外部订单：

```python
strategy_config = MyStrategyConfig(
    external_order_claims=[
        "EURUSD.FXCM",           # 声明此工具的所有外部订单
        "MyStrategy-001",        # 声明此策略ID的订单
        "hedge-*",               # 声明标签模式匹配的订单
    ]
)
```

#### 处理外部订单事件

声明外部订单后，策略接收所有相关的订单事件：

```python
def on_order_filled(self, event: OrderFilled) -> None:
    if event.client_order_id.is_external():
        # 处理外部订单成交
        self.log.info(f"外部订单成交：{event}")
        
        # 可选：根据外部活动调整策略
        self.adjust_strategy_for_external_fill(event)
```

### 最佳实践

#### 监控和警报

1. **设置协调监控**：监控协调事件并在检测到差异时设置警报
2. **定期审计**：对账户和头寸状态进行人工审查
3. **记录外部活动**：记录在Nautilus之外进行的任何手动交易

#### 配置指南

1. **新账户**：对新设置的账户启用完全协调
2. **稳定系统**：对成熟的、主要自动化的系统使用增量协调
3. **高频交易**：调整协调间隔以平衡准确性与性能

#### 故障排除

1. **持续差异**：检查是否有未报告的外部活动或配置问题
2. **性能问题**：考虑调整协调间隔或范围
3. **数据质量**：验证场所数据源的准确性和及时性

:::warning
对于持续的协调问题，考虑在系统重启前删除缓存状态或平仓账户。
:::
