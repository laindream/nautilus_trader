# 日志

平台为回测和实时交易提供日志记录，使用在Rust中实现的高性能日志子系统，并从`log` crate提供标准化外观。

核心日志记录器在单独的线程中运行，并使用多生产者单消费者（MPSC）通道接收日志消息。
这种设计确保主线程保持高性能，避免了由日志字符串格式化或文件I/O操作引起的潜在瓶颈。

日志输出是可配置的，支持：

- **stdout/stderr写入器**用于控制台输出
- **文件写入器**用于日志的持久存储

:::info
诸如[Vector](https://github.com/vectordotdev/vector)之类的基础设施可以集成以收集和聚合系统内的事件。
:::

## 配置

可以通过导入`LoggingConfig`对象配置日志记录。
默认情况下，具有'INFO' `LogLevel`及更高级别的日志事件会写入stdout/stderr。

日志级别（`LogLevel`）值包括（并且通常与Rust的`tracing`级别过滤器匹配）。

Python日志记录器公开以下级别：

- `OFF`
- `DEBUG`
- `INFO`
- `WARNING`
- `ERROR`

:::warning
Python `Logger`不提供`trace()`方法；`TRACE`级别日志仅由底层Rust组件发出，不能直接从Python代码生成。

有关更多详细信息，请参阅`LoggingConfig` [API参考](../api_reference/config_CN.md#class-loggingconfig)。
:::

日志记录可以通过以下方式配置：

- stdout/stderr的最低`LogLevel`
- 日志文件的最低`LogLevel`
- 轮换日志文件前的最大大小
- 轮换时要维护的最大备份日志文件数
- 使用日期或时间戳组件自动日志文件命名，或自定义日志文件名
- 写入日志文件的目录
- 纯文本或JSON日志文件格式
- 按日志级别过滤单个组件
- 日志行中的ANSI颜色
- 完全绕过日志记录
- 在初始化时将Rust配置打印到stdout
- 通过PyO3桥接可选地初始化日志记录（`use_pyo3`）以捕获Rust组件发出的日志事件
- 如果启动时已存在日志文件，则截断现有日志文件（`clear_log_file`）

### 标准输出日志记录

日志消息通过stdout/stderr写入器写入控制台。可以使用`log_level`参数配置最低日志级别。

### 文件日志记录

默认情况下，日志文件写入当前工作目录。命名约定和轮换行为是可配置的，并根据您的设置遵循特定模式。

您可以使用`log_directory`指定自定义日志目录和/或使用`log_file_name`指定自定义文件基本名称。日志文件始终以`.log`（纯文本）或`.json`（JSON）为后缀。

有关日志文件命名约定和轮换行为的详细信息，请参阅下面的[日志文件轮换](#log-file-rotation)和[日志文件命名约定](#log-file-naming-convention)部分。

#### 日志文件轮换

轮换行为取决于大小限制的存在和是否提供自定义文件名：

- **基于大小的轮换**：
  - 通过指定`log_file_max_size`参数启用（例如，`100_000_000`表示100 MB）。
  - 当写入日志条目会使当前文件超过此大小时，文件被关闭并创建新文件。
- **基于日期的轮换（仅默认命名）**：
  - 当未指定`log_file_max_size`且未提供自定义`log_file_name`时适用。
  - 在每个UTC日期更改（午夜）时，当前日志文件被关闭并启动新文件，每个UTC天创建一个文件。
- **无轮换**：
  - 当提供自定义`log_file_name`而没有`log_file_max_size`时，日志继续附加到同一文件。
- **备份文件管理**：
  - 由`log_file_max_backup_count`参数控制（默认：5），限制保留的轮换文件总数。
  - 超过此限制时，最旧的备份文件会自动删除。

#### 日志文件命名约定

默认命名约定确保日志文件可唯一识别并带有时间戳。
格式取决于是否启用文件轮换：

**启用文件轮换时**：

- **格式**：`{trader_id}_{%Y-%m-%d_%H%M%S:%3f}_{instance_id}.{log|json}`
- **示例**：`TESTER-001_2025-04-09_210721:521_d7dc12c8-7008-4042-8ac4-017c3db0fc38.log`
- **组件**：
  - `{trader_id}`：交易者标识符（例如，`TESTER-001`）。
  - `{%Y-%m-%d_%H%M%S:%3f}`：完整的ISO 8601兼容日期时间，具有毫秒精度。
  - `{instance_id}`：唯一实例标识符。
  - `{log|json}`：基于格式设置的文件后缀。

**禁用文件轮换时**：

- **格式**：`{trader_id}_{%Y-%m-%d}_{instance_id}.{log|json}`
- **示例**：`TESTER-001_2025-04-09_d7dc12c8-7008-4042-8ac4-017c3db0fc38.log`
- **组件**：
  - `{trader_id}`：交易者标识符。
  - `{%Y-%m-%d}`：仅日期（YYYY-MM-DD）。
  - `{instance_id}`：唯一实例标识符。
  - `{log|json}`：基于格式设置的文件后缀。

**自定义命名**：

如果设置了`log_file_name`（例如，`my_custom_log`）：

- 禁用轮换时：文件将完全按提供的名称命名（例如，`my_custom_log.log`）。
- 启用轮换时：文件将包含自定义名称和时间戳（例如，`my_custom_log_2025-04-09_210721:521.log`）。

### 组件日志过滤

`log_component_levels`参数可用于单独设置每个组件的日志级别。
输入值应该是组件ID字符串到日志级别字符串的字典：`dict[str, str]`。

下面是一个交易节点日志配置的示例，包括上述提到的一些选项：

```python
from nautilus_trader.config import LoggingConfig
from nautilus_trader.config import TradingNodeConfig

config_node = TradingNodeConfig(
    trader_id="TESTER-001",
    logging=LoggingConfig(
        log_level="INFO",
        log_level_file="DEBUG",
        log_file_format="json",
        log_component_levels={ "Portfolio": "INFO" },
    ),
    ... # 省略
)
```

对于回测，可以使用`BacktestEngineConfig`类代替`TradingNodeConfig`，因为相同的选项都可用。

### 日志颜色

使用ANSI颜色代码来增强在终端中查看日志时的可读性。
这些颜色代码可以更容易地区分日志消息的不同部分。
在不支持ANSI颜色渲染的环境中（如某些云环境或文本编辑器），
这些颜色代码可能不合适，因为它们可能显示为原始文本。

为了适应这种情况，可以将`LoggingConfig.log_colors`选项设置为`false`。
禁用`log_colors`将防止向日志消息添加ANSI颜色代码，确保
在不支持颜色渲染的不同环境中的兼容性。

## 直接使用Logger

可以直接使用`Logger`对象，这些可以在任何地方初始化（与Python内置的`logging` API非常相似）。

如果您***没有***使用已经初始化`NautilusKernel`（和日志记录）的对象，如`BacktestEngine`或`TradingNode`，
那么您可以通过以下方式激活日志记录：

```python
from nautilus_trader.common.component import init_logging
from nautilus_trader.common.component import Logger

log_guard = init_logging()
logger = Logger("MyLogger")
```

:::info
有关更多详细信息，请参阅`init_logging` [API参考](../api_reference/common_CN)。
:::

:::warning
每个进程只能通过`init_logging`调用初始化一个日志子系统，返回的`LogGuard`必须在程序的生命周期内保持活跃。
:::

## LogGuard：管理日志生命周期

`LogGuard`确保日志子系统在进程的生命周期内保持活跃和可操作。
它防止在同一进程中运行多个引擎时日志子系统过早关闭。

### 为什么使用LogGuard？

没有`LogGuard`，在同一进程中尝试运行顺序引擎可能会导致错误，如：

```
Error sending log event: [INFO] ...
```

这是因为当第一个引擎被处置时，日志子系统的底层通道和Rust `Logger`被关闭。
因此，后续引擎失去对日志子系统的访问，导致这些错误。

通过利用`LogGuard`，您可以确保在同一进程中多个回测或引擎运行时的健壮日志行为。
`LogGuard`保留日志子系统的资源并确保日志继续正确工作，
即使引擎被处置和初始化。

:::note
使用`LogGuard`对于在多个引擎的进程中维护一致的日志行为至关重要。
:::

## 运行多个引擎

以下示例演示了在同一进程中顺序运行多个引擎时如何使用`LogGuard`：

```python
log_guard = None  # 初始化LogGuard引用

for i in range(number_of_backtests):
    engine = setup_engine(...)

    # 分配LogGuard引用
    if log_guard is None:
        log_guard = engine.get_log_guard()

    # 添加角色并执行引擎
    actors = setup_actors(...)
    engine.add_actors(actors)
    engine.run()
    engine.dispose()  # 安全处置
```

### 步骤

- **一次初始化LogGuard**：`LogGuard`从第一个引擎（`engine.get_log_guard()`）获得并在整个过程中保留。这确保日志子系统保持活跃。
- **安全处置引擎**：每个引擎在其回测完成后被安全处置，而不影响日志子系统。
- **重用LogGuard**：相同的`LogGuard`实例被后续引擎重用，防止日志子系统过早关闭。

### 注意事项

- **每个进程一个LogGuard**：每个进程只能使用一个`LogGuard`。
- **线程安全**：日志子系统，包括`LogGuard`，是线程安全的，即使在多线程环境中也能确保一致的行为。
- **终止时刷新日志**：在进程终止时始终确保日志被正确刷新。`LogGuard`在超出范围时自动处理这个问题。
