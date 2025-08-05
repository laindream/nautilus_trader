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