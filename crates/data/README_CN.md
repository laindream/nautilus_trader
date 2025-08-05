# nautilus-data

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-data)](https://docs.rs/nautilus-data/latest/nautilus-data/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-data.svg)](https://crates.io/crates/nautilus-data)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

[NautilusTrader](http://nautilustrader.io) 的数据引擎和市场数据处理。

*data* 包在 NautilusTrader 生态系统中提供了一个全面的框架，用于处理市场数据摄取、处理和聚合。这包括实时数据流、历史数据管理和各种聚合方法：

- 用于编排数据操作的高性能数据引擎
- 用于连接市场数据提供商的数据客户端基础设施
- 支持成交、成交量、价值和基于时间聚合的K线聚合机制
- 订单簿管理和增量处理能力
- 订阅管理和数据请求处理
- 可配置的数据路由和处理管道

## 平台

[NautilusTrader](http://nautilustrader.io) 是一个开源、高性能、生产级别的算法交易平台，为量化交易者提供了在历史数据上使用事件驱动引擎回测自动化交易策略投资组合的能力，同时也能部署这些相同的策略进行实盘交易，无需更改代码。

NautilusTrader 的设计、架构和实现理念将软件正确性和安全性置于最高优先级，旨在支持关键任务的交易系统回测和实盘部署工作负载。

## 功能标志

此包提供功能标志来控制编译期间的源代码包含，取决于预期的使用场景，即是否为 [nautilus_trader](https://pypi.org/project/nautilus_trader) Python 包提供 Python 绑定，还是作为仅 Rust 构建的一部分。

- `ffi`: 启用来自 [cbindgen](https://github.com/mozilla/cbindgen) 的 C 外部函数接口 (FFI)。
- `python`: 启用来自 [PyO3](https://pyo3.rs) 的 Python 绑定。
- `high-precision`: 启用[高精度模式](https://nautilustrader.io/docs/nightly/getting_started/installation#precision-mode)以使用 128 位值类型。
- `defi`: 启用 DeFi（去中心化金融）支持。

## 文档

有关更详细的使用方法，请参阅[文档](https://docs.rs/nautilus-data)。

## 许可证

NautilusTrader 的源代码在 GitHub 上以 [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html) 许可证提供。
欢迎对项目贡献，需要完成标准的[贡献者许可协议 (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™ 由 Nautech Systems 开发和维护，这是一家专门开发高性能交易系统的技术公司。
有关更多信息，请访问 <https://nautilustrader.io>。

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. 版权所有。</span>
