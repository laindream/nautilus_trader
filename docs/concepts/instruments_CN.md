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

当`DataEngine`接收到工具的更新时，对象将传递给角色/策略的`on_instrument()`方法。用户可以重写此方法，在接收到工具更新时执行操作：

```python
from nautilus_trader.model.instruments import Instrument

def on_instrument(self, instrument: Instrument) -> None:
    # 在工具更新时执行某些操作
    pass
```

## 精度和增量

工具对象是通过*只读*属性组织工具规范的便捷方式。提供正确的价格和数量精度，以及最小价格和大小增量、乘数和标准手数。

:::note
这些限制中的大多数由Nautilus `RiskEngine`检查，否则价格和数量的无效值*可能*导致交易所拒绝订单。
:::

## 限制

某些值限制对工具是可选的，可以是`None`，这些取决于交易所，可以包括：

- `max_quantity`（单个订单的最大数量）
- `min_quantity`（单个订单的最小数量）
- `max_notional`（单个订单的最大名义价值）
- `min_notional`（单个订单的最小名义价值）
- `max_price`（最大有效报价或订单价格）
- `min_price`（最小有效报价或订单价格）

:::note
这些限制中的大多数由Nautilus `RiskEngine`检查，否则超过公布的限制*可能*导致交易所拒绝订单。
:::

## 价格和数量

工具对象还提供了一种基于给定值创建正确价格和数量的便捷方式。

```python
instrument = self.cache.instrument(instrument_id)

price = instrument.make_price(0.90500)
quantity = instrument.make_qty(150)
```

:::tip
上述是创建有效价格和数量的推荐方法，例如在将它们传递给订单工厂以创建订单时。
:::

## 保证金和费用

保证金计算由`MarginAccount`类处理。本节解释保证金如何工作并介绍您需要了解的关键概念。

### 何时适用保证金？

每个交易所（例如，CME或Binance）都使用特定的账户类型运行，该类型决定是否适用保证金计算。
在设置交易所场所时，您将指定以下账户类型之一：

- `AccountType.MARGIN`：使用保证金计算的账户，如下所述。
- `AccountType.CASH`：不适用保证金计算的简单账户。
- `AccountType.BETTING`：为博彩设计的账户，也不涉及保证金计算。

### 词汇

为了理解保证金交易，让我们从一些关键术语开始：

**名义价值**：以报价货币表示的总合约价值。它代表您头寸的完整市场价值。例如，在CME上的EUR/USD期货（符号6E）。

- 每个合约代表125,000 EUR（EUR是基础货币，USD是报价货币）。
- 如果当前市场价格是1.1000，名义价值等于125,000 EUR × 1.1000（EUR/USD价格）= 137,500 USD。

**杠杆**（`leverage`）：决定相对于您的账户存款可以控制多少市场敞口的比率。例如，使用10倍杠杆，您可以用账户中的1,000 USD控制价值10,000 USD的头寸。

**初始保证金**（`margin_init`）：开仓所需的保证金率。它代表开仓时账户中必须可用的最低资金量。这只是预检查——实际上没有资金被锁定。

**维持保证金**（`margin_maint`）：保持头寸开放所需的保证金率。此金额在您的账户中被锁定以维持头寸。它总是低于初始保证金。您可以在策略中使用以下内容查看总锁定资金（未平仓头寸维持保证金的总和）：

```python
self.portfolio.balances_locked(venue)
```

**Maker/Taker费用**：交易所根据您的订单与市场的交互收取的费用：

- Maker费用（`maker_fee`）：当您通过下订单簿上的订单"制造"流动性时收取的费用（通常较低）。例如，当前价格以下的限价买单增加流动性，当成交时适用*maker*费用。
- Taker费用（`taker_fee`）：当您通过立即执行的订单"获取"流动性时收取的费用（通常较高）。例如，市价买单或当前价格以上的限价买单移除流动性，适用*taker*费用。

:::tip
并非所有交易所或工具都实施maker/taker费用。如果不存在，请为`Instrument`（例如，`FuturesContract`、`Equity`、`CurrencyPair`、`Commodity`、`Cfd`、`BinaryOption`、`BettingInstrument`）将`maker_fee`和`taker_fee`都设置为0。
:::

### 保证金计算公式

`MarginAccount`类使用以下公式计算保证金：

```python
# 初始保证金计算
margin_init = (notional_value / leverage * margin_init) + (notional_value / leverage * taker_fee)

# 维持保证金计算
margin_maint = (notional_value / leverage * margin_maint) + (notional_value / leverage * taker_fee)
```

**要点**：

- 两个公式都遵循相同的结构，但使用各自的保证金率（`margin_init`和`margin_maint`）。
- 每个公式由两部分组成：
  - **主要保证金计算**：基于名义价值、杠杆和保证金率。
  - **费用调整**：考虑maker/taker费用。

### 实现细节

对于那些有兴趣探索技术实现的人：

- [nautilus_trader/accounting/accounts/margin.pyx](https://github.com/nautechsystems/nautilus_trader/blob/develop/nautilus_trader/accounting/accounts/margin.pyx)
- 关键方法：`calculate_margin_init(self, ...)`和`calculate_margin_maint(self, ...)`

## 佣金

交易佣金代表交易所或经纪商为执行交易收取的费用。
虽然maker/taker费用在加密货币市场中很常见，但像CME这样的传统交易所通常采用其他费用结构，如每合约佣金。
NautilusTrader支持多种佣金模型以适应不同市场的多样化费用结构。

### 内置费用模型

框架提供两种内置费用模型实现：

1. `MakerTakerFeeModel`：实现加密货币交易所常见的maker/taker费用结构，其中费用按交易价值的百分比计算。
2. `FixedFeeModel`：对每笔交易应用固定佣金，无论交易规模如何。

### 创建自定义费用模型

虽然内置费用模型涵盖常见场景，但您可能遇到需要特定佣金结构的情况。
NautilusTrader的灵活架构允许您通过继承基础`FeeModel`类来实现自定义费用模型。

例如，如果您在收取每合约佣金的交易所（如CME）上交易期货，您可以实现自定义费用模型。在创建自定义费用模型时，我们继承`FeeModel`基类，该类出于性能原因用Cython实现。这种Cython实现在参数命名约定中反映，其中类型信息使用下划线合并到参数名称中（如`Order_order`或`Quantity_fill_qty`）。

虽然这些参数名称对Python开发人员来说可能看起来不寻常，但它们是Cython类型系统的结果，有助于保持与框架核心组件的一致性。以下是您如何创建每合约佣金模型：

```python
class PerContractFeeModel(FeeModel):
    def __init__(self, commission: Money):
        super().__init__()
        self.commission = commission

    def get_commission(self, Order_order, Quantity_fill_qty, Price_fill_px, Instrument_instrument):
        total_commission = Money(self.commission * Quantity_fill_qty, self.commission.currency)
        return total_commission
```

这个自定义实现通过将*固定每合约费用*乘以交易的*合约数量*来计算总佣金。`get_commission(...)`方法接收关于订单、成交数量、成交价格和工具的信息，允许基于这些参数的灵活佣金计算。

我们的新类`PerContractFeeModel`继承类`FeeModel`，该类用Cython实现，
所以注意方法签名中的Cython风格参数名称：

- `Order_order`：订单对象，带有类型前缀`Order_`。
- `Quantity_fill_qty`：成交数量，带有类型前缀`Quantity_`。
- `Price_fill_px`：成交价格，带有类型前缀`Price_`。
- `Instrument_instrument`：工具对象，带有类型前缀`Instrument_`。

这些参数名称遵循NautilusTrader的Cython命名约定，其中前缀表示预期类型。
虽然这比典型的Python命名约定可能显得冗长，但它确保类型安全并与框架的Cython代码库保持一致。

### 在实践中使用费用模型

要在您的交易系统中使用任何费用模型，无论是内置还是自定义，您在设置场所时指定它。
以下是使用自定义每合约费用模型的示例：

```python
from nautilus_trader.model.currencies import USD
from nautilus_trader.model.objects import Money, Currency

engine.add_venue(
    venue=venue,
    oms_type=OmsType.NETTING,
    account_type=AccountType.MARGIN,
    base_currency=USD,
    fee_model=PerContractFeeModel(Money(2.50, USD)),  # 每合约2.50 USD
    starting_balances=[Money(1_000_000, USD)],  # 从1,000,000 USD余额开始
)
```

:::tip
在实现自定义费用模型时，确保它们准确反映目标交易所的费用结构。
佣金计算中的小差异可能会显著影响回测期间的策略性能指标。
:::

## 附加信息

交易所提供的原始工具定义（通常来自JSON序列化数据）也作为通用Python字典包含在内。这是为了保留不一定属于统一Nautilus API的所有信息，用户可以通过调用`.info`属性在运行时获得。

## 合成工具

平台支持创建自定义合成工具，可以生成合成报价和交易。这些对于以下用途很有用：

- 使`Actor`和`Strategy`组件能够订阅报价或交易数据流。
- 触发模拟订单。
- 从合成报价或交易构建K线。

合成工具不能直接交易，因为它们是仅存在于平台本地的构造。它们作为分析工具，基于其组件工具提供有用的指标。

在未来，我们计划支持合成工具的订单管理，基于合成工具的行为启用其组件工具的交易。

:::info
合成工具的场所始终指定为`'SYNTH'`。
:::

### 公式

合成工具由两个或更多组件工具的组合（可以包括来自多个场所的工具）以及"推导公式"组成。
利用由[evalexpr](https://github.com/ISibboI/evalexpr) Rust crate提供支持的动态表达式引擎，平台可以评估公式以从传入的组件工具价格计算最新的合成价格tick。

请参阅`evalexpr`文档以获取可用功能、运算符和优先级的完整描述。

:::tip
在定义新的合成工具之前，确保所有组件工具都已定义并存在于缓存中。
:::

### 订阅

以下示例演示了使用角色/策略创建新的合成工具。
这个合成工具将代表Binance上比特币和以太坊现货价格之间的简单价差。对于此示例，假设`BTCUSDT.BINANCE`和`ETHUSDT.BINANCE`的现货工具已存在于缓存中。

```python
from nautilus_trader.model.instruments import SyntheticInstrument

btcusdt_binance_id = InstrumentId.from_str("BTCUSDT.BINANCE")
ethusdt_binance_id = InstrumentId.from_str("ETHUSDT.BINANCE")

# 定义合成工具
synthetic = SyntheticInstrument(
    symbol=Symbol("BTC-ETH:BINANCE"),
    price_precision=8,
    components=[
        btcusdt_binance_id,
        ethusdt_binance_id,
    ],
    formula=f"{btcusdt_binance_id} - {ethusdt_binance_id}",
    ts_event=self.clock.timestamp_ns(),
    ts_init=self.clock.timestamp_ns(),
)

# 建议将合成工具的ID存储在某处
self._synthetic_id = synthetic.id

# 添加合成工具供其他组件使用
self.add_synthetic(synthetic)

# 订阅合成工具的报价
self.subscribe_quote_ticks(self._synthetic_id)
```

:::note
上述示例中合成工具的`instrument_id`将构造为`{symbol}.{SYNTH}`，结果为`'BTC-ETH:BINANCE.SYNTH'`。
:::

### 更新公式

也可以随时更新合成工具公式。以下示例显示如何使用角色/策略实现这一点。

```
# 从缓存中恢复合成工具（假设分配了`synthetic_id`）
synthetic = self.cache.synthetic(self._synthetic_id)

# 更新公式，这里是一个简单的示例，只是取平均值
new_formula = "(BTCUSDT.BINANCE + ETHUSDT.BINANCE) / 2"
synthetic.change_formula(new_formula)

# 现在更新合成工具
self.update_synthetic(synthetic)
```

### 触发工具ID

平台允许基于合成工具价格触发模拟订单。在以下示例中，我们基于前一个示例提交新的模拟订单。
此订单将保留在模拟器中，直到来自合成报价的触发器释放它。
然后它将作为MARKET订单提交给Binance：

```python
order = self.strategy.order_factory.limit(
    instrument_id=ETHUSDT_BINANCE.id,
    order_side=OrderSide.BUY,
    quantity=Quantity.from_str("1.5"),
    price=Price.from_str("30000.00000000"),  # <-- 合成工具价格
    emulation_trigger=TriggerType.DEFAULT,
    trigger_instrument_id=self._synthetic_id,  # <-- 合成工具标识符
)

self.strategy.submit_order(order)
```

### 错误处理

已经付出了相当大的努力来验证输入，包括合成工具的推导公式。尽管如此，建议谨慎，因为无效或错误的输入可能导致未定义的行为。

:::info
请参阅`SyntheticInstrument` [API参考](../../api_reference/model/instruments_CN.md#class-syntheticinstrument-1)以详细了解输入要求和潜在异常。
:::
