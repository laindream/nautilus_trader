# Coinbase International

**本指南将指导您使用Coinbase International与NautilusTrader进行数据摄入和/或实时交易**。

:::warning
Coinbase International集成目前处于测试阶段。
请谨慎使用并在GitHub上报告任何问题。
:::

[Coinbase International Exchange](https://www.coinbase.com/en/international-exchange)为非美国机构客户提供加密货币永续期货和现货市场的访问。
该交易所通过提供在这些地区受限或不可用的杠杆加密衍生品服务欧洲和国际交易者。

Coinbase International带来了高标准的客户保护、强大的风险管理框架和高性能交易技术，包括：

- 实时24/7/365风险管理。
- 来自外部做市商的流动性（无自营交易）。
- 动态保证金要求和抵押品评估。
- 符合严格合规标准的清算框架。
- 资本充足的交易所以支持尾部市场事件。
- 与顶级全球监管机构合作。

:::info
更多详情请参阅[介绍Coinbase International Exchange](https://www.coinbase.com/en-au/blog/introducing-coinbase-international-exchange)博客文章。
:::

## 安装

:::note
无需额外的`coinbase_intx`安装；适配器的核心组件用Rust编写，在构建过程中自动编译和链接。
:::

## 示例

您可以在[这里](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/coinbase_intx)找到实时示例脚本。
这些示例演示了如何设置实时市场数据流和执行客户端以在Coinbase International上进行交易。

## 概述

Coinbase International交易所支持以下产品：

- 永续期货合约
- 现货加密货币

本指南假设交易者正在设置实时市场数据流和交易执行。
Coinbase International适配器包含多个组件，这些组件可以根据用例一起使用或分别使用。这些组件协同工作，连接到Coinbase International的API以获取市场数据和执行。

- `CoinbaseIntxHttpClient`: REST API连接。
- `CoinbaseIntxWebSocketClient`: WebSocket API连接。
- `CoinbaseIntxInstrumentProvider`: 工具解析和加载功能。
- `CoinbaseIntxDataClient`: 市场数据流管理器。
- `CoinbaseIntxExecutionClient`: 账户管理和交易执行网关。
- `CoinbaseIntxLiveDataClientFactory`: Coinbase International数据客户端工厂。
- `CoinbaseIntxLiveExecClientFactory`: Coinbase International执行客户端工厂。

:::note
大多数用户只需定义实时交易节点的配置（如下所述），
不一定需要直接使用上述组件。
:::

## Coinbase文档

Coinbase International为用户提供了广泛的API文档，可在[Coinbase开发者平台](https://docs.cdp.coinbase.com/intx/docs/welcome)中找到。
我们建议结合此NautilusTrader集成指南参考Coinbase International文档。

## 数据

### 工具

启动时，适配器自动从Coinbase International REST API加载所有可用工具，
并订阅`INSTRUMENTS` WebSocket频道以获取更新。这确保缓存和需要最新定义进行解析的客户端始终拥有最新的工具数据。

可用的工具类型包括：

- `CurrencyPair`（现货加密货币）
- `CryptoPerpetual`

:::note
指数产品尚未实现。
:::

以下数据类型可用：

- `OrderBookDelta`（L2按价格市场）
- `QuoteTick`（L1顶级最佳买/卖价）
- `TradeTick`
- `Bar`
- `MarkPriceUpdate`
- `IndexPriceUpdate`

:::note
历史数据请求尚未实现。
:::

### WebSocket市场数据

数据客户端连接到Coinbase International的WebSocket流以传输实时市场数据。
WebSocket客户端处理自动重连并在重连时重新订阅活动订阅。

## 执行

**适配器设计为每个执行客户端交易一个Coinbase International投资组合。**

### 选择投资组合

要识别您的可用投资组合及其ID，请运行以下脚本使用REST客户端：

```bash
python nautilus_trader/adapters/coinbase_intx/scripts/list_portfolios.py
```

这将输出投资组合详情列表，类似于以下示例：

```bash
[{'borrow_disabled': False,
  'cross_collateral_enabled': False,
  'is_default': False,
  'is_lsp': False,
  'maker_fee_rate': '-0.00008',
  'name': 'hrp5587988499',
  'portfolio_id': '3mnk59ap-1-22',  # 您的投资组合ID
  'portfolio_uuid': 'dd0958ad-0c9d-4445-a812-1870fe40d0e1',
  'pre_launch_trading_enabled': False,
  'taker_fee_rate': '0.00012',
  'trading_lock': False,
  'user_uuid': 'd4fbf7ea-9515-1068-8d60-4de91702c108'}]
```

### 配置投资组合

要指定用于交易的投资组合，请将`COINBASE_INTX_PORTFOLIO_ID`环境变量设置为
所需的`portfolio_id`。如果您使用多个执行客户端，您也可以在每个客户端的执行配置中定义`portfolio_id`。

## 功能矩阵

Coinbase International提供市价、限价和止损订单类型，支持广泛的策略。

### 订单类型

| 订单类型             | 衍生品 | 现货 | 备注                                   |
|------------------------|-------------|------|-----------------------------------------|
| `MARKET`               | ✓           | ✓    | 必须使用`IOC`或`FOK`时间有效性    |
| `LIMIT`                | ✓           | ✓    |                                         |
| `STOP_MARKET`          | ✓           | ✓    |                                         |
| `STOP_LIMIT`           | ✓           | ✓    |                                         |
| `MARKET_IF_TOUCHED`    | -           | -    | *不支持*.                        |
| `LIMIT_IF_TOUCHED`     | -           | -    | *不支持*.                        |
| `TRAILING_STOP_MARKET` | -           | -    | *不支持*.                        |

### 执行指令

| 指令   | 衍生品 | 现货 | 备注                                            |
|---------------|-------------|------|--------------------------------------------------|
| `post_only`   | ✓           | ✓    | 确保订单只提供流动性。           |
| `reduce_only` | ✓           | ✓    | 确保订单只减少现有持仓。   |

### 时间有效性选项

| 时间有效性 | 衍生品 | 现货 | 备注                                            |
|---------------|-------------|------|--------------------------------------------------|
| `GTC`         | ✓           | ✓    | 取消前有效。                              |
| `GTD`         | ✓           | ✓    | 指定日期前有效。                                  |
| `FOK`         | ✓           | ✓    | 全部成交或取消。                                    |
| `IOC`         | ✓           | ✓    | 立即成交或取消。                             |

### 高级订单功能

| 功能            | 衍生品 | 现货 | 备注                                       |
|--------------------|-------------|------|---------------------------------------------|
| 订单修改 | ✓           | ✓    | 价格和数量修改。             |
| 括号/OCO订单 | ?           | ?    | 需要进一步调查。              |
| 冰山订单     | ✓           | ✓    | 通过FIX协议可用。                 |

### 配置选项

以下执行客户端配置选项可用：

| 选项                       | 默认值 | 描述                                          |
|------------------------------|---------|------------------------------------------------------|
| `portfolio_id`               | `None`  | 指定要交易的Coinbase International投资组合。执行所需。 |
| `http_timeout_secs`          | `60`    | HTTP请求的默认超时时间（秒）。 |

### FIX Drop Copy集成

Coinbase International适配器包含FIX（金融信息交换）[drop copy](https://docs.cdp.coinbase.com/intx/docs/fix-msg-drop-copy)客户端。
这提供直接来自Coinbase撮合引擎的可靠、低延迟执行更新。

:::note
这种方法是必要的，因为执行消息不通过WebSocket流提供，并且比轮询REST API提供更快、更可靠的订单执行更新。
:::

FIX客户端：

- 建立安全的TCP/TLS连接，并在交易节点启动时自动登录。
- 处理连接监控和自动重连，如果连接中断则自动登录。
- 在交易节点停止时正确注销并关闭连接。

客户端处理几种类型的执行消息：

- 订单状态报告（已取消、已过期、已触发）。
- 成交报告（部分和完全成交）。

FIX凭据使用与REST和WebSocket客户端相同的API凭据自动管理。
除了提供有效的API凭据外，不需要额外的配置。

:::note
REST客户端处理订单提交时的`REJECTED`和`ACCEPTED`状态执行消息。
:::

### 账户和持仓管理

启动时，执行客户端请求并加载您当前的账户和执行状态，包括：

- 所有资产的可用余额。
- 未平仓订单。
- 未平仓持仓。

这为您的交易策略提供了在下达新订单之前对您账户的完整了解。

## 配置

### 策略

:::warning
Coinbase International对客户端订单ID有严格的规范。
Nautilus可以通过为客户端订单ID使用UUID4值来满足规范。
为了符合要求，在您的策略配置中设置`use_uuid_client_order_ids=True`配置选项（否则，订单提交将触发API错误）。

有关更多详情，请参阅Coinbase International [创建订单](https://docs.cdp.coinbase.com/intx/reference/createorder) REST API文档。
:::

一个示例配置可能是：

```python
from nautilus_trader.adapters.coinbase_intx import COINBASE_INTX, CoinbaseIntxDataClientConfig, CoinbaseIntxExecClientConfig
from nautilus_trader.live.node import TradingNode

config = TradingNodeConfig(
    ...,  # 省略其他配置
    data_clients={
        COINBASE_INTX: CoinbaseIntxDataClientConfig(
            instrument_provider=InstrumentProviderConfig(load_all=True),
        ),
    },
    exec_clients={
        COINBASE_INTX: CoinbaseIntxExecClientConfig(
            instrument_provider=InstrumentProviderConfig(load_all=True),
        ),
    },
)

strat_config = TOBQuoterConfig(
    use_uuid_client_order_ids=True,  # <-- Coinbase Intx所需
    instrument_id=instrument_id,
    external_order_claims=[instrument_id],
    ...,  # 省略其他配置
)
```

然后，创建一个`TradingNode`并添加客户端工厂：

```python
from nautilus_trader.adapters.coinbase_intx import COINBASE_INTX, CoinbaseIntxLiveDataClientFactory, CoinbaseIntxLiveExecClientFactory
from nautilus_trader.live.node import TradingNode

# 使用配置实例化实时交易节点
node = TradingNode(config=config)

# 向节点注册客户端工厂
node.add_data_client_factory(COINBASE_INTX, CoinbaseIntxLiveDataClientFactory)
node.add_exec_client_factory(COINBASE_INTX, CoinbaseIntxLiveExecClientFactory)

# 最后构建节点
node.build()
```

### API凭据

使用以下方法之一向客户端提供凭据。

要么传递以下配置选项的值：

- `api_key`
- `api_secret`
- `api_passphrase`
- `portfolio_id`

或者，设置以下环境变量：

- `COINBASE_INTX_API_KEY`
- `COINBASE_INTX_API_SECRET`
- `COINBASE_INTX_API_PASSPHRASE`
- `COINBASE_INTX_PORTFOLIO_ID`

:::tip
我们建议使用环境变量来管理您的凭据。
:::

启动交易节点时，您将立即收到凭据是否有效并具有交易权限的确认。

## 实现说明

- **心跳**：适配器在WebSocket和FIX连接上维护心跳以确保可靠的连接。
- **速率限制**：REST API客户端配置为限制请求到40/秒，如Coinbase International所指定。
- **优雅关闭**：适配器正确处理优雅关闭，确保在断开连接之前处理所有待处理的消息。
- **线程安全**：所有适配器组件都是线程安全的，允许它们从多个线程并发使用。
- **执行模型**：适配器可以配置为每个执行客户端使用单个Coinbase International投资组合。要交易多个投资组合，您可以创建多个执行客户端。 