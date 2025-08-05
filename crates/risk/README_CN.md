# nautilus-risk

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-risk)](https://docs.rs/nautilus-risk/latest/nautilus-risk/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-risk.svg)](https://crates.io/crates/nautilus-risk)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

[NautilusTrader](http://nautilustrader.io)的风险引擎。

`nautilus-risk` crate提供了全面的风险管理功能，包括交易前订单验证、头寸规模计算和交易控制。该系统确保交易操作保持在定义的风险参数和监管约束范围内：

- **风险引擎**：具有可配置交易状态的中央风险管理编排。
- **订单验证**：针对价格、数量、名义限额和市场条件的交易前检查。
- **头寸规模**：支持佣金和汇率的固定风险头寸规模计算。
- **交易控制**：限制、余额验证和风险暴露管理。
- **账户保护**：多币种余额检查和保证金要求验证。

## 平台

[NautilusTrader](http://nautilustrader.io)是一个开源、高性能、生产级的算法交易平台，为量化交易者提供使用事件驱动引擎在历史数据上回测自动化交易策略组合的能力，并且可以实时部署这些相同的策略，无需任何代码更改。

NautilusTrader的设计、架构和实现理念将软件正确性和安全性置于最高优先级，旨在支持关键任务、交易系统回测和实时部署工作负载。

## 功能标志

该crate提供功能标志来控制编译期间的源代码包含，取决于预期的使用场景，即是否为[nautilus_trader](https://pypi.org/project/nautilus_trader) Python包提供Python绑定，或作为纯Rust构建的一部分。

- `python`：启用来自[PyO3](https://pyo3.rs)的Python绑定。

## 文档

查看[文档](https://docs.rs/nautilus-risk)获取更详细的使用说明。

## 许可证

NautilusTrader的源代码在GitHub上可用，采用[GNU较宽通用公共许可证v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html)。欢迎为项目做出贡献，需要完成标准的[贡献者许可协议(CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™由Nautech Systems开发和维护，这是一家专门开发高性能交易系统的技术公司。更多信息请访问<https://nautilustrader.io>。

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. 保留所有权利。</span> 