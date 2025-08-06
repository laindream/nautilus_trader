# OKX

:::warning
OKX集成仍在积极开发中。
:::

OKX成立于2017年，是一家领先的加密货币交易所，提供现货、永续掉期、期货和期权交易。此集成支持在OKX上的实时市场数据摄取和订单执行。

## 概述

此适配器用Rust实现，具有可选的Python绑定，便于在基于Python的工作流中使用。
**它不需要任何外部OKX客户端库依赖项**。

:::info
`okx`**无需**额外安装步骤。
适配器的核心组件编译为静态库，并在构建过程中自动链接。
:::

## 示例

您可以在[这里](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/okx/)找到实时示例脚本。

### 产品支持

| 产品类型      | 支持 | 备注                                          |
|-------------------|-----------|------------------------------------------------|
| 现货              | ✓         | 用于指数价格。                          |
| 永续掉期   | ✓         | 线性和反向合约。                  |
| 期货           | ✓         | 特定到期日期。                     |
| 保证金            | -         | *尚未支持*。                           |
| 期权           | -         | *尚未支持*。                           |

OKX适配器包含多个组件，这些组件可以根据您的用例单独或一起使用。

- `OKXHttpClient`: 低级HTTP API连接。
- `OKXWebSocketClient`: 低级WebSocket API连接。
- `OKXInstrumentProvider`: 工具解析和加载功能。
- `OKXDataClient`: 市场数据流管理器。
- `OKXExecutionClient`: 账户管理和交易执行网关。
- `OKXLiveDataClientFactory`: OKX数据客户端工厂（由交易节点构建器使用）。
- `OKXLiveExecClientFactory`: OKX执行客户端工厂（由交易节点构建器使用）。

:::note
大多数用户只需定义实时交易节点的配置（如下所示），
不需要直接使用这些低级组件。
:::

## 符号体系

OKX使用原生符号，如线性永续掉期合约的`BTC-USDT-SWAP`。
工具使用OKX原生格式标识。

## 订单功能

以下是OKX上线性永续掉期产品支持的订单类型、执行指令和时间有效性选项。

### 订单类型

| 订单类型          | 线性永续掉期 | 备注                |
|---------------------|-----------------------|----------------------|
| `MARKET`            | ✓                     |                      |
| `LIMIT`             | ✓                     |                      |
| `STOP_MARKET`       | ✓                     |                      |
| `STOP_LIMIT`        | ✓                     |                      |
| `MARKET_IF_TOUCHED` | ✓                     |                      |
| `LIMIT_IF_TOUCHED`  | ✓                     |                      |
| `TRAILING_STOP`     | -                     | *尚未支持*. |

### 执行指令

| 指令    | 线性永续掉期 | 备注                  |
|----------------|-----------------------|------------------------|
| `post_only`    | ✓                     | 仅限LIMIT订单。 |
| `reduce_only`  | ✓                     | 仅限衍生品。  |

### 时间有效性

| 时间有效性 | 线性永续掉期 | 备注                |
|---------------|-----------------------|----------------------|
| `GTC`         | ✓                     | 取消前有效。  |
| `FOK`         | ✓                     | 全部成交或取消。        |
| `IOC`         | ✓                     | 立即成交或取消。 |

## 身份验证

要使用OKX适配器，您需要在OKX账户中创建API凭据：

1. 登录您的OKX账户并导航到API管理页面。
2. 创建一个具有交易和数据访问所需权限的新API密钥。
3. 记下您的API密钥、秘密密钥和密码短语。

您可以通过环境变量提供这些凭据：

```bash
export OKX_API_KEY="your_api_key"
export OKX_API_SECRET="your_api_secret"
export OKX_API_PASSPHRASE="your_passphrase"
```

或者在配置中直接传递它们（不建议用于生产环境）。

## 速率限制

OKX适配器为HTTP和WebSocket连接实现自动速率限制，以遵守OKX的API限制并防止速率限制错误。

### HTTP速率限制

HTTP客户端实现了**每秒250个请求**的保守速率限制。此限制基于OKX记录的速率限制：

- 子账户订单限制：2秒内1000个请求。
- 账户余额：2秒内10个请求。
- 账户工具：2秒内20个请求。

### WebSocket速率限制

WebSocket客户端实现带有不同操作类型不同配额的键控速率限制：

- **一般操作**（订阅）：每秒3个请求。
- **订单操作**（下单/取消/修改）：每秒250个请求。

这种方法确保订阅管理不会干扰订单执行性能，同时遵守OKX的连接速率限制。

### OKX API速率限制

OKX在其API端点上强制执行各种速率限制：

- **REST API**：因端点而异（通常根据端点每2秒20-1000个请求）。
- **WebSocket**：每个IP每秒3个连接请求，具有订阅和订单限制。
- **交易**：订单下达根据账户级别和工具类型具有特定限制。

有关完整和最新的速率限制信息，请参阅[OKX API文档](https://www.okx.com/docs-v5/en/#overview-rate-limit)。

## 配置

以下是使用OKX数据和执行客户端的实时交易节点的示例配置：

```python
from nautilus_trader.adapters.okx import OKX
from nautilus_trader.adapters.okx import OKXDataClientConfig, OKXExecClientConfig
from nautilus_trader.adapters.okx.factories import OKXLiveDataClientFactory, OKXLiveExecClientFactory
from nautilus_trader.config import InstrumentProviderConfig, LiveExecEngineConfig, LoggingConfig, TradingNodeConfig
from nautilus_trader.live.node import TradingNode

config = TradingNodeConfig(
    ...,
    data_clients={
        OKX: OKXDataClientConfig(
            api_key=None,           # 来自OKX_API_KEY环境变量
            api_secret=None,        # 来自OKX_API_SECRET环境变量
            api_passphrase=None,    # 来自OKX_API_PASSPHRASE环境变量
            base_url_http=None,
            instrument_provider=InstrumentProviderConfig(load_all=True),
            instrument_types=("SWAP",),
            contract_types=None,
            is_demo=False,
        ),
    },
    exec_clients={
        OKX: OKXExecClientConfig(
            api_key=None,
            api_secret=None,
            api_passphrase=None,
            base_url_http=None,
            base_url_ws=None,
            instrument_provider=InstrumentProviderConfig(load_all=True),
            instrument_types=("SWAP",),
            contract_types=None,
            is_demo=False,
        ),
    },
)
node = TradingNode(config=config)
node.add_data_client_factory(OKX, OKXLiveDataClientFactory)
node.add_exec_client_factory(OKX, OKXLiveExecClientFactory)
node.build()
```

## 错误处理

使用OKX适配器时的常见问题：

- **身份验证错误**：验证您的API凭据并确保它们具有所需权限。
- **权限不足**：如果执行订单，请验证您的API密钥具有交易权限。
- **超出速率限制**：减少请求频率或在请求之间实现延迟。
- **无效符号**：确保您使用有效的OKX工具标识符。

有关详细的错误信息，请检查NautilusTrader日志。

## 参考

有关更多详情，请参阅OKX [API文档](https://www.okx.com/docs-v5/)和NautilusTrader [API参考](../api_reference/adapters/okx.md)。
