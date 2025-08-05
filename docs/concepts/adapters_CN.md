# 适配器

NautilusTrader设计通过适配器实现集成数据提供商和/或交易场所。这些可以在顶级`adapters`子包中找到。

集成适配器*通常*由以下主要组件组成：

- `HttpClient`
- `WebSocketClient`
- `InstrumentProvider`
- `DataClient`
- `ExecutionClient`

## 工具提供商

工具提供商如其名称所示——通过解析发布者或场所的原始API来实例化Nautilus `Instrument`对象。

`InstrumentProvider`可用工具的用例包括：

- 独立使用以发现集成可用的工具，将这些用于研究或回测目的
- 在`sandbox`或`live` [环境上下文](architecture_CN.md#environment-contexts)中用于角色/策略消费

### 研究和回测

以下是发现Binance期货测试网当前工具的示例：

```python
import asyncio
import os

from nautilus_trader.adapters.binance.common.enums import BinanceAccountType
from nautilus_trader.adapters.binance import get_cached_binance_http_client
from nautilus_trader.adapters.binance.futures.providers import BinanceFuturesInstrumentProvider
from nautilus_trader.common.component import LiveClock


clock = LiveClock()
account_type = BinanceAccountType.USDT_FUTURE

client = get_cached_binance_http_client(
    loop=asyncio.get_event_loop(),
    clock=clock,
    account_type=account_type,
    key=os.getenv("BINANCE_FUTURES_TESTNET_API_KEY"),
    secret=os.getenv("BINANCE_FUTURES_TESTNET_API_SECRET"),
    is_testnet=True,
)
await client.connect()

provider = BinanceFuturesInstrumentProvider(
    client=client,
    account_type=BinanceAccountType.USDT_FUTURE,
)

await provider.load_all_async()
```

### 实时交易

每个集成都是特定于实现的，通常有两个选项用于`TradingNode`中`InstrumentProvider`在实时交易中的行为，如配置：

- 所有工具在启动时自动加载：

```python
from nautilus_trader.config import InstrumentProviderConfig

InstrumentProviderConfig(load_all=True)
```

- 只有在配置中明确指定的工具在启动时加载：

```python
InstrumentProviderConfig(load_ids=["BTCUSDT-PERP.BINANCE", "ETHUSDT-PERP.BINANCE"])
```

## 数据客户端

### 请求

`Actor`或`Strategy`可以通过发送`DataRequest`从`DataClient`请求自定义数据。如果接收`DataRequest`的客户端实现了请求的处理程序，数据将返回给`Actor`或`Strategy`。

#### 示例

这方面的一个例子是对`Instrument`的`DataRequest`，`Actor`类实现了这个功能（下面复制）。任何`Actor`或`Strategy`都可以使用`InstrumentId`调用`request_instrument`方法来从`DataClient`请求工具。

在这种特殊情况下，`Actor`实现了一个单独的方法`request_instrument`。类似类型的`DataRequest`可以从角色/策略代码中的任何地方和/或任何时间实例化和调用。

角色/策略的`request_instrument`的简化版本是：

```python
# nautilus_trader/common/actor.pyx

cpdef void request_instrument(self, InstrumentId instrument_id, ClientId client_id=None):
    """
    为给定的工具ID请求`Instrument`数据。
