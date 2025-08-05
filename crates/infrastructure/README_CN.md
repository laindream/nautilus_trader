# nautilus-infrastructure

[![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/nautilus-infrastructure)](https://docs.rs/nautilus-infrastructure/latest/nautilus-infrastructure/)
[![crates.io version](https://img.shields.io/crates/v/nautilus-infrastructure.svg)](https://crates.io/crates/nautilus-infrastructure)
![license](https://img.shields.io/github/license/nautechsystems/nautilus_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

[NautilusTrader](http://nautilustrader.io) 的数据库和消息基础设施。

*infrastructure* 包提供了后端数据库实现和消息总线适配器，使 NautilusTrader 能够从开发扩展到生产部署。这包括企业级数据持久化和消息传递能力：

- **Redis 集成**: 使用 Redis 的缓存数据库和消息总线实现。
- **PostgreSQL 集成**: 基于 SQL 的缓存数据库，具有全面的数据模型。
- **连接管理**: 具有重试逻辑和健康监控的鲁棒连接处理。
- **序列化选项**: 支持 JSON 和 MessagePack 编码格式。
- **Python 绑定**: PyO3 集成，实现无缝的 Python 互操作性。

该包通过功能标志支持多个数据库后端，允许用户根据其特定的部署要求和规模选择适当的基础设施组件。

## 平台

[NautilusTrader](http://nautilustrader.io) 是一个开源、高性能、生产级别的算法交易平台，为量化交易者提供了在历史数据上使用事件驱动引擎回测自动化交易策略投资组合的能力，同时也能部署这些相同的策略进行实盘交易，无需更改代码。

NautilusTrader 的设计、架构和实现理念将软件正确性和安全性置于最高优先级，旨在支持关键任务的交易系统回测和实盘部署工作负载。

## 功能标志

此包提供功能标志来控制编译期间的源代码包含，取决于预期的使用场景，即是否为 [nautilus_trader](https://pypi.org/project/nautilus_trader) Python 包提供 Python 绑定，还是作为仅 Rust 构建的一部分。

- `python`: 启用来自 [PyO3](https://pyo3.rs) 的 Python 绑定。
- `redis`: 启用 Redis 缓存数据库和消息总线后端实现。
- `sql`: 启用 SQL 模型和缓存数据库。

## 文档

有关更详细的使用方法，请参阅[文档](https://docs.rs/nautilus-infrastructure)。

## 许可证

NautilusTrader 的源代码在 GitHub 上以 [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html) 许可证提供。
欢迎对项目贡献，需要完成标准的[贡献者许可协议 (CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™ 由 Nautech Systems 开发和维护，这是一家专门开发高性能交易系统的技术公司。
有关更多信息，请访问 <https://nautilustrader.io>。

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. 版权所有。</span>
