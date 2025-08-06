# Bybit

:::info
我们目前正在完善这个集成指南。
:::

Bybit成立于2018年，是按日交易量以及加密资产和加密衍生品持仓量计算的最大加密货币交易所之一。此集成支持与Bybit的实时市场数据接收和订单执行。

## 安装

要安装支持Bybit的NautilusTrader：

```bash
pip install --upgrade "nautilus_trader[bybit]"
```

要使用所有extras从源码构建（包括Bybit）：

```bash
uv sync --all-extras
```

## 示例

您可以在[这里](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/bybit/)找到实时示例脚本。

## 概述

本指南假设交易员正在设置实时市场数据源和交易执行。
Bybit适配器包含多个组件，可以根据用例一起使用或单独使用。

- `BybitHttpClient`：低级HTTP API连接。
- `BybitWebSocketClient`：低级WebSocket API连接。
- `BybitInstrumentProvider`：工具解析和加载功能。
- `BybitDataClient`：市场数据源管理器。
- `BybitExecutionClient`：账户管理和交易执行网关。
- `BybitLiveDataClientFactory`：Bybit数据客户端工厂（由交易节点构建器使用）。
- `BybitLiveExecClientFactory`：Bybit执行客户端工厂（由交易节点构建器使用）。

:::note
大多数用户只需为实时交易节点定义配置（如下所示），不需要直接与这些较低级别的组件工作。
:::

## Bybit文档

Bybit为用户提供了广泛的文档，可以在[Bybit帮助中心](https://www.bybit.com/en/help-center)找到。
建议您将Bybit文档与此NautilusTrader集成指南结合参考。

## 产品

产品是相关工具类型组的总称。

:::note
在Bybit v5 API中，产品也称为`category`。
:::

Bybit支持以下产品类型：

- 现货加密货币
- 永续合约
- 永续反向合约
- 期货合约
- 期货反向合约

期权合约目前不受支持（将在未来版本中实现）

## 符号体系

为了区分Bybit上的不同产品类型，Nautilus为符号使用特定的产品类别后缀：

- `-SPOT`：现货加密货币
- `-LINEAR`：永续和期货合约
- `-INVERSE`：反向永续和反向期货合约
- `-OPTION`：期权合约（目前不支持）

这些后缀必须附加到Bybit原始符号字符串以识别工具ID的特定产品类型。例如：

- 以太坊/泰达币现货货币对用`-SPOT`标识，如`ETHUSDT-SPOT`。
- BTCUSDT永续期货合约用`-LINEAR`标识，如`BTCUSDT-LINEAR`。
- BTCUSD反向永续期货合约用`-INVERSE`标识，如`BTCUSD-INVERSE`。

## 能力矩阵

Bybit提供了触发类型的灵活组合，支持更广泛的Nautilus订单。
下面列出的所有订单类型都可以用作*入场*或*出场*，除了追踪止损（使用与仓位相关的API）。

### 订单类型

| 订单类型                | 现货 | 线性  | 反向  | 注释                     |
|------------------------|------|--------|---------|--------------------------|
| `MARKET`               | ✓    | ✓      | ✓       |                          |
| `LIMIT`                | ✓    | ✓      | ✓       |                          |
| `STOP_MARKET`          | ✓    | ✓      | ✓       |                          |
| `STOP_LIMIT`           | ✓    | ✓      | ✓       |                          |
| `MARKET_IF_TOUCHED`    | ✓    | ✓      | ✓       |                          |
| `LIMIT_IF_TOUCHED`     | ✓    | ✓      | ✓       |                          |
| `TRAILING_STOP_MARKET` | -    | ✓      | ✓       | 现货不支持。            |

### 执行指令

| 指令          | 现货 | 线性  | 反向  | 注释                               |
|---------------|------|--------|---------|-----------------------------------|
| `post_only`   | ✓    | ✓      | ✓       | 仅支持`LIMIT`订单。              |
| `reduce_only` | -    | ✓      | ✓       | 现货产品不支持。                 |

### 时间有效期选项

| 时间有效期    | 现货 | 线性  | 反向  | 注释                        |
|---------------|------|--------|---------|------------------------------|
| `GTC`         | ✓    | ✓      | ✓       | 撤销前有效。                |
| `GTD`         | -    | -      | -       | *不支持*。                   |
| `FOK`         | ✓    | ✓      | ✓       | 完全成交或取消。            |
| `IOC`         | ✓    | ✓      | ✓       | 立即成交或取消。            |

### 高级订单功能

| 功能               | 现货 | 线性  | 反向  | 注释                                  |
|--------------------|------|--------|---------|----------------------------------------|
| 订单修改           | ✓    | ✓      | ✓       | 价格和数量修改。                     |
| 括号/OCO订单       | ✓    | ✓      | ✓       | 仅限UI；API用户需手动实现。          |
| 冰山订单           | ✓    | ✓      | ✓       | 每账户最多10个，每符号1个。          |

### 配置选项

以下执行客户端配置选项影响订单行为：

| 选项                         | 默认值   | 描述                                          |
|------------------------------|---------|------------------------------------------------------|
| `use_gtd`                    | `False` | 不支持GTD；订单被重新映射为GTC以进行本地管理。      |
| `use_ws_trade_api`           | `False` | 如果为`True`，使用WebSocket进行订单请求而不是HTTP。 |
| `use_http_batch_api`         | `False` | 如果为`True`，当启用WebSocket交易时使用HTTP批量API。|
| `futures_leverages`          | `None`  | 用于设置期货符号杠杆的字典。                       |
| `position_mode`              | `None`  | 用于设置USDT永续和反向期货仓位模式的字典。         |
| `margin_mode`                | `None`  | 设置账户的保证金模式。                           |

### 产品特定限制

以下限制适用于SPOT产品，因为场所端不跟踪仓位：

- *不支持*`reduce_only`订单。
- *不支持*追踪止损订单。

### 追踪止损

Bybit上的追踪止损在场所端没有客户端订单ID（虽然有`venue_order_id`）。
这是因为追踪止损与工具的净仓位相关联。
在Bybit上使用追踪止损时请考虑以下几点：

- `reduce_only`指令可用
- 当与追踪止损相关的仓位关闭时，追踪止损在场所端自动"停用"（关闭）
- 您无法查询尚未开放的追踪止损订单（在那之前`venue_order_id`是未知的）
- 您可以在GUI中手动调整触发价格，这将更新Nautilus订单

## 配置

必须在配置中指定每个客户端的产品类型。

### 数据客户端

如果未指定产品类型，则将加载并提供所有产品类型。

### 执行客户端

由于Nautilus不支持"统一"账户，账户类型必须是现金**或**保证金。
这意味着您不能将SPOT与任何其他衍生品产品类型一起指定。

- `CASH`账户类型将用于SPOT产品。
- `MARGIN`账户类型将用于所有其他衍生品产品。

最常见的用例是配置实时`TradingNode`以包含Bybit数据和执行客户端。为此，在您的客户端配置中添加`BYBIT`部分：

```python
from nautilus_trader.adapters.bybit import BYBIT
from nautilus_trader.adapters.bybit import BybitProductType
from nautilus_trader.live.node import TradingNode

config = TradingNodeConfig(
    ...,  # 省略
    data_clients={
        BYBIT: {
            "api_key": "YOUR_BYBIT_API_KEY",
            "api_secret": "YOUR_BYBIT_API_SECRET",
            "base_url_http": None,  # 用自定义端点覆盖
            "product_types": [BybitProductType.LINEAR]
            "testnet": False,
        },
    },
    exec_clients={
        BYBIT: {
            "api_key": "YOUR_BYBIT_API_KEY",
            "api_secret": "YOUR_BYBIT_API_SECRET",
            "base_url_http": None,  # 用自定义端点覆盖
            "product_types": [BybitProductType.LINEAR]
            "testnet": False,
        },
    },
)
```

然后，创建一个`TradingNode`并添加客户端工厂：

```python
from nautilus_trader.adapters.bybit import BYBIT
from nautilus_trader.adapters.bybit import BybitLiveDataClientFactory
from nautilus_trader.adapters.bybit import BybitLiveExecClientFactory
from nautilus_trader.live.node import TradingNode

# 使用配置实例化实时交易节点
node = TradingNode(config=config)

# 向节点注册客户端工厂
node.add_data_client_factory(BYBIT, BybitLiveDataClientFactory)
node.add_exec_client_factory(BYBIT, BybitLiveExecClientFactory)

# 最后构建节点
node.build()
```

### API凭据

有两种选项为Bybit客户端提供凭据。
要么将相应的`api_key`和`api_secret`值传递给配置对象，要么设置以下环境变量：

对于Bybit实时客户端，您可以设置：

- `BYBIT_API_KEY`
- `BYBIT_API_SECRET`

对于Bybit演示客户端，您可以设置：

- `BYBIT_DEMO_API_KEY`
- `BYBIT_DEMO_API_SECRET`

对于Bybit测试网客户端，您可以设置：

- `BYBIT_TESTNET_API_KEY`
- `BYBIT_TESTNET_API_SECRET`

:::tip
我们建议使用环境变量来管理您的凭据。
:::

当启动交易节点时，您将立即收到关于您的凭据是否有效且具有交易权限的确认。
