# nautilus-serialization

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-serialization)](https://docs.rs/nautilus-serialization/latest/nautilus-serialization/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-serialization.svg)](https://crates.io/crates/nautilus-serialization)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

[NautilusTrader](http://nautilustrader.io)的数据序列化和格式转换。

*serialization* crate提供了全面的数据序列化功能，用于在不同格式（包括Apache Arrow、Parquet和自定义模式）之间转换交易数据。这使得能够高效地进行数据存储、检索和跨不同系统的互操作性：

- **Apache Arrow集成**：市场数据类型的模式定义和编码/解码。
- **Parquet文件操作**：用于历史数据分析的高性能列式存储。
- **记录批处理**：时间序列数据的高效批处理操作。
- **模式管理**：具有元数据保存的类型安全模式定义。
- **跨格式转换**：Arrow、Parquet和原生类型之间的无缝数据交换。

## 平台

[NautilusTrader](http://nautilustrader.io)是一个开源、高性能、生产级的算法交易平台，为量化交易者提供使用事件驱动引擎在历史数据上回测自动化交易策略组合的能力，并且可以实时部署这些相同的策略，无需任何代码更改。

NautilusTrader的设计、架构和实现理念将软件正确性和安全性置于最高优先级，旨在支持关键任务、交易系统回测和实时部署工作负载。

## 功能标志

该crate提供功能标志来控制编译期间的源代码包含，取决于预期的使用场景，即是否为[nautilus_trader](https://pypi.org/project/nautilus_trader) Python包提供Python绑定，或作为纯Rust构建的一部分。

- `python`：启用来自[PyO3](https://pyo3.rs)的Python绑定。
- `high-precision`：启用[高精度模式](https://nautilustrader.io/docs/nightly/getting_started/installation#precision-mode)以使用128位值类型。

## 文档

查看[文档](https://docs.rs/nautilus-serialization)获取更详细的使用说明。

## 许可证

NautilusTrader的源代码在GitHub上可用，采用[GNU较宽通用公共许可证v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html)。欢迎为项目做出贡献，需要完成标准的[贡献者许可协议(CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™由Nautech Systems开发和维护，这是一家专门开发高性能交易系统的技术公司。更多信息请访问<https://nautilustrader.io>。

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. 保留所有权利。</span> 