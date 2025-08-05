# nautilus-testkit

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-testkit)](https://docs.rs/nautilus-testkit/latest/nautilus-testkit/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-testkit.svg)](https://crates.io/crates/nautilus-testkit)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

[NautilusTrader](http://nautilustrader.io) 的测试实用程序和数据管理。

*testkit* 包提供全面的测试实用程序，包括测试数据管理、文件处理和通用测试模式。该包通过自动化数据下载和验证支持整个 NautilusTrader 生态系统的强大测试工作流：

- **测试数据管理**: 自动下载和缓存测试数据集。
- **文件实用程序**: 使用 SHA-256 校验和进行文件完整性验证。
- **路径解析**: 平台无关的测试数据路径管理。
- **精度处理**: 支持 64 位和 128 位精度测试数据。
- **通用模式**: 可重用的测试实用程序和辅助函数。

## 平台

[NautilusTrader](http://nautilustrader.io) 是一个开源、高性能、生产级别的算法交易平台，为量化交易者提供了在历史数据上使用事件驱动引擎回测自动化交易策略投资组合的能力，同时也能部署这些相同的策略进行实盘交易，无需更改代码。

NautilusTrader 的设计、架构和实现理念将软件正确性和安全性置于最高优先级，旨在支持关键任务的交易系统回测和实盘部署工作负载。

## 文档

有关更详细的使用方法，请参阅[文档](https://docs.rs/nautilus-testkit)。

## 许可证

NautilusTrader 的源代码在 GitHub 上以 [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html) 许可证提供。
欢迎对项目贡献，需要完成标准的[贡献者许可协议 (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™ 由 Nautech Systems 开发和维护，这是一家专门开发高性能交易系统的技术公司。
有关更多信息，请访问 <https://nautilustrader.io>。

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. 版权所有。</span> 