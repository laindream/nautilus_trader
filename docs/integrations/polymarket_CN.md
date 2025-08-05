# Polymarket

成立于 2020 年，Polymarket 是全球最大的去中心化预测市场平台，
使交易者能够通过使用加密货币买卖二元期权合约来推测世界事件的结果。

NautilusTrader 通过 Polymarket 的中央限价订单簿 (CLOB) API 提供数据和执行的交易所集成。
该集成利用[官方 Python CLOB 客户端库](https://github.com/Polymarket/py-clob-client)
来促进与 Polymarket 平台的交互。

NautilusTrader 设计为与 Polymarket 的签名类型 0 配合工作，支持来自外部拥有账户 (EOA) 的 EIP712 签名。
此集成确保交易者可以安全高效地执行订单，使用最常见的链上签名方法，
同时 NautilusTrader 抽象了签名和准备订单的复杂性，实现无缝执行。

## 安装

使用 Polymarket 支持安装 NautilusTrader：

```bash
pip install --upgrade "nautilus_trader[polymarket]"
```

从源码构建所有额外功能（包括 Polymarket）：

```bash
uv sync --all-extras
```

## 示例

您可以在[此处](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/polymarket/)找到实时示例脚本。

## 二元期权

[二元期权](https://en.wikipedia.org/wiki/Binary_option)是一种金融异国期权合约，交易者对是否命题的结果进行押注。
如果预测正确，交易者将获得固定收益；否则，他们什么也得不到。

Polymarket 上交易的所有资产都以 **USDC.e (PoS)** 计价和结算，[请参阅下文](#usdce-pos)了解更多信息。

## Polymarket 文档

Polymarket 为不同受众提供全面的资源：

- [Polymarket Learn](https://learn.polymarket.com/)：为用户提供教育内容和指南，帮助理解平台以及如何与其互动。
- [Polymarket CLOB API](https://docs.polymarket.com/#introduction)：为与 Polymarket CLOB API 交互的开发者提供技术文档。

## 概述

本指南假设交易者正在设置实时市场数据源和交易执行。
Polymarket 集成适配器包含多个组件，可以一起使用或
根据用例单独使用。

- `PolymarketWebSocketClient`：低级 WebSocket API 连接（基于用 Rust 编写的 Nautilus `WebSocketClient` 构建）。
- `PolymarketInstrumentProvider`：`BinaryOption` 工具的工具解析和加载功能。
- `PolymarketDataClient`：市场数据源管理器。
- `PolymarketExecutionClient`：交易执行网关。
- `PolymarketLiveDataClientFactory`：Polymarket 数据客户端工厂（由交易节点构建器使用）。
- `PolymarketLiveExecClientFactory`：Polymarket 执行客户端工厂（由交易节点构建器使用）。

:::note
大多数用户只需要为实时交易节点定义配置（如下所示），
不一定需要直接使用这些较低级别的组件。
:::

## USDC.e (PoS)

**USDC.e** 是从以太坊桥接到 Polygon 网络的 USDC 版本，在 Polygon 的**权益证明 (PoS)** 链上运行。
这使得在 Polygon 上的交易更快、更具成本效益，同时保持以太坊上 USDC 的支持。

合约地址在 Polygon 区块链上是 [0x2791bca1f2de4661ed88a30c99a7a9449aa84174](https://polygonscan.com/address/0x2791bca1f2de4661ed88a30c99a7a9449aa84174)。
更多信息可以在这篇[博客](https://polygon.technology/blog/phase-one-of-native-usdc-migration-on-polygon-pos-is-underway)中找到。

## 钱包和账户

要通过 NautilusTrader 与 Polymarket 交互，您需要一个与 **Polygon** 兼容的钱包（如 MetaMask）。
该集成使用与 EIP712 兼容的外部拥有账户 (EOA) 签名类型，这意味着钱包直接由交易者/用户拥有。
这与用于 Polymarket 管理钱包（如通过其用户界面访问的钱包）的签名类型形成对比。

使用环境变量时，每个交易者实例支持一个钱包地址，
或者可以通过多个 `PolymarketExecutionClient` 实例配置多个钱包。

:::info
确保您的钱包有 **USDC.e** 资金，否则在提交订单时会遇到"余额/允许量不足"API 错误。
:::

### 为 Polymarket 合约设置允许量

在您开始交易之前，您需要确保您的钱包已为 Polymarket 的智能合约设置了允许量。
您可以通过运行位于 `/adapters/polymarket/scripts/set_allowances.py` 的提供脚本来做到这一点。

此脚本改编自 @poly-rodr 创建的 [gist](https://gist.github.com/poly-rodr/44313920481de58d5a3f6d1f8226bd5e)。

:::note
对于您打算在 Polymarket 上用于交易的每个 EOA 钱包，您只需要运行此脚本**一次**。
:::

此脚本自动化了为 Polymarket 合约批准必要允许量的过程。
它为 USDC 代币和条件代币框架 (CTF) 合约设置批准，以允许
Polymarket CLOB 交易所与您的资金交互。

在运行脚本之前，请确保满足以下先决条件：

- 安装 web3 Python 包：`pip install --upgrade web3==5.28`
- 拥有一个与 **Polygon** 兼容的钱包，其中有一些 MATIC（用于 gas 费用）。
- 在您的 shell 中设置以下环境变量：
  - `POLYGON_PRIVATE_KEY`：您的 **Polygon** 兼容钱包的私钥。
  - `POLYGON_PUBLIC_KEY`：您的 **Polygon** 兼容钱包的公钥。

一旦您准备好这些，脚本将：

- 为 Polymarket USDC 代币合约批准最大可能数量的 USDC（使用 `MAX_INT` 值）。
- 为 CTF 合约设置批准，允许其与您的账户交互以进行交易。

:::note
您也可以在脚本中调整批准金额而不是使用 `MAX_INT`，
金额以 **USDC.e** 的*小数单位*指定，尽管这尚未经过测试。
:::

在运行脚本之前，请确保您的私钥和公钥正确存储在环境变量中。
以下是如何在终端会话中设置变量的示例：

```bash
export POLYGON_PRIVATE_KEY="YOUR_PRIVATE_KEY"
export POLYGON_PUBLIC_KEY="YOUR_PUBLIC_KEY"
```

使用以下命令运行脚本：

```bash
python nautilus_trader/adapters/polymarket/scripts/set_allowances.py
```

### 脚本细分

脚本执行以下操作：

- 通过 RPC URL (<https://polygon-rpc.com/>) 连接到 Polygon 网络。
- 签名并发送交易以批准 Polymarket 合约的最大 USDC 允许量。
- 为 CTF 合约设置批准以代表您管理条件代币。
- 对特定地址（如 Polymarket CLOB 交易所和 Neg Risk 适配器）重复批准过程。

这允许 Polymarket 在执行交易时与您的资金交互，并确保与 CLOB 交易所的顺畅集成。

## API 密钥

要使用 EOA 钱包与 Polymarket 交易，请按照以下步骤生成您的 API 密钥：

1. 确保设置了以下环境变量：

- `POLYMARKET_PK`：您用于签署交易的私钥。
- `POLYMARKET_FUNDER`：在 **Polygon** 网络上用于为 Polymarket 交易提供资金的钱包地址（公钥）。

1. 使用以下命令运行脚本：

   ```bash
   python nautilus_trader/adapters/polymarket/scripts/create_api_key.py
   ```

脚本将生成并打印 API 凭据，您应该将其保存到以下环境变量中：

- `POLYMARKET_API_KEY`
- `POLYMARKET_API_SECRET`
- `POLYMARKET_PASSPHRASE`

这些然后可以用于 Polymarket 客户端配置：

- `PolymarketDataClientConfig`
- `PolymarketExecClientConfig`

## 配置

当设置 NautilusTrader 与 Polymarket 配合工作时，正确配置必要的参数（特别是私钥）至关重要。

**关键参数**

- `private_key`：这是您的外部 EOA 钱包的私钥（*不是*通过其 GUI 访问的 Polymarket 钱包）。此私钥允许系统代表与 Polymarket 交互的外部账户签署和发送交易。如果在配置中未明确提供，它将自动获取 `POLYMARKET_PK` 环境变量。
- `funder`：这指的是用于为交易提供资金的 **USDC.e** 钱包地址。如果未提供，将获取 `POLYMARKET_FUNDER` 环境变量。
- API 凭据：您需要提供以下 API 凭据以与 Polymarket CLOB 交互：
  - `api_key`：如果未提供，将获取 `POLYMARKET_API_KEY` 环境变量。
  - `api_secret`：如果未提供，将获取 `POLYMARKET_API_SECRET` 环境变量。
  - `passphrase`：如果未提供，将获取 `POLYMARKET_PASSPHRASE` 环境变量。

:::tip
我们建议使用环境变量来管理您的凭据。
:::

## 功能矩阵

Polymarket 作为预测市场运营，与传统交易所相比，订单复杂性有限。

### 订单类型

| 订单类型               | 二元期权 | 备注                        |
|------------------------|----------|-----------------------------|
| `MARKET`               | ✓        | 作为可执行限价订单执行。    |
| `LIMIT`                | ✓        |                             |
| `STOP_MARKET`          | -        | *不支持*。                  |
| `STOP_LIMIT`           | -        | *不支持*。                  |
| `MARKET_IF_TOUCHED`    | -        | *不支持*。                  |
| `LIMIT_IF_TOUCHED`     | -        | *不支持*。                  |
| `TRAILING_STOP_MARKET` | -        | *不支持*。                  |

### 执行指令

| 指令          | 二元期权 | 备注                          |
|---------------|----------|-------------------------------|
| `post_only`   | -        | *不支持*。                    |
| `reduce_only` | -        | *不支持*。                    |

### 有效期选项

| 有效期        | 二元期权 | 备注                          |
|---------------|----------|-------------------------------|
| `GTC`         | ✓        | 撤销前有效。                  |
| `GTD`         | ✓        | 指定日期前有效。              |
| `FOK`         | ✓        | 全部成交或取消。              |
| `IOC`         | ✓        | 立即成交或取消（映射到 FAK）。|

### 高级订单功能

| 功能              | 二元期权 | 备注                         |
|-------------------|----------|------------------------------|
| 订单修改          | -        | 仅取消功能。                 |
| 括号/OCO 订单     | -        | *不支持*。                   |
| 冰山订单          | -        | *不支持*。                   |

### 配置选项

以下执行客户端配置选项可用：

| 选项                                 | 默认值  | 描述                         |
|--------------------------------------|---------|------------------------------|
| `signature_type`                     | `0`     | Polymarket 签名类型（EOA）。 |
| `funder`                             | `None`  | 用于为 USDC 交易提供资金的钱包地址。|
| `generate_order_history_from_trades` | `False` | 从交易历史生成订单报告的实验性功能（*不推荐*）。|
| `log_raw_ws_messages`                | `False` | 如果为 `True`，记录原始 WebSocket 消息（来自漂亮 JSON 格式化的性能损失）。|

## 交易

Polymarket 上的交易可以有以下状态：

- `MATCHED`：交易已匹配并由操作员发送到执行器服务，执行器服务将交易作为交易提交给交易所合约。
- `MINED`：观察到交易被挖入链中，未建立最终性阈值。
- `CONFIRMED`：交易已实现强概率最终性并且成功。
- `RETRYING`：交易失败（回滚或重组）并正由操作员重试/重新提交。
- `FAILED`：交易失败且不再重试。

一旦交易初始匹配，后续交易状态更新将通过 WebSocket 接收。
NautilusTrader 在 `OrderFilled` 事件的 `info` 字段中记录初始交易详细信息，
额外的交易事件作为 JSON 存储在缓存中的自定义键下以保留此信息。

## 对账

Polymarket API 在查询时返回所有**活跃**（开放）订单或通过 Polymarket 订单 ID（`venue_order_id`）查询的特定订单。Polymarket 的执行对账程序如下：

- 为 Polymarket 报告的所有具有活跃（开放）订单的工具生成订单报告。
- 从 Polymarket 报告的合约余额生成位置报告，*每个缓存中可用的工具*。
- 将这些报告与 Nautilus 执行状态进行比较。
- 生成缺失的订单以使 Nautilus 执行状态与 Polymarket 报告的位置保持一致。

**注意**：Polymarket 不直接提供不再活跃的订单的数据。

:::warning
可选的执行客户端配置 `generate_order_history_from_trades` 目前正在开发中。
目前不建议用于生产环境。
:::

## WebSockets

`PolymarketWebSocketClient` 基于用 Rust 编写的高性能 Nautilus `WebSocketClient` 基类构建。

### 数据

主数据 WebSocket 处理在初始连接序列期间接收的所有 `market` 频道订阅，直到 `ws_connection_delay_secs`。对于任何其他订阅，为每个新工具（资产）创建一个新的 `PolymarketWebSocketClient`。

### 执行

主执行 WebSocket 基于初始连接序列期间缓存中可用的 Polymarket 工具管理所有 `user` 频道订阅。当为其他工具发出交易命令时，为每个新工具（资产）创建一个单独的 `PolymarketWebSocketClient`。

:::note
Polymarket 不支持一旦订阅就取消订阅频道流。
:::

## 限制和注意事项

目前已知的以下限制和注意事项：

- 通过 Polymarket Python 客户端进行订单签名很慢，大约需要一秒钟。
- 不支持只发布订单。
- 不支持只减少订单。
