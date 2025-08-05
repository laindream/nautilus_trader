# nautilus-model

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-model)](https://docs.rs/nautilus-model/latest/nautilus-model/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-model.svg)](https://crates.io/crates/nautilus-model)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

[NautilusTrader](http://nautilustrader.io) 的交易领域模型。

*model* 包提供了一个类型安全的领域模型，构成了 NautilusTrader 的骨干，并可以作为其他算法交易系统的基础。

## 平台

[NautilusTrader](http://nautilustrader.io) 是一个开源、高性能、生产级别的算法交易平台，为量化交易者提供了在历史数据上使用事件驱动引擎回测自动化交易策略投资组合的能力，同时也能部署这些相同的策略进行实盘交易，无需更改代码。

NautilusTrader 的设计、架构和实现理念将软件正确性和安全性置于最高优先级，旨在支持关键任务的交易系统回测和实盘部署工作负载。

## 功能标志

此包提供功能标志来控制编译期间的源代码包含，取决于预期的使用场景，即是否为 [nautilus_trader](https://pypi.org/project/nautilus_trader) Python 包提供 Python 绑定，还是作为仅 Rust 构建的一部分。

- `ffi`: 启用来自 [cbindgen](https://github.com/mozilla/cbindgen) 的 C 外部函数接口 (FFI)。
- `python`: 启用来自 [PyO3](https://pyo3.rs) 的 Python 绑定。
- `stubs`: 启用用于测试场景的类型存根。
- `high-precision`: 启用[高精度模式](https://nautilustrader.io/docs/nightly/getting_started/installation#precision-mode)以使用 128 位值类型。
- `defi`: 启用 DeFi（去中心化金融）领域模型。

## 文档

有关更详细的使用方法，请参阅[文档](https://docs.rs/nautilus-model)。

## 许可证

NautilusTrader 的源代码在 GitHub 上以 [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html) 许可证提供。
欢迎对项目贡献，需要完成标准的[贡献者许可协议 (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™ 由 Nautech Systems 开发和维护，这是一家专门开发高性能交易系统的技术公司。
有关更多信息，请访问 <https://nautilustrader.io>。

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. 版权所有。</span> 