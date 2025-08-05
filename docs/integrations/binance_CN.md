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

请参阅Binance [API参考](../api_reference/adapters/binance_CN.md)获取完整定义。

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