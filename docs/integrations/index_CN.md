# 集成

NautilusTrader使用模块化的*适配器*连接到交易场所和数据提供商，将原始API转换为统一接口和标准化域模型。

目前支持以下集成：

| 名称                                                                         | ID                    | 类型                    | 状态                                                  | 文档                                   |
| :--------------------------------------------------------------------------- | :-------------------- | :---------------------- | :------------------------------------------------------ | :------------------------------------- |
| [Betfair](https://betfair.com)                                               | `BETFAIR`             | 体育博彩交易所 | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/betfair_CN.md)       |
| [Binance](https://binance.com)                                               | `BINANCE`             | 加密货币交易所 (CEX)   | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/binance_CN.md)       |
| [Binance US](https://binance.us)                                             | `BINANCE`             | 加密货币交易所 (CEX)   | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/binance_CN.md)       |
| [Binance Futures](https://www.binance.com/en/futures)                        | `BINANCE`             | 加密货币交易所 (CEX)   | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/binance_CN.md)       |
| [Bybit](https://www.bybit.com)                                               | `BYBIT`               | 加密货币交易所 (CEX)   | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/bybit_CN.md)         |
| [Coinbase International](https://www.coinbase.com/en/international-exchange) | `COINBASE_INTX`       | 加密货币交易所 (CEX)   | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/coinbase_intx_CN.md) |
| [Databento](https://databento.com)                                           | `DATABENTO`           | 数据提供商           | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/databento_CN.md)     |
| [dYdX](https://dydx.exchange/)                                               | `DYDX`                | 加密货币交易所 (DEX)   | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/dydx_CN.md)          |
| [Hyperliquid](https://hyperliquid.xyz)                                       | `HYPERLIQUID`         | 加密货币交易所 (DEX)   | ![status](https://img.shields.io/badge/building-orange) | [指南](integrations/hyperliquid_CN.md)   |
| [Interactive Brokers](https://www.interactivebrokers.com)                    | `INTERACTIVE_BROKERS` | 经纪商 (多场所) | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/ib_CN.md)            |
| [OKX](https://okx.com)                                                       | `OKX`                 | 加密货币交易所 (CEX)   | ![status](https://img.shields.io/badge/building-orange) | [指南](integrations/okx_CN.md)           |
| [Polymarket](https://polymarket.com)                                         | `POLYMARKET`          | 预测市场 (DEX) | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/polymarket_CN.md)    |
| [Tardis](https://tardis.dev)                                                 | `TARDIS`              | 加密数据提供商    | ![status](https://img.shields.io/badge/stable-green)    | [指南](integrations/tardis_CN.md)        |

- **ID**：集成适配器客户端的默认客户端ID。
- **类型**：集成类型（通常是场所类型）。

## 状态

- `building`：正在构建中，可能无法使用。
- `beta`：已完成到最低工作状态，处于'beta'测试阶段。
- `stable`：功能集和API已稳定，集成已经过开发者和用户的合理测试（可能仍存在一些错误）。

## 实现目标

NautilusTrader的主要目标是提供一个统一的交易系统，用于各种集成。
为了支持最广泛的交易策略，将优先考虑*标准*功能：

- 请求历史市场数据
- 流式传输实时市场数据
- 协调执行状态
- 提交带有标准执行指令的标准订单类型
- 修改现有订单（如果交易所可能）
- 取消订单

每个集成的实现都旨在满足以下标准：

- 低级客户端组件应尽可能与交易所API匹配。
- 交易所的全部功能（适用于NautilusTrader的地方）*最终*应该得到支持。
- 将添加交易所特定的数据类型以支持用户合理期望的功能和返回类型。
- 交易所或NautilusTrader不支持的操作在调用时将记录为警告或错误。

## API统一

所有集成都必须符合NautilusTrader的系统API，需要标准化：

- 符号应使用场所的本机符号格式，除非需要消除歧义（例如，Binance现货与Binance期货）。
- 时间戳必须使用UNIX纪元纳秒。如果使用毫秒，字段/属性名称应明确以`_ms`结尾。
