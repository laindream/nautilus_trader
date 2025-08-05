# 工具

`Instrument`基类表示任何可交易资产/合约的核心规范。目前有许多子类代表平台支持的一系列*资产类别*和*工具类别*：

- `Equity`（通用股票）
- `FuturesContract`（通用期货合约）
- `FuturesSpread`（通用期货差价）
- `OptionContract`（通用期权合约）
- `OptionSpread`（通用期权差价）
- `BinaryOption`（通用二元期权工具）
- `Cfd`（差价合约工具）
- `Commodity`（现货/现金市场的商品工具）
- `CurrencyPair`（代表现货/现金市场中的法币外汇或加密货币对）
- `CryptoOption`（加密期权工具）
- `CryptoPerpetual`（永续期货合约，又称永续掉期）
- `CryptoFuture`（以加密资产为标的的可交割期货合约，用于价格报价和结算）
- `IndexInstrument`（通用指数工具）
- `BettingInstrument`（体育、游戏或其他博彩）

## 符号体系

所有工具都应该有一个唯一的`InstrumentId`，它由原生符号和场所ID组成，用句号分隔。
例如，在Binance期货加密交易所，以太坊永续期货合约的工具ID为`ETHUSDT-PERP.BINANCE`。

所有原生符号*应该*对于场所是唯一的（这并不总是如此，例如Binance在现货和期货市场之间共享原生符号），
`{symbol.venue}`组合*必须*对于Nautilus系统是唯一的。

:::warning
正确的工具必须与市场数据集（如tick或订单簿数据）匹配，以进行逻辑合理的操作。
错误指定的工具可能会截断数据或产生其他令人惊讶的结果。
:::

## 回测

可以通过`TestInstrumentProvider`实例化通用测试工具：

```python
from nautilus_trader.test_kit.providers import TestInstrumentProvider

audusd = TestInstrumentProvider.default_fx_ccy("AUD/USD")
```

可以使用适配器的`InstrumentProvider`从实时交易所数据中发现交易所特定工具：

```python
from nautilus_trader.adapters.binance.spot.providers import BinanceSpotInstrumentProvider
from nautilus_trader.model import InstrumentId

provider = BinanceSpotInstrumentProvider(client=binance_http_client)
await provider.load_all_async()

btcusdt = InstrumentId.from_str("BTCUSDT.BINANCE")
instrument = provider.find(btcusdt)
```

或者通过`Instrument`构造函数或其更具体的子类之一由用户灵活定义：

```python
from nautilus_trader.model.instruments import Instrument

instrument = Instrument(...)  # <-- 提供所有必要参数
```

请参阅完整的工具[API参考](../api_reference/model/instruments_CN.md)。

## 实时交易

实时集成适配器定义了`InstrumentProvider`类，它们以自动方式工作以缓存交易所的最新工具定义。通过将匹配的`InstrumentId`传递给需要它的数据和执行相关方法和类来引用特定的`Instrument`对象。

## 查找工具

由于相同的角色/策略类可以用于回测和实时交易，您可以通过中央缓存以完全相同的方式获取工具：

```python
from nautilus_trader.model import InstrumentId

instrument_id = InstrumentId.from_str("ETHUSDT-PERP.BINANCE")
instrument = self.cache.instrument(instrument_id)
```

也可以订阅特定工具的任何更改：

```python
self.subscribe_instrument(instrument_id)
```

或订阅整个场所的所有工具更改：

```python
from nautilus_trader.model import Venue

binance = Venue("BINANCE")
self.subscribe_instruments(binance)
```

当`DataEngine`接收到工具的更新时，对象将 