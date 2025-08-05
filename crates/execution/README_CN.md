# nautilus-execution

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-execution)](https://docs.rs/nautilus-execution/latest/nautilus-execution/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-execution.svg)](https://crates.io/crates/nautilus-execution)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

[NautilusTrader](http://nautilustrader.io) 的订单执行引擎。

*execution* 包提供了一个全面的订单执行系统，处理从提交到成交处理的完整订单生命周期。这包括复杂的订单匹配、执行场所集成和高级订单类型模拟：

- **执行引擎**: 订单路由和头寸管理的中央编排。
- **订单匹配引擎**: 用于回测和模拟交易的高保真市场模拟。
- **订单模拟器**: 场所原生不支持的高级订单类型（追踪止损、条件订单）。
- **执行客户端**: 用于连接交易场所和经纪商的抽象接口。
- **订单管理器**: 本地订单生命周期管理和状态跟踪。
- **匹配核心**: 低级订单簿和价格-时间优先匹配算法。
- **费用和成交模型**: 可配置的执行成本模拟和真实成交行为。

该包同时支持实盘交易环境（使用真实执行客户端）和模拟环境（使用匹配引擎），使其适用于生产交易、策略开发和全面回测。

## 平台

[NautilusTrader](http://nautilustrader.io) 是一个开源、高性能、生产级别的算法交易平台，为量化交易者提供了在历史数据上使用事件驱动引擎回测自动化交易策略投资组合的能力，同时也能部署这些相同的策略进行实盘交易，无需更改代码。

NautilusTrader 的设计、架构和实现理念将软件正确性和安全性置于最高优先级，旨在支持关键任务的交易系统回测和实盘部署工作负载。

## 功能标志

此包提供功能标志来控制编译期间的源代码包含，取决于预期的使用场景，即是否为 [nautilus_trader](https://pypi.org/project/nautilus_trader) Python 包提供 Python 绑定，还是作为仅 Rust 构建的一部分。

- `ffi`: 启用来自 [cbindgen](https://github.com/mozilla/cbindgen) 的 C 外部函数接口 (FFI)。
- `python`: 启用来自 [PyO3](https://pyo3.rs) 的 Python 绑定。

## 文档

有关更详细的使用方法，请参阅[文档](https://docs.rs/nautilus-execution)。

## 许可证

NautilusTrader 的源代码在 GitHub 上以 [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html) 许可证提供。
欢迎对项目贡献，需要完成标准的[贡献者许可协议 (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™ 由 Nautech Systems 开发和维护，这是一家专门开发高性能交易系统的技术公司。
有关更多信息，请访问 <https://nautilustrader.io>。

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. 版权所有。</span>
