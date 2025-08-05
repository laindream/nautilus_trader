# 示例

以下代码示例按系统环境上下文组织：

- **回测（Backtest）**：历史数据与模拟交易所。
- **沙箱（Sandbox）**：实时数据与模拟交易所。
- **实盘（Live）**：实时数据与真实交易所（模拟交易或真实账户）。
- **其他（Other）**：策略之外的各种示例。

每个环境上下文目录中的脚本按集成方式组织。

在运行示例之前，请确保 `nautilus_trader` 包已通过源码编译或通过 pip 安装。
更多信息请参阅[安装指南](https://nautilustrader.io/docs/latest/getting_started/installation)。

要从 `examples` 目录执行示例脚本，请使用类似以下的命令：

```
python backtest/crypto_ema_cross_ethusdt_trade_ticks.py
``` 