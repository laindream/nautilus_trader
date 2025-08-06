# Interactive Brokers

Interactive Brokers（IB）是一个交易平台，提供跨广泛金融工具的市场准入，包括股票、期权、期货、货币、债券、基金和加密货币。NautilusTrader提供适配器通过其[Trader Workstation（TWS）API](https://ibkrcampus.com/ibkr-api-page/trader-workstation-api/)使用其Python库[ibapi](https://github.com/nautechsystems/ibapi)与IB集成。

TWS API作为IB独立交易应用程序的接口：TWS和IB Gateway。两者都可以从IB网站下载。如果您尚未安装TWS或IB Gateway，请参阅[初始设置](https://ibkrcampus.com/ibkr-api-page/trader-workstation-api/#tws-download)指南。在NautilusTrader中，您将通过`InteractiveBrokersClient`建立与这些应用程序之一的连接。

或者，您可以从IB Gateway的[Docker化版本](https://github.com/gnzsnz/ib-gateway-docker)开始，这在托管云平台上部署交易策略时特别有用。这需要在您的机器上安装[Docker](https://www.docker.com/)，以及[docker](https://pypi.org/project/docker/) Python包，NautilusTrader方便地将其作为额外包包含。

:::note
独立的TWS和IB Gateway应用程序需要在启动时手动输入用户名、密码和交易模式（实时或模拟）。IB Gateway的Docker化版本以编程方式处理这些步骤。
:::

## 安装

要安装支持Interactive Brokers（和Docker）的NautilusTrader：

```bash
pip install --upgrade "nautilus_trader[ib,docker]"
```

要使用所有extras从源码构建（包括IB和Docker）：

```bash
uv sync --all-extras
```

:::note
因为IB不为`ibapi`提供wheel，NautilusTrader [重新打包](https://pypi.org/project/nautilus-ibapi/)它以在PyPI上发布。
:::

## 示例

您可以在[这里](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/interactive_brokers/)找到实时示例脚本。

## 入门

在实施您的交易策略之前，请确保TWS（Trader Workstation）或IB Gateway当前正在运行。您可以选择使用您的个人凭据登录这些独立应用程序之一，或者通过`DockerizedIBGateway`。

### 连接方法

连接到Interactive Brokers有两种主要方式：

1. **连接到现有的TWS或IB Gateway实例**
2. **使用Docker化的IB Gateway（推荐用于自动化部署）**

### 默认端口

Interactive Brokers根据应用程序和交易模式使用不同的默认端口：

| 应用程序 | 模拟交易 | 实时交易 |
|-------------|---------------|--------------|
| TWS         | 7497          | 7496         |
| IB Gateway  | 4002          | 4001         |

### 建立与现有Gateway或TWS的连接

当连接到预先存在的Gateway或TWS时，在`InteractiveBrokersDataClientConfig`和`InteractiveBrokersExecClientConfig`中指定`ibg_host`和`ibg_port`参数：

```python
from nautilus_trader.adapters.interactive_brokers.config import InteractiveBrokersDataClientConfig
from nautilus_trader.adapters.interactive_brokers.config import InteractiveBrokersExecClientConfig

# TWS模拟交易示例（默认端口7497）
data_config = InteractiveBrokersDataClientConfig(
    ibg_host="127.0.0.1",
    ibg_port=7497,
    ibg_client_id=1,
)

exec_config = InteractiveBrokersExecClientConfig(
    ibg_host="127.0.0.1",
    ibg_port=7497,
    ibg_client_id=1,
    account_id="DU123456",  # 您的模拟交易账户ID
)
```

### 建立与DockerizedIBGateway的连接

对于自动化部署，推荐使用Docker化网关。在两个客户端配置中为`dockerized_gateway`提供`DockerizedIBGatewayConfig`实例。不需要`ibg_host`和`ibg_port`参数，因为它们会自动管理。

```python
from nautilus_trader.adapters.interactive_brokers.config import DockerizedIBGatewayConfig
from nautilus_trader.adapters.interactive_brokers.gateway import DockerizedIBGateway

gateway_config = DockerizedIBGatewayConfig(
    username="your_username",  # 或设置TWS_USERNAME环境变量
    password="your_password",  # 或设置TWS_PASSWORD环境变量
    trading_mode="paper",      # "paper"或"live"
    read_only_api=True,        # 设置为False以允许订单执行
    timeout=300,               # 启动超时秒数
)

# 这可能需要一些时间来启动，特别是第一次
gateway = DockerizedIBGateway(config=gateway_config)
gateway.start()

# 确认您已登录
print(gateway.is_logged_in(gateway.container))

# 检查日志
print(gateway.container.logs())
```

### 环境变量

要为Interactive Brokers Gateway提供凭据，可以将`username`和`password`传递给`DockerizedIBGatewayConfig`，或设置以下环境变量：

- `TWS_USERNAME` - 您的IB账户用户名
- `TWS_PASSWORD` - 您的IB账户密码
- `TWS_ACCOUNT` - 您的IB账户ID（用作`account_id`的后备）

### 连接管理

适配器包括强大的连接管理功能：

- **自动重连**：通过`IB_MAX_CONNECTION_ATTEMPTS`环境变量可配置
- **连接超时**：通过`connection_timeout`参数可配置（默认：300秒）
- **连接看门狗**：监控连接健康并在需要时触发重连
- **优雅错误处理**：全面处理各种连接场景的错误

## 概述

Interactive Brokers适配器提供与IB的TWS API的全面集成。适配器包括几个主要组件：

### 核心组件

- **`InteractiveBrokersClient`**：执行TWS API请求的中央客户端，使用`ibapi`。管理连接、处理错误并协调所有API交互。
- **`InteractiveBrokersDataClient`**：连接到Gateway以流式传输市场数据，包括报价、交易和K线。
- **`InteractiveBrokersExecutionClient`**：处理账户信息、订单管理和交易执行。
- **`InteractiveBrokersInstrumentProvider`**：检索和管理工具定义，包括支持期权和期货链。
- **`HistoricInteractiveBrokersClient`**：提供检索工具和历史数据的方法，对回测和研究有用。

### 支持组件

- **`DockerizedIBGateway`**：管理Docker化IB Gateway实例用于自动化部署。
- **配置类**：为所有组件提供全面配置选项。
- **工厂类**：创建和配置具有适当依赖关系的客户端实例。

### 支持的资产类别

适配器支持通过Interactive Brokers提供的所有主要资产类别的交易：

- **股票**：股票、ETF和股票期权
- **固定收益**：债券和债券基金
- **衍生品**：期货、期权和权证
- **外汇**：现货外汇和外汇远期
- **加密货币**：比特币、以太坊和其他数字资产
- **商品**：实物商品和商品期货
- **指数**：指数产品和指数期权

## Interactive Brokers客户端

`InteractiveBrokersClient`作为IB适配器的中央组件，监督一系列关键功能。这些包括建立和维护连接、处理API错误、执行交易以及收集各种类型的数据，如市场数据、合约/工具数据和账户详细信息。

为确保有效管理这些多样化的责任，`InteractiveBrokersClient`被分为几个专门的mixin类。这种模块化方法增强了可管理性和清晰度。

### 客户端架构

客户端使用基于mixin的架构，其中每个mixin处理IB API的特定方面：

#### 连接管理（`InteractiveBrokersClientConnectionMixin`）

- 建立和维护与TWS/Gateway的套接字连接
- 处理连接超时和重连逻辑
- 管理连接状态和健康监控
- 通过`IB_MAX_CONNECTION_ATTEMPTS`环境变量支持可配置的重连尝试

#### 错误处理（`InteractiveBrokersClientErrorMixin`）

- 处理所有API错误和警告
- 按类型分类错误（客户端错误、连接问题、请求错误）
- 处理订阅和请求特定的错误场景
- 提供全面的错误日志记录和调试信息

#### 账户管理（`InteractiveBrokersClientAccountMixin`）

- 检索账户信息和余额
- 管理仓位数据和投资组合更新
- 处理多账户场景
- 处理账户相关通知

#### 合约/工具管理（`InteractiveBrokersClientContractMixin`）

- 检索合约详细信息和规格
- 处理工具搜索和查找
- 管理合约验证和确认
- 支持复杂工具类型（期权链、期货链）

#### 市场数据管理（`InteractiveBrokersClientMarketDataMixin`）

- 处理实时和历史市场数据订阅
- 处理报价、交易和K线数据
- 管理市场数据类型设置（实时、延迟、冻结）
- 处理逐笔数据和市场深度

#### 订单管理（`InteractiveBrokersClientOrderMixin`）

- 处理订单下达、修改和取消
- 处理订单状态更新和执行报告
- 管理订单验证和错误处理
- 支持复杂订单类型和条件

### 关键特性

- **异步操作**：所有操作使用Python的asyncio完全异步
- **强大的错误处理**：全面的错误分类和处理
- **连接弹性**：具有可配置重试逻辑的自动重连
- **消息处理**：为高吞吐量场景进行高效的消息队列处理
- **状态管理**：适当跟踪连接、订阅和请求的状态

:::tip
要排除TWS API传入消息问题，考虑从`InteractiveBrokersClient._process_message`方法开始，该方法充当处理从API接收的所有消息的主要网关。
:::

## 符号体系

`InteractiveBrokersInstrumentProvider`支持三种构造`InstrumentId`实例的方法，可以通过`InteractiveBrokersInstrumentProviderConfig`中的`symbology_method`枚举配置。

### 符号体系方法

#### 1. 简化符号体系（`IB_SIMPLIFIED`）- 默认

当`symbology_method`设置为`IB_SIMPLIFIED`（默认设置）时，系统使用直观、人类可读的符号体系规则：

**按资产类别的格式规则：**

- **外汇**：`{symbol}/{currency}.{exchange}`
  - 示例：`EUR/USD.IDEALPRO`
- **股票**：`{localSymbol}.{primaryExchange}`
  - localSymbol中的空格替换为连字符
  - 示例：`BF-B.NYSE`，`SPY.ARCA`
- **期货**：`{localSymbol}.{exchange}`
  - 个别合约使用单数字年份
  - 示例：`ESM4.CME`，`CLZ7.NYMEX`
- **连续期货**：`{symbol}.{exchange}`
  - 代表主力合约，自动滚动
  - 示例：`ES.CME`，`CL.NYMEX`
- **期货期权（FOP）**：`{localSymbol}.{exchange}`
  - 格式：`{symbol}{month}{year} {right}{strike}`
  - 示例：`ESM4 C4200.CME`
- **期权**：`{localSymbol}.{exchange}`
  - 从localSymbol中删除所有空格
  - 示例：`AAPL230217P00155000.SMART`
- **指数**：`^{localSymbol}.{exchange}`
  - 示例：`^SPX.CBOE`，`^NDX.NASDAQ`
- **债券**：`{localSymbol}.{exchange}`
  - 示例：`912828XE8.SMART`
- **加密货币**：`{symbol}/{currency}.{exchange}`
  - 示例：`BTC/USD.PAXOS`，`ETH/USD.PAXOS`

#### 2. 原始符号体系（`IB_RAW`）

将`symbology_method`设置为`IB_RAW`强制执行更严格的解析规则，直接与IB API中定义的字段对齐。此方法为所有地区和工具类型提供最大兼容性：

**格式规则：**

- **CFD**：`{localSymbol}={secType}.IBCFD`
- **商品**：`{localSymbol}={secType}.IBCMDTY`
- **其他类型的默认值**：`{localSymbol}={secType}.{exchange}`

**示例：**

- `IBUS30=CFD.IBCFD`
- `XAUUSD=CMDTY.IBCMDTY`
- `AAPL=STK.SMART`

此配置确保明确的工具识别，并支持来自任何地区的工具，特别是那些具有非标准符号体系的工具，其中简化解析可能失败。

### MIC场所转换

适配器支持将Interactive Brokers交易所代码转换为市场标识符代码（MIC），以实现标准化场所识别：

#### `convert_exchange_to_mic_venue`

当设置为`True`时，适配器自动将IB交易所代码转换为其对应的MIC代码：

```python
instrument_provider_config = InteractiveBrokersInstrumentProviderConfig(
    convert_exchange_to_mic_venue=True,  # 启用MIC转换
    symbology_method=SymbologyMethod.IB_SIMPLIFIED,
)
```

**MIC转换示例：**

- `CME` → `XCME`（芝加哥商品交易所）
- `NASDAQ` → `XNAS`（纳斯达克股票市场）
- `NYSE` → `XNYS`（纽约证券交易所）
- `LSE` → `XLON`（伦敦证券交易所）

#### `symbol_to_mic_venue`

对于自定义场所映射，使用`symbol_to_mic_venue`字典覆盖默认转换：

```python
instrument_provider_config = InteractiveBrokersInstrumentProviderConfig(
    convert_exchange_to_mic_venue=True,
    symbol_to_mic_venue={
        "ES": "XCME",  # 所有ES期货/期权使用CME MIC
        "SPY": "ARCX", # SPY特别使用ARCA
    },
)
```

### 支持的工具格式

适配器基于Interactive Brokers的合约规格支持各种工具格式：

#### 期货月份代码

- **F** = 一月，**G** = 二月，**H** = 三月，**J** = 四月
- **K** = 五月，**M** = 六月，**N** = 七月，**Q** = 八月
- **U** = 九月，**V** = 十月，**X** = 十一月，**Z** = 十二月

#### 按资产类别支持的交易所

**期货交易所：**

- `CME`，`CBOT`，`NYMEX`，`COMEX`，`KCBT`，`MGE`，`NYBOT`，`SNFE`

**期权交易所：**

- `SMART`（IB的智能路由）

**外汇交易所：**

- `IDEALPRO`（IB的外汇平台）

**加密货币交易所：**

- `PAXOS`（IB的加密平台）

**CFD/商品交易所：**

- `IBCFD`，`IBCMDTY`（IB的内部路由）

### 选择正确的符号体系方法

- **使用`IB_SIMPLIFIED`**（默认）适用于大多数用例 - 提供干净、可读的工具ID
- **使用`IB_RAW`**处理复杂的国际工具或简化解析失败时
- **启用`convert_exchange_to_mic_venue`**当您需要标准化MIC场所代码以确保合规性或数据一致性时

## 工具和合约

在Interactive Brokers中，NautilusTrader `Instrument`对应于IB [Contract](https://ibkrcampus.com/ibkr-api-page/trader-workstation-api/#contracts)。适配器处理两种类型的合约表示：

### 合约类型

#### 基本合约（`IBContract`）

- 包含基本合约识别字段
- 用于合约搜索和基本操作
- 不能直接转换为NautilusTrader `Instrument`

#### 合约详细信息（`IBContractDetails`）

- 包含全面的合约信息，包括：
  - 支持的订单类型
  - 交易时间和日历
  - 保证金要求
  - 价格增量和乘数
  - 市场数据权限
- 可以转换为NautilusTrader `Instrument`
- 交易操作所需

### 合约发现

要搜索合约信息，请使用[IB合约信息中心](https://pennies.interactivebrokers.com/cstools/contract_info/)。

### 加载工具

有两种主要方法加载工具：

#### 1. 使用`load_ids`（推荐）
使用`symbology_method=SymbologyMethod.IB_SIMPLIFIED`（默认）与`load_ids`进行干净、直观的工具识别：

```python
from nautilus_trader.adapters.interactive_brokers.config import InteractiveBrokersInstrumentProviderConfig
from nautilus_trader.adapters.interactive_brokers.config import SymbologyMethod

instrument_provider_config = InteractiveBrokersInstrumentProviderConfig(
    symbology_method=SymbologyMethod.IB_SIMPLIFIED,
    load_ids=frozenset([
        "EUR/USD.IDEALPRO",    # 外汇
        "SPY.ARCA",            # 股票
        "ESM24.CME",           # 期货
        "BTC/USD.PAXOS",       # 加密货币
        "^SPX.CBOE",           # 指数
    ]),
)
```

#### 2. 使用`load_contracts`（用于复杂工具）
对于期权/期货链等复杂场景，使用带有`IBContract`实例的`load_contracts`：

```python
from nautilus_trader.adapters.interactive_brokers.common import IBContract

# 加载特定到期日的期权链
options_chain_expiry = IBContract(
    secType="IND",
    symbol="SPX",
    exchange="CBOE",
    build_options_chain=True,
    lastTradeDateOrContractMonth='20240718',
)

# 加载日期范围的期权链
options_chain_range = IBContract(
    secType="IND",
    symbol="SPX",
    exchange="CBOE",
    build_options_chain=True,
    min_expiry_days=0,
    max_expiry_days=30,
)

# 加载期货链
futures_chain = IBContract(
    secType="CONTFUT",
    exchange="CME",
    symbol="ES",
    build_futures_chain=True,
)

instrument_provider_config = InteractiveBrokersInstrumentProviderConfig(
    load_contracts=frozenset([
        options_chain_expiry,
        options_chain_range,
        futures_chain,
    ]),
)
```

### 按资产类别的IBContract示例

```python
from nautilus_trader.adapters.interactive_brokers.common import IBContract

# 股票
IBContract(secType='STK', exchange='SMART', primaryExchange='ARCA', symbol='SPY')
IBContract(secType='STK', exchange='SMART', primaryExchange='NASDAQ', symbol='AAPL')

# 债券
IBContract(secType='BOND', secIdType='ISIN', secId='US03076KAA60')
IBContract(secType='BOND', secIdType='CUSIP', secId='912828XE8')

# 个别期权
IBContract(secType='OPT', exchange='SMART', symbol='SPY',
           lastTradeDateOrContractMonth='20251219', strike=500, right='C')

# 期权链（加载所有行权价/到期日）
IBContract(secType='STK', exchange='SMART', primaryExchange='ARCA', symbol='SPY',
           build_options_chain=True, min_expiry_days=10, max_expiry_days=60)

# CFD
IBContract(secType='CFD', symbol='IBUS30')
IBContract(secType='CFD', symbol='DE40EUR', exchange='SMART')

# 个别期货
IBContract(secType='FUT', exchange='CME', symbol='ES',
           lastTradeDateOrContractMonth='20240315')

# 期货链（加载所有到期日）
IBContract(secType='CONTFUT', exchange='CME', symbol='ES', build_futures_chain=True)

# 期货期权（FOP）- 个别
IBContract(secType='FOP', exchange='CME', symbol='ES',
           lastTradeDateOrContractMonth='20240315', strike=4200, right='C')

# 期货期权链（加载所有行权价/到期日）
IBContract(secType='CONTFUT', exchange='CME', symbol='ES',
           build_options_chain=True, min_expiry_days=7, max_expiry_days=60)

# 外汇
IBContract(secType='CASH', exchange='IDEALPRO', symbol='EUR', currency='USD')
IBContract(secType='CASH', exchange='IDEALPRO', symbol='GBP', currency='JPY')

# 加密货币
IBContract(secType='CRYPTO', symbol='BTC', exchange='PAXOS', currency='USD')
IBContract(secType='CRYPTO', symbol='ETH', exchange='PAXOS', currency='USD')

# 指数
IBContract(secType='IND', symbol='SPX', exchange='CBOE')
IBContract(secType='IND', symbol='NDX', exchange='NASDAQ')

# 商品
IBContract(secType='CMDTY', symbol='XAUUSD', exchange='SMART')
```

### 高级配置选项

```python
# 带有自定义交易所的期权链
IBContract(
    secType="STK",
    symbol="AAPL",
    exchange="SMART",
    primaryExchange="NASDAQ",
    build_options_chain=True,
    options_chain_exchange="CBOE",  # 使用CBOE期权而不是SMART
    min_expiry_days=7,
    max_expiry_days=45,
)

# 具有特定月份的期货链
IBContract(
    secType="CONTFUT",
    exchange="NYMEX",
    symbol="CL",  # 原油
    build_futures_chain=True,
    min_expiry_days=30,
    max_expiry_days=180,
)
```

### 连续期货

对于连续期货合约（使用`secType='CONTFUT'`），适配器使用符号和场所创建工具ID：

```python
# 连续期货示例
IBContract(secType='CONTFUT', exchange='CME', symbol='ES')  # → ES.CME
IBContract(secType='CONTFUT', exchange='NYMEX', symbol='CL') # → CL.NYMEX

# 启用MIC场所转换
instrument_provider_config = InteractiveBrokersInstrumentProviderConfig(
    convert_exchange_to_mic_venue=True,
)
# 结果：
# ES.XCME（而不是ES.CME）
# CL.XNYM（而不是CL.NYMEX）
```

**连续期货vs个别期货：**

- **连续**：`ES.CME` - 代表主力合约，自动滚动
- **个别**：`ESM4.CME` - 特定的2024年3月合约

:::note
使用`build_options_chain=True`或`build_futures_chain=True`时，应为基础合约指定`secType`和`symbol`。适配器将自动发现并加载指定到期范围内的所有相关衍生品合约。
:::

## 历史数据和回测

`HistoricInteractiveBrokersClient`提供从Interactive Brokers检索历史数据的全面方法，用于回测和研究目的。

### 支持的数据类型

- **K线数据**：具有各种聚合的OHLCV K线（基于时间、基于tick、基于成交量）
- **Tick数据**：具有微秒精度的交易tick和报价tick
- **工具数据**：完整的合约规格和交易规则

### 历史数据客户端

```python
from nautilus_trader.adapters.interactive_brokers.historical.client import HistoricInteractiveBrokersClient
from ibapi.common import MarketDataTypeEnum

# 初始化客户端
client = HistoricInteractiveBrokersClient(
    host="127.0.0.1",
    port=7497,
    client_id=1,
    market_data_type=MarketDataTypeEnum.DELAYED_FROZEN,  # 如果没有订阅则使用延迟数据
    log_level="INFO"
)

# 连接到TWS/Gateway
await client.connect()
```

### 检索工具

```python
from nautilus_trader.adapters.interactive_brokers.common import IBContract

# 定义合约
contracts = [
    IBContract(secType="STK", symbol="AAPL", exchange="SMART", primaryExchange="NASDAQ"),
    IBContract(secType="STK", symbol="MSFT", exchange="SMART", primaryExchange="NASDAQ"),
    IBContract(secType="CASH", symbol="EUR", currency="USD", exchange="IDEALPRO"),
]

# 请求工具定义
instruments = await client.request_instruments(contracts=contracts)
```

### 检索历史K线

```python
import datetime

# 请求历史K线
bars = await client.request_bars(
    bar_specifications=[
        "1-MINUTE-LAST",    # 使用最后价格的1分钟K线
        "5-MINUTE-MID",     # 使用中点的5分钟K线
        "1-HOUR-LAST",      # 使用最后价格的1小时K线
        "1-DAY-LAST",       # 使用最后价格的日K线
    ],
    start_date_time=datetime.datetime(2023, 11, 1, 9, 30),
    end_date_time=datetime.datetime(2023, 11, 6, 16, 30),
    tz_name="America/New_York",
    contracts=contracts,
    use_rth=True,  # 仅常规交易时间
    timeout=120,   # 请求超时秒数
)
```

### 检索历史Tick

```python
# 请求历史tick数据
ticks = await client.request_ticks(
    tick_types=["TRADES", "BID_ASK"],  # 交易tick和报价tick
    start_date_time=datetime.datetime(2023, 11, 6, 9, 30),
    end_date_time=datetime.datetime(2023, 11, 6, 16, 30),
    tz_name="America/New_York",
    contracts=contracts,
    use_rth=True,
    timeout=120,
)
```

### K线规格

适配器支持各种K线规格：

#### 基于时间的K线

- `"1-SECOND-LAST"`，`"5-SECOND-LAST"`，`"10-SECOND-LAST"`，`"15-SECOND-LAST"`，`"30-SECOND-LAST"`
- `"1-MINUTE-LAST"`，`"2-MINUTE-LAST"`，`"3-MINUTE-LAST"`，`"5-MINUTE-LAST"`，`"10-MINUTE-LAST"`，`"15-MINUTE-LAST"`，`"20-MINUTE-LAST"`，`"30-MINUTE-LAST"`
- `"1-HOUR-LAST"`，`"2-HOUR-LAST"`，`"3-HOUR-LAST"`，`"4-HOUR-LAST"`，`"8-HOUR-LAST"`
- `"1-DAY-LAST"`，`"1-WEEK-LAST"`，`"1-MONTH-LAST"`

#### 价格类型

- `LAST` - 最后成交价
- `MID` - 买卖价中点
- `BID` - 买价
- `ASK` - 卖价

### 完整示例

```python
import asyncio
import datetime
from nautilus_trader.adapters.interactive_brokers.common import IBContract
from nautilus_trader.adapters.interactive_brokers.historical.client import HistoricInteractiveBrokersClient
from nautilus_trader.persistence.catalog import ParquetDataCatalog


async def download_historical_data():
    # 初始化客户端
    client = HistoricInteractiveBrokersClient(
        host="127.0.0.1",
        port=7497,
        client_id=5,
    )

    # 连接
    await client.connect()
    await asyncio.sleep(2)  # 允许连接稳定

    # 定义合约
    contracts = [
        IBContract(secType="STK", symbol="AAPL", exchange="SMART", primaryExchange="NASDAQ"),
        IBContract(secType="CASH", symbol="EUR", currency="USD", exchange="IDEALPRO"),
    ]

    # 请求工具
    instruments = await client.request_instruments(contracts=contracts)

    # 请求历史K线
    bars = await client.request_bars(
        bar_specifications=["1-HOUR-LAST", "1-DAY-LAST"],
        start_date_time=datetime.datetime(2023, 11, 1, 9, 30),
        end_date_time=datetime.datetime(2023, 11, 6, 16, 30),
        tz_name="America/New_York",
        contracts=contracts,
        use_rth=True,
    )

    # 请求tick数据
    ticks = await client.request_ticks(
        tick_types=["TRADES"],
        start_date_time=datetime.datetime(2023, 11, 6, 14, 0),
        end_date_time=datetime.datetime(2023, 11, 6, 15, 0),
        tz_name="America/New_York",
        contracts=contracts,
    )

    # 保存到目录
    catalog = ParquetDataCatalog("./catalog")
    catalog.write_data(instruments)
    catalog.write_data(bars)
    catalog.write_data(ticks)

    print(f"Downloaded {len(instruments)} instruments")
    print(f"Downloaded {len(bars)} bars")
    print(f"Downloaded {len(ticks)} ticks")

    # 断开连接
    await client.disconnect()

# 运行示例
if __name__ == "__main__":
    asyncio.run(download_historical_data())
```

### 数据限制

请注意Interactive Brokers的历史数据限制：

- **速率限制**：IB对历史数据请求强制执行速率限制
- **数据可用性**：历史数据可用性因工具和订阅级别而异
- **市场数据权限**：某些数据需要特定的市场数据订阅
- **时间范围**：最大回溯期因K线大小和工具类型而异

### 最佳实践

1. **使用延迟数据**：对于回测，`MarketDataTypeEnum.DELAYED_FROZEN`通常足够
2. **批量请求**：尽可能在单个请求中分组多个工具
3. **处理超时**：为大数据请求设置适当的超时值
4. **尊重速率限制**：在请求之间添加延迟以避免达到速率限制
5. **验证数据**：在回测前始终检查数据质量和完整性

## 实时交易

使用Interactive Brokers进行实时交易需要设置一个包含`InteractiveBrokersDataClient`和`InteractiveBrokersExecutionClient`的`TradingNode`。这些客户端依赖于`InteractiveBrokersInstrumentProvider`进行工具管理。

### 架构概述

实时交易设置包括三个主要组件：

1. **InstrumentProvider**：管理工具定义和合约详细信息
2. **DataClient**：处理实时市场数据订阅
3. **ExecutionClient**：管理订单、仓位和账户信息

### InstrumentProvider配置

`InteractiveBrokersInstrumentProvider`作为访问IB金融工具数据的桥梁。它支持加载个别工具、期权链和期货链。

#### 基本配置

```python
from nautilus_trader.adapters.interactive_brokers.config import InteractiveBrokersInstrumentProviderConfig
from nautilus_trader.adapters.interactive_brokers.config import SymbologyMethod
from nautilus_trader.adapters.interactive_brokers.common import IBContract

instrument_provider_config = InteractiveBrokersInstrumentProviderConfig(
    symbology_method=SymbologyMethod.IB_SIMPLIFIED,
    build_futures_chain=False,  # 如果获取期货链则设置为True
    build_options_chain=False,  # 如果获取期权链则设置为True
    min_expiry_days=10,         # 衍生品的最小到期天数
    max_expiry_days=60,         # 衍生品的最大到期天数
    convert_exchange_to_mic_venue=False,  # 使用MIC代码进行场所映射
    cache_validity_days=1,      # 缓存工具数据1天
    load_ids=frozenset([
        # 使用简化符号体系的个别工具
        "EUR/USD.IDEALPRO",     # 外汇
        "BTC/USD.PAXOS",        # 加密货币
        "SPY.ARCA",             # 股票ETF
        "V.NYSE",               # 个股
        "ESM4.CME",             # 期货合约（单数字年份）
        "^SPX.CBOE",            # 指数
    ]),
    load_contracts=frozenset([
        # 使用IBContract的复杂工具
        IBContract(secType='STK', symbol='AAPL', exchange='SMART', primaryExchange='NASDAQ'),
        IBContract(secType='CASH', symbol='GBP', currency='USD', exchange='IDEALPRO'),
    ]),
)
```

#### 衍生品的高级配置

```python
# 期权和期货链的配置
advanced_config = InteractiveBrokersInstrumentProviderConfig(
    symbology_method=SymbologyMethod.IB_SIMPLIFIED,
    build_futures_chain=True,   # 启用期货链加载
    build_options_chain=True,   # 启用期权链加载
    min_expiry_days=7,          # 加载7天以上到期的合约
    max_expiry_days=90,         # 加载90天内到期的合约
    load_contracts=frozenset([
        # 加载SPY期权链
        IBContract(
            secType='STK',
            symbol='SPY',
            exchange='SMART',
            primaryExchange='ARCA',
            build_options_chain=True,
        ),
        # 加载ES期货链
        IBContract(
            secType='CONTFUT',
            exchange='CME',
            symbol='ES',
            build_futures_chain=True,
        ),
    ]),
)
```

### 与外部数据提供商的集成

Interactive Brokers适配器可以与其他数据提供商一起使用，以增强市场数据覆盖。使用多个数据源时：

- 跨提供商使用一致的符号体系方法
- 考虑使用`convert_exchange_to_mic_venue=True`进行标准化场所识别
- 确保正确处理工具缓存管理以避免冲突

### 数据客户端配置

`InteractiveBrokersDataClient`与IB接口以流式传输和检索实时市场数据。连接后，它配置[市场数据类型](https://ibkrcampus.com/ibkr-api-page/trader-workstation-api/#delayed-market-data)并根据`InteractiveBrokersInstrumentProviderConfig`设置加载工具。

#### 支持的数据类型

- **报价Tick**：实时买卖价格和数量
- **交易Tick**：实时交易价格和成交量
- **K线数据**：实时OHLCV K线（1秒到1天间隔）
- **市场深度**：二级订单簿数据（如有）

#### 市场数据类型

Interactive Brokers支持几种市场数据类型：

- `REALTIME`：实时市场数据（需要市场数据订阅）
- `DELAYED`：15-20分钟延迟数据（大多数市场免费）
- `DELAYED_FROZEN`：不更新的延迟数据（对测试有用）
- `FROZEN`：最后已知的实时数据（市场关闭时）

#### 基本数据客户端配置

```python
from nautilus_trader.adapters.interactive_brokers.config import IBMarketDataTypeEnum
from nautilus_trader.adapters.interactive_brokers.config import InteractiveBrokersDataClientConfig

data_client_config = InteractiveBrokersDataClientConfig(
    ibg_host="127.0.0.1",
    ibg_port=7497,  # TWS模拟交易端口
    ibg_client_id=1,
    use_regular_trading_hours=True,  # 仅股票的RTH
    market_data_type=IBMarketDataTypeEnum.DELAYED_FROZEN,  # 使用延迟数据
    ignore_quote_tick_size_updates=False,  # 包括仅大小更新
    instrument_provider=instrument_provider_config,
    connection_timeout=300,  # 5分钟
    request_timeout=60,      # 1分钟
)
```

#### 高级数据客户端配置

```python
# 实时数据的生产配置
production_data_config = InteractiveBrokersDataClientConfig(
    ibg_host="127.0.0.1",
    ibg_port=4001,  # IB Gateway实时交易端口
    ibg_client_id=1,
    use_regular_trading_hours=False,  # 包括盘后时间
    market_data_type=IBMarketDataTypeEnum.REALTIME,  # 实时数据
    ignore_quote_tick_size_updates=True,  # 减少tick量
    handle_revised_bars=True,  # 处理K线修订
    instrument_provider=instrument_provider_config,
    dockerized_gateway=dockerized_gateway_config,  # 如果使用Docker
    connection_timeout=300,
    request_timeout=60,
)
```

#### 配置选项说明

- **`use_regular_trading_hours`**：当为`True`时，仅在常规交易时间请求数据。主要影响股票的K线数据。
- **`ignore_quote_tick_size_updates`**：当为`True`时，过滤掉仅大小变化（非价格）的报价tick，减少数据量。
- **`handle_revised_bars`**：当为`True`时，处理来自IB的K线修订（K线可在初始发布后更新）。
- **`connection_timeout`**：等待初始连接建立的最大时间。
- **`request_timeout`**：等待历史数据请求的最大时间。

### 执行客户端配置

`InteractiveBrokersExecutionClient`处理交易执行、订单管理、账户信息和仓位跟踪。它提供全面的订单生命周期管理和实时账户更新。

#### 支持的功能

- **订单管理**：下达、修改和取消订单
- **订单类型**：市价、限价、止损、止损限价、追踪止损等
- **账户信息**：实时余额和保证金更新
- **仓位跟踪**：实时仓位更新和盈亏
- **交易报告**：执行报告和成交通知
- **风险管理**：交易前风险检查和仓位限制

#### 支持的订单类型

适配器支持大多数Interactive Brokers订单类型：

- **市价订单**：`OrderType.MARKET`
- **限价订单**：`OrderType.LIMIT`
- **止损订单**：`OrderType.STOP_MARKET`
- **止损限价订单**：`OrderType.STOP_LIMIT`
- **触价市价**：`OrderType.MARKET_IF_TOUCHED`
- **触价限价**：`OrderType.LIMIT_IF_TOUCHED`
- **追踪止损市价**：`OrderType.TRAILING_STOP_MARKET`
- **追踪止损限价**：`OrderType.TRAILING_STOP_LIMIT`
- **收盘市价**：`OrderType.MARKET`与`TimeInForce.AT_THE_CLOSE`
- **收盘限价**：`OrderType.LIMIT`与`TimeInForce.AT_THE_CLOSE`

#### 有效期选项

- **日内订单**：`TimeInForce.DAY`
- **撤销前有效**：`TimeInForce.GTC`
- **立即成交或取消**：`TimeInForce.IOC`
- **全部成交或取消**：`TimeInForce.FOK`
- **指定日期前有效**：`TimeInForce.GTD`
- **开盘时**：`TimeInForce.AT_THE_OPEN`
- **收盘时**：`TimeInForce.AT_THE_CLOSE`

#### 基本执行客户端配置

```python
from nautilus_trader.adapters.interactive_brokers.config import InteractiveBrokersExecClientConfig
from nautilus_trader.config import RoutingConfig

exec_client_config = InteractiveBrokersExecClientConfig(
    ibg_host="127.0.0.1",
    ibg_port=7497,  # TWS模拟交易端口
    ibg_client_id=1,
    account_id="DU123456",  # 您的IB账户ID（模拟或实时）
    instrument_provider=instrument_provider_config,
    connection_timeout=300,
    routing=RoutingConfig(default=True),  # 通过此客户端路由所有订单
)
```

#### 高级执行客户端配置

```python
# 带有Docker化网关的生产配置
production_exec_config = InteractiveBrokersExecClientConfig(
    ibg_host="127.0.0.1",
    ibg_port=4001,  # IB Gateway实时交易端口
    ibg_client_id=1,
    account_id=None,  # 将使用TWS_ACCOUNT环境变量
    instrument_provider=instrument_provider_config,
    dockerized_gateway=dockerized_gateway_config,
    connection_timeout=300,
    routing=RoutingConfig(default=True),
)
```

#### 账户ID配置

`account_id`参数至关重要，必须与登录到TWS/Gateway的账户匹配：

```python
# 选项1：在配置中直接指定
exec_config = InteractiveBrokersExecClientConfig(
    account_id="DU123456",  # 模拟交易账户
    # ... 其他参数
)

# 选项2：使用环境变量
import os
os.environ["TWS_ACCOUNT"] = "DU123456"
exec_config = InteractiveBrokersExecClientConfig(
    account_id=None,  # 将使用TWS_ACCOUNT环境变量
    # ... 其他参数
)
```

#### 订单标签和高级功能

适配器通过订单标签支持IB特定的订单参数：

```python
from nautilus_trader.adapters.interactive_brokers.common import IBOrderTags

# 创建带有IB特定参数的订单
order_tags = IBOrderTags(
    allOrNone=True,           # 全部成交订单
    ocaGroup="MyGroup1",      # 一取消全部组
    ocaType=1,                # 阻止取消
    activeStartTime="20240315 09:30:00 EST",  # GTC激活时间
    activeStopTime="20240315 16:00:00 EST",   # GTC停用时间
    goodAfterTime="20240315 09:35:00 EST",    # 生效时间
)

# 将标签应用到订单（实现取决于您的策略代码）
```

### 完整交易节点配置

设置完整交易环境涉及为所有必要组件配置`TradingNodeConfig`。以下是不同场景的全面示例。

#### 模拟交易配置

```python
import os
from nautilus_trader.adapters.interactive_brokers.common import IB
from nautilus_trader.adapters.interactive_brokers.common import IB_VENUE
from nautilus_trader.adapters.interactive_brokers.config import InteractiveBrokersDataClientConfig
from nautilus_trader.adapters.interactive_brokers.config import InteractiveBrokersExecClientConfig
from nautilus_trader.adapters.interactive_brokers.config import InteractiveBrokersInstrumentProviderConfig
from nautilus_trader.adapters.interactive_brokers.config import IBMarketDataTypeEnum
from nautilus_trader.adapters.interactive_brokers.config import SymbologyMethod
from nautilus_trader.adapters.interactive_brokers.factories import InteractiveBrokersLiveDataClientFactory
from nautilus_trader.adapters.interactive_brokers.factories import InteractiveBrokersLiveExecClientFactory
from nautilus_trader.config import LiveDataEngineConfig
from nautilus_trader.config import LoggingConfig
from nautilus_trader.config import RoutingConfig
from nautilus_trader.config import TradingNodeConfig
from nautilus_trader.live.node import TradingNode

# 工具提供商配置
instrument_provider_config = InteractiveBrokersInstrumentProviderConfig(
    symbology_method=SymbologyMethod.IB_SIMPLIFIED,
    load_ids=frozenset([
        "EUR/USD.IDEALPRO",
        "GBP/USD.IDEALPRO",
        "SPY.ARCA",
        "QQQ.NASDAQ",
        "AAPL.NASDAQ",
        "MSFT.NASDAQ",
    ]),
)

# 数据客户端配置
data_client_config = InteractiveBrokersDataClientConfig(
    ibg_host="127.0.0.1",
    ibg_port=7497,  # TWS模拟交易
    ibg_client_id=1,
    use_regular_trading_hours=True,
    market_data_type=IBMarketDataTypeEnum.DELAYED_FROZEN,
    instrument_provider=instrument_provider_config,
)

# 执行客户端配置
exec_client_config = InteractiveBrokersExecClientConfig(
    ibg_host="127.0.0.1",
    ibg_port=7497,  # TWS模拟交易
    ibg_client_id=1,
    account_id="DU123456",  # 您的模拟交易账户
    instrument_provider=instrument_provider_config,
    routing=RoutingConfig(default=True),
)

# 交易节点配置
config_node = TradingNodeConfig(
    trader_id="PAPER-TRADER-001",
    logging=LoggingConfig(log_level="INFO"),
    data_clients={IB: data_client_config},
    exec_clients={IB: exec_client_config},
    data_engine=LiveDataEngineConfig(
        time_bars_timestamp_on_close=False,  # IB标准：使用K线开盘时间
        validate_data_sequence=True,         # 丢弃乱序K线
    ),
    timeout_connection=90.0,
    timeout_reconciliation=5.0,
    timeout_portfolio=5.0,
    timeout_disconnection=5.0,
    timeout_post_stop=2.0,
)

# 创建和配置交易节点
node = TradingNode(config=config_node)
node.add_data_client_factory(IB, InteractiveBrokersLiveDataClientFactory)
node.add_exec_client_factory(IB, InteractiveBrokersLiveExecClientFactory)
node.build()
node.portfolio.set_specific_venue(IB_VENUE)

if __name__ == "__main__":
    try:
        node.run()
    finally:
        node.dispose()
```

#### 使用Docker化网关的实时交易

```python
from nautilus_trader.adapters.interactive_brokers.config import DockerizedIBGatewayConfig

# Docker化网关配置
dockerized_gateway_config = DockerizedIBGatewayConfig(
    username=os.environ.get("TWS_USERNAME"),
    password=os.environ.get("TWS_PASSWORD"),
    trading_mode="live",  # "paper"或"live"
    read_only_api=False,  # 允许订单执行
    timeout=300,
)

# 带有Docker化网关的数据客户端
data_client_config = InteractiveBrokersDataClientConfig(
    ibg_client_id=1,
    use_regular_trading_hours=False,  # 包括盘后时间
    market_data_type=IBMarketDataTypeEnum.REALTIME,
    instrument_provider=instrument_provider_config,
    dockerized_gateway=dockerized_gateway_config,
)

# 带有Docker化网关的执行客户端
exec_client_config = InteractiveBrokersExecClientConfig(
    ibg_client_id=1,
    account_id=os.environ.get("TWS_ACCOUNT"),  # 实时账户ID
    instrument_provider=instrument_provider_config,
    dockerized_gateway=dockerized_gateway_config,
    routing=RoutingConfig(default=True),
)

# 实时交易节点配置
config_node = TradingNodeConfig(
    trader_id="LIVE-TRADER-001",
    logging=LoggingConfig(log_level="INFO"),
    data_clients={IB: data_client_config},
    exec_clients={IB: exec_client_config},
    data_engine=LiveDataEngineConfig(
        time_bars_timestamp_on_close=False,
        validate_data_sequence=True,
    ),
)
```

#### 多客户端配置

对于高级设置，您可以配置具有不同用途的多个客户端：

```python
# 具有不同客户端ID的独立数据和执行客户端
data_client_config = InteractiveBrokersDataClientConfig(
    ibg_host="127.0.0.1",
    ibg_port=7497,
    ibg_client_id=1,  # 数据客户端使用ID 1
    market_data_type=IBMarketDataTypeEnum.REALTIME,
    instrument_provider=instrument_provider_config,
)

exec_client_config = InteractiveBrokersExecClientConfig(
    ibg_host="127.0.0.1",
    ibg_port=7497,
    ibg_client_id=2,  # 执行客户端使用ID 2
    account_id="DU123456",
    instrument_provider=instrument_provider_config,
    routing=RoutingConfig(default=True),
)
```

### 运行交易节点

```python
def run_trading_node():
    """运行具有适当错误处理的交易节点。"""
    node = None
    try:
        # 创建和构建节点
        node = TradingNode(config=config_node)
        node.add_data_client_factory(IB, InteractiveBrokersLiveDataClientFactory)
        node.add_exec_client_factory(IB, InteractiveBrokersLiveExecClientFactory)
        node.build()

        # 为投资组合设置场所
        node.portfolio.set_specific_venue(IB_VENUE)

        # 在此添加您的策略
        # node.trader.add_strategy(YourStrategy())

        # 运行节点
        node.run()

    except KeyboardInterrupt:
        print("正在关闭...")
    except Exception as e:
        print(f"错误：{e}")
    finally:
        if node:
            node.dispose()

if __name__ == "__main__":
    run_trading_node()
```

### 附加配置选项

#### 环境变量

设置这些环境变量以便于配置：

```bash
export TWS_USERNAME="your_ib_username"
export TWS_PASSWORD="your_ib_password"
export TWS_ACCOUNT="your_account_id"
export IB_MAX_CONNECTION_ATTEMPTS="5"  # 可选：限制重连尝试
```

#### 日志配置

```python
# 增强的日志配置
logging_config = LoggingConfig(
    log_level="INFO",
    log_level_file="DEBUG",
    log_file_format="json",  # 结构化日志的JSON格式
    log_component_levels={
        "InteractiveBrokersClient": "DEBUG",
        "InteractiveBrokersDataClient": "INFO",
        "InteractiveBrokersExecutionClient": "INFO",
    },
)
```

您可以在此处找到其他示例：<https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/interactive_brokers>

## 故障排除

### 常见连接问题

#### 连接被拒绝

- **原因**：TWS/Gateway未运行或端口错误
- **解决方案**：验证TWS/Gateway正在运行并检查端口配置
- **默认端口**：TWS（7497/7496），IB Gateway（4002/4001）

#### 认证错误

- **原因**：凭据不正确或账户未登录
- **解决方案**：验证用户名/密码并确保账户已登录TWS/Gateway

#### 客户端ID冲突

- **原因**：多个客户端使用相同的客户端ID
- **解决方案**：为每个连接使用唯一的客户端ID

#### 市场数据权限

- **原因**：市场数据订阅不足
- **解决方案**：使用`IBMarketDataTypeEnum.DELAYED_FROZEN`进行测试或订阅所需的数据源

### 错误代码

Interactive Brokers使用特定的错误代码。常见的包括：

- **200**：未找到证券定义
- **201**：订单被拒绝 - 原因如下
- **202**：订单已取消
- **300**：找不到ticker ID的EId
- **354**：请求的市场数据未订阅
- **2104**：市场数据农场连接正常
- **2106**：HMDS数据农场连接正常

### 性能优化

#### 减少数据量

```python
# 通过忽略仅大小更新来减少报价tick量
data_config = InteractiveBrokersDataClientConfig(
    ignore_quote_tick_size_updates=True,
    # ... 其他配置
)
```

#### 连接管理

```python
# 设置合理的超时
config = InteractiveBrokersDataClientConfig(
    connection_timeout=300,  # 5分钟
    request_timeout=60,      # 1分钟
    # ... 其他配置
)
```

#### 内存管理

- 为您的策略使用适当的K线大小
- 限制同时订阅的数量
- 考虑使用历史数据进行回测而不是实时数据

### 最佳实践

#### 安全

- 永远不要在源代码中硬编码凭据
- 使用环境变量存储敏感信息
- 使用模拟交易进行开发和测试
- 为仅数据应用程序设置`read_only_api=True`

#### 开发工作流程

1. **从模拟交易开始**：始终先用模拟交易测试
2. **使用延迟数据**：为开发使用`DELAYED_FROZEN`市场数据
3. **实现适当的错误处理**：优雅地处理连接丢失和API错误
4. **监控日志**：启用适当的日志级别进行调试
5. **测试重连**：测试您的策略在连接中断期间的行为

#### 生产部署

- 为自动化部署使用Docker化网关
- 实现适当的监控和告警
- 设置日志聚合和分析
- 仅在必要时使用实时数据订阅
- 实现熔断器和仓位限制

#### 订单管理

- 提交前始终验证订单
- 实现适当的仓位规模
- 为您的策略使用适当的订单类型
- 监控订单状态并处理拒绝
- 为订单操作实现超时处理

### 调试技巧

#### 启用调试日志

```python
logging_config = LoggingConfig(
    log_level="DEBUG",
    log_component_levels={
        "InteractiveBrokersClient": "DEBUG",
    },
)
```

#### 监控连接状态

```python
# 在您的策略中检查连接状态
if not self.data_client.is_connected:
    self.log.warning("数据客户端断开连接")
```

#### 验证工具

```python
# 确保在交易前已加载工具
instruments = self.cache.instruments()
if not instruments:
    self.log.error("未加载工具")
```

### 支持和资源

- **IB API文档**：[TWS API指南](https://ibkrcampus.com/ibkr-api-page/trader-workstation-api/)
- **NautilusTrader示例**：[GitHub示例](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/live/interactive_brokers)
- **IB合约搜索**：[合约信息中心](https://pennies.interactivebrokers.com/cstools/contract_info/)
- **市场数据订阅**：[IB市场数据](https://www.interactivebrokers.com/en/trading/market-data.php)
