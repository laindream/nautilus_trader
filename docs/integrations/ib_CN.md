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
``` 