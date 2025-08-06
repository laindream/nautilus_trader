# Binance

成立于2017年，Binance是按日交易量和加密资产及加密衍生品持仓量计算的最大加密货币交易所之一。此集成支持与Binance的实时市场数据接收和订单执行。

## 安装

要安装支持Binance的NautilusTrader：

```bash
pip install --upgrade "nautilus_trader[binance]"
```

要使用所有extras从源码构建（包括Binance）：

```bash
uv sync --all-extras
```

## 示例

您可以在[这里](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/binance/)找到实时示例脚本。

## 概述

本指南假设交易者正在设置实时市场数据源和交易执行。
Binance适配器包括多个组件，可以根据用例一起使用或单独使用。

- `BinanceHttpClient`：低级HTTP API连接。
- `BinanceWebSocketClient`：低级WebSocket API连接。
- `BinanceInstrumentProvider`：工具解析和加载功能。
- `BinanceSpotDataClient`/`BinanceFuturesDataClient`：市场数据源管理器。
- `BinanceSpotExecutionClient`/`BinanceFuturesExecutionClient`：账户管理和交易执行网关。
- `BinanceLiveDataClientFactory`：Binance数据客户端工厂（由交易节点构建器使用）。
- `BinanceLiveExecClientFactory`：Binance执行客户端工厂（由交易节点构建器使用）。

:::note
大多数用户只需为实时交易节点定义配置（如下所示），不需要直接使用这些较低级别的组件。
:::

### 产品支持

| 产品类型                             | 支持 | 备注                               |
|------------------------------------------|-----------|-------------------------------------|
| 现货市场（包括Binance US）          | ✓         |                                     |
| 保证金账户（全仓和逐仓）       | ✗         | 保证金交易未实现      |
| USDT保证金期货（永续和交割）  | ✓         |                                     |
| 币本位期货                    | ✓         |                                     |

:::note
保证金交易（全仓和逐仓）目前尚未实现。
欢迎通过[GitHub issue #2631](https://github.com/nautechsystems/nautilus_trader/issues/#2631)或拉取请求贡献以添加保证金交易功能。
:::

## 数据类型

为了向交易者提供完整的API功能，该集成包括几种自定义数据类型：

- `BinanceTicker`：代表Binance 24小时ticker订阅返回的数据，包括综合价格和统计信息。
- `BinanceBar`：代表历史请求或实时订阅Binance K线的数据，包含额外的成交量指标。
- `BinanceFuturesMarkPriceUpdate`：代表Binance期货订阅的标记价格更新。

请参阅Binance [API参考](../api_reference/adapters/binance.md)获取完整定义。

## 符号体系

根据Nautilus的符号统一政策，尽可能使用原生Binance符号，包括现货资产和期货合约。由于NautilusTrader能够进行多场所+多账户交易，有必要明确区分作为现货和保证金交易对的`BTCUSDT`与`BTCUSDT`永续期货合约（Binance原生对*两者*都使用此符号）。

因此，Nautilus为所有永续符号添加后缀`-PERP`。
例如，对于Binance期货，`BTCUSDT`永续期货合约符号在Nautilus系统边界内将是`BTCUSDT-PERP`。

## 订单功能

以下表格详细说明了不同Binance账户类型支持的订单类型、执行指令和时间有效期选项：

### 订单类型

| 订单类型             | 现货 | 保证金 | USDT期货 | 币期货 | 备注                   |
|------------------------|------|--------|--------------|--------------|-------------------------|
| `MARKET`               | ✓    | ✓      | ✓            | ✓            |                         |
| `LIMIT`                | ✓    | ✓      | ✓            | ✓            |                         |
| `STOP_MARKET`          | -    | ✓      | ✓            | ✓            | 现货不支持。 |
| `STOP_LIMIT`           | ✓    | ✓      | ✓            | ✓            |                         |
| `MARKET_IF_TOUCHED`    | -    | -      | ✓            | ✓            | 仅期货。           |
| `LIMIT_IF_TOUCHED`     | ✓    | ✓      | ✓            | ✓            |                         |
| `TRAILING_STOP_MARKET` | -    | -      | ✓            | ✓            | 仅期货。           |

### 执行指令

| 指令   | 现货 | 保证金 | USDT期货 | 币期货 | 备注                                 |
|---------------|------|--------|--------------|--------------|---------------------------------------|
| `post_only`   | ✓    | ✓      | ✓            | ✓            | 请参阅下面的限制。               |
| `reduce_only` | -    | -      | ✓            | ✓            | 仅期货；在对冲模式下禁用。 |

#### Post-only限制

仅*限价*订单类型支持`post_only`。

| 订单类型               | 现货 | 保证金 | USDT期货 | 币期货 | 备注                                                      |
|--------------------------|------|--------|--------------|--------------|------------------------------------------------------------|
| `LIMIT`                  | ✓    | ✓      | ✓            | ✓            | 现货/保证金使用`LIMIT_MAKER`，期货使用`GTX` TIF。 |
| `STOP_LIMIT`             | -    | -      | ✓            | ✓            | 现货/保证金不支持。                             |

### 时间有效期

| 时间有效期 | 现货 | 保证金 | USDT期货 | 币期货 | 备注                                           |
|---------------|------|--------|--------------|--------------|-------------------------------------------------|
| `GTC`         | ✓    | ✓      | ✓            | ✓            | 撤销前有效。                             |
| `GTD`         | ✓*   | ✓*     | ✓            | ✓            | *现货/保证金转换为GTC并发出警告。 |
| `FOK`         | ✓    | ✓      | ✓            | ✓            | 全部成交或取消。                                   |
| `IOC`         | ✓    | ✓      | ✓            | ✓            | 立即成交或取消。                            |

### 高级订单功能

| 功能            | 现货 | 保证金 | USDT期货 | 币期货 | 备注                                        |
|--------------------|------|--------|--------------|--------------|----------------------------------------------|
| 订单修改 | ✓    | ✓      | ✓            | ✓            | 仅限`LIMIT`订单的价格和数量。  |
| 括号/OCO订单 | ✓    | ✓      | ✓            | ✓            | 止损/止盈的一取消其他订单。 |
| 冰山订单     | ✓    | ✓      | ✓            | ✓            | 将大订单分割为可见部分。    |

### 配置选项

以下执行客户端配置选项影响订单行为：

| 选项                       | 默认值 | 描述                                          |
|------------------------------|---------|------------------------------------------------------|
| `use_gtd`                    | `True`  | 如果为`True`，使用Binance GTD TIF；如果为`False`，将GTD重新映射到GTC进行本地管理。 |
| `use_reduce_only`            | `True`  | 如果为`True`，向交易所发送`reduce_only`指令；如果为`False`，始终发送`False`。 |
| `use_position_ids`           | `True`  | 如果为`True`，使用Binance期货对冲仓位ID；如果为`False`，启用虚拟仓位。 |
| `treat_expired_as_canceled`  | `False` | 如果为`True`，将`EXPIRED`执行类型视为`CANCELED`以进行一致处理。 |
| `futures_leverages`          | `None`  | 为期货账户设置每个符号初始杠杆的字典。 |
| `futures_margin_types`       | `None`  | 为期货账户设置每个符号保证金类型（逐仓/全仓）的字典。 |

### 追踪止损

Binance使用激活价格的概念进行追踪止损，详细信息请参考他们的[文档](https://www.binance.com/en/support/faq/what-is-a-trailing-stop-order-360042299292)。
这种方法有些不同寻常。为了让追踪止损订单在Binance上起作用，应该使用`activation_price`参数设置激活价格。

请注意，激活价格**不是**触发/STOP价格。Binance将始终根据当前市场价格和`trailing_offset`提供的回调率计算订单的触发价格。
激活价格只是订单将根据回调率开始追踪的价格。

:::warning
对于Binance追踪止损订单，您必须使用`activation_price`而不是`trigger_price`。使用`trigger_price`将导致订单被拒绝。
:::

从策略提交追踪止损订单时，您有两个选择：

1. 使用`activation_price`手动设置激活价格。
2. 将`activation_price`保留为`None`，立即激活追踪机制。

您还必须至少具有以下*其中一项*：

- 为订单设置`activation_price`。
- （或）您已订阅要提交订单的工具的报价（用于推断激活价格）。
- （或）您已订阅要提交订单的工具的交易（用于推断激活价格）。

## 配置

最常见的用例是配置实时`TradingNode`以包含Binance数据和执行客户端。为此，向客户端配置添加`BINANCE`部分：

```python
from nautilus_trader.adapters.binance import BINANCE
from nautilus_trader.live.node import TradingNode

config = TradingNodeConfig(
    ...,  # 省略
    data_clients={
        BINANCE: {
            "api_key": "YOUR_BINANCE_API_KEY",
            "api_secret": "YOUR_BINANCE_API_SECRET",
            "account_type": "spot",  # {spot, margin, usdt_future, coin_future}
            "base_url_http": None,  # 用自定义端点覆盖
            "base_url_ws": None,  # 用自定义端点覆盖
            "us": False,  # 客户端是否为Binance US
        },
    },
    exec_clients={
        BINANCE: {
            "api_key": "YOUR_BINANCE_API_KEY",
            "api_secret": "YOUR_BINANCE_API_SECRET",
            "account_type": "spot",  # {spot, margin, usdt_future, coin_future}
            "base_url_http": None,  # 用自定义端点覆盖
            "base_url_ws": None,  # 用自定义端点覆盖
            "us": False,  # 客户端是否为Binance US
        },
    },
)
```

然后，创建一个`TradingNode`并添加客户端工厂：

```python
from nautilus_trader.adapters.binance import BINANCE
from nautilus_trader.adapters.binance import BinanceLiveDataClientFactory
from nautilus_trader.adapters.binance import BinanceLiveExecClientFactory
from nautilus_trader.live.node import TradingNode

# 使用配置实例化实时交易节点
node = TradingNode(config=config)

# 向节点注册客户端工厂
node.add_data_client_factory(BINANCE, BinanceLiveDataClientFactory)
node.add_exec_client_factory(BINANCE, BinanceLiveExecClientFactory)

# 最后构建节点
node.build()
```

### API凭据

有两种选项为Binance客户端提供凭据。
要么将相应的`api_key`和`api_secret`值传递给配置对象，要么设置以下环境变量：

对于Binance现货/保证金实时客户端，您可以设置：

- `BINANCE_API_KEY`
- `BINANCE_API_SECRET`

对于Binance现货/保证金测试网客户端，您可以设置：

- `BINANCE_TESTNET_API_KEY`
- `BINANCE_TESTNET_API_SECRET`

对于Binance期货实时客户端，您可以设置：

- `BINANCE_FUTURES_API_KEY`
- `BINANCE_FUTURES_API_SECRET`

对于Binance期货测试网客户端，您可以设置：

- `BINANCE_FUTURES_TESTNET_API_KEY`
- `BINANCE_FUTURES_TESTNET_API_SECRET`

启动交易节点时，您将立即收到关于您的凭据是否有效且具有交易权限的确认。

### 账户类型

所有Binance账户类型都将支持实时交易。使用`BinanceAccountType`枚举设置`account_type`。账户类型选项包括：

- `SPOT`
- `MARGIN`（开放仓位之间共享保证金）
- `ISOLATED_MARGIN`（分配给单个仓位的保证金）
- `USDT_FUTURE`（USDT或BUSD稳定币作为抵押品）
- `COIN_FUTURE`（其他加密货币作为抵押品）

:::tip
我们建议使用环境变量来管理您的凭据。
:::

### 基础URL覆盖

可以覆盖HTTP Rest和WebSocket API的默认基础URL。这对于出于性能原因配置API集群或当Binance为您提供专门端点时很有用。

### Binance US

通过在配置中将`us`选项设置为`True`来支持Binance US账户（默认为`False`）。US账户可用的所有功能应与标准Binance的行为相同。

### 测试网

也可以配置一个或两个客户端连接到Binance测试网。
只需将`testnet`选项设置为`True`（默认为`False`）：

```python
from nautilus_trader.adapters.binance import BINANCE

config = TradingNodeConfig(
    ...,  # 省略
    data_clients={
        BINANCE: {
            "api_key": "YOUR_BINANCE_TESTNET_API_KEY",
            "api_secret": "YOUR_BINANCE_TESTNET_API_SECRET",
            "account_type": "spot",  # {spot, margin, usdt_future}
            "testnet": True,  # 客户端是否使用测试网
        },
    },
    exec_clients={
        BINANCE: {
            "api_key": "YOUR_BINANCE_TESTNET_API_KEY",
            "api_secret": "YOUR_BINANCE_TESTNET_API_SECRET",
            "account_type": "spot",  # {spot, margin, usdt_future}
            "testnet": True,  # 客户端是否使用测试网
        },
    },
)
```

### 聚合交易

Binance提供聚合交易数据端点作为交易的替代来源。
与默认交易端点相比，聚合交易数据端点可以返回`start_time`和`end_time`之间的所有tick。

要使用聚合交易和端点功能，将`use_agg_trade_ticks`选项设置为`True`（默认为`False`。）

### 解析器警告

一些Binance工具如果包含超出平台处理能力的巨大字段值，则无法解析为Nautilus对象。
在这些情况下，采用*警告并继续*的方法（该工具将不可用）。

这些警告可能会造成不必要的日志噪声，因此可以配置提供商不记录警告，如下面的客户端配置示例：

```python
from nautilus_trader.config import InstrumentProviderConfig

instrument_provider=InstrumentProviderConfig(
    load_all=True,
    log_warnings=False,
)
```

### 期货对冲模式

Binance期货对冲模式是交易者在多头和空头方向开仓以降低风险并可能从市场波动中获利的仓位模式。

要使用Binance期货对冲模式，您需要遵循以下三个步骤：

- 1. 在启动策略之前，确保在Binance上配置了对冲模式。
- 2. 在BinanceExecClientConfig中将`use_reduce_only`选项设置为`False`（默认为`True`）。

    ```python
    from nautilus_trader.adapters.binance import BINANCE

    config = TradingNodeConfig(
        ...,  # 省略
        data_clients={
            BINANCE: BinanceDataClientConfig(
                api_key=None,  # 'BINANCE_API_KEY'环境变量
                api_secret=None,  # 'BINANCE_API_SECRET'环境变量
                account_type=BinanceAccountType.USDT_FUTURE,
                base_url_http=None,  # 用自定义端点覆盖
                base_url_ws=None,  # 用自定义端点覆盖
            ),
        },
        exec_clients={
            BINANCE: BinanceExecClientConfig(
                api_key=None,  # 'BINANCE_API_KEY'环境变量
                api_secret=None,  # 'BINANCE_API_SECRET'环境变量
                account_type=BinanceAccountType.USDT_FUTURE,
                base_url_http=None,  # 用自定义端点覆盖
                base_url_ws=None,  # 用自定义端点覆盖
                use_reduce_only=False,  # 对冲模式必须禁用
            ),
        }
    )
    ```

- 3. 提交订单时，在`position_id`中使用后缀（`LONG`或`SHORT`）来指示仓位方向。

    ```python
    class EMACrossHedgeMode(Strategy):
        ...,  # 省略
        def buy(self) -> None:
            """
            用户简单买入方法（示例）。
            """
            order: MarketOrder = self.order_factory.market(
                instrument_id=self.instrument_id,
                order_side=OrderSide.BUY,
                quantity=self.instrument.make_qty(self.trade_size),
                # time_in_force=TimeInForce.FOK,
            )

            # LONG后缀被Binance适配器识别为多头仓位。
            position_id = PositionId(f"{self.instrument_id}-LONG")
            self.submit_order(order, position_id)

        def sell(self) -> None:
            """
            用户简单卖出方法（示例）。
            """
            order: MarketOrder = self.order_factory.market(
                instrument_id=self.instrument_id,
                order_side=OrderSide.SELL,
                quantity=self.instrument.make_qty(self.trade_size),
                # time_in_force=TimeInForce.FOK,
            )
            # SHORT后缀被Binance适配器识别为空头仓位。
            position_id = PositionId(f"{self.instrument_id}-SHORT")
            self.submit_order(order, position_id)
    ```

## 订单簿

订单簿可以根据订阅保持完全或部分深度。现货和期货交易所之间的WebSocket流限制不同，Nautilus将使用可能的最高流传输速率：

订单簿可以根据订阅设置保持完全或部分深度。现货和期货交易所之间的WebSocket流更新速率不同，Nautilus使用最高可用的流传输速率：

- **现货**：100ms
- **期货**：0ms（*无限制*）

每个交易者实例每个工具有一个订单簿的限制。
由于流订阅可能会有所不同，Binance数据客户端将使用最新的订单簿数据（增量或快照）订阅。

订单簿快照重建将在以下情况下触发：

- 订单簿数据的初始订阅。
- 数据websocket重新连接。

事件序列如下：

- 增量将开始缓冲。
- 请求并等待快照。
- 快照响应被解析为`OrderBookDeltas`。
- 快照增量发送到`DataEngine`。
- 迭代缓冲的增量，丢弃序列号不大于快照中最后一个增量的增量。
- 增量将停止缓冲。
- 剩余增量发送到`DataEngine`。

## Binance数据差异

现货和期货交易所之间`QuoteTick`对象的`ts_event`字段值会有所不同，前者不提供事件时间戳，因此使用`ts_init`（这意味着`ts_event`和`ts_init`相同）。

## Binance特定数据

随着时间的推移，适配器可以订阅Binance特定数据流。

:::note
K线不被视为'Binance特定'，可以以正常方式订阅。
随着更多适配器的构建，例如需要标记价格和资金费率更新，这些方法最终可能成为一级（不需要如下的自定义/通用订阅）。
:::

### BinanceFuturesMarkPriceUpdate

您可以通过从您的actor或策略中以以下方式订阅来订阅`BinanceFuturesMarkPriceUpdate`（包括资金费率信息）数据流：

```python
from nautilus_trader.adapters.binance import BinanceFuturesMarkPriceUpdate
from nautilus_trader.model import DataType
from nautilus_trader.model import ClientId

# 在您的`on_start`方法中
self.subscribe_data(
    data_type=DataType(BinanceFuturesMarkPriceUpdate, metadata={"instrument_id": self.instrument.id}),
    client_id=ClientId("BINANCE"),
)
```

这将导致您的actor/策略将这些接收到的`BinanceFuturesMarkPriceUpdate`对象传递给您的`on_data`方法。您需要检查类型，因为此方法充当所有自定义/通用数据的灵活处理程序。

```python
from nautilus_trader.core import Data

def on_data(self, data: Data):
    # 首先检查数据类型
    if isinstance(data, BinanceFuturesMarkPriceUpdate):
        # 对数据进行处理
``` 