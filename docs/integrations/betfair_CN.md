# Betfair

Betfair成立于2000年，运营着世界上最大的在线博彩交易所，总部位于伦敦，在全球设有卫星办事处。

NautilusTrader提供了一个适配器，用于与Betfair REST API和Exchange Streaming API集成。

## 安装

通过pip安装支持Betfair的NautilusTrader：

```bash
pip install --upgrade "nautilus_trader[betfair]"
```

从源码构建并包含Betfair额外功能：

```bash
uv sync --all-extras
```

## 示例

您可以在[这里](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/betfair/)找到实时示例脚本。

## Betfair文档

有关API详细信息和故障排除，请参阅官方[Betfair开发者文档](https://developer.betfair.com/en/get-started/)。

## 应用密钥

Betfair需要应用密钥来验证API请求。注册并为您的账户充值后，使用[API-NG开发者应用密钥工具](https://apps.betfair.com/visualisers/api-ng-account-operations/)获取您的密钥。

:::info
另请参阅[Betfair入门 - 应用密钥](https://betfair-developer-docs.atlassian.net/wiki/spaces/1smk3cen4v3lu3yomq5qye0ni/pages/2687105/Application+Keys)指南。
:::

## API凭据

通过环境变量或客户端配置提供您的Betfair凭据：

```bash
export BETFAIR_USERNAME=<your_username>
export BETFAIR_PASSWORD=<your_password>
export BETFAIR_APP_KEY=<your_app_key>
export BETFAIR_CERTS_DIR=<path_to_certificate_dir>
```

:::tip
我们建议使用环境变量来管理您的凭据。
:::

## 概述

Betfair适配器提供三个主要组件：

- `BetfairInstrumentProvider`：加载Betfair市场并将其转换为Nautilus工具。
- `BetfairDataClient`：从Exchange Streaming API流式传输实时市场数据。
- `BetfairExecutionClient`：通过REST API提交订单（投注）并跟踪执行状态。

## 能力矩阵

Betfair作为博彩交易所运营，与传统金融交易所相比具有独特特征：

### 订单类型

| 订单类型                | Betfair | 注释                                |
|------------------------|---------|-------------------------------------|
| `MARKET`               | -       | 不适用于博彩交易所。                |
| `LIMIT`                | ✓       | 以特定赔率下注的订单。              |
| `STOP_MARKET`          | -       | *不支持*。                          |
| `STOP_LIMIT`           | -       | *不支持*。                          |
| `MARKET_IF_TOUCHED`    | -       | *不支持*。                          |
| `LIMIT_IF_TOUCHED`     | -       | *不支持*。                          |
| `TRAILING_STOP_MARKET` | -       | *不支持*。                          |

### 执行指令

| 指令          | Betfair | 注释                                   |
|---------------|---------|-----------------------------------------|
| `post_only`   | -       | 不适用于博彩交易所。                   |
| `reduce_only` | -       | 不适用于博彩交易所。                   |

### 时间有效期选项

| 时间有效期    | Betfair | 注释                                   |
|---------------|---------|-----------------------------------------|
| `GTC`         | -       | 博彩交易所使用不同模型。               |
| `GTD`         | -       | 博彩交易所使用不同模型。               |
| `FOK`         | -       | 博彩交易所使用不同模型。               |
| `IOC`         | -       | 博彩交易所使用不同模型。               |

### 高级订单功能

| 功能               | Betfair | 注释                                    |
|--------------------|---------|------------------------------------------|
| 订单修改           | ✓       | 仅限于不改变风险敞口的字段。             |
| 括号/OCO订单       | -       | *不支持*。                               |
| 冰山订单           | -       | *不支持*。                               |

### 配置选项

以下执行客户端配置选项影响订单行为：

| 选项                         | 默认值   | 描述                                          |
|------------------------------|---------|------------------------------------------------------|
| `calculate_account_state`    | `True`  | 如果为`True`，从事件计算账户状态。                |
| `request_account_state_secs` | `300`   | 账户状态检查间隔（秒）（0表示禁用）。             |
| `reconcile_market_ids_only`  | `False` | 如果为`True`，仅协调配置的市场ID的订单。          |
| `ignore_external_orders`     | `False` | 如果为`True`，静默忽略缓存中未找到的订单。        |

## 配置

以下是显示如何使用Betfair客户端配置实时`TradingNode`的最小示例：

```python
from nautilus_trader.adapters.betfair import BETFAIR
from nautilus_trader.adapters.betfair import BetfairLiveDataClientFactory
from nautilus_trader.adapters.betfair import BetfairLiveExecClientFactory
from nautilus_trader.config import TradingNodeConfig
from nautilus_trader.live.node import TradingNode

# 配置Betfair数据和执行客户端（使用AUD账户货币）
config = TradingNodeConfig(
    data_clients={BETFAIR: {"account_currency": "AUD"}},
    exec_clients={BETFAIR: {"account_currency": "AUD"}},
)

# 使用Betfair适配器工厂构建TradingNode
node = TradingNode(config)
node.add_data_client_factory(BETFAIR, BetfairLiveDataClientFactory)
node.add_exec_client_factory(BETFAIR, BetfairLiveExecClientFactory)
node.build()
```
