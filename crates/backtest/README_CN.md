# nautilus-backtest

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-backtest)](https://docs.rs/nautilus-backtest/latest/nautilus-backtest/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-backtest.svg)](https://crates.io/crates/nautilus-backtest)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

[NautilusTrader](http://nautilustrader.io) 的回测引擎。

*backtest* 包提供了一个全面的事件驱动回测框架，允许量化交易者在历史数据上测试和验证交易策略，具有高保真的市场模拟功能。该系统复制真实的市场条件，包括：

- 具有模拟交易所的事件驱动回测引擎
- 具有可配置延迟和成交模型的市场数据回放
- 具有真实执行模拟的订单匹配引擎
- 多场所和多资产回测能力
- 全面的配置和状态管理

## 平台

[NautilusTrader](http://nautilustrader.io) 是一个开源、高性能、生产级别的算法交易平台，为量化交易者提供了在历史数据上使用事件驱动引擎回测自动化交易策略投资组合的能力，同时也能部署这些相同的策略进行实盘交易，无需更改代码。

NautilusTrader 的设计、架构和实现理念将软件正确性和安全性置于最高优先级，旨在支持关键任务的交易系统回测和实盘部署工作负载。

## 功能标志

此包提供功能标志来控制编译期间的源代码包含，取决于预期的使用场景，即是否为 [nautilus_trader](https://pypi.org/project/nautilus_trader) Python 包提供 Python 绑定，还是作为仅 Rust 构建的一部分。

- `ffi`: 启用来自 [cbindgen](https://github.com/mozilla/cbindgen) 的 C 外部函数接口 (FFI)。
- `python`: 启用来自 [PyO3](https://pyo3.rs) 的 Python 绑定。

## 文档

有关更详细的使用方法，请参阅[文档](https://docs.rs/nautilus-backtest)。

## 许可证

NautilusTrader 的源代码在 GitHub 上以 [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html) 许可证提供。
欢迎对项目贡献，需要完成标准的[贡献者许可协议 (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™ 由 Nautech Systems 开发和维护，这是一家专门开发高性能交易系统的技术公司。
有关更多信息，请访问 <https://nautilustrader.io>。

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. 版权所有。</span>
