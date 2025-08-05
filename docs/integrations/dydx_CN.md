# dYdX

:::info
我们正在编写此集成指南。
:::

dYdX 是全球最大的去中心化加密货币交易所之一，以加密衍生品的日交易量衡量。dYdX 在以太坊区块链上运行智能合约，允许用户无需中介进行交易。此集成支持 dYdX v4 的实时市场数据获取和订单执行，这是该协议首个完全去中心化且无中心化组件的版本。

## 安装

使用 dYdX 支持安装 NautilusTrader：

```bash
pip install --upgrade "nautilus_trader[dydx]"
```

从源码构建所有额外功能（包括 dYdX）：

```bash
uv sync --all-extras
```

## 示例

您可以在[此处](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/dydx/)找到实时示例脚本。

## 概述

本指南假设交易者正在设置实时市场数据源和交易执行。dYdX 适配器包含多个组件，可以一起使用或根据用例单独使用。

- `DYDXHttpClient`：低级 HTTP API 连接。
- `DYDXWebSocketClient`：低级 WebSocket API 连接。
- `DYDXAccountGRPCAPI`：低级 gRPC API 连接，用于账户更新。
- `DYDXInstrumentProvider`：工具解析和加载功能。
- `DYDXDataClient`：市场数据源管理器。
- `DYDXExecutionClient`：账户管理和交易执行网关。
- `DYDXLiveDataClientFactory`：dYdX 数据客户端工厂（由交易节点构建器使用）。
- `DYDXLiveExecClientFactory`：dYdX 执行客户端工厂（由交易节点构建器使用）。

:::note
大多数用户只需要为实时交易节点定义配置（如下所示），
不一定需要直接使用这些较低级别的组件。
:::

:::warning 首次账户激活
dYdX v4 交易账户（子账户 0）**仅在**钱包首次存款或交易后创建。
在此之前，每个 gRPC/索引器查询都返回 `NOT_FOUND`，因此 `DYDXExecutionClient.connect()` 失败。

**行动 →** 在启动实时 `TradingNode` 之前，从同一钱包**在同一网络**（主网/测试网）发送任何正数金额的 USDC（≥ 1 wei）或其他支持的抵押品。
一旦交易完成（几个区块），重启节点；客户端将正常连接。
:::

## 故障排除

### `StatusCode.NOT_FOUND` — account … /0 not found

**原因** *钱包/子账户从未被注资，因此在链上尚不存在。*

**修复**

1. 在正确的网络上向子账户 0 存入任何正数金额的 USDC。
2. 等待最终确认（主网约 30 秒，测试网时间更长）。
3. 重启 `TradingNode`；连接现在应该成功。

:::tip
在无人值守的部署中，将 `connect()` 调用包装在指数退避循环中，以便客户端重试直到存款出现。
:::

## 符号体系

dYdX 上仅提供永续合约。为了与其他适配器保持一致，以及为 dYdX 上可能提供的其他产品做好准备，NautilusTrader 为所有可用的永续符号附加 `-PERP`。例如，比特币/USD-C 永续期货合约标识为 `BTC-USD-PERP`。所有市场的报价货币都是 USD-C。因此，dYdX 将其缩写为 USD。

## 短期和长期订单

dYdX 区分短期订单和长期订单（或有状态订单）。
短期订单旨在立即下达，属于接收订单的同一区块。
这些订单在内存中保留最多 20 个区块，只有其成交金额和到期区块高度被提交到状态。
短期订单主要用于高吞吐量的做市商或市价订单。

默认情况下，所有订单都作为短期订单发送。要构造长期订单，您可以这样为订单附加标签：

```python
from nautilus_trader.adapters.dydx import DYDXOrderTags

order: LimitOrder = self.order_factory.limit(
    instrument_id=self.instrument_id,
    order_side=OrderSide.BUY,
    quantity=self.instrument.make_qty(self.trade_size),
    price=self.instrument.make_price(price),
    time_in_force=TimeInForce.GTD,
    expire_time=self.clock.utc_now() + pd.Timedelta(minutes=10),
    post_only=True,
    emulation_trigger=self.emulation_trigger,
    tags=[DYDXOrderTags(is_short_term_order=False).value],
)
```

要指定订单活跃的区块数：

```python
from nautilus_trader.adapters.dydx import DYDXOrderTags

order: LimitOrder = self.order_factory.limit(
    instrument_id=self.instrument_id,
    order_side=OrderSide.BUY,
    quantity=self.instrument.make_qty(self.trade_size),
    price=self.instrument.make_price(price),
    time_in_force=TimeInForce.GTD,
    expire_time=self.clock.utc_now() + pd.Timedelta(seconds=5),
    post_only=True,
    emulation_trigger=self.emulation_trigger,
    tags=[DYDXOrderTags(is_short_term_order=True, num_blocks_open=5).value],
)
```

## 市价订单

市价订单需要指定价格以进行价格滑点保护并使用隐藏订单。
通过为市价订单设置价格，您可以限制潜在的价格滑点。例如，
如果您为市价买单设置 100 美元的价格，订单仅在市价为 100 美元或以下时执行。如果市价高于 100 美元，订单将不会执行。

一些交易所，包括 dYdX，支持隐藏订单。隐藏订单是对其他市场参与者不可见的订单，但仍然可以执行。通过为市价订单设置价格，您可以创建一个隐藏订单，只有在市价达到指定价格时才会执行。

如果未指定市价，则使用默认值 0。

在创建市价订单时指定价格：

```python
order = self.order_factory.market(
    instrument_id=self.instrument_id,
    order_side=OrderSide.BUY,
    quantity=self.instrument.make_qty(self.trade_size),
    time_in_force=TimeInForce.IOC,
    tags=[DYDXOrderTags(is_short_term_order=True, market_order_price=Price.from_str("10_000")).value],
)
```

## 止损限价和止损市价订单

可以提交止损限价和止损市价条件订单。dYdX 仅对条件订单支持长期订单。

## 功能矩阵

dYdX 支持永续期货交易，具有全面的订单类型和执行功能。

### 订单类型

| 订单类型               | 永续合约 | 备注                      |
|------------------------|----------|---------------------------|
| `MARKET`               | ✓        | 需要价格进行滑点保护。     |
| `LIMIT`                | ✓        |                           |
| `STOP_MARKET`          | ✓        | 仅限长期订单。             |
| `STOP_LIMIT`           | ✓        | 仅限长期订单。             |
| `MARKET_IF_TOUCHED`    | -        | *不支持*。                |
| `LIMIT_IF_TOUCHED`     | -        | *不支持*。                |
| `TRAILING_STOP_MARKET` | -        | *不支持*。                |

### 执行指令

| 指令          | 永续合约 | 备注                       |
|---------------|----------|----------------------------|
| `post_only`   | ✓        | 所有订单类型均支持。        |
| `reduce_only` | ✓        | 所有订单类型均支持。        |

### 有效期选项

| 有效期       | 永续合约 | 备注                |
|--------------|----------|---------------------|
| `GTC`        | ✓        | 撤销前有效。        |
| `GTD`        | ✓        | 指定日期前有效。    |
| `FOK`        | ✓        | 全部成交或取消。    |
| `IOC`        | ✓        | 立即成交或取消。    |

### 高级订单功能

| 功能              | 永续合约 | 备注                                   |
|-------------------|----------|----------------------------------------|
| 订单修改          | ✓        | 仅限短期订单；取消-替换方式。           |
| 括号/OCO 订单     | -        | *不支持*。                             |
| 冰山订单          | -        | *不支持*。                             |

### 配置选项

以下执行客户端配置选项可用：

| 选项             | 默认值  | 描述                                                         |
|------------------|---------|--------------------------------------------------------------|
| `subaccount`     | `0`     | 子账户号码（交易所默认创建子账户 0）。                        |
| `wallet_address` | `None`  | 账户的 dYdX 钱包地址。                                       |
| `mnemonic`       | `None`  | 用于生成订单签名私钥的助记词。                               |
| `is_testnet`     | `False` | 如果为 `True`，连接到测试网；如果为 `False`，连接到主网。     |

### 订单分类

dYdX 将订单分类为**短期**或**长期**订单：

- **短期订单**：所有订单的默认设置；用于高频交易和市价订单。
- **长期订单**：条件订单所需；使用 `DYDXOrderTags` 指定。

## 配置

必须在配置中指定每个客户端的产品类型。

### 执行客户端

账户类型必须是保证金账户才能交易永续期货合约。

最常见的用例是配置实时 `TradingNode` 以包含 dYdX 数据和执行客户端。要实现这一点，在您的客户端配置中添加 `DYDX` 部分：

```python
from nautilus_trader.live.node import TradingNode

config = TradingNodeConfig(
    ...,  # 省略
    data_clients={
        "DYDX": {
            "wallet_address": "YOUR_DYDX_WALLET_ADDRESS",
            "is_testnet": False,
        },
    },
    exec_clients={
        "DYDX": {
            "wallet_address": "YOUR_DYDX_WALLET_ADDRESS",
            "subaccount": "YOUR_DYDX_SUBACCOUNT_NUMBER"
            "mnemonic": "YOUR_MNEMONIC",
            "is_testnet": False,
        },
    },
)
```

然后，创建一个 `TradingNode` 并添加客户端工厂：

```python
from nautilus_trader.adapters.dydx import DYDXLiveDataClientFactory
from nautilus_trader.adapters.dydx import DYDXLiveExecClientFactory
from nautilus_trader.live.node import TradingNode

# 使用配置实例化实时交易节点
node = TradingNode(config=config)

# 向节点注册客户端工厂
node.add_data_client_factory("DYDX", DYDXLiveDataClientFactory)
node.add_exec_client_factory("DYDX", DYDXLiveExecClientFactory)

# 最后构建节点
node.build()
```

### API 凭据

有两种方式为 dYdX 客户端提供凭据。
要么将相应的 `wallet_address` 和 `mnemonic` 值传递给配置对象，要么设置以下环境变量：

对于 dYdX 实时客户端，您可以设置：

- `DYDX_WALLET_ADDRESS`
- `DYDX_MNEMONIC`

对于 dYdX 测试网客户端，您可以设置：

- `DYDX_TESTNET_WALLET_ADDRESS`
- `DYDX_TESTNET_MNEMONIC`

:::tip
我们建议使用环境变量来管理您的凭据。
:::

数据客户端使用钱包地址来确定交易费用。交易费用仅在回测期间使用。

### 测试网

也可以配置一个或两个客户端连接到 dYdX 测试网。
只需将 `is_testnet` 选项设置为 `True`（默认为 `False`）：

```python
config = TradingNodeConfig(
    ...,  # 省略
    data_clients={
        "DYDX": {
            "wallet_address": "YOUR_DYDX_WALLET_ADDRESS",
            "is_testnet": True,
        },
    },
    exec_clients={
        "DYDX": {
            "wallet_address": "YOUR_DYDX_WALLET_ADDRESS",
            "subaccount": "YOUR_DYDX_SUBACCOUNT_NUMBER"
            "mnemonic": "YOUR_MNEMONIC",
            "is_testnet": True,
        },
    },
)
```

### 解析器警告

如果某些 dYdX 工具包含平台无法处理的巨大字段值，它们将无法解析为 Nautilus 对象。
在这些情况下，采用*警告并继续*的方法（该工具将不可用）。

## 订单簿

订单簿可以根据订阅维护完整深度或顶级报价。交易所不提供报价，但适配器订阅订单簿增量，并在顶级价格或数量发生变化时向 `DataEngine` 发送新报价。 