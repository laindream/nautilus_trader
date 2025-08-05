# 架构

欢迎来到NautilusTrader的架构概述。

本指南深入探讨支撑平台的基础原理、结构和设计。无论您是开发人员、系统架构师，还是对NautilusTrader内部工作原理感兴趣，本节涵盖：

- 驱动决策并塑造系统演进的设计哲学。
- 提供整个系统框架鸟瞰图的总体系统架构。
- 框架如何组织以促进模块化和可维护性。
- 确保可读性和可扩展性的代码结构。
- 组件组织和交互的细分，以理解不同部分如何通信和协作。
- 最后，对性能、可靠性和健壮性至关重要的实现技术。

:::note
在整个文档中，术语*"Nautilus系统边界"*指的是单个Nautilus节点运行时内的操作（也称为"交易者实例"）。
:::

## 设计哲学

NautilusTrader采用的主要架构技术和设计模式包括：

- [领域驱动设计(DDD)](https://en.wikipedia.org/wiki/Domain-driven_design)
- [事件驱动架构](https://en.wikipedia.org/wiki/Event-driven_programming)
- [消息传递模式](https://en.wikipedia.org/wiki/Messaging_pattern)（发布/订阅、请求/响应、点对点）
- [端口和适配器](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software))
- [仅崩溃设计](https://en.wikipedia.org/wiki/Crash-only_software)

这些技术已被用来帮助实现某些架构质量属性。

### 质量属性

架构决策通常是竞争优先级之间的权衡。下面是在做出设计和架构决策时考虑的一些最重要的质量属性列表，大致按"权重"顺序排列。

- 可靠性
- 性能
- 模块化
- 可测试性
- 可维护性
- 可部署性

## 系统架构

NautilusTrader代码库实际上既是组成交易系统的框架，也是一组可以在各种[环境上下文](#environment-contexts)中运行的默认系统实现。

![Architecture](https://github.com/nautechsystems/nautilus_trader/blob/develop/assets/architecture-overview.png?raw=true "architecture")

### 核心组件

平台围绕几个关键组件构建，这些组件协同工作以提供全面的交易系统：

#### NautilusKernel

负责以下功能的中央编排组件：

- 初始化和管理所有系统组件。
- 配置消息传递基础设施。
- 维护环境特定的行为。
- 协调共享资源和生命周期管理。
- 为系统操作提供统一入口点。

#### MessageBus

组件间通信的骨干，实现：

- **发布/订阅模式**：用于向多个消费者广播事件和数据。
- **请求/响应通信**：用于需要确认的操作。
- **命令/事件消息传递**：用于触发操作和通知状态更改。
- **可选状态持久化**：使用Redis进行持久性和重启功能。

#### Cache

高性能内存存储系统，可以：

- 存储工具、账户、订单、头寸等。
- 为交易组件提供高性能的获取功能。
- 在整个系统中维护一致状态。
- 支持具有优化访问模式的读写操作。

#### DataEngine

在整个系统中处理和路由市场数据：

- 处理多种数据类型（报价、交易、K线、订单簿、自定义数据等）。
- 根据订阅将数据路由到适当的消费者。
- 管理从外部源到内部组件的数据流。

#### ExecutionEngine

管理订单生命周期和执行：

- 将交易命令路由到适当的适配器客户端。
- 跟踪订单和头寸状态。
- 与风险管理系统协调。 