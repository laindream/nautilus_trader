# nautilus-system

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-system)](https://docs.rs/nautilus-system/latest/nautilus-system/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-system.svg)](https://crates.io/crates/nautilus-system)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)

[NautilusTrader](http://nautilustrader.io)的系统级组件和编排。

*system* crate提供了用于编排交易系统的核心系统架构，包括管理所有引擎的内核、配置管理以及用于创建组件的系统级工厂：

- `NautilusKernel` - 管理引擎和组件的核心系统编排器。
- `NautilusKernelConfig` - 内核初始化的配置。
- 用于组件创建的系统构建器和工厂。

## 平台

[NautilusTrader](http://nautilustrader.io)是一个开源、高性能、生产级的算法交易平台，为量化交易者提供使用事件驱动引擎在历史数据上回测自动化交易策略组合的能力，并且可以实时部署这些相同的策略，无需任何代码更改。

NautilusTrader的设计、架构和实现理念将软件正确性和安全性置于最高优先级，旨在支持关键任务、交易系统回测和实时部署工作负载。

## 文档

查看[文档](https://docs.rs/nautilus-system)获取更详细的使用说明。

## 许可证

NautilusTrader的源代码在GitHub上可用，采用[GNU较宽通用公共许可证v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html)。欢迎为项目做出贡献，需要完成标准的[贡献者许可协议(CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™由Nautech Systems开发和维护，这是一家专门开发高性能交易系统的技术公司。更多信息请访问<https://nautilustrader.io>。

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. 保留所有权利。</span> 