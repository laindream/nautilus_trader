# NautilusTrader 1.220.0 Beta

发布于 TBD (UTC)。

### 增强功能

- 为 Interactive Brokers 添加了期权组合支持 (#2812)，感谢 @faysou
- 为 Interactive Brokers 从 `request_instruments` 添加期权链加载 (#2809)，感谢 @faysou
- 为回测添加了 `MarginModel` 概念、基础模型、配置和工厂 (#2794)，感谢 @faysou 和 @stefansimik
- 添加了额外的内置回测成交模型 (#2795)，感谢 @faysou 和 @stefansimik
- 添加了 `OrderBookDepth10DataWrangler` (#2801)，感谢 @trylovetom
- 为 `CacheConfig` 添加了 `persist_account_events` 配置选项（默认 `True` 以保持当前行为）
- 为 `Strategy` 添加了 `query_account` 方法
- 添加了 `QueryAccount` 执行消息
- 为 `TardisCSVDataLoader` 添加了流式方法
- 为 `BacktestEngine` 低级流式 API 添加了流迭代器支持
- 添加了 `YEAR` 聚合并改进了 K 线规格验证 (#2771)，感谢 @stastnypremysl
- 为 dYdX 添加了对请求任意数量历史 K 线的支持 (#2766, #2777)，感谢 @DeirhX
- 为 `StrategyConfig` 添加了 `use_hyphens_in_client_order_ids` 配置选项
- 为 `portfolio_greeks` 添加了 `greeks_filter` 函数 (#2756)，感谢 @faysou
- 为 `GreeksCalculator` 添加了时间加权和百分比 vega (#2817)，感谢 @faysou
- 为通用 make 目标添加了 `VERBOSE` 选项 (#2759)，感谢 @faysou
- 为 Redis 缓存数据库适配器添加了批量键加载功能
- 为 `CurrencyPair` 工具添加了 `multiplier` 字段（某些加密货币对需要）
- 为工具字典转换添加了 `tick_scheme_name` 字段
- 为所有有效精度添加了默认 `FixedTickScheme`

### 破坏性变更

- 为 `CurrencyPair` Arrow 模式添加了 `multiplier` 字段
- 将 `Actor` 数据请求方法的 `start` 参数改为必需
- 恢复了缓存数据库中 `delete_account_event` 的实现，因为效率太低，现在是无操作，等待重新设计
- 移除了通用的 `cvec_drop` FFI 函数，因为它未使用且容易误用，可能导致内存泄漏

### 内部改进

- 将 OKX 适配器重构为 Rust API 客户端
- 重构了 `BacktestDataIterator` (#2791) 以整合数据生成器使用，感谢 @faysou
- 引入了 `SharedCell` / `WeakCell` 包装器，用于人体工程学和更安全地处理 `Rc<RefCell<T>>` / `Weak<RefCell<T>>` 对
- 为 `BacktestDataIterator` 添加了流迭代器支持
- 为执行报告添加了序列化支持
- 为执行报告命令添加了序列化支持
- 为集成适配器添加了 `DataTester` 标准化数据测试参与者
- 为响应数据添加了 `start` 和 `stop` (#2748)，感谢 @stastnypremysl
- 添加了集成测试服务管理目标 (#2765)，感谢 @stastnypremysl
- 为 dYdX K 线分区和大历史处理添加了集成测试 (#2773)，感谢 @nicolad
- 添加了 make build-debug-pyo3 (#2802)，感谢 @faysou
- 优化了标识符哈希以避免使用 C 字符串频繁重新计算
- 优化了数据引擎主题字符串缓存，用于消息总线发布，避免频繁的 f-string 构造
- 优化了 Redis 键扫描以提高网络效率
- 完成了 OKX 的 K 线请求实现 (#2789)，感谢 @nicolad
- 使用 `pytest-xdist` 启用了并行 pytest 测试 (#2808)，感谢 @stastnypremysl
- 为 `InstrumentId` 标准化了 DeFi 链名称验证 (#2826)，感谢 @filipmacek
- 改进了内部生成订单的对账处理以对齐仓位（现在使用 `INTERNAL-DIFF` 策略 ID）
- 改进了区块链适配器的数据客户端 (#2787)，感谢 @filipmacek
- 改进了区块链适配器中的 DEX 池同步过程 (#2796)，感谢 @filipmacek
- 改进了消息总线外部流缓冲区刷新的效率
- 改进了 `databento_test_request_bars` 示例 (#2762)，感谢 @faysou
- 改进了 Tardis CSV 加载器对零大小交易的处理（将记录警告）
- 改进了 `TardisInstrumentProvider` 日期时间过滤器参数的人体工程学（可以是 `pd.Timestamp` 或 Unix 纳秒 `int`）
- 改进了 Tardis Machine websocket 连接错误的处理
- 优化了 Rust 目录路径处理 (#2743)，感谢 @faysou
- 优化了 Rust `GreeksCalculator` (#2760)，感谢 @faysou
- 优化了 Databento K 线时间戳解码和回测执行使用 (#2800)，感谢 @faysou
- 优化了 `FillModel` (#2795)，感谢 @faysou 和 @stefansimik
- 优化了工具请求 (#2822)，感谢 @faysou
- 更新了 `request_aggregated_bars` 示例 (#2815)，感谢 @faysou
- 更新了 PostgreSQL 连接参数以使用 'nautilus' 用户 (#2805)，感谢 @stastnypremysl
- 为 Polymarket 津贴脚本升级了 `web3` (#2814)，感谢 @DeirhX
- 将 `databento` crate 升级到 v0.30.0
- 将 `datafusion` crate 升级到 v49.0.0
- 将 `redis` crate 升级到 v0.32.4
- 将 `tokio` crate 升级到 v1.47.1

### 修复

- 通过在回调持有结构中用普通 `PyObject` 替换 `Arc<PyObject>` 修复了 Rust-Python 引用循环，消除了内存泄漏
- 修复了 FFI 层中 Python 回调引用的 `TimeEventHandler` 内存泄漏
- 通过添加析构函数来启用正确的 Rust 值清理，修复了 `PyCapsule` 内存泄漏
- 使用新的 `SharedCell`/`WeakCell` 助手修复了网络和 K 线 Python 回调的多个循环依赖内存泄漏
- 修复了高精度模式下匹配引擎的原始价格类型错误，该错误可能在交易处理期间溢出 (#2810)，感谢报告 @Frzgunr1 和 @happysammy
- 修复了 Databento MBO 数据的零大小交易解码
- 修复了条件订单的清除，其中开放的链接订单仍会被清除
- 修复了 Tardis Machine 重放处理和 Parquet 文件写入
- 修复了 Kraken Futures 的 Tardis 交易所-场所映射（应映射到 `cryptofacilities`）
- 修复了签名类型 2 交易的 Polymarket 对账，其中钱包地址与资助者地址不同
- 修复了相同类型的多个工具的目录查询 (#2772)，感谢 @faysou
- 修复了回测中条件订单的修改 (#2761)，感谢 faysou
- 修复了订单成交时的余额计算，允许在接近账户余额容量时运行 (#2752)，感谢 @petioptrv
- 修复了某些 databento 请求函数中的时间范围结束 (#2755)，感谢 @faysou
- 修复了 Interactive Brokers 的 EOD K 线 (#2764)，感谢 @faysou
- 修复了 dYdX 止盈订单类型映射错误 (#2758)，感谢 @nicolad
- 修复了 dYdX 适配器日志记录中的拼写错误 (#2790)，感谢 @DeirhX
- 修复了 OKX 的 K 线请求分页逻辑 (#2798)，感谢 @nicolad
- 修复了 dYdX 订单和成交消息模式 (#2824)，感谢 @davidsblom
- 修复了 Binance 现货测试网流式 URL

### 文档更新

- 添加了 FFI 内存合约开发指南
- 添加了混合调试说明和示例 (#2806)，感谢 @faysou
- 改进了 dYdX 集成指南 (#2751)，感谢 @nicolad

### 弃用
无

---

# NautilusTrader 1.219.0 Beta

发布于 2025年7月5日 (UTC)。

### 增强功能

- 为实时引擎添加了 `graceful_shutdown_on_exception` 配置选项（默认 `False` 以保持在意外系统异常时的预期硬崩溃）
- 为 `LiveExecEngineConfig` 添加了 `purge_from_database` 配置选项以支持缓存备份数据库管理
- 添加了回测期间数据下载支持 (#2652)，感谢 @faysou
- 为目录添加了删除数据范围功能 (#2744)，感谢 @faysou
- 为目录添加了按周期合并功能 (#2727)，感谢 @faysou
- 为定时器添加了 `fire_immediately` 标志参数，其中时间事件将在 `start` 时刻触发，然后每隔一个间隔触发（默认 `False` 以保持当前行为）(#2600)，感谢想法 @stastnypremysl
- 为 `DataEngineConfig` 添加了 `time_bars_build_delay` 配置选项 (#2676)，感谢 @faysou
- 为时间警报添加了立即触发功能和相应测试 (#2745)，感谢 @stastnypremysl
- 为某些工具添加了缺失的序列化映射 (#2702)，感谢 @faysou
- 为区块链适配器添加了 DEX 交换支持 (#2683)，感谢 @filipmacek
- 为区块链适配器添加了池流动性更新支持 (#2692)，感谢 @filipmacek
- 当与现有成交存在差异时添加了成交报告对账警告 (#2706)，感谢 @faysou
- 为自定义数据查询添加了可选元数据函数 (#2724)，感谢 @faysou
- 在沙盒执行客户端中添加了订单列表提交支持 (#2714)，感谢 @petioptrv
- 为 IBKR 添加了隐藏订单支持 (#2739)，感谢 @sunlei
- 为 IBKR 添加了 `subscribe_order_book_deltas` 支持 (#2749)，感谢 @sunlei
- 为 `OrderBook.pprint` 添加了 `bid_levels` 和 `ask_levels`
- 为 `Cache.own_bid_orders(...)` 和 `Cache.own_ask_orders(...)` 添加了 `accepted_buffer_ns` 过滤器参数
- 在 Rust 中添加了追踪止损订单 `activation_price` 支持 (#2750)，感谢 @nicolad

### 破坏性变更

- 更改了定时器 `allow_past=False` 行为：现在验证 `next_event_time` 而不是 `start_time`。这允许具有过去开始时间的定时器，只要它们的下一个计划事件仍在未来
- 更改了定时器 `allow_past=False` 的行为，如果下一个事件时间仍在未来，则允许过去的开始时间
- 将 Databento DBN 升级策略更改为默认 v3
- 从 `ParquetDataCatalog.write_data(...)` 移除了 `basename_template`，运行 `catalog.reset_catalog_file_names()` 以将文件名更新为新约定
- 移除了保证金账户的有问题的负余额检查（现金账户负余额检查保持不变）
- 移除了对 Databento DBN v1 模式的支持（迁移到 DBN v2 或 v3，参见 [DBN 更新日志](https://github.com/databento/dbn/blob/main/CHANGELOG.md#0350---2025-05-28)）

### 内部改进

- 在 Rust 中为自定义组件和颜色添加了日志宏
- 为定时器操作添加了 Cython 级参数验证，以防止 Rust panic 并提供更清晰的 Python 错误消息
- 在 Rust 中为 `Price`、`Quantity`、`Money` 值类型添加了基于属性的测试
- 在 Rust 中为 `UnixNanos` 添加了基于属性的测试
- 在 Rust 中为 `OrderBook` 添加了基于属性的测试
- 在 Rust 中为 `TestTimer` 添加了基于属性的测试
- 在 Rust 中为 `network` crate 添加了基于属性的测试
- 在 Rust 中为套接字客户端添加了使用 `turmoil` 的混沌测试
- 添加了 `check_positive_decimal` 正确性函数并用于工具验证 (#2736)，感谢 @nicolad
- 添加了 `check_positive_money` 正确性函数并用于工具验证 (#2738)，感谢 @nicolad
- 将数据目录重构移植到 Rust (#2681, #2720)，感谢 @faysou
- 优化了 `TardisCSVDataLoader` 性能（~90% 内存使用减少，~60-70% 更快）
- 整合了来自 @twitu 的时钟和定时器 v2 功能
- 整合了纯 Rust 加密库，不依赖本地证书或 openssl
- 整合了 `aws-lc-rs` 加密以符合 FIPS
- 确认了 Cython 和 Rust 指标之间的对等性 (#2700, #2710, #2713)，感谢 @nicolad
- 实现了 `From<Pool>` -> `CurrencyPair` & `InstrumentAny` (#2693)，感谢 @nicolad
- 更新了 `Makefile` 以使用新的 docker compose 语法 (#2746)，感谢 @stastnypremysl
- 更新了 Tardis 交易所映射
- 改进了实时引擎消息处理，确保意外异常导致立即硬崩溃而不是继续而不处理队列消息
- 改进了实时对账稳健性和测试
- 改进了 Binance 的监听键错误处理和恢复
- 改进了回测中负余额的处理 (#2730)，感谢 @ms32035
- 改进了现金和保证金账户锁定余额计算的稳健性，以避免负自由余额
- 改进了 Betfair 成交价格解析的稳健性
- 改进了 Rust 工具的实现、验证和测试 (#2723, #2733)，感谢 @nicolad
- 改进了 `Currency` 相等性以使用 `strcmp` 来避免 `ustr` 字符串内部化的 C 指针比较问题
- 改进了 Bybit 适配器的取消订阅清理
- 改进了 `Makefile` 以具有自文档化功能 (#2741)，感谢 @sunlei
- 重构了 IB 适配器 (#2647)，感谢 @faysou
- 重构了数据目录 (#2652, #2740)，感谢 @faysou
- 优化了 Rust 数据目录 (#2734)，感谢 @faysou
- 优化了日志子系统生命周期管理并引入全局日志发送器
- 优化了信号序列化和测试 (#2705)，感谢 @faysou
- 优化了 CI/CD 和构建系统 (#2707)，感谢 @stastnypremysl
- 将 Rust (MSRV) 升级到 1.88.0
- 将 Cython 升级到 v3.1.2
- 将 `databento` crate 升级到 v0.28.0
- 将 `datafusion` crate 升级到 v48.0.0
- 将 `pyo3` 和 `pyo3-async-runtimes` crates 升级到 v0.25.1
- 将 `redis` crate 升级到 v0.32.3
- 将 `tokio` crate 升级到 v1.46.1
- 将 `tokio-tungstenite` crate 升级到 v0.27.0

### 修复

- 修复了 `AccountState` 事件中的 `AccountBalance` 变异 (#2701)，感谢报告 @DeirhX
- 修复了更新和删除操作中的订单簿缓存一致性（通过基于属性的测试发现）
- 修复了 Polymarket 的订单状态报告生成，其中 `venue_order_id` 是无界的
- 修复了 `LiveDataClient` 的数据请求标识符属性访问
- 修复了 Binance 执行客户端中的 `generate_order_modify_rejected` 拼写错误 (#2682)，感谢报告 @etiennepar
- 修复了 Binance 订阅中的订单簿深度处理
- 修复了 Binance 空 K 线请求的潜在 `IndexError`
- 修复了 Binance 的 GTD-GTC 时间有效性转换
- 修复了 Binance 触发类型的错误日志记录
- 修复了 Binance 交易 tick 取消订阅，该操作没有区分聚合交易
- 修复了 Betfair 执行客户端的待处理更新热缓存清理
- 修复了 Betfair 执行客户端账户更新时的无效会话信息
- 修复了 Tardis 数据客户端的订单簿快照取消订阅
- 修复了 `BinanceBar` 的 Arrow 模式注册
- 修复了运行 dYdX 集成测试时的 gRPC 服务器关闭警告
- 修复了 `BinanceBar` 的编码器和解码器注册，感谢报告 @miller-moore
- 修复了 Binance 的现货和期货沙盒 (#2687)，感谢 @petioptrv
- 修复了 `clean` 和 `distclean` make 目标进入 `.venv` 并破坏 Python 虚拟环境，感谢 @faysou
- 修复了目录标识符匹配以精确匹配 (#2732)，感谢 @faysou
- 修复了 RSI 指标的最后值更新 (#2703)，感谢 @bartlaw
- 修复了 IBKR 的网关/TWS 重连过程 (#2710)，感谢 @bartlaw
- 修复了 Interactive Brokers 期权链问题 (#2711)，感谢 @FGU1
- 修复了 IBKR 的部分成交括号订单和 SL 触发 (#2704, #2717)，感谢 @bartlaw
- 修复了 Databento 美国股票没有 `exchange` 值时的工具消息解码
- 修复了 `Binance` 单个工具交易费用获取，感谢 @petioptrv
- 修复了国际语言的 IB-TWS 连接问题 (#2726)，感谢 @DracheShiki
- 修复了 Bybit 的 K 线请求，其中分页不正确限制了返回的 K 线
- 修复了 Bybit 未知错误 (#2742)，感谢 @DeevsDeevs
- 修复了 Bybit 的保证金余额解析
- 恢复了 IBKR 的任务错误日志 (#2716)，感谢 @bartlaw

### 文档更新

- 更新了 IB 适配器文档 (#2729)，感谢 @faysou
- 改进了实时概念指南中的对账文档

### 弃用

- 弃用了 `Portfolio.set_specific_venue(...)`，将在未来版本中移除；请改用 `Cache.set_specific_venue(...)`

---

# NautilusTrader 1.218.0 Beta

发布于 2025年5月31日 (UTC)。

### 增强功能

- 为 Betfair 适配器添加了便利的重新导出（常量、配置、工厂、类型）
- 为 Binance 适配器添加了便利的重新导出（常量、配置、工厂、加载器、类型）
- 为 Bybit 适配器添加了便利的重新导出（常量、配置、工厂、加载器、类型）
- 为 Coinbase International 适配器添加了便利的重新导出（常量、配置、工厂）
- 为 Databento 适配器添加了便利的重新导出（常量、配置、工厂、加载器、类型）
- 为 dYdX 适配器添加了便利的重新导出（常量、配置、工厂）
- 为 Polymarket 适配器添加了便利的重新导出（常量、配置、工厂）
- 为 Tardis 适配器添加了便利的重新导出（常量、配置、工厂、加载器）
- 在 BacktestNode 中添加了对 `FillModel`、`LatencyModel` 和 `FeeModel` 的支持 (#2601)，感谢 @faysou
- 从 `request_aggregated_bars` 添加了 K 线缓存 (#2649)，感谢 @faysou
- 为回测引擎添加了 `BacktestDataIterator` 以提供即时数据加载 (#2545)，感谢 @faysou
- 从目录添加了 `MarkPriceUpdate` 流式支持 (#2582)，感谢 @bartolootrit
- 添加了对 Binance Futures 保证金类型的支持 (#2660)，感谢 @bartolootrit
- 添加了对所有市场中 Binance 标记价格流的支持 (#2670)，感谢 @sunlei
- 为 Databento 添加了 `bars_timestamp_on_close` 配置选项，默认为 `True` 以与 Nautilus 约定保持一致
- 为追踪止损订单添加了 `activation_price` 支持 (#2610)，感谢 @hope2see
- 为 OrderFactory 括号订单添加了追踪止损 (#2654)，感谢 @hope2see
- 为 `BacktestRunConfig` 添加了 `raise_exception` 配置选项（默认 `False` 以保持当前行为），这将引发异常以中断节点运行过程
- 添加了 `UnixNanos::is_zero()` 便利方法来检查零/纪元值
- 为 `OrderCancelRejected` 添加了 SQL 模式、模型和查询
- 为 `OrderModifyRejected` 添加了 SQL 模式、模型和查询
- 为区块链适配器添加了 HyperSync 客户端 (#2606)，感谢 @filipmacek
- 为区块链适配器添加了对 DEX、池和代币的支持 (#2638)，感谢 @filipmacek

### 破坏性变更

- 更改追踪止损以使用 `activation_price` 而不是 `trigger_price` 用于 Binance，以更紧密地匹配 Binance API 约定

### 内部改进

- 为追踪止损订单添加了 `activation_price` str 和 repr 测试 (#2620)，感谢 @hope2see
- 为订单 `contingency_type` 和 `linked_order_ids` 添加了条件检查，其中条件应该有关联的链接订单 ID
- 改进了套接字客户端重连和断开连接的稳健性，以避免状态竞争条件
- 改进了套接字客户端的错误处理，现在将在发送错误时引发 Python 异常而不是仅使用 `tracing` 记录
- 通过将许多 unwrap 更改为记录或引发 Python 异常（在适用的情况下）改进了 Databento 适配器的错误处理
- 通过将许多 unwrap 更改为记录或引发 Python 异常（在适用的情况下）改进了 Tardis 适配器的错误处理
- 改进了 `L1_MBP` 订单簿中限价订单的成交行为，现在当可成交时将作为 `TAKER` 成交整个大小，或者当市场穿过限价时作为 `MAKER`
- 改进了保证金账户的账户状态事件生成，避免为同一执行事件生成冗余的中间账户状态
- 改进了 Rust 中消息主题、模式和端点的人体工程学 (#2658)，感谢 @twitu
- 通过 cranelift 后端改进了 Rust 的开发调试构建 (#2640)，感谢 @twitu
- 改进了 Rust 中 `LimitOrder` 的验证 (#2613)，感谢 @nicolad
- 改进了 Rust 中 `LimitIfTouchedOrder` 的验证 (#2533)，感谢 @nicolad
- 改进了 Rust 中 `MarketIfTouchedOrder` 的验证 (#2577)，感谢 @nicolad
- 改进了 Rust 中 `MarketToLimitOrder` 的验证 (#2584)，感谢 @nicolad
- 改进了 Rust 中 `StopLimitOrder` 的验证 (#2593)，感谢 @nicolad
- 改进了 Rust 中 `StopMarketOrder` 的验证 (#2596)，感谢 @nicolad
- 改进了 Rust 中 `TrailingStopMarketOrder` 的验证 (#2607)，感谢 @nicolad
- 改进了 Rust 中的订单初始化和显示测试 (#2617)，感谢 @nicolad
- 改进了 Rust 订单模块的测试 (#2578)，感谢 @dakshbtc
- 改进了 `AdaptiveMovingAverage` (AMA) 的 Cython-Rust 指标对等性 (#2626)，感谢 @nicolad
- 改进了 `DoubleExponentialMovingAverage` (DEMA) 的 Cython-Rust 指标对等性 (#2633)，感谢 @nicolad
- 改进了 `ExponentialMovingAverage` (EMA) 的 Cython-Rust 指标对等性 (#2642)，感谢 @nicolad
- 改进了 `HullMovingAverage` (HMA) 的 Cython-Rust 指标对等性 (#2648)，感谢 @nicolad
- 改进了 `LinearRegression` 的 Cython-Rust 指标对等性 (#2651)，感谢 @nicolad
- 改进了 `WilderMovingAverage` (RMA) 的 Cython-Rust 指标对等性 (#2653)，感谢 @nicolad
- 改进了 `VariableIndexDynamicAverage` (VIDYA) 的 Cython-Rust 指标对等性 (#2659)，感谢 @nicolad
- 改进了 `SimpleMovingAverage` (SMA) 的 Cython-Rust 指标对等性 (#2655)，感谢 @nicolad
- 改进了 `VolumeWeightedAveragePrice` (VWAP) 的 Cython-Rust 指标对等性 (#2661)，感谢 @nicolad
- 改进了 `WeightedMovingAverage` (WMA) 的 Cython-Rust 指标对等性 (#2662)，感谢 @nicolad
- 改进了 `ArcherMovingAveragesTrends` (AMAT) 的 Cython-Rust 指标对等性 (#2669)，感谢 @nicolad
- 改进了 Binance Futures 的零大小交易日志记录 (#2588)，感谢 @bartolootrit
- 改进了 Polymarket API 密钥认证错误的错误处理
- 改进了 Polymarket 执行客户端调试日志记录
- 改进了从缓存数据库反序列化订单时的异常
- 改进了值类型的 `None` 条件检查，现在引发 `TypeError` 而不是晦涩的 `AttributeError`
- 在 SMA 指标中将 `VecDeque` 更改为固定容量 `ArrayDeque` (#2666)，感谢 @nicolad
- 在 LinearRegression 中将 `VecDeque` 更改为固定容量 `ArrayDeque` (#2667)，感谢 @nicolad
- 在 Rust 中实现了订单的剩余 Display (#2614)，感谢 @nicolad
- 为 dYdX 和 Bybit 实现了 `_subscribe_instrument` (#2636)，感谢 @davidsblom
- 从 `python` 标志中解开了 `ratelimiter` 配额 (#2595)，感谢 @twitu
- 优化了 `BacktestDataIterator` 正确性 (#2591)，感谢 @faysou
- 优化了 IB 适配器文件的格式 (#2639)，感谢 @faysou
- 通过 100× 优化了 Rust 中的消息总线主题匹配逻辑 (#2634)，感谢 @twitu
- 从 Rust 更改为更快的消息总线模式匹配逻辑 (#2643)，感谢 @twitu
- 将 Rust (MSRV) 升级到 1.87.0
- 将 Cython 升级到 v3.1.0（现在稳定）
- 将 `databento` crate 升级到 v0.26.0
- 将 `redis` crate 升级到 v0.31.0
- 将 `sqlx` crate 升级到 v0.8.6
- 将 `tokio` crate 升级到 v1.45.1

### 修复

- 修复了导致余额错误的投资组合账户更新 (#2632, #2637)，感谢报告 @bartolootrit 和 @DeirhX
- 修复了投资组合对 `OrderExpired` 事件的处理不更新状态（保证金要求可能改变）
- 修复了 `ExecutionEngine` 的事件处理，使其在发布执行事件之前完全更新 `Portfolio` (#2513)，感谢报告 @stastnypremysl
- 修复了保证金账户在仓位翻转时的 PnL 计算 (#2657)，感谢报告 @Egisess
- 修复了使用报价数量订单时的名义价值交易前风险检查 (#2628)，感谢报告 @DeevsDeevs
- 修复了 `ExecutionEngine` 的仓位快照缓存访问
- 通过简单使用 `pickle` 往返来复制仓位实例，修复了调用 `copy.deepcopy()` 的仓位快照 `SystemError`
- 修复了账户和仓位的事件清除边缘情况，其中必须保证至少一个事件
- 修复了在提供密码但没有用户名时的 Redis 认证
- 修复了各种 numpy 和 pandas FutureWarning
- 修复了重置时套接字指数退避立即重连值（这阻止了下一个重连序列的立即重连）
- 修复了 Rust 中的消息总线订阅匹配逻辑 (#2646)，感谢 @twitu
- 修复了当顶级耗尽时追踪止损市价成交行为以与市价订单对齐 (#2540)，感谢报告 @stastnypremysl
- 修复了初始触发时的止损限价成交行为，其中限价订单继续作为 taker 成交超过可用流动性，感谢报告 @hope2see
- 修复了当攻击方为 `NO_AGGRESSOR` 时的匹配引擎交易处理（我们仍然可以更新匹配核心）
- 修复了追踪止损订单的修改和更新 (#2619)，感谢 @hope2see
- 修复了当没有触发价格时处理激活的追踪止损更新，感谢报告 @hope2see
- 修复了流式传输时在 `AccountError` 上终止回测，需要重新引发异常以中断块的流式传输 (#2546)，感谢报告 @stastnypremysl
- 修复了 Bybit 的 HTTP 批量订单操作 (#2627)，感谢 @sunlei
- 修复了 Bybit 批量下单中的 `reduce_only` 属性访问
- 修复了 Polymarket 单边订单簿的报价 tick 解析
- 修复了 Polymarket 上 `MAKER` 流动性方的限价订单的订单成交处理
- 修复了 Polymarket 上 `BinaryOption` 的货币解析，以一致使用 USDC.e（Polygon 上的 PoS USDC）
- 修复了 Betfair 保持活动期间的身份错误处理，现在将重连
- 使用额外变体更新了 `BinanceFuturesEventType` 枚举，感谢报告 @miller-moore

### 文档更新

- 为集成指南添加了能力矩阵
- 为架构概念指南添加了内容
- 为实时交易概念指南添加了内容
- 为开发者指南添加了内容
- 为大多数 crate 添加了错误和 panic 文档
- 为大多数 crate 添加了错误和 panic 文档
- 改进了各种概念指南的清晰度
- 修复了概念指南中的几个错误

### 弃用

- 弃用了对 Databento [工具定义](https://databento.com/docs/schemas-and-data-formats/instrument-definitions) v1 数据的支持，v2 和 v3 继续受支持，v1 数据可以迁移（参见 Databento 文档）

---

# NautilusTrader 1.217.0 Beta

发布于 2025年4月30日 (UTC)。

### 增强功能

- 为 `BacktestEngine` 和 `OrderMatchingEngine` 添加了 `OrderBookDepth10` 处理 (#2542)，感谢 @limx0
- 添加了 `Actor.subscribe_order_book_depth(...)` 订阅方法 (#2555)，感谢 @limx0
- 添加了 `Actor.unsubscribe_order_book_depth(...)` 订阅方法
- 添加了 `Actor.on_order_book_depth(...)` 处理方法 (#2555)，感谢 @limx0
- 添加了 `UnixNanos::max()` 便利方法用于最大有效值
- 为 `TardisInstrumentProvider` 添加了 `available_offset` 过滤器参数
- 为通用 tokio 运行时构建器添加了 `NAUTILUS_WORKER_THREADS` 环境变量
- 添加了 `Quantity::non_zero(...)` 方法
- 添加了 `Quantity::non_zero_checked(...)` 方法
- 为 `Instrument.make_qty(...)` 添加了 `round_down` 参数，默认为 `False` 以保持当前行为
- 为 Bybit 添加了 WebSocket 批量订单操作 (#2521)，感谢 @sunlei
- 为 Binance Futures 添加了标记价格订阅 (#2548)，感谢 @bartolootrit
- 添加了 `Chain` 结构以表示区块链网络 (#2526)，感谢 @filipmacek
- 为区块链领域模型添加了 `Block` 原语 (#2535)，感谢 @filipmacek
- 为区块链领域模型添加了 `Transaction` 原语 (#2551)，感谢 @filipmacek
- 添加了带有实时区块订阅的初始区块链适配器 (#2557)，感谢 @filipmacek

### 破坏性变更

- 从 `CASH` 账户的锁定余额计算中移除了费用
- 从 `MARGIN` 账户的保证金计算中移除了费用
- 将所有 PyO3 工具中的 `id` 构造函数参数重命名为 `instrument_id`，与等效的 Cython 工具构造函数对齐

### 内部改进

- 为 `RetryManager` 实现了指数退避和抖动 (#2518)，感谢 @davidsblom
- 简化了默认锁定余额和保证金计算，不包括费用
- 改进了 `TardisInstrumentProvider` 的时间范围和有效日期过滤器处理
- 改进了 Bybit 私有/交易频道的重连稳健性 (#2520)，感谢 @sunlei
- 改进了回测后的日志器缓冲区刷新
- 改进了 Tardis 交易数据的验证
- 改进了 `ExecutionEngine` 客户端注册和注销的正确性
- 通过仅编译库改进了构建时间 (#2539)，感谢 @twitu
- 改进了日志刷新 (#2568)，感谢 @faysou
- 改进了 `clear_log_file` 以在每次内核初始化时发生 (#2569)，感谢 @faysou
- 优化了 `Price` 和 `Quantity` 验证和正确性
- 如果订单已成交，则过滤 dYdX 的成交事件 (#2547)，感谢 @davidsblom
- 修复了一些 clippy lint (#2517)，感谢 @twitu
- 将 `databento` crate 升级到 v0.24.0
- 将 `datafusion` crate 升级到 v47.0.0
- 将 `redis` crate 升级到 v0.30.0
- 将 `sqlx` crate 升级到 v0.8.5
- 将 `pyo3` crate 升级到 v0.24.2

### 修复

- 修复了执行事件的一致排序 (#2513, #2554)，感谢报告 @stastnypremysl
- 修复了为没有经过时间的回测生成经过时间时的类型错误
- 通过简化获取-释放模式修复了 `RetryManager` 中的内存泄漏，避免了导致状态共享的异步上下文管理器协议，感谢报告 @DeevsDeevs
- 修复了仅减少订单的锁定余额和初始保证金计算 (#2505)，感谢报告 @stastnypremysl
- 修复了从仓位清除订单事件（这些需要在移除缓存索引条目之前清除），感谢 @DeevsDeevs
- 修复了格式化为 `None` 的回测运行后时间戳时的 `TypeError` (#2514)，感谢报告 @stastnypremysl
- 修复了 `BetfairSequenceCompleted` 作为自定义数据的处理
- 修复了 `IndexInstrument` 的工具类别，更改为 `SPOT` 以正确表示基础成分的现货指数
- 修复了 `DataEngine` 的数据范围请求 `end` 处理
- 修复了 `DataEngine` 的取消订阅工具关闭
- 修复了 OKX 的网络客户端认证 (#2553)，感谢报告 @S3toGreen
- 修复了 dYdX 的账户余额计算 (#2563)，感谢 @davidsblom
- 修复了 databento 历史数据的 `ts_init` (#2566)，感谢 @faysou
- 修复了 `query_catalog` 中的 `RequestInstrument` (#2567)，感谢 @faysou
- 恢复了在 UTC 日期更改时移除轮换日志文件 (#2552)，感谢 @twitu

### 文档更新

- 使用推荐的 rust analyzer 设置改进了环境设置指南 (#2538)，感谢 @twitu
- 修复了一些 `ExecutionEngine` 文档字符串与代码的对齐

### 弃用
无

---

# NautilusTrader 1.216.0 Beta

发布于 2025年4月13日 (UTC)。

此版本添加了对 Python 3.13 的支持（*尚未*与自由线程兼容），并引入了对 ARM64 架构上 Linux 的支持。

### 增强功能

- 为 `Clock.set_timer(...)` 添加了 `allow_past` 布尔标志以控制过去开始时间的行为（默认 `True` 允许过去的开始时间）
- 为 `Clock.set_time_alert(...)` 添加了 `allow_past` 布尔标志以控制过去警报时间的行为（默认 `True` 立即触发警报）
- 为 GTD 订单到期时间添加了风险引擎检查，如果到期时间已经过去将拒绝
- 为交易所和匹配引擎添加了工具更新
- 为匹配引擎添加了额外的价格和数量精度验证
- 添加了日志文件轮换和额外配置选项 `max_file_size` 和 `max_backup_count` (#2468)，感谢 @xingyanan 和 @twitu
- 为 `BybitDataClientConfig` 添加了 `bars_timestamp_on_close` 配置选项（默认 `True` 以匹配 Nautilus 约定）
- 为 Betfair 添加了 `BetfairSequenceCompleted` 自定义数据类型以标记消息序列的完成
- 在 Rust 中为 `MarkPriceUpdate` 添加了 Arrow 模式
- 在 Rust 中为 `IndexPriceUpdate` 添加了 Arrow 模式
- 在 Rust 中为 `InstrumentClose` 添加了 Arrow 模式
- 添加了 `BookLevel.side` 属性
- 添加了 `Position.closing_order_side()` 实例方法
- 改进了 `LiveExecutionEngine` 的飞行中订单检查稳健性，一旦超过查询重试将解决提交的订单为拒绝，待处理的订单为取消
- 改进了 `BacktestNode` 崩溃的日志记录，带有完整堆栈跟踪和更美观的配置日志记录

### 破坏性变更

- 将 Bybit 的外部 K 线请求 `ts_event` 时间戳从开盘时更改为收盘时

### 内部改进

- 添加了 Betfair 零大小成交的处理和警告
- 改进了 dYdX 的 WebSocket 错误处理 (#2499)，感谢 @davidsblom
- 将 `GreeksCalculator` 移植到 Rust (#2493, #2496)，感谢 @faysou
- 将 Cython 升级到 v3.1.0b1
- 将 `redis` crate 升级到 v0.29.5
- 将 `tokio` crate 升级到 v1.44.2

### 修复

- 修复了将组件时钟设置为回测开始时间
- 修复了追踪止损计算中的溢出错误
- 修复了 Binance 缺失的 `SymbolFilterType` 枚举成员 (#2495)，感谢 @sunlei
- 修复了 Bybit K 线的 `ts_event` (#2502)，感谢 @davidsblom
- 修复了 Binance Futures 在对冲模式下执行算法订单的仓位 ID 处理 (#2504)，感谢报告 @Oxygen923

### 文档更新

- 移除了投资组合文档中过时的 K 线限制 (#2501)，感谢 @stefansimik

### 弃用
无

---

# NautilusTrader 1.215.0 Beta

发布于 2025年4月5日 (UTC)。

### 增强功能

- 添加了 `Cache.purge_closed_order(...)`
- 添加了 `Cache.purge_closed_orders(...)`
- 添加了 `Cache.purge_closed_position(...)`
- 添加了 `Cache.purge_closed_positions(...)`
- 添加了 `Cache.purge_account_events(...)`
- 添加了 `Account.purge_account_events(...)`
- 为 `LiveExecEngineConfig` 添加了 `purge_closed_orders_interval_mins` 配置选项
- 为 `LiveExecEngineConfig` 添加了 `purge_closed_orders_buffer_mins` 配置选项
- 为 `LiveExecEngineConfig` 添加了 `purge_closed_positions_interval_mins` 配置选项
- 为 `LiveExecEngineConfig` 添加了 `purge_closed_positions_buffer_mins` 配置选项
- 为 `LiveExecEngineConfig` 添加了 `purge_account_events_interval_mins` 配置选项
- 为 `LiveExecEngineConfig` 添加了 `purge_account_events_lookback_mins` 配置选项
- 添加了 `Order.ts_closed` 属性
- 为 `BacktestDataConfig` 添加了 `instrument_ids` 和 `bar_types` 以提高目录查询效率 (#2478)，感谢 @faysou
- 为 `DatabentoDataConfig` 添加了 `venue_dataset_map` 配置选项以覆盖场所使用的默认数据集 (#2483, #2485)，感谢 @faysou

### 破坏性变更
无

### 内部改进

- 添加了 `Position.purge_events_for_order(...)` 用于清除与客户端订单 ID 关联的 `OrderFilled` 事件和 `TradeId`
- 为 `WebSocketClient` 添加了 `Consumer` (#2488)，感谢 @twitu
- 改进了 Tardis 工具解析，一致的 `effective` 时间戳过滤、结算货币、增量和费用变更
- 改进了 Betfair `update_account_state` 任务的错误日志记录，在错误时记录完整堆栈跟踪
- 改进了 Redis 缓存数据库操作的日志记录
- 标准化了意外异常日志记录以包含完整堆栈跟踪
- 优化了回测配置的类型处理
- 优化了 databento 场所数据集映射和配置 (#2483)，感谢 @faysou
- 优化了 databento `use_exchange_as_venue` 的使用 (#2487)，感谢 @faysou
- 优化了回测中组件的时间初始化 (#2490)，感谢 @faysou
- 将 Rust (MSRV) 升级到 1.86.0
- 将 `pyo3` crate 升级到 v0.24.1

### 修复

- 修复了 Databento 的 MBO 流处理，其中初始快照解码零大小的交易 tick (#2476)，感谢报告 @JackWCollins
- 修复了已关闭仓位的仓位状态快照，这些快照被错误过滤
- 修复了 `PolymarketTickSizeChanged` 消息的处理
- 修复了 Tardis 现货工具解析，其中 `size_increment` 为零，现在从基础货币推断
- 修复了 Rust 的默认日志颜色 (#2489)，感谢 @filipmacek
- 修复了 CI 中 uv 的 sccache 键 (#2482)，感谢 @davidsblom

### 文档更新

- 澄清了回测概念指南中的部分成交 (#2481)，感谢 @stefansimik

### 弃用

- 弃用了用 Cython 编写的策略并移除了 `ema_cross_cython` 策略示例

---

# NautilusTrader 1.214.0 Beta

发布于 2025年3月28日 (UTC)。

### 增强功能

- 添加了 [Coinbase International Exchange](https://www.coinbase.com/en/international-exchange) 初始集成适配器
- 为 `Strategy.close_position(...)` 添加了 `time_in_force` 参数
- 为 `Strategy.close_all_positions(...)` 添加了 `time_in_force` 参数
- 添加了 `MarkPriceUpdate` 数据类型
- 添加了 `IndexPriceUpdate` 数据类型
- 添加了 `Actor.subscribe_mark_prices(...)`
- 添加了 `Actor.subscribe_index_prices(...)`
- 添加了 `Actor.unsubscribe_mark_prices(...)`
- 添加了 `Actor.unsubscribe_index_prices(...)`
- 添加了 `Actor.on_mark_price(...)`
- 添加了 `Actor.on_index_price(...)`
- 添加了 `Cache.mark_price(...)`
- 添加了 `Cache.index_price(...)`
- 添加了 `Cache.mark_prices(...)`
- 添加了 `Cache.index_prices(...)`
- 添加了 `Cache.mark_price_count(...)`
- 添加了 `Cache.index_price_count(...)`
- 添加了 `Cache.has_mark_prices(...)`
- 添加了 `Cache.has_index_prices(...)`
- 添加了 `UnixNanos.to_rfc3339()` 用于 ISO 8601 (RFC 3339) 字符串
- 为 Bybit WebSocket 订单客户端添加了 `recv_window_ms` 配置 (#2466)，感谢 @sunlei
- 增强了 `UnixNanos` 字符串解析以支持 YYYY-MM-DD 日期格式（解释为 UTC 午夜）

### 破坏性变更

- 将 `Cache.add_mark_price(self, InstrumentId instrument_id, Price price)` 更改为 `add_mark_price(self, MarkPriceUpdate mark_price)`

### 内部改进

- 改进了 `WebSocketClient` 和 `SocketClient` 设计，具有专用写入任务和消息通道
- 完成了 Rust 中的全局消息总线设计 (#2460)，感谢 @filipmacek
- 重构了枚举分发 (#2461)，感谢 @filipmacek
- 将数据接口重构为 Rust 中的消息
- 优化了 Rust 中的目录文件操作 (#2454)，感谢 @faysou
- 优化了 Bybit 的报价 tick 和 K 线 (#2465)，感谢 @davidsblom
- 标准化了 `anyhow::bail` 的使用 (#2459)，感谢 @faysou
- 在 Rust 中为 `BacktestEngine` 移植了 `add_venue` (#2457)，感谢 @filipmacek
- 在 Rust 中为 `BacktestEngine` 移植了 `add_instrument` (#2469)，感谢 @filipmacek
- 将 `redis` crate 升级到 v0.29.2

### 修复

- 修复了 `WebSocketClient` 和 `SocketClient` 多次重连尝试的竞争条件
- 修复了仓位状态快照 `ts_snapshot` 值，它总是 `ts_last` 而不是快照拍摄时的时间戳
- 修复了 Tardis 的工具解析，现在正确应用更改并按 `effective` 过滤
- 修复了 dYdX 条件订单的 `OrderStatusReport` (#2467)，感谢 @davidsblom
- 修复了 dYdX 提交止损市价订单 (#2471)，感谢 @davidsblom
- 修复了 dYdX 在 `DecodeError` 上重试 HTTP 调用 (#2472)，感谢 @davidsblom
- 修复了 Bybit 的 `LIMIT_IF_TOUCHED` 订单类型枚举解析
- 修复了 Bybit 的 `MARKET` 订单类型枚举解析
- 修复了 Polymarket 的报价 tick，仅在顶级订单簿更改时发出新的报价 tick
- 修复了 IB 取消订单的错误 (#2475)，感谢 @FGU1

### 文档更新

- 改进了自定义数据文档 (#2470)，感谢 @faysou

### 弃用
无

---

# NautilusTrader 1.213.0 Beta

发布于 2025年3月16日 (UTC)。

### 增强功能

- 添加了 `CryptoOption` 工具，支持反向和分数大小
- 添加了 `Cache.prices(...)` 以返回每个工具按价格类型的最新价格映射
- 为 `StrategyConfig` 添加了 `use_uuid_client_order_ids` 配置选项
- 添加了将多个 parquet 文件合并为一个的目录合并功能 (#2421)，感谢 @faysou
- 添加了 FDUSD (First Digital USD) 加密货币 `Currency` 常量
- 为 Bybit 添加了初始杠杆、`margin_mode` 和 `position_mode` 配置选项 (#2441)，感谢 @sunlei
- 使用最近功能更新了 Rust 中的 parquet 目录 (#2442)，感谢 @faysou

### 破坏性变更
无

### 内部改进

- 为 `HttpClient` 添加了 `timeout_secs` 参数用于默认超时
- 为 `OrderMatchingEngine` 添加了额外的精度验证
- 添加了 `u64` 和 `UnixNanos` 之间的对称比较实现
- 改进了加载时 `InstrumentProvider` 的错误处理 (#2444)，感谢 @davidsblom
- 改进了余额影响的订单拒绝原因消息
- 为 ByBit 更新工具时处理 BybitErrors (#2437)，感谢 @davidsblom
- 为 dYdX 获取订单簿时处理意外错误 (#2445)，感谢 @davidsblom
- 如果 dYdX 引发 HttpError 则重试 (#2438)，感谢 @davidsblom
- 重构了一些 Rust 日志以在格式字符串中使用命名参数 (#2443)，感谢 @faysou
- 为 Bybit 和 dYdX 适配器进行了一些小的性能优化 (#2448)，感谢 @sunlei
- 将回测引擎和内核移植到 Rust (#2449)，感谢 @filipmacek
- 将 `pyo3` 和 `pyo3-async-runtimes` crates 升级到 v0.24.0
- 将 `tokio` crate 升级到 v1.44.1

### 修复

- 修复了源分发 (sdist) 打包
- 修复了 `Clock.timer_names()` 导致空列表的内存问题
- 修复了在过去设置时间警报时的下溢 panic (#2446)，感谢报告 @uxbux
- 修复了自定义 `strategy_id` 的 `Strategy` 的日志器名称
- 修复了 Bybit 的未绑定变量 (#2433)，感谢 @davidsblom

### 文档更新

- 澄清了 `Data` 中时间戳属性的文档 (#2450)，感谢 @stefansimik
- 更新了环境设置文档 (#2452)，感谢 @faysou

### 弃用
无

---

# NautilusTrader 1.212.0 Beta

发布于 2025年3月11日 (UTC)。

此版本引入了 [uv](https://docs.astral.sh/uv) 作为 Python 项目和依赖管理工具。

### 增强功能

- 添加了 `OwnOrderBook` 和 `OwnBookOrder` 来跟踪自己的订单并防止做市中的自成交
- 为 `ExecEngineConfig` 添加了 `manage_own_order_books` 配置选项以启用自己的订单跟踪
- 为自己的订单跟踪添加了 `Cache.own_order_book(...)`、`Cache.own_bid_orders(...)` 和 `Cache.own_ask_orders(...)`
- 添加了可选的 beta 加权和百分比期权希腊字母 (#2317)，感谢 @faysou
- 为希腊字母数据添加了 pnl 信息 (#2378)，感谢 @faysou
- 为 `TardisCSVDataLoader` 添加了精度推断，其中 `price_precision` 和 `size_precision` 现在是可选的
- 添加了 `Order.ts_accepted` 属性
- 添加了 `Order.ts_submitted` 属性
- 在 Rust 中添加了 `UnixNanos::to_datetime_utc()`
- 为 `PriceType` 枚举添加了 `Mark` 变体
- 为 `Cache` 添加了标记价格处理
- 为 `Cache` 添加了标记汇率处理
- 为 `Portfolio` 特定的配置设置添加了 `PortfolioConfig`
- 为 `PortfolioConfig` 添加了 `use_mark_prices`、`use_mark_xrates` 和 `convert_to_account_base_currency` 选项
- 为 `Portfolio` 添加了标记价格计算和汇率处理
- 添加了 Rust 调试支持并优化了 cargo nextest 使用 (#2335, #2339)，感谢 @faysou
- 添加了目录写入模式选项 (#2365)，感谢 @faysou
- 为 msgspec 编码和解码钩子添加了 `BarSpecification` (#2373)，感谢 @pierianeagle
- 为 `BetfairExecClientConfig` 添加了 `ignore_external_orders` 配置选项，默认 `False` 以保持当前行为
- 为 dYdX 添加了使用 HTTP 请求订单簿快照 (#2393)，感谢 @davidsblom

### 破坏性变更

- 移除了 [talib](https://github.com/nautechsystems/nautilus_trader/tree/develop/nautilus_trader/indicators/ta_lib) 子包（参见 v1.211.0 的弃用）
- 移除了内部 `ExchangeRateCalculator`，替换为在 Rust 中实现的 `get_exchange_rate(...)` 函数
- 用来自 PyO3 的等效项替换了 `ForexSession` 枚举
- 用来自 PyO3 的等效函数替换了 `ForexSessionFilter`
- 将 `InterestRateData` 重命名为 `YieldCurveData`
- 将 `Cache.add_interest_rate_curve` 重命名为 `add_yield_curve`
- 将 `Cache.interest_rate_curve` 重命名为 `yield_curve`
- 为了清晰将 `OrderBook.count` 重命名为 `update_count`
- 将 `ExecEngineConfig.portfolio_bar_updates` 配置选项移动到 `PortfolioConfig.bar_updates`

### 内部改进

- 为订单添加了初始 `Cache` 基准测试 (#2341)，感谢 @filipmacek
- 在 `build.py` 中添加了对 `CARGO_BUILD_TARGET` 环境变量的支持 (#2385)，感谢 @sunlei
- 为时间 K 线聚合添加了测试 (#2391)，感谢 @stefansimik 和 @faysou
- 实现了参与者框架和消息总线 v3 (#2402)，感谢 @twitu
- 在 Rust 中为 SimulatedExchange 实现了延迟建模 (#2423)，感谢 @filipmacek
- 在 Rust 中实现了汇率计算
- 改进了 `StrategyConfig` 的 `oms_type` 处理，现在正确处理 `OmsType` 枚举
- 改进了 Binance websocket 连接管理以允许超过 200 个流 (#2369)，感谢 @lidarbtc
- 改进了日志事件时间戳以避免事件交叉到日志线程时的时钟或时间错位
- 改进了实时引擎的错误日志记录，现在包含堆栈跟踪以便更容易调试
- 改进了日志初始化错误处理以避免在 Rust 中 panic
- 改进了 Redis 缓存数据库查询、序列化、错误处理和连接管理 (#2295, #2308, #2318)，感谢 @Pushkarm029
- 改进了 `OrderList` 的验证以检查所有订单是否为同一工具 ID
- 改进了 `Controller` 功能，能够从配置创建参与者和策略 (#2322)，感谢 @faysou
- 改进了 `Controller` 创建以实现更流畅的交易者注册和用于定时器命名空间的单独时钟 (#2357)，感谢 @faysou
- 通过添加占位符以避免不必要的重新构建改进了构建 (#2336)，感谢 @bartolootrit
- 改进了 Cython 和 Rust 之间 `OrderMatchingEngine` 的一致性并修复问题 (#2350)，感谢 @filipmacek
- 移除了 dYdX 过时的重连保护 (#2334)，感谢 @davidsblom
- 将数据请求接口重构为消息 (#2260)，感谢 @faysou
- 将数据订阅接口重构为消息 (#2280)，感谢 @faysou
- 将对账接口重构为消息 (#2375)，感谢 @faysou
- 重构了 `_handle_query_group` 以配合 `update_catalog` (#2412)，感谢 @faysou
- 在 Rust 中重构了执行消息处理 (#2291)，感谢 @filipmacek
- 重构了回测示例中的重复代码 (#2387, #2395)，感谢 @stefansimik
- 优化了收益率曲线数据 (#2300)，感谢 @faysou
- 优化了 Rust 中的 K 线聚合器 (#2311)，感谢 @faysou
- 优化了希腊字母计算 (#2312)，感谢 @faysou
- 优化了 portfolio_greeks 中的基础过滤 (#2382)，感谢 @faysou
- 优化了 Databento 的 `request_instruments` 粒度 (#2347)，感谢 @faysou
- 优化了 Rust 日期函数 (#2356)，感谢 @faysou
- 优化了 IB 符号解析 (#2388)，感谢 @faysou
- 优化了 parquet write_data 中的 `base_template` 行为 (#2389)，感谢 @faysou
- 优化了混合目录客户端请求 (#2405)，感谢 @faysou
- 优化了更新目录文档字符串 (#2411)，感谢 @faysou
- 优化为标识符标签函数使用 `next_back` 而不是 `last` (#2414)，感谢 @twitu
- 优化并优化了 Rust 中的 `OrderBook`
- 清理了 PyO3 迁移产物 (#2326)，感谢 @twitu
- 将 `StreamingFeatherWriter` 移植到 Rust (#2292)，感谢 @twitu
- 为 Rust 中的 `OrderMatchingEngine` 移植了 `update_limit_order` (#2301)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了 `update_stop_market_order` (#2310)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了 `update_stop_limit_order` (#2314)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了 market-if-touched 订单处理 (#2329)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了 limit-if-touched 订单处理 (#2333)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了 market-to-limit 订单处理 (#2354)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了追踪止损订单处理 (#2366, #2376)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了条件订单处理 (#2404)，感谢 @filipmacek
- 更新了 Databento `publishers.json` 映射文件
- 为 Interactive Brokers 将 `nautilus-ibapi` 升级到 10.30.1 并进行必要更改 (#2420)，感谢 @FGU1
- 将 Rust 升级到 1.85.0 和 2024 版本
- 将 `arrow` 和 `parquet` crates 升级到 v54.2.1
- 将 `databento` crate 升级到 v0.20.0（将 `dbn` crate 升级到 v0.28.0）
- 将 `datafusion` crate 升级到 v46.0.0
- 将 `pyo3` crate 升级到 v0.23.5
- 将 `tokio` crate 升级到 v1.44.0

### 修复

- 修复了 `Data` 枚举变体之间的巨大差异 (#2315)，感谢 @twitu
- 修复了 `TardisHttpClient` 的 `start` 和 `end` 范围过滤以使用 API 查询参数
- 修复了 `StreamingFeatherWriter` 的内置数据类型 Arrow 模式，感谢报告 @netomenoci
- 修复了 `TardisCSVDataLoader` 的内存分配性能问题
- 修复了 `TardisHttpClient` 的 `effective` 时间戳过滤，现在仅保留在 `effective` 时或之前的最新版本
- 修复了 Binance Futures 的合约 `activation`，现在基于 `onboardDate` 字段
- 修复了 `PolymarketExecutionClient` 的硬编码签名类型
- 修复了 dYdX 的报价取消订阅 (#2331)，感谢 @davidsblom
- 修复了 dYdX 工厂的文档字符串 (#2415)，感谢 @davidsblom
- 修复了 `_request_instrument` 签名中的错误类型注释 (#2332)，感谢 @faysou
- 修复了复合 K 线订阅 (#2337)，感谢 @faysou
- 修复了某些适配器中的子命令问题 (#2343)，感谢 @faysou
- 修复了 `bypass_logging` 固定装置以在整个测试会话中保持日志保护活动
- 修复了 IB 适配器的时间解析 (#2360)，感谢 @faysou
- 修复了 IB 周和月 K 线中的错误 `ts_init` 值 (#2355)，感谢 @Endura2024
- 修复了 IB 的 K 线时间戳 (#2380)，感谢 @Endura2024
- 修复了从自定义 CSV 加载 K 线的回测示例 (#2383)，感谢 @hanksuper
- 修复了订阅复合 K 线 (#2390)，感谢 @faysou
- 修复了 IB 文档中的无效链接 (#2401)，感谢 @stefansimik
- 修复了缓存索引加载以确保持久化数据在启动后保持可用，感谢报告 @Saransh-28
- 修复了 Bybit 的 K 线分页、排序和限制
- 修复了 `update_bar` 聚合函数以保证高价和低价不变量 (#2430)，感谢 @hjander 和 @faysou

### 文档更新

- 为消息样式添加了文档 (#2410)，感谢 @stefansimik
- 添加了回测时钟和定时器示例 (#2327)，感谢 @stefansimik
- 添加了回测 K 线聚合示例 (#2340)，感谢 @stefansimik
- 添加了回测投资组合示例 (#2362)，感谢 @stefansimik
- 添加了回测缓存示例 (#2370)，感谢 @stefansimik
- 添加了回测级联指标示例 (#2398)，感谢 @stefansimik
- 添加了带有 msgbus 的回测自定义事件示例 (#2400)，感谢 @stefansimik
- 添加了带有 msgbus 的回测消息传递示例 (#2406)，感谢 @stefansimik
- 添加了带有参与者和数据的回测消息传递示例 (#2407)，感谢 @stefansimik
- 添加了带有参与者和信号的回测消息传递示例 (#2408)，感谢 @stefansimik
- 添加了指标示例 (#2396)，感谢 @stefansimik
- 添加了使用 Rust 调试的文档 (#2325)，感谢 @faysou
- 添加了 MRE 策略示例 (#2352)，感谢 @stefansimik
- 添加了数据目录示例 (#2353)，感谢 @stefansimik
- 改进并扩展了 K 线聚合文档 (#2384)，感谢 @stefansimik
- 改进了文档字符串中 `emulation_trigger` 参数的描述 (#2313)，感谢 @stefansimik
- 改进了模拟订单的文档 (#2316)，感谢 @stefansimik
- 改进了回测 API 级别的入门文档 (#2324)，感谢 @faysou
- 改进了针对初学者的 FSM 示例解释 (#2351)，感谢 @stefansimik
- 优化了期权希腊字母文档字符串 (#2320)，感谢 @faysou
- 优化了适配器概念文档 (#2358)，感谢 @faysou
- 修复了 docs/concepts/actors.md 中的拼写错误 (#2422)，感谢 @lsamaciel
- 修复了 docs/concepts/instruments.md 中的单数名词 (#2424)，感谢 @lsamaciel
- 修复了 docs/concepts/data.md 中的拼写错误 (#2426)，感谢 @lsamaciel
- 修复了 docs/concepts/orders.md 中的 Limit-If-Touched 示例 (#2429)，感谢 @lsamaciel

### 弃用
无

---

# NautilusTrader 1.211.0 Beta

发布于 2025年2月9日 (UTC)。

此版本引入了[高精度模式](https://nautilustrader.io/docs/nightly/concepts/overview#value-types)，其中诸如 `Price`、`Quantity` 和 `Money` 等值类型现在由 128 位整数（而不是 64 位）支持，从而将最大精度增加到 16，并大幅扩展了允许的值范围。

这将解决一些加密货币用户遇到的精度和值范围问题，缓解更高时间框架 K 线成交量限制，并为平台未来做好准备。

有关更多详细信息，请参见 [RFC](https://github.com/nautechsystems/nautilus_trader/issues/2084)。
有关在有或没有高精度模式下编译的说明，请参见安装指南的[精度模式](https://nautilustrader.io/docs/nightly/getting_started/installation/#precision-mode)部分。

**由于破坏性变更，有关迁移数据目录，请参见[数据迁移指南](https://nautilustrader.io/docs/nightly/concepts/data#data-migrations)**。

**此版本将是使用 Poetry 进行包和依赖管理的最终版本。**

### 增强功能

- 为 128 位整数支持的值类型添加了 `high-precision` 模式 (#2072)，感谢 @twitu
- 为 `TardisHttpClient` 添加了工具定义范围请求，带有可选的 `start` 和 `end` 过滤器参数
- 为 `TardisInstrumentProvider` 添加了 `quote_currency`、`base_currency`、`instrument_type`、`contract_type`、`active`、`start` 和 `end` 过滤器
- 为 `ActorConfig`、`StrategyConfig`、`ExecAlgorithmConfig` 添加了 `log_commands` 配置选项以实现更高效的日志过滤
- 为 `BettingInstrument` 构造函数添加了额外的限制参数
- 为 `OrderStatusReport` 添加了 `venue_position_id` 参数
- 为 `Portfolio` PnL 添加了 K 线更新支持 (#2239)，感谢 @faysou
- 为 `Strategy` 订单管理方法添加了可选的 `params`（与 `Actor` 数据方法对称）(#2251)，感谢 @faysou
- 为 Betfair 客户端添加了心跳以保持流活动（在初始订阅延迟时更稳健）
- 为 `NautilusKernelConfig` 添加了 `timeout_shutdown` 配置选项
- 为 Betfair 订单添加了 IOC 时间有效性映射
- 为 `BetfairInstrumentProviderConfig` 添加了 `min_market_start_time` 和 `max_market_start_time` 时间范围过滤
- 为 `BetfairInstrumentProviderConfig` 添加了 `default_min_notional` 配置选项
- 为 `BetfairDataClientConfig` 添加了 `stream_conflate_ms` 配置选项
- 为 `BybitDataClientConfig` 和 `BybitExecClientConfig` 添加了 `recv_window_ms` 配置选项
- 为 `LiveExecEngineConfig` 添加了 `open_check_open_only` 配置选项
- 添加了 `BetSide` 枚举（支持 `Bet` 和 `BetPosition`）
- 为博彩市场风险和 PnL 计算添加了 `Bet` 和 `BetPosition`
- 为 `Portfolio` 添加了 `total_pnl` 和 `total_pnls` 方法
- 为 `Portfolio` 未实现 PnL 和净敞口方法添加了可选的 `price` 参数

### 破坏性变更

- 将 `OptionsContract` 工具重命名为 `OptionContract` 以采用更技术正确的术语（单数）
- 将 `OptionsSpread` 工具重命名为 `OptionSpread` 以采用更技术正确的术语（单数）
- 将 `options_contract` 模块重命名为 `option_contract`（见上文）
- 将 `options_spread` 模块重命名为 `option_spread`（见上文）
- 将 `InstrumentClass.FUTURE_SPREAD` 重命名为 `InstrumentClass.FUTURES_SPREAD` 以采用更技术正确的术语
- 将 `event_logging` 配置选项重命名为 `log_events`
- 将 `BetfairExecClientConfig.request_account_state_period` 重命名为 `request_account_state_secs`
- 将 SQL 模式目录移动到 `schemas/sql`（使用 `make install-cli` 重新安装 Nautilus CLI）
- 更改 `OrderBookDelta` Arrow 模式以使用 `FixedSizeBinary` 字段来支持新的精度模式
- 更改 `OrderBookDepth10` Arrow 模式以使用 `FixedSizeBinary` 字段来支持新的精度模式
- 更改 `QuoteTick` Arrow 模式以使用 `FixedSizeBinary` 字段来支持新的精度模式
- 更改 `TradeTick` Arrow 模式以使用 `FixedSizeBinary` 字段来支持新的精度模式
- 更改 `Bar` Arrow 模式以使用 `FixedSizeBinary` 字段来支持新的精度模式
- 将 `BettingInstrument` 默认 `min_notional` 更改为 `None`
- 将 [PolymarketDataClientConfig](https://github.com/nautechsystems/nautilus_trader/blob/develop/nautilus_trader/adapters/polymarket/config.py) 的 `ws_connection_delay_secs` 的含义更改为**非初始**延迟 (#2271)
- 将 `GATEIO` Tardis 场所更改为 `GATE_IO` 以与 `CRYPTO_COM` 和 `BLOCKCHAIN_COM` 保持一致
- 移除了 dYdX 配置的 `max_ws_reconnection_tries`（不再适用于无限重试和指数退避）
- 移除了 Bybit 配置的 `max_ws_reconnection_tries`（不再适用于无限重试和指数退避）
- 移除了 Bybit 配置的剩余 `max_ws_reconnection_tries` (#2290)，感谢 @sunlei

### 内部改进

- 为更高效和稳健的实时引擎队列管理和日志记录添加了 `ThrottledEnqueuer`
- 在 Rust 中添加了 `OrderBookDeltaTestBuilder` 以改进测试 (#2234)，感谢 @filipmacek
- 为 `SocketClient` TLS 添加了自定义证书加载
- 为 Rust 中的字符串验证添加了 `check_nonempty_string`
- 通过可配置延迟改进了 Polymarket WebSocket 订阅处理 (#2271)，感谢 @ryantam626
- 改进了 `WebSocketClient` 的状态管理、错误处理、超时和使用指数退避的稳健重连
- 改进了 `SocketClient` 的状态管理、错误处理、超时和使用指数退避的稳健重连
- 改进了使用 `asyncio.run()` 运行时的 `TradingNode` 关闭（事件循环的更有序处理）
- 改进了关闭时 `NautilusKernel` 待处理任务取消
- 改进了 `TardisHttpClient` 请求和错误处理
- 改进了日志文件写入器以去除 ANSI 转义代码和不可打印字符
- 改进了 `clean` make 目标行为并添加了 `distclean` make 目标 (#2286)，@demonkoryu
- 优化了 `Currency` `name` 以接受非 ASCII 字符（外币常见）
- 使用复合操作重构了 CI (#2242)，感谢 @sunlei
- 重构了期权希腊字母功能 (#2266)，感谢 @faysou
- 更改验证以允许 `PerContractFeeModel` 的零佣金 (#2282)，感谢 @stefansimik
- 在 CI 中更改为使用 `mold` 作为链接器 (#2254)，感谢 @sunlei
- 为 Rust 中的 `OrderMatchingEngine` 移植了市价订单处理 (#2202)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了限价订单处理 (#2212)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了止损限价订单处理 (#2225)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了 `CancelOrder` 处理 (#2231)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了 `CancelAllOrders` 处理 (#2253)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了 `BatchCancelOrders` 处理 (#2256)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了过期订单处理 (#2259)，感谢 @filipmacek
- 为 Rust 中的 `OrderMatchingEngine` 移植了修改订单处理 (#2261)，感谢 @filipmacek
- 为 Rust 中的 `SimulatedExchange` 移植了生成新账户状态 (#2272)，感谢 @filipmacek
- 为 Rust 中的 SimulatedExchange 移植了调整账户 (#2273)，感谢 @filipmacek
- 继续将 `RiskEngine` 移植到 Rust (#2210)，感谢 @Pushkarm029
- 继续将 `ExecutionEngine` 移植到 Rust (#2214)，感谢 @Pushkarm029
- 继续将 `OrderEmulator` 移植到 Rust (#2219, #2226)，感谢 @Pushkarm029
- 将 `model` crate 存根移动到默认值 (#2235)，感谢 @fhill2
- 将 `pyo3` crate 升级到 v0.23.4
- 将 `pyo3-async-runtimes` crate 升级到 v0.23.0

### 修复

- 修复了开始时间为零时 `LiveTimer` 立即触发 (#2270)，感谢报告 @bartolootrit
- 修复了 Tardis 的订单簿操作解析（确保快照中的零大小与 `action` 与 `size` 的更严格验证一起工作）
- 修复了 `Portfolio` 中博彩工具的 PnL 计算
- 修复了 `Portfolio` 中博彩工具的净敞口
- 修复了回测开始和结束时间验证断言 (#2203)，感谢 @davidsblom
- 修复了 `DataEngine` 中的 `CustomData` 导入 (#2207)，感谢 @graceyangfan 和 @faysou
- 修复了 databento 辅助函数 (#2208)，感谢 @faysou
- 修复了生成订单成交的实时对账以使用 `venue_position_id`（当提供时），感谢报告 @sdk451
- 修复了当 `reload` 标志为 `True` 时的 `InstrumentProvider` 初始化行为，感谢 @ryantam626
- 修复了 Binance HTTP 错误消息的处理（不总是可解析为 JSON，导致 `msgspec.DecodeError`）
- 修复了构建脚本的 `CARGO_TARGET_DIR` 环境变量 (#2228)，感谢 @sunlei
- 修复了 `delta.rs` 文档注释中的拼写错误 (#2230)，感谢 @eltociear
- 修复了由 `gil-refs` 功能引起的网络 PyO3 层中的内存泄漏 (#2229)，感谢报告 @davidsblom
- 修复了 Betfair 的重连处理 (#2232, #2288, #2289)，感谢 @limx0
- 修复了错误日志中的 `instrument.id` 空引用 (#2237)，感谢报告 @ryantam626
- 修复了 dYdX 列出市场的模式 (#2240)，感谢 @davidsblom
- 修复了 `Portfolio` 中的已实现 pnl 计算，其中平仓位不包含在累计总和中 (#2243)，感谢 @faysou
- 修复了 Rust 中 `Cache` 的更新订单 (#2248)，感谢 @filipmacek
- 修复了 dYdX 市场更新的 websocket 模式 (#2258)，感谢 @davidsblom
- 修复了 Tardis 空订单簿消息的处理（导致 `deltas` 不能为空的 panic）
- 修复了 `Cache.bar_types` `aggregation_source` 过滤，错误地使用了 `price_type` (#2269)，感谢 @faysou
- 修复了 Tardis 集成缺失的 `combo` 工具类型
- 修复了 `OrderMatchingEngine` 中从 K 线处理报价 tick 导致大小低于最小增量 (#2275)，感谢报告 @miller-moore
- 修复了需要 `int` 的 `BinanceErrorCode` 初始化
- 修复了 COIN 保证金合约的 Tardis `BINANCE_DELIVERY` 场所解析
- 修复了速率限制器中的挂起 (#2285)，感谢 @WyldeCat
- 修复了 `InstrumentProviderConfig` 文档字符串中的拼写错误 (#2284)，感谢 @ikeepo
- 修复了 Polymarket 的 `tick_size_change` 消息处理

### 文档更新

- 添加了 Databento 概述教程 (#2233, #2252)，感谢 @stefansimik
- 为 Actor 添加了文档 (#2233)，感谢 @stefansimik
- 为具有 K 线数据的 Portfolio 限制添加了文档 (#2233)，感谢 @stefansimik
- 为仓库中的示例位置添加了文档概述 (#2287)，感谢 @stefansimik
- 改进了 Actor 订阅和请求方法的文档字符串
- 优化了 `streaming` 参数描述 (#2293)，感谢 @faysou 和 @stefansimik

### 弃用

- 指标的 [talib](https://github.com/nautechsystems/nautilus_trader/tree/develop/nautilus_trader/indicators/ta_lib) 子包已弃用，将在未来版本中移除，参见 [RFC](https://github.com/nautechsystems/nautilus_trader/issues/2206)

---

# NautilusTrader 1.210.0 Beta

发布于 2025年1月10日 (UTC)。

### 增强功能

- 添加了 `PerContractFeeModel`，感谢 @stefansimik
- 为 dYdX 添加了 `DYDXInternalError` 和 `DYDXOraclaPrice` 数据类型 (#2155)，感谢 @davidsblom
- 为 Betfair 添加了正确的 `OrderBookDeltas` 标志解析
- 添加了 Binance TradeLite 消息支持 (#2156)，感谢 @DeevsDeevs
- 添加了 `DataEngineConfig.time_bars_skip_first_non_full_bar` 配置选项 (#2160)，感谢 @faysou
- 为 Bybit 添加了 `execution.fast` 支持 (#2165)，感谢 @sunlei
- 添加了目录辅助函数以导出数据 (#2135)，感谢 @twitu
- 为 `NautilusKernel` 添加了额外的时间戳属性
- 为 `StrategyConfig` 添加了 `event_logging` 配置选项 (#2183)，感谢 @sunlei
- 为 `BacktestVenueConfig` 添加了 `bar_adaptive_high_low_ordering` (#2188)，感谢 @faysou 和 @stefansimik

### 破坏性变更

- 移除了 `UUID4` 的可选 `value` 参数（请改用 `UUID4.from_str(...)`），与 Nautilus PyO3 API 对齐
- 更改 `unix_nanos_to_iso8601` 以输出具有纳秒精度的 ISO 8601 (RFC 3339) 格式字符串
- 更改 `format_iso8601` 以输出具有纳秒精度的 ISO 8601 (RFC 3339) 格式字符串
- 更改 `format_iso8601` `dt` 参数以强制 `pd.Timestamp`（具有纳秒精度）
- 将 `TradingNode.is_built` 从属性更改为方法 `.is_built()`
- 将 `TradingNode.is_running` 从属性更改为方法 `.is_running()`
- 更改 `OrderInitialized` Arrow 模式（`linked_order_ids` 和 `tags` 数据类型从 `string` 更改为 `binary`）
- 将订单字典表示的 `avg_px` 和 `slippage` 字段类型从 `str` 更改为 `float`（因为与仓位事件不对齐）
- 将 `Cache.bar_types(...)` 的 `aggregation_source` 过滤器参数更改为可选，默认为 `None`

### 内部改进

- 改进了订单簿中无可用大小时的市价订单处理（现在明确拒绝）
- 通过确保 `size` 始终为正改进了 `TradeTick` 的验证
- 通过确保 `action` 为 `ADD` 或 `UPDATE` 时 `order.size` 为正改进了 `OrderBookDelta` 的验证
- 通过确保 `step` 始终为正改进了 `BarSpecification` 的验证
- 将 ISO 8601 时间戳标准化为具有纳秒精度的 RFC 3339 规范
- 标准化了适配器间 `OrderBookDeltas` 解析的标志
- 优化了 dYdX 的解析蜡烛图 (#2148)，感谢 @davidsblom
- 优化了 Bybit 中类型提示的导入 (#2149)，感谢 @sunlei
- 优化了 Bybit 的私有 WebSocket 消息处理 (#2170)，感谢 @sunlei
- 优化了 Bybit 的 WebSocket 客户端重新订阅日志 (#2179)，感谢 @sunlei
- 优化了 dYdX 的保证金余额报告 (#2154)，感谢 @davidsblom
- 增强了 Bybit 的 `lotSizeFilter` 字段 (#2166)，感谢 @sunlei
- 重命名了 Bybit 的 WebSocket 私有客户端 (#2180)，感谢 @sunlei
- 为自定义 dYdX 类型添加了单元测试 (#2163)，感谢 @davidsblom
- 允许 K 线聚合器在 `request_aggregated_bars` 后持久化 (#2144)，感谢 @faysou
- 处理目录和实时流到目录 (#2153)，感谢 @limx0
- 为 dYdX 初始化账户时使用超时 (#2169)，感谢 @davidsblom
- 为 dYdX 发送 websocket 消息时使用重试管理器 (#2196)，感谢 @davidsblom
- 优化了 dYdX 发送 pong 时的错误日志 (#2184)，感谢 @davidsblom
- 优化了消息总线主题 `is_matching` (#2151)，感谢 @ryantam626
- 为 `bar_adaptive_high_low_ordering` 添加了测试 (#2197)，感谢 @faysou
- 将 `OrderManager` 移植到 Rust (#2161)，感谢 @Pushkarm029
- 将追踪止损逻辑移植到 Rust (#2174)，感谢 @DeevsDeevs
- 将 `FeeModel` 移植到 Rust (#2191)，感谢 @filipmacek
- 在 Rust 中为 `OrderMatchingEngine` 实现了 ID 生成器 (#2193)，感谢 @filipmacek
- 将 Cython 升级到 v3.1.0a1
- 将 `tokio` crate 升级到 v1.43.0
- 将 `datafusion` crate 升级到 v44.0.0

### 修复

- 修复了请求时 `DataClient` 的类型检查以支持除 `MarketDataClient` 之外的客户端
- 修复了 `OrderMatchingEngine` 中从 K 线处理交易 tick - 可能导致零大小交易，感谢报告 @stefansimik
- 修复了 `DataEngine` 和 `PolymarketExecutionClient` 的 `instrument is None` 检查流程
- 修复了 `BetfairDataClient` 中的工具更新 (#2152)，感谢 @limx0
- 修复了回测完成时时间事件的处理，当它们发生在最终数据时间戳之后
- 修复了 `PolymarketOrderStatus` 缺失的枚举成员 `CANCELED_MARKET_RESOLVED`
- 修复了某些订单 `.to_dict()` 表示中缺失的 `init_id` 字段
- 修复了将 `DYDXOraclePrice` 写入目录 (#2158)，感谢 @davidsblom
- 修复了 dYdX 的账户余额 (#2167)，感谢 @davidsblom
- 修复了 dYdX 的市场模式 (#2190)，感谢 @davidsblom
- 修复了缺失的 `OrderEmulated` 和 `OrderReleased` Arrow 模式
- 修复了 Bybit 的 websocket 公共频道重连 (#2176)，感谢 @sunlei
- 修复了 Binance 现货的执行报告解析（客户端订单 ID 空字符串现在变成 UUID4 字符串）
- 修复了 `OrderMatchingEngine` 中 `fill_order` 函数的文档拼写错误 (#2189)，感谢 @filipmacek

### 文档更新

- 为回测中的 `Cache`、滑点和价差处理添加了文档 (#2162)，感谢 @stefansimik
- 为 `FillModel` 和基于 K 线的执行添加了文档 (#2187)，感谢 @stefansimik
- 为选择数据（成本与准确性）和 K 线 OHLC 处理添加了文档 (#2195)，感谢 @stefansimik
- 为回测中的 K 线处理添加了文档 (#2198)，感谢 @stefansimik
- 为时间戳和 UUID 规范添加了文档

---

# NautilusTrader 1.209.0 Beta

发布于 2024年12月25日 (UTC)。

### 增强功能

- 为 Bybit 添加了 WebSocket API 交易支持 (#2129)，感谢 @sunlei
- 添加了 `BybitOrderBookDeltaDataLoader` 和 Bybit 回测教程 (#2131)，感谢 @DeevsDeevs
- 添加了保证金和佣金文档 (#2128)，感谢 @stefansimik
- 为某些 `OrderBook` 方法添加了可选的 `depth` 参数
- 添加了交易执行支持，其中交易由匹配引擎处理（对于节流订单簿和交易数据的回测可能有用）
- 重构为对 Databento GLBX 数据集的工具 ID 使用 `exchange` MIC 代码作为 `venue` (#2108, #2121, #2124, #2126)，感谢 @faysou
- 重构为一致使用 `self.config` 属性 (#2120)，感谢 @stefansimik

### 内部改进

- 优化了 `UUID4::new()` 避免不必要的字符串分配，实现了约 2.8 倍的性能改进（添加了基准测试）
- 为 dYdX 升级了 v4-proto (#2136)，感谢 @davidsblom
- 将 `databento` crate 升级到 v0.17.0

### 破坏性变更

- 将 `BinanceOrderBookDeltaDataLoader` 从 `nautilus_trader.persistence.loaders` 移动到 `nautilus_trader.adapters.binance.loaders`

### 修复

- 修复了实时模式下 `AtomicTime` 的多线程单调性
- 修复了 Bybit 的超时错误代码 (#2130)，感谢 @sunlei
- 修复了 Bybit 的工具信息检索 (#2134)，感谢 @sunlei
- 修复了 `request_aggregated_bars` 元数据处理 (#2137)，感谢 @faysou
- 修复了演示笔记本 `backtest_high_level.ipynb` (#2142)，感谢 @stefansimik

---

# NautilusTrader 1.208.0 Beta

发布于 2024年12月15日 (UTC)。

### 增强功能

- 为数据订阅和请求添加了特定的 `params`，支持 Databento `bbo-1s` 和 `bbo-1m` 报价 (#2083, #2094)，感谢 @faysou
- 为 `OrderFactory.bracket(...)` 添加了对 `STOP_LIMIT` 入场订单类型的支持
- 为 `OrderBook` 添加了 `.group_bids(...)` 和 `.group_asks(...)`
- 为 `OrderBook` 添加了 `.bids_to_dict()` 和 `.asks_to_dict()`
- 添加了 `ShutdownSystem` 命令和组件的 `shutdown_system(...)` 方法（回测、沙盒或实时环境的系统范围关闭）
- 为 `BybitDataClientConfig` 添加了 `max_ws_reconnection_tries` (#2100)，感谢 @sunlei
- 为 Bybit 添加了额外的 API 功能 (#2102)，感谢 @sunlei
- 为 Bybit 添加了仓位和 execution.fast 订阅 (#2104)，感谢 @sunlei
- 为 `BybitExecClientConfig` 添加了 `max_ws_reconnection_tries` (#2109)，感谢 @sunlei
- 为 `FuturesContract` 添加了 `margin_init`、`margin_maint`、`maker_fee`、`taker_fee` 参数和属性
- 为 `FuturesSpread` 添加了 `margin_init`、`margin_maint`、`maker_fee`、`taker_fee` 参数和属性
- 为 `OptionContract` 添加了 `margin_init`、`margin_maint`、`maker_fee`、`taker_fee` 参数和属性
- 为 `OptionSpread` 添加了 `margin_init`、`margin_maint`、`maker_fee`、`taker_fee` 参数和属性
- 改进了 Interactive Brokers 的 Databento 符号支持 (#2113)，感谢 @rsmb7z
- 改进了 dYdX 的 `STOP_MARKET` 和 `STOP_LIMIT` 订单支持 (#2069)，感谢 @Saransh-Bhandari
- 改进了 `interval_ns` 的定时器验证（避免从 Rust panic）

### 内部改进

- 在 Rust 中为 `OrderBook` 添加了 `.bids_as_map()` 和 `.asks_as_map()`
- 为 `core` 子包添加了类型存根
- 为 `common` 和 `model` 枚举添加了类型存根
- 为 `common.messages` 添加了类型存根
- 添加了重新导出和模块声明以增强代码人体工程学并改进导入可发现性
- 为 dYdX 添加了区块高度 websocket 消息订阅 (#2085)，感谢 @davidsblom
- 在 CI 中添加了 sccache (#2093)，感谢 @sunlei
- 优化了 `BybitWebSocketClient` 私有频道身份验证 (#2101)，感谢 @sunlei
- 优化了 `BybitWebSocketClient` 订阅和取消订阅 (#2105)，感谢 @sunlei
- 优化了 Bybit 的下单类定义 (#2106)，感谢 @sunlei
- 优化了 `BybitEnumParser` (#2107)，感谢 @sunlei
- 优化了 Bybit 的批量取消订单 (#2111)，感谢 @sunlei
- 将 `tokio` crate 升级到 v1.42.0

### 破坏性变更

- 将 `Level` 重命名为 `BookLevel`（标准化订单簿类型命名约定）
- 将 `Ladder` 重命名为 `BookLadder`（标准化订单簿类型命名约定）
- 更改 `FuturesContract` Arrow 模式（添加了 `margin_init`、`margin_maint`、`maker_fee`、`taker_fee`）
- 更改 `FuturesSpread` Arrow 模式（添加了 `margin_init`、`margin_maint`、`maker_fee`、`taker_fee`）
- 更改 `OptionContract` Arrow 模式（添加了 `margin_init`、`margin_maint`、`maker_fee`、`taker_fee`）
- 更改 `OptionSpread` Arrow 模式（添加了 `margin_init`、`margin_maint`、`maker_fee`、`taker_fee`）

### 修复

- 修复了在没有注册目录的情况下指定 `end` 的数据请求（`pd.Timestamp` 和 `NoneType` 之间的比较）
- 修复了 dYdX 的 `BEST_EFFORT_CANCELED` 订单状态报告 (#2082)，感谢 @davidsblom
- 修复了 dYdX 的 `BEST_EFFORT_CANCELED` 消息的订单处理 (#2095)，感谢 @davidsblom
- 修复了 dYdX 上市价订单的价格指定 (#2088)，感谢 @davidsblom
- 修复了利率曲线自定义数据和插值 (#2090)，感谢 @gcheshkov
- 修复了当不是 JSON 字符串时 `BybitHttpClient` 的错误处理 (#2096)，感谢 @sunlei
- 修复了 `BybitWebSocketClient` 私有频道重连 (#2097)，感谢 @sunlei
- 修复了 `BybitExecutionClient` 中错误的订单方向使用 (#2098)，感谢 @sunlei
- 修复了 Bybit 的默认 `http_base_url` (#2110)，感谢 @sunlei

---

# NautilusTrader 1.207.0 Beta

发布于 2024年11月29日 (UTC)。

### 增强功能

- 实现了带有目录更新的混合目录数据请求 (#2043)，感谢 @faysou
- 为 Interactive Brokers 添加了 Databento 符号支持 (#2073)，感谢 @rsmb7z
- 为数据请求添加了 `metadata` 参数 (#2043)，感谢 @faysou
- 为 dYdX 添加了 `STOP_MARKET` 和 `STOP_LIMIT` 订单支持 (#2066)，感谢 @davidsblom
- 为 dYdX 的数据客户端配置添加了 `max_reconnection_tries` (#2066)，感谢 @davidsblom
- 为 Bybit 添加了钱包订阅 (#2076)，感谢 @sunlei
- 添加了加载历史 K 线的文档说明 (#2078)，感谢 @dodofarm
- 为 `DatabentoDataLoader` 方法添加了 `price_precision` 可选参数
- 改进了添加更近期报价、交易或 K 线时的 `Cache` 行为（现在添加到缓存）

### 内部改进

- 将 `Portfolio` 和 `AccountManager` 移植到 Rust (#2058)，感谢 @Pushkarm029
- 为 `Price`、`Money` 和 `Currency` 实现了 `AsRef<str>`
- 改进了时钟中过期定时器清理 (#2064)，感谢 @twitu
- 改进了实时引擎错误日志记录（现在将记录所有异常而不仅仅是 `RuntimeError`）
- 改进了 Tardis 的符号标准化
- 改进了 Tardis 的历史 K 线请求性能
- 改进了 `TradeId` Debug 实现以将值显示为正确的 UTF-8 字符串
- 优化了直接从 Rust 使用的 `HttpClient`
- 优化了 Databento 解码器（移除了货币硬编码和 `unsafe` 的使用）
- 将 `datafusion` crate 升级到 v43.0.0 (#2056)，感谢 @twitu

### 破坏性变更

- 将 `TriggerType.LAST_TRADE` 重命名为 `LAST_PRICE`（更常规的术语）

### 修复

- 修复了 Tardis 集成缺失的场所 -> 交易所映射
- 修复了 dYdX 的账户余额和订单状态解析 (#2067)，感谢 @davidsblom
- 修复了 dYdX 的最佳努力已开订单状态解析 (#2068)，感谢 @davidsblom
- 修复了 Databento 工具偶尔错误的 `price_precision`、`multiplier` 和 `lot_size` 解码
- 修复了工具反序列化缺失的 Arrow 模式
- 当 dYdX 订单簿不一致时进行对账 (#2077)，感谢 @davidsblom

---

# NautilusTrader 1.206.0 Beta

发布于 2024年11月17日 (UTC)。

### 增强功能

- 添加了 `TardisDataClient`，提供来自 Tardis Machine WebSocket 服务器的实时数据流
- 添加了 `TardisInstrumentProvider`，通过 HTTP 工具元数据 API 从 Tardis 提供工具定义
- 为每个工具已实现 PnL 添加了 `Portfolio.realized_pnl(...)` 方法（基于仓位）
- 为每个场所已实现 PnL 添加了 `Portfolio.realized_pnls(...)` 方法（基于仓位）
- 为 `InstrumentProvider` 添加了配置警告（当节点启动时没有工具加载时发出警告）
- 实现了 Tardis 可选的[符号标准化](https://nautilustrader.io/docs/nightly/integrations/tardis/#symbology-and-normalization)
- 实现了 `WebSocketClient` 重连重试 (#2044)，感谢 @davidsblom
- 为 Binance 和 Bybit 实现了 `OrderCancelRejected` 事件生成
- 为 Binance 和 Bybit 实现了 `OrderModifyRejected` 事件生成
- 改进了 `OrderRejected` 对 `reason` 字符串的处理（现在允许 `None`，它将变成字符串 `'None'`）
- 改进了 `OrderCancelRejected` 对 `reason` 字符串的处理（现在允许 `None`，它将变成字符串 `'None'`）
- 改进了 `OrderModifyRejected` 对 `reason` 字符串的处理（现在允许 `None`，它将变成字符串 `'None'`）

### 内部改进

- 将 `RiskEngine` 移植到 Rust (#2035)，感谢 @Pushkarm029 和 @twitu
- 将 `ExecutionEngine` 移植到 Rust (#2048)，感谢 @twitu
- 添加了全局共享数据通道以在 Rust 中将事件从引擎发送到 Runner (#2042)，感谢 @twitu
- 为 dYdX HTTP 客户端添加了 LRU 缓存 (#2049)，感谢 @davidsblom
- 改进了标识符构造函数以接受 `AsRef<str>` 以获得更清洁、更灵活的 API
- 优化了标识符 `From` trait 实现
- 优化了 `InstrumentProvider` 初始化行为和日志记录
- 优化了 `LiveTimer` 取消和性能测试
- 简化了 `LiveTimer` 取消模型 (#2046)，感谢 @twitu
- 优化了 Bybit HMAC 身份验证签名（现在使用 Rust 实现的函数）
- 优化了 Tardis 工具 ID 解析
- 移除了 Bybit `msgspec` 冗余导入别名 (#2050)，感谢 @sunlei
- 将 `databento` crate 升级到 v0.16.0

### 破坏性变更
无

### 修复

- 修复了 `InstrumentProviderConfig` 特定工具 ID 的加载
- 修复了 `raw_symbol` 的 PyO3 工具转换（错误地使用了标准化符号）
- 修复了 dYdX 的对账开放订单和账户 websocket 消息 (#2039)，感谢 @davidsblom
- 修复了 Polymarket 交易报告的市价订单 `avg_px`
- 修复了 Betfair 客户端保持活动 (#2040)，感谢 @limx0
- 修复了 Betfair 对账 (#2041)，感谢 @limx0
- 修复了 Betfair 客户订单引用限制为 32 个字符
- 修复了 Bybit 对 `PARTIALLY_FILLED_CANCELED` 状态订单的处理
- 修复了 `BinaryOption` 工具的 Polymarket 大小精度（精度 6 以匹配 USDC.e）
- 修复了适配器工具重新加载（由于内部状态标志，提供程序未在配置的间隔重新加载工具）
- 修复了使用 `use_pyo3` 日志配置运行时 `BacktestEngine` 的静态时间日志记录
- 修复了飞行中订单检查并改进错误处理 (#2053)，感谢 @davidsblom
- 修复了 dYdX 清算成交的处理 (#2052)，感谢 @davidsblom
- 修复了 `BybitResponse.time` 字段作为可选 `int` (#2051)，感谢 @sunlei
- 修复了 `DatabentoDataClient` 的单个工具请求（错误地调用 `_handle_instruments` 而不是 `_handle_instrument`），感谢报告 @Emsu
- 修复了 `fsspec` 递归全局行为以确保仅包含文件路径，并将依赖项升级到版本 2024.10.0
- 修复了 jupyterlab url 拼写错误 (#2057)，感谢 @Alsheh

---

# NautilusTrader 1.205.0 Beta

发布于 2024年11月3日 (UTC)。

### 增强功能

- 在 Python 和 Rust 中添加了 Tardis Machine 和 HTTP API 集成
- 添加了 `LiveExecEngineConfig.open_check_interval_secs` 配置选项以主动与场所对账开放订单
- 添加了来自历史数据的 K 线聚合 (#2002)，感谢 @faysou
- 添加了月度和周度 K 线聚合 (#2025)，感谢 @faysou
- 为 `TradingNode.run` 添加了 `raise_exception` 可选参数 (#2021)，感谢 @faysou
- 在 Rust 中添加了 `OrderBook.get_avg_px_qty_for_exposure` (#1893)，感谢 @elementace
- 为 Interactive Brokers 适配器配置添加了超时 (#2026)，感谢 @rsmb7z
- 为时间 K 线聚合添加了可选时间起点 (#2028)，感谢 @faysou
- 添加了基于成交报告的 Polymarket 仓位状态报告和订单状态报告生成
- 将 USDC.e (PoS) 货币（由 Polymarket 使用）添加到内部货币映射
- 将 Polymarket WebSocket API 升级到新版本

### 内部改进

- 将分析子包移植到 Rust (#2016)，感谢 @Pushkarm029
- 改进了 Postgres 测试 (#2018)，感谢 @filipmacek
- 改进了 Redis 版本解析以支持截断版本（提高与 Redis 兼容数据库的兼容性）
- 优化了 Arrow 序列化（记录批处理函数现在也可在 Rust 中使用）
- 优化了核心 `Bar` API 以移除不必要的 unwrap
- 标准化了网络客户端日志记录
- 修复了 API 破坏性变更的所有 PyO3 弃用
- 修复了 PyO3 变更的所有 clippy 警告 lint (#2030)，感谢 @Pushkarm029
- PyO3 升级重构并修复目录测试 (#2032)，感谢 @twitu
- 将 `pyo3` crate 升级到 v0.22.5
- 将 `pyo3-async-runtimes` crate 升级到 v0.22.0
- 将 `tokio` crate 升级到 v1.41.0

### 破坏性变更

- 移除了 PyO3 `DataTransformer`（被用于命名空间，因此重构为单独的函数）
- 将 `TEST_DATA_DIR` 常量从 `tests` 移动到 `nautilus_trader` 包 (#2020)，感谢 @faysou

### 修复

- 修复了在集群环境中不支持的 Redis `KEYS` 命令的使用（为兼容性替换为 `SCAN`）
- 修复了 dYdX 的成交 HTTP 消息解码 (#2022)，感谢 @davidsblom
- 修复了 dYdX 的账户余额报告 (#2024)，感谢 @davidsblom
- 修复了 Interactive Brokers 市场数据客户端订阅日志消息 (#2012)，感谢 @marcodambros
- 修复了 Polymarket 执行对账（无法从已关闭订单对账）
- 修复了目录查询内存泄漏测试 (#2031)，感谢 @Pushkarm029
- 修复了 `OrderInitialized.to_dict()` `tags` 值类型为 `list[str]`（之前是连接的 `str`）
- 修复了 `OrderInitialized.to_dict()` `linked_order_ids` 值类型为 `list[str]`（之前是连接的 `str`）
- 修复了 Betfair 客户端关闭 (#2037)，感谢 @limx0

---

# NautilusTrader 1.204.0 Beta

发布于 2024年10月22日 (UTC)。

### 增强功能

- 添加了 `TardisCSVDataLoader`，用于从 Tardis 格式 CSV 文件加载数据作为传统 Cython 或 PyO3 对象
- 添加了 `Clock.timestamp_us()` 方法用于微秒 (μs) 的 UNIX 时间戳
- 为 Databento 适配器添加了对 `bbo-1s` 和 `bbo-1m` 报价模式的支持 (#1990)，感谢 @faysou
- 添加了场所 `book_type` 配置与数据的验证（防止当期望订单簿数据时使用顶级订单簿数据的问题）
- 为 `PolymarketDataClientConfig` 添加了 `compute_effective_deltas` 配置选项，减少快照大小（默认 `False` 以保持当前行为）
- 为 `WebSocketClient` 添加了速率限制器 (#1994)，感谢 @Pushkarm029
- 为 GreeksData 添加了实值概率字段 (#1995)，感谢 @faysou
- 为自定义信号数据添加了 `on_signal(signal)` 处理器
- 添加了 `nautilus_trader.common.events` 模块，重新导出 `TimeEvent` 和其他系统事件
- 通过用空订单和零计数填充部分级别改进了 `OrderBookDepth10` 的可用性
- 改进了 Postgres 配置 (#2010)，感谢 @filipmacek
- 优化了 `DatabentoInstrumentProvider` 对大批量工具定义的处理（改进了父符号支持）
- 标准化了 Betfair 符号以使用连字符而不是句点（防止 Betfair 符号被视为复合）
- 集成指南文档修复 (#1991)，感谢 @FarukhS52

### 内部改进

- 将 `Throttler` 移植到 Rust (#1988)，感谢 @Pushkarm029 和 @twitu
- 将 `BettingInstrument` 移植到 Rust
- 优化了 `WebSocketClient` 的 `RateLimiter` 并添加测试 (#2000)，感谢 @Pushkarm029
- 优化了 `WebSocketClient` 以在重连时关闭现有任务 (#1986)，感谢 @davidsblom
- 在 Rust 中移除了 `CacheDatabaseAdapter` trait 中的可变引用 (#2015)，感谢 @filipmacek
- 为 dYdX websockets 使用 Rust 速率限制器 (#1996, #1999)，感谢 @davidsblom
- 改进了 dYdX websocket 订阅的错误日志 (#1993)，感谢 @davidsblom
- 标准化了 Rust 中的日志和错误消息语法
- 继续将 `SimulatedExchange` 和 `OrderMatchingEngine` 移植到 Rust (#1997, #1998, #2001, #2003, #2004, #2006, #2007, #2009, #2014)，感谢 @filipmacek

### 破坏性变更

- 移除了传统的 `TardisQuoteDataLoader`（新的 Rust 实现加载器已冗余）
- 移除了传统的 `TardisTradeDataLoader`（新的 Rust 实现加载器已冗余）
- 自定义信号现在传递给 `on_signal(signal)` 而不是 `on_data(data)`
- 将 `Position.to_dict()` `commissions` 值类型更改为 `list[str]`（之前是字符串列表的可选 `str`）
- 将 `Position.to_dict()` `avg_px_open` 值类型更改为 `float`
- 将 `Position.to_dict()` `avg_px_close` 值类型更改为 `float | None`
- 将 `Position.to_dict()` `realized_return` 值类型更改为 `float | None`
- 将 `BettingInstrument` Arrow 模式字段 `event_open_date` 和 `market_start_time` 从 `string` 更改为 `uint64`

### 修复

- 修复了 `SocketClient` TLS 实现
- 修复了写入器关闭时 `WebSocketClient` 的错误处理，感谢报告 @davidsblom
- 修复了 dYdX 批量模式下的订单簿重新订阅 (#1985)，感谢 @davidsblom
- 修复了与符号相关的 Betfair 测试 (#1988)，感谢 @limx0
- 修复了 `OrderMatchingEngine` 仓位 ID 处理中的 `OmsType` 检查 (#2003)，感谢 @filipmacek
- 修复了 `TardisCSVDataLoader` snapshot5 和 snapshot25 解析 (#2005)，感谢 @Pushkarm029
- 修复了 Binance 客户端场所分配，我们应该使用 `client_id` 参数（与自定义客户端 `name` 匹配）与客户端通信，并使用相同的 `'BINANCE'` 场所标识符
- 修复了 `OrderMatchingEngine` 错误地尝试处理月度 K 线以执行（将失败，因为没有合理的 `timedelta` 可用），感谢报告 @frostRed
- 修复了 `cache.bar_types()` 的 `MONTH` 聚合处理（排序需要内部调用 K 线间隔 `timedelta`），感谢报告 @frostRed

---

# NautilusTrader 1.203.0 Beta

发布于 2024年10月5日 (UTC)。

### 增强功能

- 为 `ParquetDataCatalog.write_data` 添加了 `mode` 参数以控制数据写入行为 (#1976)，感谢 @faysou
- 为 dYdX 添加了短期订单的批量取消 (#1978)，感谢 @davidsblom
- 改进了 OKX 配置 (#1966)，感谢 @miller-moore
- 改进了期权希腊字母 (#1964)，感谢 @faysou

### 内部改进

- 在 Rust 中为 `SimulatedExchange` 实现了订单簿增量处理 (#1975)，感谢 @filipmacek
- 在 Rust 中为 `SimulatedExchange` 实现了 K 线处理 (#1969)，感谢 @filipmacek
- 在 Rust 中为 `SimulatedExchange` 实现了其余的 getter 函数 (#1970)，感谢 @filipmacek
- 为 dYdX websocket 订阅实现了速率限制 (#1977)，感谢 @davidsblom
- 重构了 dYdX 的重连处理 (#1983)，感谢 @davidsblom
- 优化了 `DatabentoDataLoader` 内部以适应从 Rust 使用
- 添加了初始大型测试数据文件下载和缓存功能

### 破坏性变更
无

### 修复

- 修复了 DataFusion 过滤查询中的乱序行组 (#1974)，感谢 @twitu
- 修复了 `BacktestNode` 数据排序回归导致时钟非递减时间断言错误
- 修复了 `Actor` 的循环导入，感谢 @limx0
- 修复了 OKX HTTP 客户端签名 (#1966)，感谢 @miller-moore
- 修复了 dYdX 的订单簿重新订阅 (#1973)，感谢 @davidsblom
- 修复了 dYdX 的取消拒绝生成 (#1982)，感谢 @davidsblom
- 修复了断开连接时 `WebSocketClient` 任务清理 (#1981)，感谢 @twitu
- 修复了 `Condition` 方法名称与 C `true` 和 `false` 宏的冲突，这在性能分析模式下编译时发生

---

# NautilusTrader 1.202.0 Beta

发布于 2024年9月27日 (UTC)。

这将是支持 Python 3.10 的最终版本。

`numpy` 版本要求已放宽至 >= 1.26.4。

### 增强功能

- 添加了 Polymarket 去中心化预测市场集成
- 添加了 OKX 加密货币交易所集成 (#1951)，感谢 @miller-moore
- 添加了 `BinaryOption` 工具（支持 Polymarket 集成）
- 添加了 `LiveExecutionEngine.inflight_check_retries` 配置选项以限制飞行中订单查询尝试
- 添加了 `Symbol.root()` 方法用于获取父或复合符号的根
- 添加了 `Symbol.topic()` 方法用于获取父或复合符号的订阅主题
- 添加了 `Symbol.is_composite()` 方法以确定符号是否由带有句点 (`.`) 分隔符的部分组成
- 为 `Cache.instruments(...)` 方法添加了 `underlying` 过滤器参数
- 为 `Strategy.close_position(...)` 方法添加了 `reduce_only` 参数（默认 `True` 以保持当前行为）
- 为 `Strategy.close_all_positions(...)` 方法添加了 `reduce_only` 参数（默认 `True` 以保持当前行为）
- 为 `PostgresCacheDatabase` 实现了带截断的 Postgres 函数刷新 (#1928)，感谢 @filipmacek
- 使用 `Clock` 和 `Cache` 的内部改进为 `StreamingFeatherWriter` 实现了文件轮换 (#1954, #1961)，感谢 @graceyangfan
- 改进了 dYdX 执行客户端以对 HTTP 请求使用 `RetryManager` (#1941)，感谢 @davidsblom
- 改进了 Interactive Brokers 适配器以从配置使用动态 IB 网关 `container_image` (#1940)，感谢 @rsmb7z
- 改进了基于 `F_LAST` 标志的 `OrderBookDeltas` 流式传输和批处理
- 标准化了回测日志记录的下划线千位分隔符
- 更新了 Databento `publishers.json`

### 内部改进

- 实现了 `OrderTestBuilder` 以协助 Rust 中的测试 (#1952)，感谢 @filipmacek
- 在 Rust 中为 SimulatedExchange 实现了报价 tick 处理 (#1956)，感谢 @filipmacek
- 在 Rust 中为 SimulatedExchange 实现了交易 tick 处理 (#1956)，感谢 @filipmacek
- 优化了 `Logger` 以使用无缓冲的 stdout/stderr 写入器 (#1960)，感谢 @twitu

### 破坏性变更

- 将 `batch_size_bytes` 重命名为 `chunk_size`（在回测流式模式下每个块处理的数据点数量的更准确命名）
- 标准化了 `OrderFactory.bracket(...)` 的止损 (SL) 和止盈 (TP) 参数排序，包括：`tp_time_in_force`、`tp_exec_algorithm_params`、`tp_tags`、`tp_client_order_id`

### 修复

- 修复了与 `use_pyo3=True` 一起使用时 `level_file` 的 `LoggingConfig` 问题（未传递 `level_file` 设置），感谢报告 @xt2014
- 修复了复合 K 线请求 (#1923)，感谢 @faysou
- 修复了 `ValueBarAggregator` 的平均价格计算 (#1927)，感谢 @faysou
- 通过为 dYdX 固定 `protobuf` 和 `grpcio` 修复了破坏性 protobuf 问题 (#1929)，感谢 @davidsblom
- 修复了在引擎初始化之前在 `BacktestNode` 中引发异常不会产生日志的边缘情况，感谢报告 @faysou
- 修复了 dYdX 的内部服务器错误处理 (#1938)，感谢 @davidsblom
- 修复了重连时 `BybitWebSocketClient` 私有频道身份验证，感谢报告 @miller-moore
- 修复了 `sl_time_in_force` 和 `tp_time_in_force` 的 `OrderFactory.bracket(...)` 参数排序，感谢报告 @marcodambros
- 修复了 `Cfd` 工具 Arrow 模式和序列化
- 修复了 Interactive Brokers 在 TWS/GW 重启时的 K 线订阅 (#1950)，感谢 @rsmb7z
- 修复了 Databento 父和连续合约订阅（使用新符号根）
- 修复了 Databento `FuturesSpread` 和 `OptionSpread` 工具解码（未正确处理价格增量和空基础）
- 修复了 `FuturesSpread` 序列化
- 修复了 `OptionSpread` 序列化

---

# NautilusTrader 1.201.0 Beta

发布于 2024年9月9日 (UTC)。

### 增强功能

- 为 `OrderEmulator` 添加了订单簿增量触发支持
- 为 dYdX 适配器添加了 `OrderCancelRejected` 事件生成 (#1916)，感谢 @davidsblom
- 优化了 Binance 私钥类型（RSA、Ed25519）的处理并集成到配置中
- 在 Rust 中实现了加密签名（为 Binance 替换 `pycryptodome`）
- 移除了供应商 `tokio-tungstenite` crate (#1902)，感谢 @VioletSakura-7

### 破坏性变更
无

### 修复

- 通过添加新的 `TRADE_LITE` 成员修复了 `BinanceFuturesEventType`，反映了 2024-09-03 (UTC) 的 Binance 更新

---

# NautilusTrader 1.200.0 Beta

发布于 2024年9月7日 (UTC)。

### 增强功能

- 添加了 dYdX 集成 (#1861, #1868, #1873, #1874, #1875, #1877, #1879, #1880, #1882, #1886, #1887, #1890, #1891, #1896, #1901, #1903, #1907, #1910, #1911, #1913, #1915)，感谢 @davidsblom
- 添加了复合 K 线类型，从其他 K 线类型聚合的 K 线 (#1859, #1885, #1888, #1894, #1905)，感谢 @faysou
- 为基于记录标志的增量批处理组添加了 `OrderBookDeltas.batch`（批处理直到 `F_LAST`）
- 为 `ParquetDataCatalog` 添加了 `OrderBookDeltas` 批处理支持（使用 `OrderBookDeltas` 的 `data_cls` 以与实时适配器相同的标志方法进行批处理）
- 添加了 `RetryManagerPool` 以抽象所有适配器的通用重试功能
- 为 `OrderMatchingEngine` 添加了 `InstrumentClose` 功能，感谢 @limx0
- 添加了 `BacktestRunConfig.dispose_on_completion` 配置选项以控制每个内部回测引擎的运行后处置行为（默认 `True` 以保持当前行为）
- 为 `BinanceExecClientConfig` 添加了 `recv_window_ms` 配置选项
- 为 `OrderFactory.bracket(...)` 方法添加了 `sl_time_in_force` 和 `tp_time_in_force` 参数
- 为 `OrderFactory` 方法添加了自定义 `client_order_id` 参数
- 添加了对 Binance RSA 和 Ed25519 API 密钥类型的支持 (#1908)，感谢 @NextThread
- 为 `CryptoPerpetual` 添加了 `multiplier` 参数（默认 1）
- 为 `submit_order`、`modify_order`、`cancel_order` 和 `cancel_all_orders` 实现了 `BybitExecutionClient` 重试逻辑
- 改进了 Rust 中的错误建模和处理 (#1866)，感谢 @twitu
- 改进了 `HttpClient` 错误处理并为 Python 添加了 `HttpClientError` 异常 (#1872)，感谢 @twitu
- 改进了 `WebSocketClient` 错误处理并为 Python 添加了 `WebSocketClientError` 异常 (#1876)，感谢 @twitu
- 改进了 `WebSocketClient.send_text` 效率（现在接受 UTF-8 编码字节，而不是 Python 字符串）
- 改进了 `@customdataclass` 装饰器的 `date` 字段并优化了 `__repr__` (#1900, #1906, #1909)，感谢 @faysou
- 改进了加密货币场所的 `OrderBookDeltas` 解析和记录标志的标准化
- 将 `RedisMessageBusDatabase` 重构为 tokio 任务
- 将 `RedisCacheDatabase` 重构为 tokio 任务
- 将 `tokio` crate 升级到 v1.40.0

### 破坏性变更

- 将 `heartbeat_interval` 重命名为 `heartbeat_interval_secs`（更明确地表示时间单位）
- 将 `heartbeat_interval_secs` 配置选项移动到 `MessageBusConfig`（消息总线处理外部流处理）
- 更改 `WebSocketClient.send_text(...)` 以接受 `data` 作为 `bytes` 而不是 `str`
- 更改 `CryptoPerpetual` Arrow 模式以包含 `multiplier` 字段
- 更改 `CryptoFuture` Arrow 模式以包含 `multiplier` 字段

### 修复

- 修复了 Python 终结器中的 `OrderBook` 内存释放（对象销毁时内存未被释放），感谢报告 @zeyuhuan
- 修复了 `Order` 标签序列化（未连接成单个字符串），感谢报告 @DevRoss
- 修复了内核设置期间 `MessageBusConfig` 中的 `types_filter` 序列化
- 修复了当元素已经是 `InstrumentId` 时 `InstrumentProvider` 对 `load_ids_on_start` 的处理
- 修复了 `filters` 字段的 `InstrumentProviderConfig` 哈希

---

# NautilusTrader 1.199.0 Beta

发布于 2024年8月19日 (UTC)。

### 增强功能

- 添加了 `LiveExecEngineConfig.generate_missing_orders` 对账配置选项以对齐内部和外部仓位状态
- 添加了 `LogLevel::TRACE`（仅在 Rust 的调试/开发构建中可用）
- 添加了 `Actor.subscribe_signal(...)` 方法和 `Data.is_signal(...)` 类方法 (#1853)，感谢 @faysou
- 添加了 Binance Futures 对 `HEDGE` 模式的支持 (#1846)，感谢 @DevRoss
- 大幅改进并优化了 Rust 中的错误建模和处理 (#1849, #1858)，感谢 @twitu
- 改进了 `BinanceExecutionClient` 仓位报告请求（现在可以按工具过滤并包括平仓位的报告）
- 改进了 `BybitExecutionClient` 仓位报告请求（现在可以按工具过滤并包括平仓位的报告）
- 改进了内部仓位与外部仓位不匹配时 `LiveExecutionEngine` 对账的稳健性和恢复
- 改进了 `@customdataclass` 装饰器构造函数以允许更多位置参数 (#1850)，感谢 @faysou
- 改进了 `@customdataclass` 文档 (#1854)，感谢 @faysou
- 将 `datafusion` crate 升级到 v41.0.0
- 将 `tokio` crate 升级到 v1.39.3
- 将 `uvloop` 升级到 v0.20.0（将 libuv 升级到 v1.48.0）

### 破坏性变更

- 更改 `VolumeWeightedAveragePrice` 计算公式以使用每个 K 线的"典型"价格 (#1842)，感谢 @evgenii-prusov
- 更改 `OptionContract` 构造函数参数顺序和 Arrow 模式（一致地分组期权类型和执行价格）
- 将 `snapshot_positions_interval` 重命名为 `snapshot_positions_interval_secs`（更明确地表示时间单位）
- 将 `snapshot_orders` 配置选项移动到 `ExecEngineConfig`（现在可用于所有环境上下文）
- 将 `snapshot_positions` 配置选项移动到 `ExecEngineConfig`（现在可用于所有环境上下文）
- 将 `snapshot_positions_interval_secs` 配置选项移动到 `ExecEngineConfig`（现在可用于所有环境上下文）

### 修复

- 修复了重复成交时 `Position` 异常类型（应该是 `KeyError` 以与 `Order` 的相同错误对齐）
- 修复了仓位为平时 Bybit 仓位报告解析（`BybitPositionSide` 现在正确处理空字符串）

---

# NautilusTrader 1.198.0 Beta

发布于 2024年8月9日 (UTC)。

### 增强功能

- 添加了 `@customdataclass` 装饰器以减少实现自定义数据类型所需的样板代码 (#1828)，感谢 @faysou
- 在 Rust 中为 HTTP 客户端添加了超时 (#1835)，感谢 @davidsblom
- 添加了流式数据到回测数据的目录转换函数 (#1834)，感谢 @faysou
- 将 Cython 升级到 v3.0.11

### 破坏性变更
无

### 修复

- 修复了在目录中写入 PyO3 K 线时创建 `instrumend_id` 文件夹 (#1832)，感谢 @faysou
- 修复了 `StreamingFeatherWriter` 对 `include_types` 选项的处理 (#1833)，感谢 @faysou
- 修复了 `BybitExecutionClient` 仓位报告错误处理和日志记录
- 修复了 `BybitExecutionClient` 订单报告处理以正确处理外部订单

---

# NautilusTrader 1.197.0 Beta

发布于 2024年8月2日 (UTC)。

### 增强功能

- 添加了用于加载和实时交易的 Databento Status 模式支持
- 为 Interactive Brokers 添加了期货期权支持 (#1795)，感谢 @rsmb7z
- 为期权希腊字母自定义数据示例添加了文档 (#1788)，感谢 @faysou
- 添加了 `MarketStatusAction` 枚举（支持 Databento `status` 模式）
- 为 Interactive Brokers 添加了 `ignore_quote_tick_size_updates` 配置选项 (#1799)，感谢 @sunlei
- 在 Rust 中实现了 `MessageBus` v2 (#1786)，感谢 @twitu
- 在 Rust 中实现了 `DataEngine` v2 (#1785)，感谢 @twitu
- 在 Rust 中实现了 `FillModel` (#1801)，感谢 @filipmacek
- 在 Rust 中实现了 `FixedFeeModel` (#1802)，感谢 @filipmacek
- 在 Rust 中实现了 `MakerTakerFeeModel` (#1803)，感谢 @filipmacek
- 在 Rust 中实现了 Postgres 原生枚举映射 (#1797, #1806)，感谢 @filipmacek
- 重构了 Interactive Brokers 的订单提交错误处理 (#1783)，感谢 @rsmb7z
- 改进了实时对账稳健性（现在将生成对齐外部仓位状态所需的推断订单）
- 改进了 Interactive Brokers 的测试 (#1776)，感谢 @mylesgamez
- 将 `tokio` crate 升级到 v1.39.2
- 将 `datafusion` crate 升级到 v40.0.0

### 破坏性变更

- 移除了 `VenueStatus` 和所有相关方法和模式（与 `InstrumentStatus` 冗余）
- 将 `QuoteTick.extract_volume(...)` 重命名为 `.extract_size(...)`（更准确的术语）
- 更改 `InstrumentStatus` 参数（支持 Databento `status` 模式）
- 更改 `InstrumentStatus` Arrow 模式
- 更改 `OrderBook` FFI API 以通过引用而不是值获取数据

### 修复

- 修复了大值会计计算中的舍入错误（内部使用 `decimal.Decimal`）
- 修复了具有多种 PnL 货币的多货币账户佣金处理 (#1805)，感谢报告 @dpmabo
- 修复了 `DataEngine` 取消订阅订单簿增量 (#1814)，感谢 @davidsblom
- 修复了 `LiveExecutionEngine` 处理适配器客户端执行报告导致 `None` 批量状态 (#1789)，感谢报告 @faysou
- 修复了 `InteractiveBrokersExecutionClient` 在生成执行报告时处理未找到的工具 (#1789)，感谢报告 @faysou
- 修复了 Bybit websocket 消息的交易和报价解析 (#1794)，感谢 @davidsblom

---

# NautilusTrader 1.196.0 Beta

发布于 2024年7月5日 (UTC)。

### 增强功能

- 添加了 `request_order_book_snapshot` 方法 (#1745)，感谢 @graceyangfan
- 当场所 `book_type` 为 `L2_MBP` 或 `L3_MBO` 时，为 `BacktestNode` 添加了订单簿数据验证
- 添加了 Bybit 演示账户支持（在配置中将 `is_demo` 设置为 `True`）
- 添加了 Bybit 止损订单类型（`STOP_MARKET`、`STOP_LIMIT`、`MARKET_IF_TOUCHED`、`LIMIT_IF_TOUCHED`、`TRAILING_STOP_MARKET`）
- 为适配器配置添加了 Binance 场所选项 (#1738)，感谢 @DevRoss
- 添加了 Betfair 修改订单数量支持 (#1687 和 #1751)，感谢 @imemo88 和 @limx0
- 为 nextest 运行器添加了 Postgres 测试串行测试组 (#1753)，感谢 @filipmacek
- 添加了 Postgres 账户持久化功能 (#1768)，感谢 @filipmacek
- 在 Rust 中重构了 `AccountAny` 模式 (#1755)，感谢 @filipmacek
- 更改 `DatabentoLiveClient` 以使用新的[订阅时快照](https://databento.com/blog/live-MBO-snapshot)功能
- 更改标识符生成器时间标签组件以包含秒（影响新的 `ClientOrderId`、`OrderId` 和 `PositionId` 生成）
- 在 Rust `network` crate 中将 `<Arc<Mutex<bool>>` 更改为 `AtomicBool`，感谢 @NextThread 和 @twitu
- 将 `KlingerVolumeOscillator` 指标移植到 Rust (#1724)，感谢 @Pushkarm029
- 将 `DirectionalMovement` 指标移植到 Rust (#1725)，感谢 @Pushkarm029
- 将 `ArcherMovingAveragesTrends` 指标移植到 Rust (#1726)，感谢 @Pushkarm029
- 将 `Swings` 指标移植到 Rust (#1731)，感谢 @Pushkarm029
- 将 `BollingerBands` 指标移植到 Rust (#1734)，感谢 @Pushkarm029
- 将 `VolatilityRatio` 指标移植到 Rust (#1735)，感谢 @Pushkarm029
- 将 `Stochastics` 指标移植到 Rust (#1736)，感谢 @Pushkarm029
- 将 `Pressure` 指标移植到 Rust (#1739)，感谢 @Pushkarm029
- 将 `PsychologicalLine` 指标移植到 Rust (#1740)，感谢 @Pushkarm029
- 将 `CommodityChannelIndex` 指标移植到 Rust (#1742)，感谢 @Pushkarm029
- 将 `LinearRegression` 指标移植到 Rust (#1743)，感谢 @Pushkarm029
- 将 `DonchianChannel` 指标移植到 Rust (#1744)，感谢 @Pushkarm029
- 将 `KeltnerChannel` 指标移植到 Rust (#1746)，感谢 @Pushkarm029
- 将 `RelativeVolatilityIndex` 指标移植到 Rust (#1748)，感谢 @Pushkarm029
- 将 `RateOfChange` 指标移植到 Rust (#1750)，感谢 @Pushkarm029
- 将 `MovingAverageConvergenceDivergence` 指标移植到 Rust (#1752)，感谢 @Pushkarm029
- 将 `OnBalanceVolume` 指标移植到 Rust (#1756)，感谢 @Pushkarm029
- 将 `SpreadAnalyzer` 指标移植到 Rust (#1762)，感谢 @Pushkarm029
- 将 `KeltnerPosition` 指标移植到 Rust (#1763)，感谢 @Pushkarm029
- 将 `FuzzyCandlesticks` 指标移植到 Rust (#1766)，感谢 @Pushkarm029

### 破坏性变更

- 将 `Actor.subscribe_order_book_snapshots` 和 `unsubscribe_order_book_snapshots` 分别重命名为 `subscribe_order_book_at_interval` 和 `unsubscribe_order_book_at_interval`（这澄清了方法行为，其中处理器然后在定期接收 `OrderBook`，与表示快照的增量集合不同）

### 修复

- 修复了 `L2_MBP` 和 `L3_MBO` 订单簿类型的 `LIMIT` 订单成交行为（未遵守限价作为制造商），感谢报告 @dpmabo
- 修复了使用多个成交开仓时 `CashAccount` PnL 计算，感谢 @Otlk
- 修复了 `NautilusKernelConfig` 的 `Environment` 枚举的 msgspec 编码和解码
- 修复了 `OrderMatchingEngine` 按订单簿类型处理报价和增量 (#1754)，感谢 @davidsblom
- 修复了当 `as_legacy_cython=False` 时 `OrderBookDelta` 的 `DatabentoDataLoader.from_dbn_file`
- 修复了 `DatabentoDataLoader` OHLCV K 线模式加载（错误地计算显示因子），感谢报告 @faysou
- 修复了 `DatabentoDataLoader` 乘数和整手大小解码，感谢报告 @faysou
- 修复了 Binance 订单报告生成 `active_symbols` 类型不匹配 (#1729)，感谢 @DevRoss
- 修复了 Binance 交易数据 websocket 模式（Binance 不再发布 `b` 买方和 `a` 卖方订单 ID）
- 修复了 `BinanceFuturesInstrumentProvider` 解析最小名义值，感谢报告 @AnthonyVince
- 修复了 `BinanceSpotInstrumentProvider` 解析最小和最大名义值
- 修复了 `INVERSE` 产品类型的 Bybit 订单簿增量订阅
- 修复了 `Cache` 的 `get` 文档（与 `add` 相同），感谢报告 @faysou

---

# NautilusTrader 1.195.0 Beta

发布于 2024年6月17日 (UTC)。

### 增强功能

- 为费率解析添加了 Bybit 基础币种 (#1696)，感谢 @filipmacek
- 添加了支持 Interactive Brokers 的 `IndexInstrument` (#1703)，感谢 @rsmb7z
- 重构了 Interactive Brokers 客户端和网关配置 (#1692)，感谢 @rsmb7z
- 改进了 `InteractiveBrokersInstrumentProvider` 合约加载 (#1699)，感谢 @rsmb7z
- 改进了 `InteractiveBrokersInstrumentProvider` 期权链加载 (#1704)，感谢 @rsmb7z
- 改进了正值四舍五入为零时 `Instrument.make_qty` 错误清晰度
- 更新了 Clang 依赖的源代码安装文档 (#1690)，感谢 @Troubladore
- 更新了 `DockerizedIBGatewayConfig` 文档 (#1691)，感谢 @Troubladore

### 破坏性变更
无

### 修复

- 修复了 DataFusion 流式后端内存使用（现在恒定内存使用）(#1693)，感谢 @twitu
- 修复了 `OrderBookDeltaDataWrangler` 快照解析（未预先添加 `CLEAR` 操作），感谢报告 @VeraLyu
- 修复了当增量具有较低精度时 `Instrument.make_price` 和 `make_qty`（未四舍五入到最小增量）
- 修复了 `EMACrossTrailingStop` 示例策略追踪止损逻辑（可能在部分成交时提交多个追踪止损）
- 修复了 Binance `TRAILING_STOP_MARKET` 订单（回调四舍五入错误，也未处理更新）
- 修复了 Interactive Brokers 多网关客户端（工厂中端口处理错误）(#1702)，感谢 @dodofarm
- 修复了文档中的时间警报 Python 示例 (#1713)，感谢 @davidsblom

---

# NautilusTrader 1.194.0 Beta

发布于 2024年5月31日 (UTC)。

### 增强功能

- 添加了 `DataEngine` 订单簿增量缓冲至 `F_LAST` 标志 (#1673)，感谢 @davidsblom
- 为上述功能添加了 `DataEngineConfig.buffer_deltas` 配置选项 (#1670)，感谢 @davidsblom
- 改进了 Bybit 订单簿增量解析以设置 `F_LAST` 标志 (#1670)，感谢 @davidsblom
- 改进了 Bybit 对顶级订单簿报价和订单簿增量的处理 (#1672)，感谢 @davidsblom
- 改进了 Interactive Brokers 集成测试模拟 (#1669)，感谢 @rsmb7z
- 改进了工具未初始化 tick 方案时的错误消息，感谢报告 @VeraLyu
- 改进了 `SandboxExecutionClient` 工具处理（工具只需添加到缓存）
- 将 `VolumeWeightedAveragePrice` 指标移植到 Rust (#1665)，感谢 @Pushkarm029
- 将 `VerticalHorizontalFilter` 指标移植到 Rust (#1666)，感谢 @Pushkarm029

### 破坏性变更
无

### 修复

- 修复了沙盒模式下 `SimulatedExchange` 实时处理命令
- 修复了 `DataEngine` 取消订阅处理（边缘情况将尝试多次从客户端取消订阅）
- 修复了 Bybit 订单簿增量解析（重复添加买方），感谢 @davidsblom
- 修复了 Binance 工具价格和大小精度解析（错误地去除尾随零）
- 修复了 `BinanceBar` 流式 feather 写入（未设置写入器）
- 修复了回测高级教程文档错误，感谢报告 @Leonz5288

---

# NautilusTrader 1.193.0 Beta

发布于 2024年5月24日 (UTC)。

### 增强功能

- 添加了 Interactive Brokers 对收盘市价 (MOC) 和收盘限价 (LOC) 订单类型的支持 (#1663)，感谢 @rsmb7z
- 添加了 Bybit 沙盒示例 (#1659)，感谢 @davidsblom
- 添加了 Binance 沙盒示例

### 破坏性变更

- 大幅改进了 `SandboxExecutionClientConfig` 以更接近 `BacktestVenueConfig`（许多变更和添加）

### 修复

- 修复了流式传输时 DataFusion 后端数据按 `ts_init` 排序 (#1656)，感谢 @twitu
- 修复了 Interactive Brokers tick 级历史数据下载 (#1653)，感谢 @DracheShiki

---

# NautilusTrader 1.192.0 Beta

发布于 2024年5月18日 (UTC)。

### 增强功能

- 添加了 Nautilus CLI（参见[文档](https://nautilustrader.io/docs/nightly/developer_guide/index.html)）(#1602)，非常感谢 @filipmacek
- 添加了支持 Interactive Brokers 的 `Cfd` 和 `Commodity` 工具 (#1604)，感谢 @DracheShiki
- 添加了 `OrderMatchingEngine` 期货和期权合约激活和到期模拟
- 添加了 Interactive Brokers 的沙盒示例 (#1618)，感谢 @rsmb7z
- 添加了 `ParquetDataCatalog` S3 支持 (#1620)，感谢 @benjaminsingleton
- 添加了 `Bar.from_raw_arrays_to_list` (#1623)，感谢 @rsmb7z
- 添加了 `SandboxExecutionClientConfig.bar_execution` 配置选项 (#1646)，感谢 @davidsblom
- 改进了场所订单 ID 生成和分配（之前 `OrderMatchingEngine` 可能为同一订单生成多个 ID）
- 改进了 `LiveTimer` 稳健性和灵活性，不再要求正间隔或未来的停止时间（将立即产生时间事件），感谢报告 @davidsblom

### 破坏性变更

- 移除了 `allow_cash_positions` 配置（简化为最常见用例，现货交易应跟踪仓位）
- 将 `tags` 参数和返回类型从 `str` 更改为 `list[str]`（更自然地表达多个标签）
- 将 `Order.to_dict()` `commission` 和 `linked_order_id` 字段更改为字符串列表而非逗号分隔字符串
- 更改 `OrderMatchingEngine` 不再处理内部聚合的 K 线以进行执行（没有测试失败，但仍归类为行为变更），感谢报告 @davidsblom

### 修复

- 修复了 `CashAccount` PnL 和余额计算（基于未平仓位数量调整成交数量 - 造成不同步和错误余额值）
- 修复了当输入字符串包含下划线时 Rust 中 `Price`、`Quantity` 和 `Money` 的 `from_str`，感谢报告 @filipmacek
- 修复了 `Money` 字符串解析，其中 `str(money)` 的值现在可以传递给 `Money.from_str`
- 修复了 `TimeEvent` 相等性（现在基于事件 `id` 而不是事件 `name`）
- 修复了按 `instrument_id` 的 `ParquetDataCatalog` K 线查询不再返回数据（意图是使用 `bar_type`，但是使用 `instrument_id` 现在返回所有匹配的 K 线）
- 修复了沙盒模式下场所订单 ID 生成和应用（之前生成额外的场所订单 ID），感谢报告 @rsmb7z 和 @davidsblom
- 修复了沙盒模式下多次成交导致超额成交（`OrderMatchingEngine` 现在缓存成交数量以防止这种情况）(#1642)，感谢 @davidsblom
- 修复了 `leaves_qty` 异常消息下溢（现在正确显示预计的负剩余数量）
- 修复了 Interactive Brokers 合约详情解析 (#1615)，感谢 @rsmb7z
- 修复了 Interactive Brokers 投资组合注册 (#1616)，感谢 @rsmb7z
- 修复了 Interactive Brokers `IBOrder` 属性分配 (#1634)，感谢 @rsmb7z
- 修复了网关/TWS 断开连接后的 IBKR 重连 (#1622)，感谢 @benjaminsingleton
- 修复了 Binance Futures 账户余额计算（用保证金抵押品高估了 `free` 余额，可能导致负 `locked` 余额）
- 修复了 Betfair 流重连并避免多次重连尝试 (#1644)，感谢 @imemo88

---

# NautilusTrader 1.191.0 Beta

发布于 2024年4月20日 (UTC)。

### 增强功能

- 实现了包括 `FixedFeeModel` 和 `MakerTakerFeeModel` 的 `FeeModel` (#1584)，感谢 @rsmb7z
- 实现了 `TradeTickDataWrangler.process_bar_data` (#1585)，感谢 @rsmb7z
- 实现了多时间框架 K 线执行（将对每个工具使用最低时间框架）
- 使用底层 `tokio` 定时器优化了 `LiveTimer` 效率和准确性
- 优化了 `QuoteTickDataWrangler` 和 `TradeTickDataWrangler` (#1590)，感谢 @rsmb7z
- 标准化了适配器客户端日志记录（处理来自客户端基类的更多日志记录）
- 简化并整合了 Rust `OrderBook` 设计
- 改进了 `CacheDatabaseAdapter` 优雅关闭和线程连接
- 改进了 `MessageBus` 优雅关闭和线程连接
- 改进了订单值保持不变时 `modify_order` 错误日志记录
- 为 Rust 和 Python 添加了 `RecordFlag` 枚举
- Interactive Brokers 进一步改进和修复，感谢 @rsmb7z
- 将 `Bias` 指标移植到 Rust，感谢 @Pushkarm029

### 破坏性变更

- 重新排序了 `OrderBookDelta` 参数 `flags` 和 `sequence` 并移除了默认 0 值（更明确，减少不匹配的机会）
- 重新排序了 `OrderBook` 参数 `flags` 和 `sequence` 并移除了默认 0 值（更明确，减少不匹配的机会）
- 为 `OrderBook.add` 添加了 `flags` 参数
- 为 `OrderBook.update` 添加了 `flags` 参数
- 为 `OrderBook.delete` 添加了 `flags` 参数
- 更改了所有工具的 Arrow 模式：添加了 `info` 二进制字段
- 更改了 `CryptoFuture` 的 Arrow 模式：添加了 `is_inverse` 布尔字段
- 将 `OrderBookMbo` 和 `OrderBookMbp` 都重命名为 `OrderBook`（合并）
- 将 `Indicator.handle_book_mbo` 和 `Indicator.handle_book_mbp` 重命名为 `handle_book`（合并）
- 将 `register_serializable_object` 重命名为 `register_serializable_type`（同时将第一个参数从 `obj` 重命名为 `cls`）

### 修复

- 修复了 `MessageBus` 模式解析（修复了发布无订阅者主题总是重新解析的性能回归）
- 修复了 `BacktestNode` 流式数据管理（未在块之间清除），感谢报告 @dpmabo
- 修复了保证金账户的 `RiskEngine` 累计名义计算（卖出时错误地使用基础货币）
- 修复了使用 `CASH` 账户和 `NETTING` OMS 卖出 `Equity` 工具错误拒绝（应该能够减少仓位）
- 修复了 Databento K 线解码（错误地应用显示因子）
- 修复了 `BinanceBar` (kline) 使用 `close_time` 作为 `ts_event` 而不是 `opentime` (#1591)，感谢报告 @OnlyC
- 修复了 `AccountMarginExceeded` 错误条件（保证金现在必须实际超额，可以为零）
- 修复了 `ParquetDataCatalog` 路径全局匹配，包含指定工具 ID 子字符串的所有路径

---

# NautilusTrader 1.190.0 Beta

发布于 2024年3月22日 (UTC)。

### 增强功能

- 添加了 Databento 适配器 `continuous`、`parent` 和 `instrument_id` 符号支持（将从符号推断）
- 添加了 `DatabaseConfig.timeout` 配置选项，用于等待新连接的超时秒数
- 添加了 CSV tick 和 K 线数据加载器参数，感谢 @rterbush
- 实现了 `LogGuard` 以确保全局日志记录器在终止时刷新，感谢 @ayush-sb 和 @twitu
- 改进了 Interactive Brokers 客户端连接弹性和组件生命周期，感谢 @benjaminsingleton
- 改进了 Binance 执行客户端 ping listen key 错误处理和日志记录
- 改进了 Redis 缓存适配器和消息总线错误处理和日志记录
- 改进了 Redis 端口解析（`DatabaseConfig.port` 现在可以是字符串或整数）
- 将 `ChandeMomentumOscillator` 指标移植到 Rust，感谢 @Pushkarm029
- 将 `VIDYA` 指标移植到 Rust，感谢 @Pushkarm029
- 重构了 `InteractiveBrokersEWrapper`，感谢 @rsmb7z
- 在字符串和日志中编辑 Redis 密码
- 将 `redis` crate 升级到 v0.25.2，提升了 TLS 依赖项，并开启了 `tls-rustls-webpki-roots` 功能标志

### 破坏性变更
无

### 修复

- 修复了日志文件输出的 JSON 格式（缺少 `timestamp` 和 `trader_id`）
- 修复了 Redis 的 `DatabaseConfig` 端口 JSON 解析（总是默认为 6379）
- 修复了 `ChandeMomentumOscillator` 指标除零错误（Rust 和 Cython 版本）

---

# NautilusTrader 1.189.0 Beta

发布于 2024年3月15日 (UTC)。

### 增强功能

- 实现了 websocket 重连时的 Binance 订单簿快照重建（参见集成指南）
- 为 `OrderMatchingEngine` 添加了额外验证（当 `OrderFilled` 的价格或大小精度不匹配工具精度时现在将引发 `RuntimeError`）
- 添加了基于 PyO3 的日志记录初始化的 `LoggingConfig.use_pyo3` 配置选项（性能较差但允许查看来自 Rust 的日志）
- 为 `FuturesContract`、`FuturesSpread`、`OptionContract` 和 `OptionSpread` 添加了 `exchange` 字段（可选）

### 破坏性变更

- 更改了 Arrow 模式，为 `FuturesContract`、`FuturesSpread`、`OptionContract` 和 `OptionSpread` 添加了 `exchange` 字段

### 修复

- 修复了主题发布后 `MessageBus` 对订阅的处理（之前为这些迟到的订阅者丢弃消息）
- 修复了某些边缘情况下 `MessageBus` 对订阅的处理（订阅列表可能在迭代时调整大小导致 `RuntimeError`）
- 修复了在消息被丢弃后 `Throttler` 发送消息的处理，感谢 @davidsblom
- 修复了 `OrderBookDelta.to_pyo3_list` 使用清除增量的零精度
- 修复了 `DataTransformer.pyo3_order_book_deltas_to_record_batch_bytes` 使用清除增量的零精度
- 修复了交叉订单簿时 `OrderBookMbo` 和 `OrderBookMbp` 完整性检查
- 修复了尝试添加到 L1_MBP 订单簿类型时 `OrderBookMbp` 错误（现在引发 `RuntimeError` 而不是 panic）
- 修复了 Interactive Brokers 连接错误日志记录 (#1524)，感谢 @benjaminsingleton
- 修复了 `SimulationModuleConfig` 位置和从 `config` 子包缺失的重新导出
- 修复了 `StdoutWriter` 日志记录也写入错误日志（写入器重复错误日志）
- 修复了 `BinanceWebSocketClient` 符合[新规范](https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams)，要求对包含 ping 载荷的 ping 响应 pong
- 修复了基于钱包和可用余额的 Binance Futures `AccountBalance` 计算
- 修复了已安装轮子的 `ExecAlgorithm` 循环导入问题（从 `execution.algorithm` 导入是循环导入）

---

# NautilusTrader 1.188.0 Beta

发布于 2024年2月25日 (UTC)。

### 增强功能

- 添加了 `FuturesSpread` 工具类型
- 添加了 `OptionSpread` 工具类型
- 添加了 `InstrumentClass.FUTURE_SPREAD`
- 添加了 `InstrumentClass.OPTION_SPREAD`
- 为 `subscribe_order_book_deltas` 添加了 `managed` 参数，默认 `True` 以保持当前行为（如果为 false，则数据引擎不会自动管理订单簿）
- 为 `subscribe_order_book_snapshots` 添加了 `managed` 参数，默认 `True` 以保持当前行为（如果为 false，则数据引擎不会自动管理订单簿）
- 为 `OrderMatchingEngine` 添加了额外验证（现在将拒绝价格或数量精度不正确的订单）
- 移除了 `subscribe_order_book_snapshots` 的 `interval_ms` 20 毫秒限制（即只需为正数），尽管我们建议在 100 毫秒以下考虑订阅增量
- 将 `LiveClock` 和 `LiveTimer` 实现移植到 Rust
- 实现了 `OrderBookDeltas` 序列化
- 在 Rust 中实现了 `AverageTrueRange`，感谢 @rsmb7z

### 破坏性变更

- 将 `TradeId` 值最大长度更改为 36 个字符（如果值超过最大值将引发 `ValueError`）

### 修复

- 修复了由于将唯一值分配给 `Ustr` 全局字符串缓存导致的 `TradeId` 内存泄漏（在程序生命周期内永不释放）
- 修复了 PyO3 转换的 `TradeTick` 大小精度（大小精度错误地是价格精度）
- 修复了卖出时 `RiskEngine` 现金值检查（之前将数量除以价格太多），感谢报告 @AnthonyVince
- 修复了 FOK 时间有效性行为（允许超出顶级成交，如果无法完全成交将取消）
- 修复了 IOC 时间有效性行为（允许超出顶级成交，在应用所有成交后将取消任何剩余）
- 修复了小间隔导致下次时间小于现在的 `LiveClock` 定时器行为（定时器然后不会运行）
- 修复了 `log_level_file` 的日志级别过滤（v1.187.0 中引入的错误），感谢 @twitu
- 修复了日志记录 `print_config` 配置选项（未传递给日志记录子系统）
- 修复了回测的日志记录时间戳（静态时钟未递增设置为单个 `TimeEvent` 时间戳）
- 修复了账户余额更新（来自零数量 `NETTING` 仓位的成交将生成账户余额更新）
- 修复了 `MessageBus` 可发布类型集合类型（需要是 `tuple` 而不是 `set`）
- 修复了 `Controller` 组件注册以确保在回测期间正确迭代所有活动时钟
- 修复了 `CASH` 账户的 `Equity` 卖空（现在将拒绝）
- 修复了 `ActorFactory.create` JSON 编码（缺少编码钩子）
- 修复了 `ImportableConfig.create` JSON 编码（缺少编码钩子）
- 修复了 `ImportableStrategyConfig.create` JSON 编码（缺少编码钩子）
- 修复了 `ExecAlgorithmFactory.create` JSON 编码（缺少编码钩子）
- 修复了 `ControllerConfig` 基类和文档字符串
- 修复了 Interactive Brokers 历史 K 线数据错误，感谢 @benjaminsingleton
- 修复了持久化 `freeze_dict` 函数以处理 `fs_storage_options`，感谢 @dimitar-petrov

---

# NautilusTrader 1.187.0 Beta

发布于 2024年2月9日 (UTC)。

### 增强功能

- 在 Rust 中优化了日志记录子系统模块和写入器，感谢 @ayush-sb 和 @twitu
- 改进了 Interactive Brokers 适配器符号和解析，添加了 `strict_symbology` 配置选项，感谢 @rsmb7z 和 @fhill2

### 破坏性变更

- 重新组织了配置对象（分离到每个子包的 `config` 模块中，从 `nautilus_trader.config` 重新导出）

### 修复

- 修复了 `BacktestEngine` 和 `Trader` 处置（现在正确释放资源），感谢报告 @davidsblom
- 修复了配置对象的循环导入问题，感谢报告 @cuberone
- 修复了文件日志记录关闭时不必要的日志文件创建

---

# NautilusTrader 1.186.0 Beta

发布于 2024年2月2日 (UTC)。

### 增强功能
无

### 破坏性变更
无

### 修复

- 修复了 Interactive Brokers 获取账户仓位错误 (#1475)，感谢 @benjaminsingleton
- 修复了 `TimeBarAggregator` 在构建时对间隔类型的处理
- 修复了 `BinanceSpotExecutionClient` 不存在的方法名，感谢 @sunlei
- 修复了未使用的 `psutil` 导入，感谢 @sunlei

---

# NautilusTrader 1.185.0 Beta

发布于 2024年1月26日 (UTC)。

### 增强功能

- 为 `LIVE` 上下文中 `bypass_logging` 设置为 true 时添加了警告日志
- 改进了 `register_serializable object` 以也将类型添加到内部 `_EXTERNAL_PUBLIHSABLE_TYPES`
- 改进了 Interactive Brokers 到期合约解析，感谢 @fhill2

### 破坏性变更

- 将 `StreamingConfig.include_types` 类型从 `tuple[str]` 更改为 `list[type]`（更好地与其他类型过滤器对齐）
- 将 `clock` 模块合并到 `component` 模块（减少二进制轮子大小）
- 将 `logging` 模块合并到 `component` 模块（减少二进制轮子大小）

### 修复

- 修复了 `OrderUpdated` 的 Arrow 序列化（`trigger_price` 类型错误），感谢 @benjaminsingleton
- 修复了 `StreamingConfig.include_types` 行为（工具写入器未被采用），感谢报告 @doublier1
- 修复了 `StrategyFactory` 中的 `ImportableStrategyConfig` 类型分配 (#1470)，感谢 @rsmb7z

---

# NautilusTrader 1.184.0 Beta

发布于 2024年1月22日 (UTC)。

### 增强功能

- 添加了 `LogLevel.OFF`（匹配 Rust `tracing` 日志级别）
- 添加了 `init_logging` 函数，具有合理的默认值来初始化 Rust 实现的日志记录子系统
- 更新了 `BinanceFuturesContractType` 和 `BinanceFuturesPositionUpdateReason` 的 Binance Futures 枚举成员
- 使用 `sysinfo` crate 改进了日志标头（添加交换空间指标和 PID 标识符）
- 移除了 Python 对 `psutil` 的依赖

### 破坏性变更

- 从 `Logger` 移除了 `clock` 参数（不再依赖 `Clock`）
- 将 `LoggerAdapter` 重命名为 `Logger`（并移除了旧的 `Logger` 类）
- 将 `Logger` `component_name` 参数重命名为 `name`（匹配 Python 内置 `logging` API）
- 将 `OptionKind` `kind` 参数和属性重命名为 `option_kind`（更好的清晰度）
- 将 `OptionContract` Arrow 模式字段 `kind` 重命名为 `option_kind`
- 将 `level_file` 日志级别更改为 `OFF`（默认关闭文件日志记录）

### 修复

- 修复了目录查询的内存泄漏 (#1430)，感谢 @twitu
- 修复了 `DataEngine` 订单簿快照定时器名称（无法解析带连字符的工具 ID），感谢报告 @x-zho14 和 @dimitar-petrov
- 修复了 `LoggingConfig` 对 `WARNING` 日志级别的解析（未被识别），感谢报告 @davidsblom
- 修复了 Binance Futures `QuoteTick` 解析以捕获 `ts_event` 的事件时间，感谢报告 @x-zho14

---

# NautilusTrader 1.183.0 Beta

发布于 2024年1月12日 (UTC)。

### 增强功能

- 添加了 `NautilusConfig.json_primitives` 以将对象转换为具有 JSON 基元值的 Python 字典
- 添加了 `InstrumentClass.BOND`
- 添加了 `MessageBusConfig` `use_trader_prefix` 和 `use_trader_id` 配置选项（提供对流名称的更多控制）
- 添加了 `CacheConfig.drop_instruments_on_reset`（默认 `True` 以保持当前行为）
- 通过 `log` crate 实现了核心日志记录接口，感谢 @twitu
- 在 Rust 中实现了全局原子时钟（提高性能并确保实时中正确的单调时间戳），感谢 @twitu
- 改进了 Interactive Brokers 适配器仅在需要时引发 docker `RuntimeError`（使用 TWS 时不会），感谢 @rsmb7z
- 将核心 HTTP 客户端升级到最新的 `hyper` 和 `reqwest`，感谢 @ayush-sb
- 优化了 Arrow 编码（为 Parquet 数据目录实现约 100 倍更快的写入）

### 破坏性变更

- 将 `ParquetDataCatalog` 自定义数据前缀从 `geneticdata_` 更改为 `custom_`（您需要重命名任何目录子目录）
- 将 `ComponentStateChanged` Arrow 模式的 `config` 从 `string` 更改为 `binary`
- 将 `OrderInitialized` Arrow 模式的 `options` 从 `string` 更改为 `binary`
- 将 `OrderBookDeltas` 字典表示的 `deltas` 字段从 JSON `bytes` 更改为 `dict` 列表（与所有其他数据类型标准化）
- 将外部消息发布流名称键更改为 `trader-{trader_id}-{instance_id}-streams`（带选项允许许多交易者发布到相同流）
- 为了清晰起见，将所有版本 2 数据整理器类重命名为 `V2` 后缀
- 将 `GenericData` 重命名为 `CustomData`（更准确地反映类型的性质）
- 将 `DataClient.subscribed_generic_data` 重命名为 `.subscribed_custom_data`
- 将 `MessageBusConfig.stream` 重命名为 `.streams_prefix`（更准确）
- 将 `ParquetDataCatalog.generic_data` 重命名为 `.custom_data`
- 将 `TradeReport` 重命名为 `FillReport`（更常规的术语，更清楚地分离市场数据和用户执行报告）
- 在代码库中将 `asset_type` 重命名为 `instrument_class`（更常规的术语）
- 将 `AssetType` 枚举重命名为 `InstrumentClass`（更常规的术语）
- 将 `AssetClass.BOND` 重命名为 `AssetClass.DEBT`（更常规的术语）
- 移除了 `AssetClass.METAL`（严格来说不是资产类别，更像是期货类别）
- 移除了 `AssetClass.ENERGY`（严格来说不是资产类别，更像是期货类别）
- 从 `Equity` 构造函数移除了 `multiplier` 参数（不适用）
- 从 `Equity` 字典表示移除了 `size_precision`、`size_increment` 和 `multiplier` 字段（不适用）
- 移除了 `TracingConfig`（在新的日志记录实现中现已冗余）
- 移除了 `Ticker` 数据类型和相关方法（不是可以实际标准化的类型，因此变成适配器特定的通用数据）
- 将 `AssetClass.SPORTS_BETTING` 移动到 `InstrumentClass.SPORTS_BETTING`

### 修复

- 修复了日志记录器线程泄漏，感谢 @twitu
- 修复了配置对象的处理以与 `StreamingFeatherWriter` 一起工作
- 修复了部分工具加载的 `BinanceSpotInstrumentProvider` 费用加载键错误，感谢报告 @doublier1
- 修复了测试网的 Binance API 密钥配置解析（被传递到非测试网环境变量）
- 修复了当第一个订单应为整个大小时 TWAP 执行算法调度大小处理，感谢报告 @pcgm-team
- 添加了 `BinanceErrorCode.SERVER_BUSY` (-1008)，也添加到重试错误代码
- 添加了 `BinanceOrderStatus.EXPIRED_IN_MATCH`，即订单因自交易防护 (STP) 被交易所取消，感谢报告 @doublier1

---

# NautilusTrader 1.182.0 Beta

发布于 2023年12月23日 (UTC)。

### 增强功能

- 添加了 `CacheDatabaseFacade` 和 `CacheDatabaseAdapter` 以从 Python 代码库抽象后端技术
- 添加了在 Rust 中实现的 `RedisCacheDatabase`，为插入、更新和删除操作使用单独的 MPSC 通道线程
- 添加了 TA-Lib 集成，感谢 @rsmb7z
- 将 `OrderBookDelta` 和 `OrderBookDeltas` 添加到可序列化和可发布类型
- 将 `PortfolioFacade` 移动到 `Actor`
- 改进了 `Actor` 和 `Strategy` 可用性，对构造函数中对 `clock` 和 `logger` 的错误调用更宽容（文档中也添加了警告）
- 从 Python 代码库移除了 `redis` 和 `hiredis` 依赖项

### 破坏性变更

- 更改配置对象以接受更强类型，因为这些现在在注册时可序列化（而不是基元）
- 将 `NautilusKernelConfig.trader_id` 更改为类型 `TraderId`
- 将 `BacktestDataConfig.instrument_id` 更改为类型 `InstrumentId`
- 将 `ActorConfig.component_id` 更改为类型 `ComponentId | None`
- 将 `StrategyConfig.strategy_id` 更改为类型 `StrategyId | None`
- 由于下面的修复，更改了 `Instrument`、`OrderFilled` 和 `AccountState` `info` 字段序列化（您需要刷新缓存）
- 更改 `CacheConfig` 以接受 `DatabaseConfig`（与 `MessageBusConfig` 更好的对称性）
- 将 `RedisCacheDatabase` 数据结构中的货币从哈希集合更改为更简单的键值（您需要清除缓存或删除所有货币键）
- 更改 `Actor` 状态加载现在使用标准 `Serializer`
- 将 `register_json_encoding` 重命名为 `register_config_encoding`
- 将 `register_json_decoding` 重命名为 `register_config_decoding`
- 移除了 `CacheDatabaseConfig`（由于上述配置更改）
- 移除了 `infrastructure` 子包（在新的 Rust 实现中现已冗余）

### 修复

- 修复了来自下面 `info` 字段序列化修复的 `CacheDatabaseAdapter` 的 `json` 编码
- 修复了 `Instrument`、`OrderFilled` 和 `AccountState` `info` 字段序列化以保留 JSON 可序列化字典（而不是双重编码和丢失信息）
- 修复了当 `time_in_force` 不是 GTD 时的 Binance Futures `good_till_date` 值，例如策略管理 GTD 时（错误地传递了 UNIX 毫秒）
- 修复了 `Executor` 对排队任务 ID 的处理（完成时未从排队任务中丢弃）
- 修复了 `DataEngine` 对非常小间隔的订单簿快照处理（现在处理短至 20 毫秒）
- 修复了 `BacktestEngine.clear_actors()`、`BacktestEngine.clear_strategies()` 和 `BacktestEngine.clear_exec_algorithms()`，感谢报告 @davidsblom
- 修复了 `BacktestEngine` OrderEmulator 重置，感谢 @davidsblom
- 修复了 `Throttler.reset` 和 `RiskEngine` 节流器的重置，感谢 @davidsblom

---

# NautilusTrader 1.181.0 Beta

发布于 2023年12月2日 (UTC)。

此版本添加了对 Python 3.12 的支持。

### 增强功能

- 重写了 Interactive Brokers 集成文档，非常感谢 @benjaminsingleton
- 添加了 Interactive Brokers 适配器对具有现金数量的加密工具的支持，感谢 @benjaminsingleton
- 添加了 `HistoricInteractiveBrokerClient`，感谢 @benjaminsingleton 和 @limx0
- 添加了 `DataEngineConfig.time_bars_interval_type`（确定用于时间聚合的间隔类型 `left-open` 或 `right-open`）
- 添加了 `LoggingConfig.log_colors` 以可选择使用 ANSI 代码产生彩色日志（默认 `True` 以保持当前行为）
- 为 `QuoteTickDataWrangler.process_bar_data` 添加了 `offset_interval_ms` 和 `timestamp_is_close` 选项
- 在 Rust 中添加了标识符生成器，感谢 @filipmacek
- 在 Rust 中添加了 `OrderFactory`，感谢 @filipmacek
- 在 Rust 中添加了 `WilderMovingAverage`，感谢 @ayush-sb
- 在 Rust 中添加了 `HullMovingAverage`，感谢 @ayush-sb
- 在 Rust 中添加了所有常见标识符生成器，感谢 @filipmacek
- 在 Rust 中添加了使用 `sqlx` 的通用 SQL 数据库支持，感谢 @filipmacek

### 破坏性变更

- 将所有 `data` 子模块合并为一个 `data` 模块（减少二进制轮子大小）
- 将 `OrderBook` 从 `model.orderbook.book` 移动到 `model.book`（子包只有这一个模块）
- 将 `Currency` 从 `model.currency` 移动到 `model.objects`（合并模块以减少二进制轮子大小）
- 将 `MessageBus` 从 `common.msgbus` 移动到 `common.component`（合并模块以减少二进制轮子大小）
- 将 `MsgSpecSerializer` 从 `serialization.msgpack.serializer` 移动到 `serialization.serializer`
- 将 `CacheConfig` `snapshot_orders`、`snapshot_positions`、`snapshot_positions_interval` 移动到 `NautilusKernelConfig`（逻辑适用性）
- 将 `MsgPackSerializer` 重命名为 `MsgSpecSeralizer`（现在处理 JSON 和 MsgPack 格式）

### 修复

- 修复了 `Position` 字典表示中缺失的 `trader_id`，感谢 @filipmacek
- 修复了定点整数到浮点数的转换（应该除法以避免舍入错误），感谢报告 @filipmacek
- 修复了 Interactive Brokers 的每日时间戳解析，感谢 @benjaminsingleton
- 修复了部分成交然后取消订单的实时对账交易处理
- 修复了多货币现金账户上 `CurrencyPair` SELL 订单的 `RiskEngine` 累计名义风险检查

---

# NautilusTrader 1.180.0 Beta

发布于 2023年11月3日 (UTC)。

### 增强功能

- 通过使用 `loop.call_soon_threadsafe(...)` 改进了实时引擎的内部延迟
- 改进了 `RedisCacheDatabase` 客户端连接错误处理和重试
- 添加了 `WebSocketClient` 连接标头，感谢 @ruthvik125 和 @twitu
- 为场所添加了 `support_contingent_orders` 配置选项（模拟不支持条件订单的场所）
- 添加了 `StrategyConfig.manage_contingent_orders` 配置选项（自动管理**开放的**条件订单）
- 添加了返回 `pd.Timestamp` 时区感知 (UTC) 的 `FuturesContract.activation_utc` 属性
- 添加了返回 `pd.Timestamp` 时区感知 (UTC) 的 `OptionContract.activation_utc` 属性
- 添加了返回 `pd.Timestamp` 时区感知 (UTC) 的 `CryptoFuture.activation_utc` 属性
- 添加了返回 `pd.Timestamp` 时区感知 (UTC) 的 `FuturesContract.expiration_utc` 属性
- 添加了返回 `pd.Timestamp` 时区感知 (UTC) 的 `OptionContract.expiration_utc` 属性
- 添加了返回 `pd.Timestamp` 时区感知 (UTC) 的 `CryptoFuture.expiration_utc` 属性

### 破坏性变更

- 将 `FuturesContract.expiry_date` 重命名为 `expiration_ns`（及相关参数）作为 `uint64_t` UNIX 纳秒
- 将 `OptionContract.expiry_date` 重命名为 `expiration_ns`（及相关参数）作为 `uint64_t` UNIX 纳秒
- 将 `CryptoFuture.expiry_date` 重命名为 `expiration_ns`（及相关参数）作为 `uint64_t` UNIX 纳秒
- 更改了 `FuturesContract` Arrow 模式
- 更改了 `OptionContract` Arrow 模式
- 更改了 `CryptoFuture` Arrow 模式
- 转换的订单现在将保留原始 `ts_init` 时间戳
- 移除了 `Strategy.modify_order` 的未实现 `batch_more` 选项
- 移除了 `InstrumentProvider.venue` 属性（冗余，因为提供商可能有多个场所）
- 停止支持 Python 3.9

### 修复

- 修复了 `ParquetDataCatalog` 文件写入模板，感谢 @limx0
- 修复了使用 `start` 参数时会省略订单报告的 Binance 全部订单请求
- 修复了重启时管理的 GTD 订单过期取消（订单未被取消）
- 修复了订单取消时管理的 GTD 订单取消定时器（定时器未被取消）
- 修复了立即停止时 `BacktestEngine` 日志记录错误（由某些时间戳为 `None` 引起）
- 修复了回测运行期间 `BacktestNode` 异常阻止下一次顺序运行，感谢报告 @cavan-black
- 修复了通过放宽 `BinanceSpotSymbolInfo.permissions` 类型来解决 `BinanceSpotPersmission` 值错误
- Interactive Brokers 适配器各种修复，感谢 @rsmb7z

---

# NautilusTrader 1.179.0 Beta

发布于 2023年10月22日 (UTC)。

此版本的主要功能是 `ParquetDataCatalog` 第2版，这代表了数月的集体努力，感谢 Brad @limx0、@twitu、@ghill2 和 @davidsblom 的贡献。

这将是支持 Python 3.9 的最后一个版本。

### 增强功能

- 添加了支持内置数据类型 `OrderBookDelta`、`QuoteTick`、`TradeTick` 和 `Bar` 的 `ParquetDataCatalog` v2
- 添加了 `Strategy` 特定的订单和仓位事件处理器
- 添加了 `ExecAlgorithm` 特定的订单和仓位事件处理器
- 添加了 `Cache.is_order_pending_cancel_local(...)`（跟踪取消转换中的本地订单）
- 添加了 `BinanceTimeInForce.GTD` 枚举成员（仅期货）
- 添加了 Binance Futures 对 GTD 订单的支持
- 添加了来自聚合交易或 1-MINUTE K 线的 Binance 内部 K 线聚合推断（取决于回望窗口）
- 添加了 `BinanceExecClientConfig.use_gtd` 配置选项（重新映射到 GTC 并本地管理 GTD 订单）
- 添加了 `nautilus_ibapi` 的包版本检查，感谢 @rsmb7z
- 添加了 `RiskEngine` 最小/最大工具名义限制检查
- 添加了用于动态控制 `Trader` 的参与者和策略实例的 `Controller`
- 添加了提供每个个人成交事件行的 `ReportProvider.generate_fills_report(...)`，感谢 @r3k4mn14r
- 将指标注册和数据处理移至 `Actor`（现在可用于 `Actor`）
- 实现了 Binance `WebSocketClient` 实时订阅和取消订阅
- 实现了 `BinanceCommonDataClient` 对 `update_instruments` 的重试
- 去 Cython 化 `Trader`

### 破坏性变更

- 将 `BookType.L1_TBBO` 重命名为 `BookType.L1_MBP`（更准确的定义，因为 L1 是任一侧的顶级价格）
- 将 `VenueStatusUpdate` 重命名为 `VenueStatus`
- 将 `InstrumentStatusUpdate` 重命名为 `InstrumentStatus`
- 将 `Actor.subscribe_venue_status_updates(...)` 重命名为 `Actor.subscribe_venue_status(...)`
- 将 `Actor.subscribe_instrument_status_updates(...)` 重命名为 `Actor.subscribe_instrument_status(...)`
- 将 `Actor.unsubscribe_venue_status_updates(...)` 重命名为 `Actor.unsubscribe_venue_status(...)`
- 将 `Actor.unsubscribe_instrument_status_updates(...)` 重命名为 `Actor.unsubscribe_instrument_status(...)`
- 将 `Actor.on_venue_status_update(...)` 重命名为 `Actor.on_venue_status(...)`
- 将 `Actor.on_instrument_status_update(...)` 重命名为 `Actor.on_instrument_status(...)`
- 更改了 `InstrumentStatus` 字段/模式和构造函数
- 将 `manage_gtd_expiry` 从 `Strategy.submit_order(...)` 和 `Strategy.submit_order_list(...)` 移动到 `StrategyConfig`（更简单，允许在启动时重新激活任何 GTD 定时器）

### 修复

- 修复了 `LimitIfTouchedOrder.create`（`exec_algorithm_params` 未被传入）
- 修复了 `OrderEmulator` 启动时处理 OTO 条件订单（当来自父级的仓位开放时）
- 修复了 `SandboxExecutionClientConfig` `kw_only=True` 以允许在不初始化的情况下导入
- 修复了 `OrderBook` 序列化（未包含所有属性），感谢 @limx0
- 修复了开放仓位快照竞态条件（添加了 `open_only` 标志）
- 修复了处于 `INITIALIZED` 状态且带有 `emulation_trigger` 的订单的 `Strategy.cancel_order`（未向 `OrderEmulator` 发送命令）
- 修复了 `BinanceWebSocketClient` 重连行为（由于来自 Rust 的事件循环问题，重连处理器未被调用）
- 修复了 Binance 工具缺失的最大名义值，感谢报告 @AnthonyVince 和修复 @filipmacek
- 修复了回测的 Binance Futures 费率
- 修复了 `Timer` 对非正间隔缺失的条件检查
- 修复了涉及整数的 `Condition` 检查，之前默认为 32 位并溢出
- 修复了 `ReportProvider.generate_order_fills_report(...)`，它缺少未处于最终 `FILLED` 状态的订单的部分成交，感谢 @r3k4mn14r

---

# NautilusTrader 1.178.0 Beta

发布于 2023年9月2日 (UTC)。

### 增强功能
无

### 破坏性变更
无

### 修复

- 修复了 `OrderBookDelta.clear` 方法（其中 `sequence` 字段与 `flags` 交换导致溢出）
- 修复了 `OrderManager` 在成交时的 OTO 条件处理
- 修复了 `OrderManager` 重复的订单取消事件（处理条件时的竞态条件）
- 修复了 `Cache` 加载已初始化的模拟订单（未被正确索引为模拟）
- 修复了全深度增量的 Binance 订单簿订阅（未请求初始快照），感谢报告 @doublier1

---

# NautilusTrader 1.177.0 Beta

发布于 2023年8月26日 (UTC)。

此版本包括对报价 tick 买卖价格属性和参数命名的重大破坏性变更。这是为了维护我们通常明确的命名标准，过去曾对一些用户造成困惑。使用 'bid' 和 'ask' 列的数据应该仍可以与传统数据整理器一起使用，因为列在底层被重命名以适应此变更。

### 增强功能

- 添加了具有 `Actor` API 的 `ActorExecutor`，用于在实时环境中创建和运行线程任务
- 添加了 `OrderEmulated` 事件和相关的 `OrderStatus.EMULATED` 枚举变量
- 添加了 `OrderReleased` 事件和相关的 `OrderStatus.RELEASED` 枚举变量
- 添加了 `BacktestVenueConfig.use_position_ids` 配置选项（默认 `True` 以保持当前行为）
- 添加了 `Cache.exec_spawn_total_quantity(...)` 便利方法
- 添加了 `Cache.exec_spawn_total_filled_qty(...)` 便利方法
- 添加了 `Cache.exec_spawn_total_leaves_qty(...)` 便利方法
- 添加了 `WebSocketClient.send_text`，感谢 @twitu
- 为 `TimeEvent` 实现了字符串内化

### 破坏性变更

- 将 `QuoteTick.bid` 重命名为 `bid_price`，包括所有相关参数（明确命名标准）
- 将 `QuoteTick.ask` 重命名为 `ask_price`，包括所有相关参数（明确命名标准）

### 修复

- 修复了 `HEDGING` 模式下执行算法的 `position_id` 分配
- 修复了 `OrderMatchingEngine` 对模拟订单的处理
- 修复了 `OrderEmulator` 对执行算法订单的处理
- 修复了 `ExecutionEngine` 对执行算法订单的处理（执行衍生 ID）
- 修复了 `Cache` 模拟订单索引（关闭时未从集合中正确丢弃）
- 修复了 `RedisCacheDatabase` 加载转换的 `LIMIT` 订单
- 修复了 IB 客户端的连接问题，感谢 @dkharrat 和 @rsmb7z

---

# NautilusTrader 1.176.0 Beta

发布于 2023年7月31日 (UTC)。

### 增强功能

- 使用 [ustr](https://github.com/anderslanglands/ustr) crate 实现了字符串内化，感谢 @twitu
- 添加了 `SyntheticInstrument` 功能，包括动态推导公式
- 添加了 `Order.commissions()` 便利方法（也添加到状态快照字典）
- 添加了 `Cache` 仓位和订单状态快照（通过 `CacheConfig` 配置）
- 添加了 `CacheDatabaseConfig.timestamps_as_iso8601` 以将时间戳持久化为 ISO 8601 字符串
- 添加了 `LiveExecEngineConfig.filter_position_reports` 以从对账中过滤仓位报告
- 添加了 `Strategy.cancel_gtd_expiry` 以取消管理的 GTD 订单到期
- 添加了 Binance Futures 对修改 `LIMIT` 订单的支持
- 添加了 `BinanceExecClientConfig.max_retries` 配置选项（用于重试订单提交和取消请求）
- 添加了 `BinanceExecClientConfig.retry_delay` 配置选项（重试尝试之间的延迟）
- 添加了 `BinanceExecClientConfig.use_reduce_only` 配置选项（默认 `True` 以保持当前行为）
- 添加了 `BinanceExecClientConfig.use_position_ids` 配置选项（默认 `True` 以保持当前行为）
- 添加了 `BinanceExecClientConfig.treat_expired_as_canceled` 选项（默认 `False` 以保持当前行为）
- 添加了 `BacktestVenueConfig.use_reduce_only` 配置选项（默认 `True` 以保持当前行为）
- 添加了 `MessageBus.is_pending_request(...)` 方法
- 为核心 `OrderBook` 添加了 `Level` API（暴露订单簿的买卖级别）
- 添加了 `Actor.is_pending_request(...)` 便利方法
- 添加了 `Actor.has_pending_requests()` 便利方法
- 添加了 `Actor.pending_requests()` 便利方法
- 添加了 `USDP`（Pax Dollar）和 `TUSD`（TrueUSD）稳定币
- 改进了 `OrderMatchingEngine` 无成交时的处理（现在记录错误）
- 改进了 Binance 实时客户端日志记录
- 将 Cython 升级到 v3.0.0 稳定版

### 破坏性变更

- 将 `filter_unclaimed_external_orders` 从 `ExecEngineConfig` 移动到 `LiveExecEngineConfig`
- 所有 `Actor.request_*` 方法不再接受 `request_id`，但现在返回 `UUID4` 请求 ID
- 移除了 `BinanceExecClientConfig.warn_gtd_to_gtd`（现在总是 `INFO` 级别日志）
- 将 `Instrument.native_symbol` 重命名为 `raw_symbol`（您必须手动迁移或刷新缓存的工具）
- 将 `Position.cost_currency` 重命名为 `settlement_currency`（标准化术语）
- 将 `CacheDatabaseConfig.flush` 重命名为 `flush_on_start`（为了清晰）
- 更改 `Order.ts_last` 以表示最后一个_事件_的 UNIX 纳秒时间戳（而不是成交）

### 修复

- 修复了 `Portfolio.net_position` 计算以使用 `Decimal` 而不是 `float` 以避免舍入错误
- 修复了 `OrderFactory` 订单标识符生成的竞态条件
- 修复了订单的 `venue_order_id` 字典表示（针对三种订单类型）
- 修复了创建时 `Currency` 与核心全局映射的注册
- 修复了 `OrderInitialized.exec_algorithm_params` 的序列化以符合规范（字节而不是字符串）
- 修复了条件订单的仓位 ID 分配（当父级成交时）
- 修复了 `PENDING_CANCEL` -> `EXPIRED` 作为有效状态转换（现实世界的可能性）
- 修复了部分成交时 `reduce_only` 订单的成交处理
- 修复了 Binance 对账，它为同一符号多次请求报告
- 修复了 Binance Futures 原生符号解析（实际上是 Nautilus 符号值）
- 修复了 Binance Futures `PositionStatusReport` 对仓位方向的解析
- 修复了 Binance Futures `TradeReport` 仓位 ID 分配（硬编码为对冲模式）
- 修复了 Binance 执行提交订单列表
- 修复了 Binance 对 `InstrumentProvider` 的佣金费率请求
- 修复了 Binance `TriggerType` 解析 #1154，感谢报告 @davidblom603
- 修复了执行报告中无效订单的 Binance 订单解析 #1157，感谢报告 @graceyangfan
- 扩展了 `BinanceOrderType` 枚举成员以包含未记录的 `INSURANCE_FUND`，感谢报告 @Tzumx
- 扩展了 `BinanceSpotPermissions` 枚举成员 #1161，感谢报告 @davidblom603

---

# NautilusTrader 1.175.0 Beta

发布于 2023年6月16日 (UTC)。

对于此版本，Betfair 适配器已损坏，等待与新 Rust 订单簿的集成。如果您正在使用 Betfair 适配器，我们建议您不要升级到此版本。

### 增强功能

- 将 Interactive Brokers 适配器 v2 集成到平台中，感谢 @rsmb7z
- 将核心 Rust `OrderBook` 集成到平台中
- 集成了核心 Rust `OrderBookDelta` 数据类型
- 添加了基于 `hyper` 的核心 Rust `HttpClient`，感谢 @twitu
- 添加了基于 `tokio-tungstenite` 的核心 Rust `WebSocketClient`，感谢 @twitu
- 添加了基于 `tokio` `TcpStream` 的核心 Rust `SocketClient`，感谢 @twitu
- 添加了 `quote_quantity` 参数以确定订单数量是否以报价货币计价
- 添加了 `trigger_instrument_id` 参数以从替代工具价格触发模拟订单
- 为 `add_venue(...)` 方法添加了 `use_random_ids`，控制场所订单、仓位和交易 ID 是否为随机 UUID4（当前行为无变化）
- 添加了 `ExecEngineConfig.filter_unclaimed_external_orders` 配置选项，如果带有 `EXTERNAL` 策略 ID 的未声明订单事件应被过滤/丢弃
- 更改 `BinanceHttpClient` 以使用新的核心 HTTP 客户端
- 定义了数据的公共 API，现在可以直接从 `nautilus_trader.model.data` 导入（去嵌套命名空间）
- 定义了事件的公共 API，现在可以直接从 `nautilus_trader.model.events` 导入（去嵌套命名空间）

### 破坏性变更

- 将 `pandas` 升级到 v2
- 移除了 `OrderBookSnapshot`（冗余，可以表示为初始 CLEAR 后跟增量）
- 移除了 `OrderBookData`（冗余）
- 将 `Actor.handle_order_book_delta` 重命名为 `handle_order_book_deltas`（更清楚地反映 `OrderBookDeltas` 数据类型）
- 将 `Actor.on_order_book_delta` 重命名为 `on_order_book_deltas`（更清楚地反映 `OrderBookDeltas` 数据类型）
- 将 `inverse_as_quote` 重命名为 `use_quote_for_inverse`（模糊名称，仅适用于反向工具的名义计算）
- 更改了 `Data` 合约（自定义数据），[参见文档](https://nautilustrader.io/docs/latest/concepts/advanced/data.html)
- 将核心 `LogMessage` 重命名为 `LogEvent`，以更清楚地区分 `message` 字段和事件结构本身（与 [vector](https://vector.dev/docs/about/under-the-hood/architecture/data-model/log/) 语言对齐）
- 将核心 `LogEvent.timestamp_ns` 重命名为 `LogEvent.timestamp`（影响 JSON 格式的字段名）
- 将核心 `LogEvent.msg` 重命名为 `LogEvent.message`（影响 JSON 格式的字段名）

### 修复

- 更新了 `BinanceAccountType` 枚举成员和相关文档
- 修复了 `BinanceCommonExecutionClient` 对 `OrderList` 订单的迭代
- 修复了 `BinanceWebSocketClient` 的心跳（新 Rust 客户端现在响应 `pong` 帧）
- 修复了 Binance 适配器对 `orderId`、`fromId`、`startTime` 和 `endTime` 的类型（都是 int），感谢报告 @davidsblom
- 修复了 `Currency` 相等性基于 `code` 字段（避免 FFI 上的相等性问题），感谢报告 @Otlk
- 修复了 `BinanceInstrumentProvider` 对初始和维护保证金值的解析

---

# NautilusTrader 1.174.0 Beta

发布于 2023年5月19日 (UTC)。

### 破坏性变更

- Parquet 模式现在正在向目录 v2 转变（如果使用传统目录，我们建议您不要升级）
- 将订单簿数据从 `model.orderbook.data` 移动到 `model.data.book` 命名空间

### 增强功能

- 改进了回测账户爆仓场景的处理（余额为负或保证金超额）
- 添加了 `AccountMarginExceeded` 异常并优化了 `AccountBalanceNegative`
- 对 Binance 客户端错误处理和日志记录进行了各种改进
- 改进了 Binance HTTP 错误消息

### 修复

- 修复了模拟订单条件的处理（不基于衍生算法订单的状态）
- 修复了从策略发送执行算法命令
- 修复了 `OrderEmulator` 释放已关闭的订单
- 修复了 `MatchingEngine` 对子条件订单的仅减少处理
- 修复了 `MatchingEngine` 对子条件订单的仓位 ID 分配
- 修复了 `Actor` 对请求历史数据的处理（现在无论状态如何都会调用 `on_historical_data`），感谢报告 @miller-moore
- 修复了 `pyarrow` 模式字典索引键过窄（int8 -> int16），感谢报告 @rterbush

---

# NautilusTrader 1.173.0 Beta

发布于 2023年5月5日 (UTC)。

### 破坏性变更
无

### 增强功能
无

### 修复

- 修复了基于时间事件 `ts_init` 的 `BacktestEngine` 对场所消息队列的处理
- 修复了 `Position.signed_decimal_qty`（f-string 中格式精度错误），感谢报告 @rsmb7z
- 修复了 `reduce_only` 指令的追踪止损类型订单更新，感谢报告 @Otlk
- 修复了活跃执行算法订单的更新（事件未被缓存）
- 修复了应用待处理事件的条件检查（不应用于 `INITIALIZED` 状态的订单）

---

# NautilusTrader 1.172.0 Beta

发布于 2023年4月30日 (UTC)。

### 破坏性变更

- 移除了传统的 Rust parquet 数据目录后端（基于 arrow2）
- 移除了 Binance 的 `clock_sync_interval_secs` 配置（冗余/未使用，应在系统级处理）
- 从 Rust 日志记录器移除了冗余的速率限制（以及相关的 `rate_limit` 配置参数）
- 将 `Future` 工具重命名为 `FuturesContract`（避免歧义）
- 将 `Option` 工具重命名为 `OptionContract`（避免歧义和 Rust 中的命名冲突）
- 恢复默认订单和仓位标识符的小时和分钟时间组件（更容易调试，减少冲突）
- 为过去或当前时间设置时间警报将生成立即的 `TimeEvent`（而不是无效）

### 增强功能

- 添加了新的 DataFusion Rust parquet 数据目录后端（尚未集成到 Python 中）
- 为 `StrategyConfig` 添加了 `external_order_claims` 配置选项（用于声明每个工具的外部订单）
- 添加了 `Order.signed_decimal_qty()`
- 添加了 `Cache.orders_for_exec_algorithm(...)`
- 添加了 `Cache.orders_for_exec_spawn(...)`
- 在示例中添加了 `TWAPExecAlgorithm` 和 `TWAPExecAlgorithmConfig`
- 构建 `ExecAlgorithm` 基类以实现"一流"执行算法
- 重新布线执行以改进模拟订单、执行算法和 `RiskEngine` 之间的流程灵活性
- 改进了 `OrderEmulator` 从执行算法更新条件订单的处理
- 定义了工具的公共 API，现在可以直接从 `nautilus_trader.model.instruments` 导入（去嵌套命名空间）
- 定义了订单的公共 API，现在可以直接从 `nautilus_trader.model.orders` 导入（去嵌套命名空间）
- 定义了订单簿的公共 API，现在可以直接从 `nautilus_trader.model.orderbook` 导入（去嵌套命名空间）
- 现在在构建后剥离调试符号（减少二进制轮子大小）
- 优化了构建并添加了额外的 `debug` Makefile 便利目标

### 修复

- 修复了处于待更新状态时条件订单的处理
- 修复了翻转仓位的 PnL 计算（仅对开放仓位记录已实现 PnL）
- 修复了 `WebSocketClient` 会话断开连接，感谢报告 @miller-moore
- 添加了缺失的 `BinanceSymbolFilterType.NOTIONAL`
- 修复了 `Price` 和 `Quantity` 的错误 `Mul` trait（未在 Cython/Python 层使用）

---

# NautilusTrader 1.171.0 Beta

发布于 2023年3月30日 (UTC)。

### 破坏性变更

- 将所有仓位 `net_qty` 字段和参数重命名为 `signed_qty`（更准确的命名）
- `NautilusKernelConfig` 移除了所有 `log_*` 配置选项（由带有 `LoggingConfig` 的 `logging` 替换）
- 不再允许使用_单一货币_ `CASH` 账户类型交易 `CurrencyPair` 工具（不现实）
- 更改了 `PositionEvent` parquet 模式（将 `net_qty` 字段重命名为 `signed_qty`）

### 增强功能

- 添加了 `LoggingConfig` 以整合日志记录配置，提供各种文件选项和每个组件级别过滤器
- 添加了 `BacktestVenueConfig.bar_execution` 以控制 K 线数据是否移动匹配引擎市场（重新启用）
- 为参与者数据请求添加了可选的 `request_id`（有助于处理响应），感谢 @rsmb7z
- 添加了 `Position.signed_decimal_qty()`
- 现在使用上述有符号数量进行 `Portfolio` 净仓位计算和 `LiveExecutionEngine` 对账比较

### 修复

- 修复了 `BacktestEngine` 时钟和日志记录器处理（有冗余的额外日志记录器，并且在运行后未交换实时时钟）
- 修复了 `MarketOrder` 和 `SubmitOrder` 的 `close_position` 订单事件发布和缓存持久化，感谢报告 @rsmb7z

---

# NautilusTrader 1.170.0 Beta

发布于 2023年3月11日 (UTC)。

### 破坏性变更

- 将 `backtest.data.providers` 移动到 `test_kit.providers`
- 将 `backtest.data.wranglers` 移动到 `persistence.wranglers`（将被整合）
- 将 `backtest.data.loaders` 移动到 `persistence.loaders`（将被整合）
- 在数据请求方法和属性中将 `from_datetime` 重命名为 `start`
- 在数据请求方法和属性中将 `to_datetime` 重命名为 `end`
- 移除了 `RiskEngineConfig.deny_modify_pending_update`（由于新的待处理事件排序现已冗余）
- 移除了冗余的日志接收器机制
- 更改了 parquet 目录模式字典整数键宽度/类型
- 由于 Cython 3.0.0b1 升级，使所有序列化数据无效

### 增强功能

- 在核心 Rust 级别添加了文件日志记录
- 添加了 `DataCatalogConfig` 以获得更一致的数据目录配置
- 添加了 `DataEngine.register_catalog` 以支持历史数据请求
- 向基础 `NautilusKernelConfig` 添加了 `catalog_config` 字段
- 更改为在 `Strategy` 中立即缓存订单和订单列表
- 更改为在 `Strategy` 中检查重复的 `client_order_id` 和 `order_list_id`
- 更改为在 `Strategy` 中生成和应用 `OrderPendingUpdate` 和 `OrderPendingCancel`
- `PortfolioAnalyzer` PnL 统计现在接受可选的 `unrealized_pnl`
- 回测性能统计现在在总 PnL 中包括未实现 PnL

### 修复

- 修复了 Binance Futures 触发类型解析
- 修复了 `DataEngine` K 线订阅和取消订阅逻辑，感谢报告 @rsmb7z
- 修复了 `Actor` 对 K 线的处理，感谢 @limx0
- 修复了对尚未在匹配核心中的条件订单的 `CancelAllOrders` 命令处理
- 修复了无 `trigger_price` 时 `TrailingStopMarketOrder` 滑点计算，感谢报告 @rsmb7z
- 修复了 `BinanceSpotInstrumentProvider` 对报价资产的解析（之前使用基础），感谢报告 @logogin
- 修复了未记录的 Binance 时间有效性 'GTE_GTC'，感谢报告 @graceyangfan
- 修复了当佣金货币等于基础货币时 `Position` 对 `last_qty` 的计算，感谢报告 @rsmb7z
- 修复了每个场所交易货币的 `BacktestEngine` 回测后运行 PnL 性能统计，感谢报告 @rsmb7z

---

# NautilusTrader 1.169.0 Beta

发布于 2023年2月18日 (UTC)。

### 破坏性变更

- `NautilusConfig` 对象现在由于新的 msgspec 0.13.0 为_伪不可变_
- 将 `OrderFactory.bracket` 参数 `post_only_entry` 重命名为 `entry_post_only`（与其他参数保持一致）
- 将 `OrderFactory.bracket` 参数 `post_only_tp` 重命名为 `tp_post_only`（与其他参数保持一致）
- 将 `build_time_bars_with_no_updates` 重命名为 `time_bars_build_with_no_updates`（与新参数保持一致）
- 将 `OrderFactory.set_order_count()` 重命名为 `set_client_order_id_count()`（清晰度）
- 将 `TradingNode.start()` 重命名为 `TradingNode.run()`

### 增强功能

- 对 Binance 适配器进行了全面改进和优化，感谢 @poshcoe
- 添加了带有 `use_agg_trade_ticks` 的 Binance 聚合交易功能，感谢 @poshcoe
- 添加了用于 K 线时间戳的 `time_bars_timestamp_on_close` 配置选项（默认 `True`）
- 添加了 `OrderFactory.generate_client_order_id()`（调用内部生成器）
- 添加了 `OrderFactory.generate_order_list_id()`（调用内部生成器）
- 添加了 `OrderFactory.create_list(...)` 作为创建订单列表的更简单方法
- 为 `OrderList` 添加了 `__len__` 实现（返回订单长度）
- 使用 Rust MPSC 通道和单独线程实现了优化的日志记录器
- 公开并改进了 `MatchingEngine` 公共 API 以实现自定义功能
- 公开了 `TradingNode.run_async()` 以便从异步上下文更容易运行
- 公开了 `TradingNode.stop_async()` 以便从异步上下文更容易停止

### 修复

- 修复了 `SimulationModule` 的注册（并优化了 `Actor` 基础注册）
- 修复了之前模拟和转换订单的加载（处理转换 `OrderInitialized` 事件）
- 修复了匹配和风险引擎中 `MARKET_TO_LIMIT` 订单的处理，感谢报告 @martinsaip

---

# NautilusTrader 1.168.0 Beta

发布于 2023年1月29日 (UTC)。

### 破坏性变更

- 移除了 `Cache.clear_cache()`（与 `.reset()` 方法冗余）

### 增强功能

- 为通用'用户/自定义'对象（作为字节）添加了 `Cache` `.add(...)` 和 `.get(...)`
- 为通用缓存对象（作为字节）添加了 `CacheDatabase` `.add(...)` 和 `.load()`
- 为通用 Redis 持久化字节对象（作为字节）添加了 `RedisCacheDatabase` `.add(...)` 和 `.load()`
- 添加了 `Cache.actor_ids()`
- 添加了 `Actor` 缓存状态保存和加载功能
- 改进了未被重写时调用操作处理器的日志记录

### 修复

- 修复了加载和保存参与者和策略状态的配置

---

# NautilusTrader 1.167.0 Beta

发布于 2023年1月28日 (UTC)。

### 破坏性变更

- 将 `OrderBookData.update_id` 重命名为 `sequence`
- 将 `BookOrder.id` 重命名为 `order_id`

### 增强功能

- 引入了基于 Rust PyO3 的 `ParquetReader` 和 `ParquetWriter`，感谢 @twitu
- 添加了 `msgbus.is_subscribed`（检查主题和处理器是否已订阅）
- 简化了消息类型模型并引入了 CQRS 风格的实时消息架构

### 修复

- 修复了 Binance 数据客户端订单簿启动缓冲区处理
- 修复了 `NautilusKernel` 对回测的事件循环冗余初始化，感谢 @limx0
- 修复了 `BacktestNode` 处置序列
- 修复了快速入门文档和笔记本

---

# NautilusTrader 1.166.0 Beta

发布于 2023年1月17日 (UTC)。

### 破坏性变更

- `Position.unrealized_pnl` 现在为 `None` 直到生成任何已实现 PnL（减少歧义）

### 增强功能

- 添加了工具状态更新订阅处理器，感谢 @limx0
- 改进了 InteractiveBrokers `DataClient`，感谢 @rsmb7z
- 改进了实时客户端的异步任务处理
- 对 Betfair 适配器的各种改进，感谢 @limx0

### 修复

- 修复了净额 `Position` `realized_pnl` 和 `realized_return` 字段，它们错误地是累积的
- 修复了净额 `Position` 翻转逻辑（现在正确'重置'仓位）
- Betfair 适配器的各种修复，感谢 @limx0
- InteractiveBrokers 集成文档修复

---

# NautilusTrader 1.165.0 Beta

发布于 2023年1月14日 (UTC)。

为了明确性和避免 C 命名冲突，一些枚举变量名称已更改。

### 破坏性变更

- 将 `AggressorSide.NONE` 重命名为 `NO_AGGRESSOR`
- 将 `AggressorSide.BUY` 重命名为 `BUYER`
- 将 `AggressorSide.SELL` 重命名为 `SELLER`
- 将 `AssetClass.CRYPTO` 重命名为 `CRYPTOCURRENCY`
- 将 `LiquiditySide.NONE` 重命名为 `NO_LIQUIDITY_SIDE`
- 将 `OMSType` 重命名为 `OmsType`
- 将 `OmsType.NONE` 重命名为 `UNSPECIFIED`
- 将 `OrderSide.NONE` 重命名为 `NO_ORDER_SIDE`
- 将 `PositionSide.NONE` 重命名为 `NO_POSITION_SIDE`
- 将 `TrailingOffsetType.NONE` 重命名为 `NO_TRAILING_OFFSET`
- 移除了 `TrailingOffsetType.DEFAULT`
- 将 `TriggerType.NONE` 重命名为 `NO_TRIGGER`
- 将 `TriggerType.LAST` 重命名为 `LAST_TRADE`
- 将 `TriggerType.MARK` 重命名为 `MARK_PRICE`
- 将 `TriggerType.INDEX` 重命名为 `INDEX_PRICE`
- 将 `ComponentState.INITIALIZED` 重命名为 `READY`
- 将 `OrderFactory.bracket(post_only)` 重命名为 `post_only_entry`
- 将 `manage_gtd_expiry` 移动到 `Strategy.submit_order(...)` 和 `Strategy.submit_order_list(...)`

### 增强功能

- 添加了 `BarSpecification.timedelta` 属性，感谢 @rsmb7z
- 添加了 `DataEngineConfig.build_time_bars_with_no_updates` 配置选项
- 添加了 `OrderFactory.bracket(post_only_tp)` 参数
- 添加了 `OrderListIdGenerator` 并与 `OrderFactory` 集成
- 添加了 `Cache.add_order_list(...)`
- 添加了 `Cache.order_list(...)`
- 添加了 `Cache.order_lists(...)`
- 添加了 `Cache.order_list_exists(...)`
- 添加了 `Cache.order_list_ids(...)`
- 改进了工厂生成 `OrderListId` 以确保唯一性
- 为回测添加了拍卖匹配，感谢 @limx0
- 为 `BarSpecification` 添加了 `.timedelta` 属性，感谢 @rsmb7z
- 对 Betfair 适配器的众多改进，感谢 @limx0
- 改进了 Interactive Brokers 数据订阅，感谢 @rsmb7z
- 添加了 `DataEngineConfig.validate_data_sequence`（默认 `False`，目前仅适用于 `Bar` 数据），感谢 @rsmb7z

### 修复

- 为 Binance 现货权限添加了 `TRD_GRP_*` 枚举变量
- 修复了 `PARTIALLY_FILLED` -> `EXPIRED` 订单状态转换，感谢 @bb01100100

---

# NautilusTrader 1.164.0 Beta

发布于 2022年12月23日 (UTC)。

### 破坏性变更
无

### 增强功能

- 添加了管理的 GTD 订单到期（实验功能，配置可能会更改）
- 添加了 Rust `ParquetReader` 和 `ParquetWriter`（仅适用于 `QuoteTick` 和 `TradeTick`）

### 修复

- 修复了 `OrderFactory.bracket(..)` 的 `MARKET_IF_TOUCHED` 订单
- 修复了实时交易的 `OrderEmulator` 触发事件处理
- 修复了 `OrderEmulator` 向具有 GTD 时间有效性的市价订单的转换
- 修复了 `OrderUpdated` 事件的序列化
- 修复了新 `msgspec` 的类型和边缘情况，感谢 @limx0
- 修复了缺失数据的数据整理器处理，感谢 @rsmb7z

---

# NautilusTrader 1.163.0 Beta

发布于 2022年12月17日 (UTC)。

### 破坏性变更
无

### 增强功能
无

### 修复

- 修复了 `MARKET_IF_TOUCHED` 和 `LIMIT_IF_TOUCHED` 触发和修改行为
- 修复了 `MatchingEngine` 对止损订单类型的更新
- 修复了被动或立即触发与被动或立即成交行为的组合
- 修复了从 Rust 传递字符串指针的内存泄漏，感谢 @twitu

---

# NautilusTrader 1.162.0 Beta

发布于 2022年12月12日 (UTC)。

### 破坏性变更

- `OrderFactory` 括号订单方法合并为 `.bracket(...)`

### 增强功能

- 扩展了 `OrderFactory` 以提供更多括号订单类型
- 简化了 GitHub CI 并移除了 `nox` 依赖

### 修复

- 修复了买方的 `OrderBook` 排序，感谢 @gaugau3000
- 修复了 `MARKET_TO_LIMIT` 订单初始成交行为
- 修复了 `BollingerBands` 指标中轨计算，感谢 zhp (Discord)

---

# NautilusTrader 1.161.0 Beta

发布于 2022年12月10日 (UTC)。

此版本添加了对 Python 3.11 的支持。

### 破坏性变更

- 将 `OrderFactory.bracket_market` 重命名为 `OrderFactory.bracket_market_entry`
- 将 `OrderFactory.bracket_limit` 重命名为 `OrderFactory.bracket_limit_entry`
- 重命名了 `OrderFactory` 括号订单 `price` 和 `trigger_price` 参数

### 增强功能

- 将配置对象合并到 `msgspec` 以提供更好的性能和正确性
- 添加了 `OrderFactory.bracket_stop_limit_entry_stop_limit_tp(...)`
- 对 Interactive Brokers 适配器进行了大量改进，感谢 @limx0 和 @rsmb7z
- 移除了对 `pydantic` 的依赖

### 修复

- 修复了 `STOP_MARKET` 订单在立即触发时以市价成交的行为
- 修复了 `STOP_LIMIT` 订单在立即触发且可市价成交时以市价成交的行为
- 修复了 `STOP_LIMIT` 订单在处理触发且可市价成交时以市价成交的行为
- 修复了 `LIMIT_IF_TOUCHED` 订单在立即触发且可市价成交时以市价成交的行为
- 修复了 Binance K 线（kline）请求的开始和停止时间单位，感谢 @Tzumx
- `RiskEngineConfig.bypass` 设置为 `True` 现在将正确绕过节流器，感谢 @DownBadCapital
- 修复了模拟订单的更新
- 对 Interactive Brokers 适配器进行了大量修复，感谢 @limx0 和 @rsmb7z

---

# NautilusTrader 1.160.0 Beta

发布于 2022年11月28日 (UTC)。

### 破坏性变更

- 从生成的 ID 中移除了时间部分（影响 `ClientOrderId` 和 `PositionOrderId`）
- 将 `orderbook.data.Order` 重命名为 `orderbook.data.BookOrder`（减少冲突/困惑）
- 将 `Instrument.get_cost_currency(...)` 重命名为 `Instrument.get_settlement_currency(...)`（更准确的术语）

### 增强功能

- 为 `OrderEmulator` 添加了模拟条件订单功能
- 将 `test_kit` 模块移动到主包以支持下游项目/包测试

### 修复

- 修复了仓位事件排序：现在在重新开启已关闭仓位时生成 `PositionOpened`
- 修复了立即可市价成交作为接受者时 `LIMIT` 订单成交特征
- 修复了作为制造者在报价穿过时被动成交的 `LIMIT` 订单成交特征
- 修复了仍在进行中时取消 OTO 条件订单
- 修复了卖出现金资产（现货货币对）时 `RiskEngine` 名义检查
- 修复了持久化流写入器的已关闭文件刷新错误

---

# NautilusTrader 1.159.0 Beta

发布于 2022年11月18日 (UTC)。

### 破坏性变更

- 移除了 FTX 集成
- 将 `SubmitOrderList.list` 重命名为 `SubmitOrderList.order_list`
- 对 K 线聚合进行了轻微调整（不会使用最后收盘价作为开盘价）

### 增强功能

- 为 Binance Futures 实现了 `TRAILING_STOP_MARKET` 订单（测试版）
- 添加了带有匹配引擎实现的 `OUO` One-Updates-Other `ContingencyType`
- 为汇率计算添加了 K 线价格回退，感谢 @ghill2

### 修复

- 修复了 Python 异常时 Rust 后台结构的释放导致段错误
- 修复了典型间隔外的 K 线规格的 K 线聚合开始时间（60-SECOND 而不是 1-MINUTE 等）
- 修复了具有相同时间戳数据的时间事件的回测引擎主循环排序
- 修复了无数量时 `ModifyOrder` 消息的 `str` 和 `repr`
- 修复了 OCO 条件订单，它们在回测中实际上被实现为 OUO
- 修复了 Interactive Brokers 集成的各种错误，感谢 @limx0 和 @rsmb7z
- 修复了 pyarrow 版本解析，感谢 @ghill2
- 修复了从 InstrumentId 返回场所，感谢 @rsmb7z

---

# NautilusTrader 1.158.0 Beta

发布于 2022年11月3日 (UTC)。

### 破坏性变更

- 添加了 `LiveExecEngineConfig.reconciliation` 布尔标志以控制对账是否激活
- 移除了 `LiveExecEngineConfig.reconciliation_auto`（不清楚的命名和概念）
- 所有 Redis 键都更改为小写约定（要么迁移要么刷新您的 Redis）
- 移除了 `BidAskMinMax` 指标（减少总包大小）
- 移除了 `HilbertPeriod` 指标（减少总包大小）
- 移除了 `HilbertSignalNoiseRatio` 指标（减少总包大小）
- 移除了 `HilbertTransform` 指标（减少总包大小）

### 增强功能

- 改进了回测时钟的准确性（所有时钟现在将匹配生成的 `TimeEvent`）
- 改进了对 `reduce_only` 订单的风险引擎检查
- 添加了 `Actor.request_instruments(...)` 方法
- 添加了 `Order.would_reduce_only(...)` 方法
- 扩展了 `DataClient` 和 `Actor` 的工具请求/响应处理

### 修复

- 修复了 Rust 后台结构的内存管理（现在被正确释放）

---

# NautilusTrader 1.157.0 Beta

发布于 2022年10月24日 (UTC)。

### 破坏性变更
无

### 增强功能

- 添加了所有订单类型（除了 `MARKET` 和 `MARKET_TO_LIMIT`）的实验性本地订单模拟，参见文档
- 为 `HttpClient` 基类添加了 `min_latency`、`max_latency` 和 `avg_latency`

### 修复

- 修复了冰山订单的 Binance Spot `display_qty`，感谢 @JackMa
- 修复了 Binance HTTP 客户端错误日志记录

---

# NautilusTrader 1.156.0 Beta

发布于 2022年10月19日 (UTC)。

这将是支持 Python 3.8 的最后一个版本。

### 破坏性变更

- 添加了 `OrderSide.NONE` 枚举变量
- 添加了 `PositionSide.NO_POSITION_SIDE` 枚举变量
- 更改了 `TriggerType` 枚举变量的顺序
- 将 `AggressorSide.UNKNOWN` 重命名为 `AggressorSide.NONE`（与其他枚举保持一致）
- 将 `Order.type` 重命名为 `Order.order_type`（减少歧义并与 Rust 结构字段对齐）
- 将 `OrderInitialized.type` 重命名为 `OrderInitialized.order_type`（减少歧义）
- 将 `Bar.type` 重命名为 `Bar.bar_type`（减少歧义并与 Rust 结构字段对齐）
- 移除了冗余的 `check_position_exists` 标志
- 移除了 `hyperopt`，因为被认为无人维护且有更好的选择
- `QuoteTick` 的现有序列化数据现在**无效**（为了正确性而更改模式）
- `OrderInitialized` 的现有目录数据现在**无效**（为了模拟而更改模式）

### 增强功能

- 添加了可配置的自动进行中订单状态检查
- 为众多缓存订单方法添加了订单 `side` 过滤器
- 为众多缓存仓位方法添加了仓位 `side` 过滤器
- 为 `cancel_all_orders` 策略方法添加了可选的 `order_side`
- 为 `close_all_positions` 策略方法添加了可选的 `position_side`
- 添加了对 Binance Spot 秒级 K 线的支持
- 添加了 `RelativeVolatilityIndex` 指标，感谢 @graceyangfan
- 从 `SimulatedExchange` 中提取了 `OrderMatchingEngine` 并进行了优化
- 从 `OrderMatchingEngine` 中提取了 `MatchingCore`
- 改进了 HTTP 错误处理和客户端日志记录（消息现在包含原因）

### 修复

- 修复了来自原始值的 `QuoteTick` 价格和大小精度验证
- 修复了 IB 适配器对十进制精度的数据解析
- 修复了 HTTP 错误处理和响应协程的释放，感谢 @JackMa
- 修复了当任何基础货币 == 佣金货币时的 `Position` 计算和核算，感谢 @JackMa

---

# NautilusTrader 1.155.0 Beta

发布于 2022年9月15日 (UTC)。

这是一个早期版本，用于解决 FTX 适配器中的一些解析错误。

### 破坏性变更
无

### 增强功能
无

### 修复

- 修复了 FTX 期货的解析错误
- 修复了 FTX `Bar` 的解析错误

---

# NautilusTrader 1.154.0 Beta

发布于 2022年9月14日 (UTC)。

### 破坏性变更

- 将 `ExecEngineConfig` `allow_cash_positions` 默认值更改为 `True`（更典型的用例）
- 从 `Bar` 移除了 `check` 参数（为简单起见总是检查）

### 增强功能

- 为 `SimulatedExchange` 添加了 `MARKET_TO_LIMIT` 订单实现
- 使策略 `order_id_tag` 真正可选且自动递增
- 添加了心理线指标，感谢 @graceyangfan
- 添加了初始的 Rust parquet 集成，感谢 @twitu 和 @ghill2
- 添加了在 `CASH` 账户上设置杠杆的验证
- 为可用性去 Cython 化实时数据和执行客户端基类

### 修复

- 修复了限价订单 `IOC` 和 `FOK` 行为，感谢 @limx0 发现
- 修复了 FTX `CryptoFuture` 工具解析，感谢 @limx0
- 修复了数据目录示例笔记本中的缺失导入，感谢 @gaugau3000
- 修复了订单更新行为，受影响的订单：
  - `LIMIT_IF_TOUCHED`
  - `MARKET_IF_TOUCHED`
  - `MARKET_TO_LIMIT`
  - `STOP_LIMIT`

---

# NautilusTrader 1.153.0 Beta

发布于 2022年9月6日 (UTC)。

### 破坏性变更
无

### 增强功能

- 为 FTX 适配器添加了触发订单
- 改进了 `BinanceBar` 以处理巨大的报价量
- 改进了 Binance 和 FTX 适配器工具解析的稳健性
- 改进了 Binance 和 FTX 适配器 WebSocket 消息处理的稳健性
- 为 FTX 适配器添加了 `override_usd` 选项
- 为 Binance 和 FTX 工具提供商添加了 `log_warnings` 配置选项
- 为 Binance 现货权限添加了 `TRD_GRP_005` 枚举变量

### 修复

- 修复了 K 线聚合器部分 K 线处理
- 修复了 Rust 中的 `CurrencyType` 变量
- 修复了目录解析方法中缺失的 `encoding`，感谢 @limx0 和 @aviatorBeijing

---

# NautilusTrader 1.152.0 Beta

发布于 2022年9月1日 (UTC)。

### 破坏性变更

- 将 `offset_type` 重命名为 `trailing_offset_type`
- 将 `is_frozen_account` 重命名为 `frozen_account`
- 从配置 API 移除了 `bar_execution`（当前使用 K 线时隐式开启）

### 增强功能

- 为 `SimulatedExchange` 添加了 `TRAILING_STOP_MARKET` 订单实现
- 为 `SimulatedExchange` 添加了 `TRAILING_STOP_LIMIT` 订单实现
- 为 `BacktestVenueConfig` 添加了所有模拟交易所选项

### 修复

- 修复了订阅增量时订单簿的创建和缓存，感谢 @limx0
- 修复了交易节点实时时钟中 `LoopTimer` 的使用，感谢 @sidnvy
- 修复了 IB 适配器的订单取消，感谢 @limx0

---

# NautilusTrader 1.151.0 Beta

发布于 2022年8月22日 (UTC)。

### 破坏性变更
无

### 增强功能

- 添加了 `on_historical_data` 方法和功能连线
- 为 Binance Futures 添加了"无节流"0ms 订单簿更新
- 改进了重连期间 `WebSocketClient` 基类的稳健性

### 修复

- 修复了 Rust Cargo 文件的 sdist 包含
- 修复了 `LatencyModel` 整数溢出，感谢 @limx0
- 修复了 Binance Futures `FUNDING_FEE` 更新的解析
- 修复了 Python 3.10+ 的 `asyncio.tasks.gather`

---

# NautilusTrader 1.150.0 Beta

发布于 2022年8月15日 (UTC)。

### 破坏性变更

- `BacktestEngine` 现在要求在添加工具之前先添加场所
- `BacktestEngine` 现在要求在添加数据之前先添加工具
- 将 `Ladder.reverse` 重命名为 `Ladder.is_reversed`
- 投资组合表现现在将佣金显示为负值

### 增强功能

- 添加了工具与场所的初始回测配置验证
- 添加了初始沙盒执行客户端
- 为 `BacktestVenueConfig` 添加了杠杆选项，感谢 @miller-moore
- 允许 `Trader` 在未加载策略的情况下运行
- 集成了核心 Rust 时钟和定时器
- 去 Cython 化 `InstrumentProvider` 基类

### 修复

- 修复了单货币和多货币账户的双重佣金计算 #657

---

# NautilusTrader 1.149.0 Beta

发布于 2022年6月27日 (UTC)。

### 破坏性变更

- `ParquetDataCatalog` 的 `Instrument.info` 模式更改

### 增强功能

- 添加了 `DirectionalMovementIndicator` 指标，感谢 @graceyangfan
- 添加了 `KlingerVolumeOscillator` 指标，感谢 @graceyangfan
- 为 IB 配置添加了 `clientId` 和 `start_gateway`，感谢 @niks199

### 修复

- 修复了 macOS ARM64 构建
- 修复了 Binance 测试网 URL
- 修复了 IB 合约 ID 字典，感谢 @niks199
- 修复了 IB `InstrumentProvider` #685，感谢 @limx0
- 修复了 IB 订单簿快照 L1 值断言 #712，感谢 @limx0

---

# NautilusTrader 1.148.0 Beta

发布于 2022年6月30日 (UTC)。

### 破坏性变更
无

### 增强功能

- 将核心 K 线对象移植到 Rust，感谢 @ghill2
- 将核心 `unix_nanos_to_iso8601` 性能提高了 30%，感谢 @ghill2
- 为 `ParquetDataCatalog` 添加了 `DataCatalog` 接口，感谢 @jordanparker6
- 添加了 `AroonOscillator` 指标，感谢 @graceyangfan
- 添加了 `ArcherMovingAveragesTrends` 指标，感谢 @graceyangfan
- 添加了 `DoubleExponentialMovingAverage` 指标，感谢 @graceyangfan
- 添加了 `WilderMovingAverage` 指标，感谢 @graceyangfan
- 添加了 `ChandeMomentumOscillator` 指标，感谢 @graceyangfan
- 添加了 `VerticalHorizontalFilter` 指标，感谢 @graceyangfan
- 添加了 `Bias` 指标，感谢 @graceyangfan

### 修复
无

---

# NautilusTrader 1.147.1 Beta

发布于 2022年6月6日 (UTC)。

### 破坏性变更
无

### 增强功能
无

### 修复

- 修复了错误的回测日志时间戳（使用的是实际时间）
- 修复了按照 RFC3339 标准的纳秒 zulu 时间戳格式

---

# NautilusTrader 1.147.0 Beta

发布于 2022年6月4日 (UTC)。

### 破坏性变更
无

### 增强功能

- 改进了无效状态触发的错误处理
- 改进了组件状态转换行为和日志记录
- 改进了 `TradingNode` 处置流程
- 实现了核心单调时钟
- 在 Rust 中实现了日志记录
- 添加了 `CommodityChannelIndex` 指标，感谢 @graceyangfan

### 修复
无

---

# NautilusTrader 1.146.0 Beta

发布于 2022年5月22日 (UTC)。

### 破坏性变更

- `AccountId` 构造函数现在接受单个值字符串
- 移除了冗余的 `UUIDFactory` 和所有相关的后台字段和调用
- 移除了 `ClientOrderLinkId`（未使用）

### 增强功能

- 对 Rust 核心进行了优化和改进

### 修复

- 修复了错误地应用于 `MARGIN` 账户的交易前名义风险检查
- 修复了 `PositionStatusReport` 中的 `net_qty`，感谢 @sidnvy
- 修复了 `LinearRegression` 指标，感谢 @graceyangfan

---

# NautilusTrader 1.145.0 Beta

发布于 2022年5月15日 (UTC)。

这是一个早期版本，由于 `1.144.0` 的 sdist 构建错误。错误是由于 `nautilus_core` Rust 源代码未包含在 sdist 包中。

### 破坏性变更

- 所有原始订单构造函数现在接受 `expire_time_ns` int64 而不是 datetime
- 由于 `expire_time_ns` 选项处理导致的所有订单序列化
- `PortfolioAnalyzer` 从 `Trader` 移动到 `Portfolio`

### 增强功能

- `PortfolioAnalyzer` 现在可通过 `self.portfolio.analyzer` 为策略使用

### 修复
无

---

# NautilusTrader 1.144.0 Beta

发布于 2022年5月10日 (UTC)。

### 破坏性变更

- 移除了 `BacktestEngine.add_ticks()` 因为与 `.add_data()` 冗余
- 移除了 `BacktestEngine.add_bars()` 因为与 `.add_data()` 冗余
- 移除了 `BacktestEngine.add_generic_data()` 因为与 `.add_data()` 冗余
- 移除了 `BacktestEngine.add_order_book_data()` 因为与 `.add_data()` 冗余
- 将 `Position.from_order` 重命名为 `Position.opening_order_id`
- 将 `StreamingPersistence` 重命名为 `StreamingFeatherWriter`
- 将 `PersistenceConfig` 重命名为 `StreamingConfig`
- 将 `PersistenceConfig.flush_interval` 重命名为 `flush_interval_ms`

### 增强功能

- 为通用动态信号数据添加了 `Actor.publish_signal`
- 添加了 `WEEK` 和 `MONTH` K 线聚合选项
- 添加了 `Position.closing_order_id` 属性
- 为 `Strategy.submit_order` 添加了 `tags` 参数
- 为 `Strategy.submit_order` 添加了可选的 `check_positon_exists` 标志
- 消除了所有 `unsafe` Rust 和 C 空终止字节字符串的使用
- `bypass_logging` 配置选项现在也将绕过 `BacktestEngine` 日志记录器

### 修复

- 修复了 `IOC` 和 `FOK` 时间有效性指令的行为
- 修复了 Binance K 线分辨率解析

---

# NautilusTrader 1.143.0 Beta

发布于 2022年4月21日 (UTC)。

### 破坏性变更
无

### 增强功能
无

### 修复

- 修复了无基础货币时 `CashAccount.calculate_balance_locked` 的段错误
- 各种 FeatherWriter 修复

---

# NautilusTrader 1.142.0 Beta

发布于 2022年4月17日 (UTC)。

### 破坏性变更

- `BacktestNode` 现在在初始化时需要配置
- 从 `BacktestNode.run()` 方法移除了 `run_configs` 参数
- 移除了 `return_engine` 标志
- 将 `TradingStrategy` 重命名为 `Strategy`
- 将 `TradingStrategyConfig` 重命名为 `StrategyConfig`
- 配置对象导入路径的更改
- 从 `Position` 移除了冗余的 `realized_points` 概念

### 增强功能

- 添加了 `BacktestNode.get_engines()` 方法
- 添加了 `BacktestNode.get_engine(run_config_id)` 方法
- 添加了 `Actor.request_instrument()` 方法（也适用于 `Strategy`）
- 添加了 `Cache.snapshot_position()` 方法
- 所有配置对象现在可以直接从 `nautilus_trader.config` 导入
- 执行引擎现在拍摄已关闭净额仓位的快照
- 基于总仓位和快照的性能统计
- 添加了 Binance Spot/Margin 外部订单处理
- 添加了对毫秒 K 线聚合的支持
- 为引擎添加了可配置的 `debug` 模式（带有额外的调试日志记录）
- 改进了带有可配置周期的年化投资组合统计

### 修复
无

---

# NautilusTrader 1.141.0 Beta

发布于 2022年4月4日 (UTC)。

### 破坏性变更

- 将 `BacktestNode.run_sync()` 重命名为 `BacktestNode.run()`
- 将 `flatten_position()` 重命名为 `close_position()`
- 将 `flatten_all_positions()` 重命名为 `close_all_positions()`
- 将 `Order.flatten_side()` 重命名为 `Order.closing_side()`
- 将 `TradingNodeConfig` `check_residuals_delay` 重命名为 `timeout_post_stop`
- `SimulatedExchange` 现在将在 `DataEngine` 之前"接收"市场数据（注意这不影响任何测试）
- 收紧了 `DataType` 类型必须是 `Data` 子类的要求
- `CacheDatabaseConfig.type` 现在默认为 `in-memory`
- `NAUTILUS_CATALOG` 环境变量更改为 `NAUTILUS_PATH`
- `DataCatalog` 根路径现在位于 Nautilus 路径的 `$OLD_PATH/catalog/` 下
- `hiredis` 和 `redis` 现在是 'redis' 的可选额外依赖
- `hyperopt` 现在是 'hyperopt' 的可选额外依赖

### 增强功能

- 在回测和实时系统中统一 `NautilusKernel`
- 通过分组到 `config` 子包改进了配置
- 改进了配置对象和流程
- 对 Binance Spot/Margin 和 Futures 集成进行了大量改进
- 添加了 Docker 镜像构建和 GH 包
- 添加了 `BinanceFuturesMarkPriceUpdate` 类型和数据流
- 为模板添加了通用 `subscribe` 和 `unsubscribe`
- 添加了 Binance Futures COIN_M 测试网
- 改进了各种错误消息的清晰度

### 修复

- 修复了 `DataCatalog` 中的多个工具 (#554), (#560)，@limx0
- 修复了从 `DataCatalog` 流式传输的时间戳排序 (#561)，@limx0
- 修复了 `CSVReader` (#563)，@limx0
- 修复了 Binance WebSocket 流的慢订阅者
- 修复了回测的 `base_currency` 配置
- 修复了可导入策略配置（之前未返回正确的类）
- 修复了 `fully_qualified_name()` 格式

---

# NautilusTrader 1.140.0 Beta

发布于 2022年3月13日 (UTC)。

这是一个补丁版本，修复了 pillow < 9.0.1 中的中等严重性安全漏洞：

    如果在 Linux 或 macOS 上临时目录的路径包含空格，
    这会破坏 im.show()（及相关操作）后临时图像文件的删除，
    并可能删除不相关的文件。这个问题自 PIL 以来一直存在。

此版本升级到 pillow 9.0.1。

注意小版本号的递增是错误的。

---

# NautilusTrader 1.139.0 Beta

发布于 2022年3月11日 (UTC)。

### 破坏性变更

- 将 `CurrencySpot` 重命名为 `CurrencyPair`
- 将 `PerformanceAnalyzer` 重命名为 `PortfolioAnalyzer`
- 将 `BacktestDataConfig.data_cls_path` 重命名为 `data_cls`
- 将 `BinanceTicker` 重命名为 `BinanceSpotTicker`
- 将 `BinanceSpotExecutionClient` 重命名为 `BinanceExecutionClient`

### 增强功能

- 添加了初始 **(测试版)** Binance Futures 适配器实现
- 添加了初始 **(测试版)** Interactive Brokers 适配器实现
- 添加了自定义投资组合统计
- 添加了 `CryptoFuture` 工具
- 添加了 `OrderType.MARKET_TO_LIMIT`
- 添加了 `OrderType.MARKET_IF_TOUCHED`
- 添加了 `OrderType.LIMIT_IF_TOUCHED`
- 添加了 `MarketToLimitOrder` 订单类型
- 添加了 `MarketIfTouchedOrder` 订单类型
- 添加了 `LimitIfTouchedOrder` 订单类型
- 添加了 `Order.has_price` 属性（便利）
- 添加了 `Order.has_trigger_price` 属性（便利）
- 为 `LoggerAdapter.exception()` 添加了 `msg` 参数
- 添加了 WebSocket `log_send` 和 `log_recv` 配置选项
- 添加了 WebSocket `auto_ping_interval`（秒）配置选项
- 用 `msgspec` 替换了 `msgpack`（更快的替代品 <https://github.com/jcrist/msgspec）>
- 通过提供有用的上下文改进了异常消息
- 改进了 `BacktestDataConfig` API：现在接受 `Data` 类型_或_完全限定路径字符串

### 修复

- 修复了 FTX 执行 WebSocket 'ping 策略'
- 修复了非确定性配置 dask 标记化

---

# NautilusTrader 1.138.0 Beta

发布于 2022年2月15日 (UTC)。

**此版本包含大量方法、参数和属性名称更改**

为了与其他协议保持一致和标准化，`ExecutionId` 类型已重命名为 `TradeId`，因为它们表达相同的概念，采用更标准化的术语。为了强制正确性和安全性，此类型现在用于 `TradeTick.trade_id`。

### 破坏性变更

- 将 `working` 订单重命名为 `open` 订单，包括所有相关方法和参数
- 将 `completed` 订单重命名为 `closed` 订单，包括所有相关方法和参数
- 移除了 `active` 订单概念（经常与 `open` 混淆）
- 将 `trigger` 重命名为 `trigger_price`
- 将 `StopMarketOrder.price` 重命名为 `StopMarketOrder.trigger_price`
- 将所有与 `StopMarketOrders` `price` 相关的参数重命名为 `trigger_price`
- 将 `ExecutionId` 重命名为 `TradeId`
- 将 `execution_id` 重命名为 `trade_id`
- 将 `Order.trade_id` 重命名为 `Order.last_trade_id`（为了清晰）
- 将其他变体和对"执行 ID"的引用重命名为"交易 ID"
- 将 `contingency` 重命名为 `contingency_type`

### 增强功能

- 引入了 `TradeId` 类型以强制 `trade_id` 类型
- 改进了非杠杆现金资产仓位的处理，包括加密货币和法币现货货币工具
- 添加了 `ExecEngineConfig` 配置选项 `allow_cash_positions`（默认 `False`）
- 添加了 `TrailingOffsetType` 枚举
- 添加了 `TrailingStopMarketOrder`
- 添加了 `TrailingStopLimitOrder`
- 添加了追踪订单工厂方法
- 为止损订单添加了 `trigger_type` 参数
- 添加了 `TriggerType` 枚举
- 对订单基类和实现类进行了大规模重构
- 大修执行报告
- 大修执行状态对账

### 修复

- 修复了 WebSocket 基础重连处理

---

# NautilusTrader 1.137.1 Beta

发布于 2022年1月15日 (UTC)。

这是一个补丁版本，修复了 `pillow < 9.0.0` 中的中等到高严重性安全漏洞：

- PIL.ImageMath.eval 允许评估任意表达式，例如使用 Python exec 方法的表达式
- path.c 中的 path_getbbox 在 ImagePath.Path 初始化期间存在缓冲区过度读取
- path.c 中的 path_getbbox 不正确地初始化 ImagePath.Path

此版本升级到 `pillow 9.0.0`。

---

# NautilusTrader 1.137.0 Beta

发布于 2022年1月12日 (UTC)。

### 破坏性变更

- 从 `AccountBalance` 移除了冗余的 `currency` 参数
- 将 `local_symbol` 重命名为 `native_symbol`
- 移除了 `VenueType` 枚举和 `venue_type` 参数，采用 `routing` 布尔标志
- 从执行客户端工厂和构造函数移除了 `account_id` 参数
- 更改了场所生成的 ID（订单、执行、仓位），现在以场所 ID 开头

### 增强功能

- 添加了用于测试的 FTX 集成
- 添加了 FTX US 配置选项
- 添加了 Binance US 配置选项
- 添加了 `MarginBalance` 对象以协助保证金账户功能

### 修复

- 修复了包含连字符 `-` 的符号的 `BarType` 解析
- 修复了 `BinanceSpotTicker` `__repr__`（逗号后缺少空格）
- 修复了历史 `TradeTick` 的 `DataEngine` 请求
- 修复了 `DataEngine` `_handle_data_response` 的 `data` 类型为 `object`

---

# NautilusTrader 1.136.0 Beta

发布于 2021年12月29日。

### 破坏性变更

- 更改了 `subscribe_data(...)` 方法（`client_id` 现在可选）
- 更改了 `unsubscribe_data(...)` 方法（`client_id` 现在可选）
- 更改了 `publish_data(...)` 方法（添加了 `data_type`）
- 将 `MessageBus.subscriptions` 方法参数重命名为 `pattern`
- 将 `MessageBus.has_subscribers` 方法参数重命名为 `pattern`
- 移除了 `subscribe_strategy_data(...)` 方法
- 移除了 `unsubscribe_strategy_data(...)` 方法
- 移除了 `publish_strategy_data(...)` 方法
- 将 `CryptoSwap` 重命名为 `CryptoPerpetual`

### 增强功能

- 现在可以实时和回测修改或取消进行中的订单
- 更新 `CancelOrder` 以允许 None `venue_order_id`
- 更新 `ModifyOrder` 以允许 None `venue_order_id`
- 更新 `OrderPendingUpdate` 以允许 None `venue_order_id`
- 更新 `OrderPendingCancel` 以允许 None `venue_order_id`
- 更新 `OrderCancelRejected` 以允许 None `venue_order_id`
- 更新 `OrderModifyRejected` 以允许 None `venue_order_id`
- 为改进的消息总线处理添加了 `DataType.topic` 字符串

### 修复

- 为 `DataType`、`BarSpecification` 和 `BarType` 实现了比较
- 修复了带有 `random_seed` 的 `QuoteTickDataWrangler.process_bar_data`

---

# NautilusTrader 1.135.0 Beta

发布于 2021年12月13日。

### 破坏性变更

- 将 `match_id` 重命名为 `trade_id`

### 增强功能

- 为 `DataCatalog` 添加了 bars 方法
- 改进了 Binance 历史 K 线数据解析
- 添加了 `CancelAllOrders` 命令
- 为 Binance 集成添加了批量取消功能
- 为 Betfair 集成添加了批量取消功能

### 修复

- 修复了 ARM 架构日志记录中 `cpu_freq` 调用的处理
- 修复了 K 线数据的市价订单成交边缘情况
- 修复了回测中 `GenericData` 的处理

---

# NautilusTrader 1.134.0 Beta

发布于 2021年11月22日。

### 破坏性变更

- 将 `hidden` 订单选项更改为 `display_qty` 以支持冰山订单
- 将 `Trader.component_ids()` 重命名为 `Trader.actor_ids()`
- 将 `Trader.component_states()` 重命名为 `Trader.actor_states()`
- 将 `Trader.add_component()` 重命名为 `Trader.add_actor()`
- 将 `Trader.add_components()` 重命名为 `Trader.add_actors()`
- 将 `Trader.clear_components()` 重命名为 `Trader.clear_actors()`

### 增强功能

- 添加了 Binance SPOT 集成的初始实现（测试阶段）
- 添加了对显示数量/冰山订单的支持

### 修复

- 修复了回测引擎中 `Actor` 时钟时间推进

---

# NautilusTrader 1.133.0 Beta

发布于 2021年11月8日。

### 破坏性变更
无

### 增强功能

- 为模拟交易所添加了 `LatencyModel`
- 为订单簿添加了 `last_update_id`
- 为订单簿数据添加了 `update_id`
- 订阅订单簿增量时添加了 `depth` 参数
- 添加了 `Clock.timestamp_ms()`
- 添加了 `TestDataProvider` 并整合测试数据
- 为 arrow 添加了 orjson 默认序列化器
- 重新组织了示例策略和启动脚本

### 修复

- 修复了回测中部分成交的逻辑
- 各种 Betfair 集成修复
- 各种 `BacktestNode` 修复

---

# NautilusTrader 1.132.0 Beta

发布于 2021年10月24日。

### 破坏性变更

- `Actor` 构造函数现在接受 `ActorConfig`

### 增强功能

- 添加了 `ActorConfig`
- 添加了 `ImportableActorConfig`
- 添加了 `ActorFactory`
- 为 `BacktestRunConfig` 添加了 `actors`
- 改进了网络基类
- 优化了 `InstrumentProvider`

### 修复

- 修复了 `BacktestNode` 的持久化配置
- 各种 Betfair 集成修复

---

# NautilusTrader 1.131.0 Beta

发布于 2021年10月10日。

### 破坏性变更

- 将 `nanos_to_unix_dt` 重命名为 `unix_nanos_to_dt`（更准确的名称）
- 更改了 `Clock.set_time_alert(...)` 方法签名
- 更改了 `Clock.set_timer(...)` 方法签名
- 从 `TimeEvent` 移除了 `pd.Timestamp`

### 增强功能

- `OrderList` 提交和 OTO、OCO 条件现在可操作
- 添加了 `Cache.orders_for_position(...)` 方法
- 添加了 `Cache.position_for_order(...)` 方法
- 添加了 `SimulatedExchange.get_working_bid_orders(...)` 方法
- 添加了 `SimulatedExchange.get_working_ask_orders(...)` 方法
- 为回测运行添加了可选的 `run_config_id`
- 添加了 `BacktestResult` 对象
- 添加了 `Clock.set_time_alert_ns(...)` 方法
- 添加了 `Clock.set_timer_ns(...)` 方法
- 添加了 `fill_limit_at_price` 模拟交易所选项
- 添加了 `fill_stop_at_price` 模拟交易所选项
- 改进了定时器和时间事件效率

### 修复

- 修复了 `OrderUpdated` 剩余数量计算
- 修复了交易所的条件订单逻辑
- 修复了缓存中仓位订单的索引
- 修复了零大小仓位的翻转逻辑（不是翻转）

---

# NautilusTrader 1.130.0 Beta

发布于 2021年9月26日。

### 破坏性变更

- `BacktestEngine.run` 方法签名更改
- 将 `BookLevel` 重命名为 `BookType`
- 重命名了 `FillModel` 参数

### 增强功能

- 添加了流式回测机制
- 添加了 `quantstats`（移除了 `empyrical`）
- 添加了 `BacktestEngine.run_streaming()`
- 添加了 `BacktestEngine.end_streaming()`
- 添加了 `Portfolio.balances_locked(venue)`
- 改进了 `DataCatalog` 功能
- 改进了 `BacktestEngine` 的日志记录
- 改进了 parquet 序列化和机制

### 修复

- 修复了 `SimulatedExchange` 消息处理
- 修复了 `BacktestEngine` 主循环中的事件排序
- 修复了 `CASH` 账户的锁定余额计算
- 修复了 `reduce-only` 订单的成交动态
- 修复了 `HEDGING` OMS 交易所的 `PositionId` 处理
- 修复了 parquet `Instrument` 序列化
- 修复了带有基础货币的 `CASH` 账户 PnL 计算

---

# NautilusTrader 1.129.0 Beta

发布于 2021年9月12日。

### 破坏性变更

- 移除了 CCXT 适配器 (#428)
- 回测配置更改
- 将 `UpdateOrder` 重命名为 `ModifyOrder`（术语标准化）
- 将 `DeltaType` 重命名为 `BookAction`（术语标准化）

### 增强功能

- 添加了 `BacktestNode`
- 添加了 `BookIntegrityError` 和改进的订单簿完整性检查
- 添加了订单自定义用户标签
- 添加了 `Actor.register_warning_event`（也适用于 `TradingStrategy`）
- 添加了 `Actor.deregister_warning_event`（也适用于 `TradingStrategy`）
- 添加了 `ContingencyType` 枚举（用于 `OrderList` 中的条件订单）
- 所有订单类型现在可以是 `reduce_only` (#437)
- 优化了回测配置选项
- 使用 Rust `fastuuid` Python 绑定改进了 `UUID4` 的效率

### 修复

- 修复了 `int64_t` 纳秒时间戳的 Redis 精度丢失 (#363)
- 修复了提交和成交时 `reduce_only` 订单的行为 (#437)
- 修复了佣金为负时 `CASH` 账户的 PnL 计算 (#436)

---

# NautilusTrader 1.128.0 Beta

发布于 2021年8月30日。

此版本继续专注于核心系统，对组件基类进行了升级和清理。引入了 `active` 订单的概念，即状态可以改变的订单（不是 `completed` 订单）。

### 破坏性变更

- 由于 `pydantic` 升级导致的所有配置
- 节流配置现在接受字符串，例如 "100/00:00:01" 表示每秒 100 次
- 将 `DataProducerFacade` 重命名为 `DataProducer`
- 将 `fill.side` 重命名为 `fill.order_side`（清晰度和标准化）
- 将 `fill.type` 重命名为 `fill.order_type`（清晰度和标准化）

### 增强功能

- 添加了利用 `pydantic` 的可序列化配置类
- 改进了向 `BacktestEngine` 添加 K 线数据
- 添加了 `BacktestEngine.add_bar_objects()`
- 添加了 `BacktestEngine.add_bars_as_ticks()`
- 添加了订单 `active` 概念，带有 `order.is_active` 和缓存方法
- 添加了 `ComponentStateChanged` 事件
- 添加了 `Component.degrade()` 和 `Component.fault()` 命令方法
- 添加了 `Component.on_degrade()` 和 `Component.on_fault()` 处理器方法
- 添加了 `ComponentState.PRE_INITIALIZED`
- 添加了 `ComponentState.DEGRADING`
- 添加了 `ComponentState.DEGRADED`
- 添加了 `ComponentState.FAULTING`
- 添加了 `ComponentState.FAULTED`
- 添加了 `ComponentTrigger.INITIALIZE`
- 添加了 `ComponentTrigger.DEGRADE`
- 添加了 `ComponentTrigger.DEGRADED`
- 添加了 `ComponentTrigger.FAULT`
- 添加了 `ComponentTrigger.FAULTED`
- 连接了 `Ticker` 数据类型

### 修复

- `DataEngine.subscribed_bars()` 现在也报告内部聚合的 K 线

---

# NautilusTrader 1.127.0 Beta

发布于 2021年8月17日。

此版本再次专注于平台的核心领域，包括对会计和投资组合组件的重大改进。`DataEngine` 和 `DataClient` 之间的连线也得到了关注，现在应该表现出正确的订阅机制。

Betfair 适配器已完全重写，提供了各种修复和增强、更高的性能以及完整的异步支持。

还进行了一些进一步的重命名，以继续使平台尽可能与领域中已建立的术语保持一致。

### 破坏性变更

- 将保证金计算方法从 `Instrument` 移动到 `Account`
- 移除了冗余的 `Portfolio.register_account`
- 将 `OrderState` 重命名为 `OrderStatus`
- 将 `Order.state` 重命名为 `Order.status`
- 将 `msgbus.message_bus` 重命名为 `msgbus.bus`

### 增强功能

- Betfair 适配器重写
- 提取了 `accounting` 子包
- 提取了 `portfolio` 子包
- 用 `CashAccount` 和 `MarginAccount` 子类化 `Account`
- 添加了 `AccountsManager`
- 添加了 `AccountFactory`
- 将自定义账户类的注册移动到 `AccountFactory`
- 将计算账户的注册移动到 `AccountFactory`
- 为每个交易策略添加了 OMS 类型注册
- 为自定义账户类添加了 `ExecutionClient.create_account`
- 将 `PortfolioFacade` 从 `Portfolio` 分离

### 修复

- `DataEngine` 中的数据订阅处理
- `Cash` 账户不再生成虚假保证金
- 修复了 `TimeBarAggregator._stored_close_ns` 属性名

---

# NautilusTrader 1.126.1 Beta

发布于 2021年8月3日。

这是一个补丁版本，修复了在订阅客户端不支持的订单簿增量时涉及 `NotImplementedError` 异常处理的错误。此错误影响了 CCXT 订单簿订阅。

### 破坏性变更
无

### 增强功能
无

### 修复

- 修复了 `DataEngine` 订单簿订阅处理

---

# NautilusTrader 1.126.0 Beta

发布于 2021年8月2日。

此版本完成了 `MessageBus` 的初始实现，现在通过发布/订阅模式处理数据，同时添加了点对点和请求/响应消息功能。

从 `TradingStrategy` 抽象出了 `Actor` 基类，允许将自定义组件添加到 `Trader` 中，这些组件不一定是交易策略，为使用自定义功能扩展 NautilusTrader 打开了更多可能性。

为了简化和支持更多惯用的 Python，不再使用空对象模式来处理标识符。这消除了代码库某些部分的"逻辑间接性"层，并允许更简单的代码。

如果订单正在积极等待状态转换，即处于 `SUBMITTED`、`PENDING_UPDATE` 或 `PENDING_CANCEL` 状态，则现在认为订单"进行中"。

现在已建立的惯例是所有基于整数的时间戳都以 UNIX 纳秒表示，因此现在已删除 `_ns` 后缀。为了清晰起见 - 单位可能不明显的时间段/间隔/对象保留了 `_ns` 后缀。

通过将 `timestamp_ns` 和 `ts_recv_ns` 重命名为 `ts_init`，确定了通过重命名统一对象实例化概念的参数命名的机会。沿着同样的思路，事件和数据发生的时间戳都已标准化为 `ts_event`。

### 破坏性变更

- 将 `timestamp_ns` 重命名为 `ts_init`
- 将 `ts_recv_ns` 重命名为 `ts_event`
- 将各种事件时间戳参数重命名为 `ts_event`
- 移除了标识符上的空对象方法

### 增强功能

- 添加了 `Actor` 组件基类
- 添加了 `MessageBus.register()`
- 添加了 `MessageBus.send()`
- 添加了 `MessageBus.request()`
- 添加了 `MessageBus.response()`
- 添加了 `Trader.add_component()`
- 添加了 `Trader.add_components()`
- 添加了 `Trader.add_log_sink()`

### 修复

- 各种 Betfair 适配器补丁和修复
- 某些边缘情况下的 `ExecutionEngine` 仓位翻转逻辑

---

# NautilusTrader 1.125.0 Beta

发布于 2021年7月18日。

此版本引入了内部消息系统的重大重新架构。已实现了通用消息总线，现在通过发布/订阅消息模式处理所有事件。下一个版本将看到所有数据由消息总线处理，有关此增强的更多详细信息请参见相关问题。

另一个值得注意的功能是引入了订单"进行中"概念，这是已提交但尚未被交易场所确认的订单。`Order` 上的几个属性和 `Cache` 上的方法现在存在以支持此功能。

`Throttler` 已重构并进一步优化。还对模型子包进行了广泛的重组，对事件上的标识符进行了标准化，以及大量的"底层"清理和两个错误修复。

### 破坏性变更

- 将 `MessageType` 枚举重命名为 `MessageCategory`
- 将 `fill.order_side` 重命名为 `fill.side`
- 将 `fill.order_type` 重命名为 `fill.type`
- 由于领域重构导致的所有 `Event` 序列化

### 增强功能

- 添加了 `MessageBus` 类
- 为 `Order` 和 `Position` 添加了 `TraderId`
- 为 OrderFilled 添加了 `OrderType`
- 为仓位事件添加了未实现 PnL
- 为 `Order` 和 `Cache` 添加了订单进行中概念
- 改进了 `Throttler` 的效率
- 标准化了事件 `str` 和 `repr`
- 标准化了命令 `str` 和 `repr`
- 标准化了事件和对象上的标识符
- 改进了 `Account` `str` 和 `repr`
- 使用 `orjson` 而不是 `json` 以提高效率
- 移除了冗余的 `BypassCacheDatabase`
- 将 `mypy` 引入代码库

### 修复

- 修复了回测日志时间戳
- 修复了回测重复的初始账户事件

---

# NautilusTrader 1.124.0 Beta

发布于 2021年7月6日。

此版本看到了交易前风险检查选项的扩展（参见 `RiskEngine` 类文档）。还进行了大量的"底层"代码清理和整合。

### 破坏性变更

- 将 `Position.opened_timestamp_ns` 重命名为 `ts_opened_ns`
- 将 `Position.closed_timestamp_ns` 重命名为 `ts_closed_ns`
- 将 `Position.open_duration_ns` 重命名为 `duration_ns`
- 将日志记录器 `bypass_logging` 重命名为 `bypass`
- 重构了 `PositionEvent` 类型

### 增强功能

- 为 `RiskEngine` 迭代 2 添加了交易前风险检查
- 改进了 `Throttler` 功能和性能
- 移除了冗余的 `OrderInvalid` 状态和相关代码
- 改进了分析报告

### 修复

- `CASH` 账户类型的 PnL 计算
- 各种事件序列化

---

# NautilusTrader 1.123.0 Beta

发布于 2021年6月20日。

此版本的主要功能是对平台序列化的完全重新设计，以及对 [Parquet](https://parquet.apache.org/) 格式的初始支持。MessagePack 序列化功能已得到优化并保留。

为了明确性，现在有一个惯例，时间戳命名为 `timestamp_ns` 或以 `ts` 为前缀。用 `int64` 表示的时间戳始终以纳秒分辨率，并相应地附加 `_ns`。

已添加新回测数据工具的初始脚手架。

### 破坏性变更

- 将 `OrderState.PENDING_REPLACE` 重命名为 `OrderState.PENDING_UPDATE`
- 将 `timestamp_origin_ns` 重命名为 `ts_event_ns`
- 将数据的 `timestamp_ns` 重命名为 `ts_recv_ns`
- 将 `updated_ns` 重命名为 `ts_updated_ns`
- 将 `submitted_ns` 重命名为 `ts_submitted_ns`
- 将 `rejected_ns` 重命名为 `ts_rejected_ns`
- 将 `accepted_ns` 重命名为 `ts_accepted_ns`
- 将 `pending_ns` 重命名为 `ts_pending_ns`
- 将 `canceled_ns` 重命名为 `ts_canceled_ns`
- 将 `triggered_ns` 重命名为 `ts_triggered_ns`
- 将 `expired_ns` 重命名为 `ts_expired_ns`
- 将 `execution_ns` 重命名为 `ts_filled_ns`
- 将 `OrderBookLevel` 重命名为 `BookLevel`
- 将 `Order.volume` 重命名为 `Order.size`

### 增强功能

- 适配器依赖现在在安装时是可选的额外项
- 添加了 arrow/parquet 序列化
- 添加了对象 `to_dict()` 和 `from_dict()` 方法
- 添加了 `Order.is_pending_update`
- 添加了 `Order.is_pending_cancel`
- 为 `BacktestEngine` 添加了 `run_analysis` 配置选项
- 移除了 `TradeMatchId` 支持纯字符串
- 移除了检查时间戳时冗余的 `pd.Timestamp` 转换
- 移除了冗余的数据 `to_serializable_str` 方法
- 移除了冗余的数据 `from_serializable_str` 方法
- 移除了冗余的 `__ne__` 实现
- 移除了冗余的 `MsgPackSerializer` 杂项
- 移除了冗余的 `ObjectCache` 和 `IdentifierCache`
- 移除了冗余的字符串常量

### 修复

- 修复了 `CCXTExecutionClient` 中的毫秒到纳秒转换
- 为 `UpdateOrder` 处理添加了缺失的触发器
- 移除了所有 `import *`

---

# NautilusTrader 1.122.0 Beta

发布于 2021年6月6日。

此版本包含许多破坏性变更，旨在增强平台的核心功能和 API。为简化起见，数据和执行缓存已统一。会计功能也发生了很大变化，添加了"钩子"以准备准确计算和处理保证金。

### 破坏性变更

- 将 `Account.balance()` 重命名为 `Account.balance_total()`
- 将 `TradingStrategy.data` 整合到 `TradingStrategy.cache`
- 将 `TradingStrategy.execution` 整合到 `TradingStrategy.cache`
- 将 `redis` 子包移动到 `infrastructure`
- 将一些会计方法移回 `Instrument`
- 移除了 `Instrument.market_value()`
- 将 `Portfolio.market_values()` 重命名为 `Portfolio.net_exposures()`
- 将 `Portfolio.market_value()` 重命名为 `Portfolio.net_exposure()`
- 将 `InMemoryExecutionDatabase` 重命名为 `BypassCacheDatabase`
- 将 `Position.relative_qty` 重命名为 `Position.net_qty`
- 将 `default_currency` 重命名为 `base_currency`
- 从 `Instrument` 移除了 `cost_currency` 属性

### 增强功能

- `ExecutionClient` 现在可以选择计算账户状态
- 将数据和执行缓存统一为单个 `Cache`
- 改进了配置选项和命名
- 简化了 `Portfolio` 组件注册
- 简化了 `Cache` 连接到组件
- 为执行消息添加了 `repr`
- 添加了 `AccountType` 枚举
- 为 `Position` 添加了 `cost_currency`
- 为 `Instrument` 添加了 `get_cost_currency()`
- 为 `Instrument` 添加了 `get_base_currency()`

### 修复

- 修复了 `PENDING_CANCEL` 和 `PENDING_REPLACE` 状态的 `Order.is_working`
- 修复了 Redis 中纳秒时间戳的精度丢失
- 修复了未实例化客户端时的状态对账

---

# NautilusTrader 1.121.0 Beta

发布于 2021年5月30日。

在此版本中，方法签名的内联使用发生了重大变化。根据 Cython 文档：
*"请注意，类级别的 cdef 函数通过虚函数表处理，因此编译器在几乎所有情况下都无法内联它们。"*
<https://cython.readthedocs.io/en/latest/src/userguide/pyrex_differences.html?highlight=inline。>

已发现在方法签名中添加 `inline` 对系统性能没有影响 - 因此已将其移除以减少"噪音"并简化代码库。请注意，模块级函数的 `inline` 使用将传递给 C 编译器，预期结果是内联函数。

### 破坏性变更

- `BacktestEngine.add_venue` 为方法参数添加了 `venue_type`
- `ExecutionClient` 为构造函数参数添加了 `venue_type`
- `TraderId` 实例化
- `StrategyId` 实例化
- `Instrument` 序列化

### 增强功能

- 如果数据不立即可用，`Portfolio` 挂起计算
- 添加了具有扩展类定义的 `instruments` 子包
- 添加了原始发生时的 `timestamp_origin_ns` 时间戳
- 添加了 `AccountState.is_reported` 标志，标记是否由交易所报告或计算
- 简化了 `TraderId` 和 `StrategyId` 标识符
- 改进了 `ExecutionEngine` 订单路由
- 改进了 `ExecutionEngine` 客户端注册
- 添加了订单路由配置
- 添加了 `VenueType` 枚举和解析器
- 改进了标识符生成器的参数类型
- 改进了 `Money` 和 `Quantity` 千位逗号的日志格式

### 修复

- CCXT `TICK_SIZE` 精度模式 - 大小精度（BitMEX、FTX）
- 状态对账（各种错误）

---

# NautilusTrader 1.120.0 Beta

此版本专注于简化和增强现有机制

### 破坏性变更

- `Position` 现在需要 `Instrument` 参数
- 从 `OrderFilled` 移除了 `is_inverse`
- 从 `TradingCommand` 和子类移除了 `ClientId`
- 从 `TradingCommand` 和子类移除了 `AccountId`
- `TradingCommand` 序列化

### 增强功能

- 为 `ExecutionCache` 添加了 `Instrument` 方法
- 为缓存查询添加了 `Venue` 过滤器
- 将订单验证移动到 `RiskEngine`
- 重构了 `RiskEngine`
- 从标识符移除了路由类型信息

### 修复
无

---

# NautilusTrader 1.119.0 Beta

此版本对 `BaseDecimal` 及其子类 `Price` 和 `Quantity` 的值对象 API 进行了另一次重大重构。以前，在传入 `decimal.Decimal` 类型时不需要明确精度，这有时会导致用户传入具有非常大精度的十进制数时出现意外行为（当用 `decimal.Decimal` 包装浮点数时）。

已为 `Price` 和 `Quantity` 添加了便利方法，其中对于整数精度隐含为零，或对于字符串在 '.' 点后的位数中隐含。还为 `Instrument` 添加了便利方法以辅助用户体验。

`Money` 的序列化已得到改进，在字符串中包含用空格分隔的货币代码。这避免了货币代码的额外字段。

`RiskEngine` 已在 `ExecutionEngine` 之前重新连线，这澄清了责任领域，清理了注册序列，并允许更自然的命令和事件消息流。

### 破坏性变更

- 涉及 `Money` 的序列化
- 更改了 `Price` 和 `Quantity` 的使用
- 将 `BypassExecutionDatabase` 重命名为 `BypassCacheDatabase`

### 增强功能

- 重新连线了 `RiskEngine` 和 `ExecutionEngine` 序列
- 添加了 `Instrument` 数据库操作
- 添加了 `MsgPackInstrumentSerializer`
- 添加了 `Price.from_str()`
- 添加了 `Price.from_int()`
- 添加了 `Quantity.zero()`
- 添加了 `Quantity.from_str()`
- 添加了 `Quantity.from_int()`
- 添加了 `Instrument.make_price()`
- 添加了 `Instrument.make_qty()`
- 改进了 `Money` 的序列化

### 修复

- 处理传递给值对象的 `decimal.Decimal` 值的精度

---

# NautilusTrader 1.118.0 Beta

此版本通过移除中间 `BacktestDataContainer` 的需要简化了回测工作流程。还对 `OrderFill` 事件进行了一些简化，以及额外的订单状态和事件。

### 破坏性变更

- 将所有 'cancelled' 引用标准化为 'canceled'
- `SimulatedExchange` 不再为 `MarketOrder` 生成 `OrderAccepted`
- 移除了冗余的 `BacktestDataContainer`
- 移除了冗余的 `OrderFilled.cum_qty`
- 移除了冗余的 `OrderFilled.leaves_qty`
- 简化了 `BacktestEngine` 构造函数
- `BacktestMarketDataClient` 不再需要工具
- 将 `PortfolioAnalyzer.get_realized_pnls` 重命名为 `.realized_pnls`

### 增强功能

- 重新设计了 `BacktestEngine` 以直接接受数据
- 添加了 `OrderState.PENDING_CANCEL`
- 添加了 `OrderState.PENDING_REPLACE`
- 添加了 `OrderPendingUpdate` 事件
- 添加了 `OrderPendingCancel` 事件
- 添加了 `OrderFilled.is_buy` 属性（对应的 `is_buy_c()` 快速方法）
- 添加了 `OrderFilled.is_sell` 属性（对应的 `is_sell_c()` 快速方法）
- 添加了 `Position.is_opposite_side(OrderSide side)` 便利方法
- 为上述修改了 `Order` FSM 和事件处理
- 将事件生成整合到 `ExecutionClient` 基类
- 重构了 `SimulatedExchange` 以获得更大的清晰度

### 修复

- `ExecutionCache` 仓位开放查询
- 交易所 `OmsType.NETTING` 的交易所会计
- 交易所 `OmsType.NETTING` 的仓位翻转逻辑
- 多货币账户术语
- Windows 轮子打包
- Windows 路径错误

---

# NautilusTrader 1.117.0 Beta

此版本的主要推动力是在回测中增加对订单簿数据的支持。`SimulatedExchange` 现在维护每个工具的订单簿，并将使用 L2/L3 数据准确模拟市场影响。对于报价和交易 tick 数据，使用 L1 订单簿作为代理。未来版本将包括改进的成交建模假设和自定义。

### 破坏性变更

- `OrderBook.create` 现在接受 `Instrument` 和 `BookLevel`

### 增强功能

- `SimulatedExchange` 现在内部维护订单簿
- `LiveLogger` 现在表现出更好的阻塞行为和日志记录

### 修复

- Betfair 适配器的各种补丁
- 文档构建

---

# NautilusTrader 1.116.1 Beta

宣布官方 Windows 64 位支持。

已识别并修复了几个错误。

### 破坏性变更
无

### 增强功能

- 性能测试重构
- 移除了冗余的性能测试框架
- 为高性能队列添加了 `Queue.peek()`
- GitHub action 重构，Windows CI
- 32 位平台构建

### 修复

- `BookLevel.L3` 的 `OrderBook.create` 现在返回正确的订单簿
- Betfair 交易 ID 处理

---

# NautilusTrader 1.116.0 Beta

**此版本包含重大破坏性变更。**

对核心 API 进行了进一步的基本更改。

### 破坏性变更

- 为数据和执行客户端标识引入了 `ClientId`
- 标准化客户端 ID 为大写
- 将 `OrderBookOperation` 重命名为 `OrderBookDelta`
- 将 `OrderBookOperations` 重命名为 `OrderBookDeltas`
- 将 `OrderBookOperationType` 重命名为 `OrderBookDeltaType`

### 增强功能
无

### 修复
无

---

# NautilusTrader 1.115.0 Beta

**此版本包含重大破坏性变更。**

由于最近的反馈和进一步的思考 - 进行了涉及订单标识符的重大重命名。`Order` 是模型中唯一使用多个 ID 标识的领域对象。因此，更多的明确性有助于确保正确的逻辑。以前 `OrderId` 被隐含地假设为由交易场所分配的那个。这已通过将标识符重命名为 `VenueOrderId` 来澄清。在此之后，通过 `Order.id` 引用它不再有意义，因此将其更改为完整名称 `Order.venue_order_id`。这自然导致 `ClientOrderId` 在属性和变量中从 `cl_ord_id` 重命名为 `client_order_id`。

### 破坏性变更

- 将 `OrderId` 重命名为 `VenueOrderId`
- 将 `Order.id` 重命名为 `Order.venue_order_id`
- 将 `Order.cl_ord_id` 重命名为 `Order.client_order_id`
- 将 `AssetClass.STOCK` 重命名为 `AssetClass.EQUITY`
- 移除了冗余标志 `generate_position_ids`（由 `OmsType` 处理）

### 增强功能

- 引入 Betfair 集成
- 添加了 `AssetClass.METAL` 和 `AssetClass.ENERGY`
- 添加了 `VenueStatusEvent`、`InstrumentStatusEvent` 和 `InstrumentClosePrice`
- 使用 `np.ndarray` 来改进函数和指标性能

### 修复

- LiveLogger 阻塞时的日志消息

---

# NautilusTrader 1.114.0 Beta

**此版本包含重大破坏性变更。**

进一步标准化命名约定以及内部优化和修复。

### 破坏性变更

- 将 `AmendOrder` 重命名为 `UpdateOrder`
- 将 `OrderAmended` 重命名为 `OrderUpdated`
- 将 `amend` 和 `amended` 相关方法重命名为 `update` 和 `updated`
- 将 `OrderCancelReject` 重命名为 `OrderCancelRejected`（标准化时态）

### 增强功能

- 改进数据整理的效率
- 简化 `Logger` 和通用系统日志记录
- 添加了带配置的 `stdout` 和 `stderr` 日志流
- 添加了 `OrderBookData` 基类

### 修复

- 回测对 `GenericData` 和 `OrderBook` 相关数据的处理
- 回测 `DataClient` 创建逻辑阻止客户端注册

---

# NautilusTrader 1.113.0 Beta

**此版本包含重大破坏性变更。**

进一步标准化命名约定以及内部优化和修复。

### 破坏性变更

- 将 `AmendOrder` 重命名为 `UpdateOrder`
- 将 `OrderAmended` 重命名为 `OrderUpdated`
- 将 `amend` 和 `amended` 相关方法重命名为 `update` 和 `updated`
- 将 `OrderCancelReject` 重命名为 `OrderCancelRejected`（标准化时态）

### 增强功能

- 引入 `OrderUpdateRejected`，为清晰起见分离事件
- 优化 LiveLogger：现在在事件循环上运行，带有高性能 `Queue`
- 改进了向 `BacktestEngine` 添加策略的灵活性
- 改进了应用订单事件时 `VenueOrderId` 相等性检查

### 修复

- 移除了 `UNDEFINED` 枚举值。不允许在系统中表示无效值（优先抛出异常）

---

# NautilusTrader 1.112.0 Beta

**此版本包含重大破坏性变更。**

平台的内部时间戳已标准化为纳秒。做出此决定是为了将回测的准确性提高到纳秒精度，改进数据处理，包括用于回测的订单簿和自定义数据，并使平台面向未来达到更专业的标准。顶级用户 API 仍然接受 `datetime` 和 `timedelta` 对象以提高可用性。

还对命名约定进行了一些标准化，以更紧密地与既定的金融市场术语保持一致，参考 FIX5.0 SP2 规范和 CME MDP 3.0。

### 破坏性变更

- 将 `BarType` 移动到 `Bar` 中作为属性
- 由于上述更改了 `Bar` 处理方法的签名
- 移除了 `Instrument.leverage`（概念错误的地方）
- 将 `ExecutionClient.venue` 作为 `Venue` 更改为 `ExecutionClient.name` 作为 `str`
- 将时间戳数据类型的序列化更改为 `int64`
- 广泛更改了序列化常量名称
- 将 `OrderFilled.filled_qty` 重命名为 `OrderFilled.last_qty`
- 将 `OrderFilled.filled_price` 重命名为 `OrderFilled.last_px`
- 在方法和属性中将 `avg_price` 重命名为 `avg_px`
- 在方法和属性中将 `avg_open` 重命名为 `avg_px_open`
- 在方法和属性中将 `avg_close` 重命名为 `avg_px_close`
- 将 `Position.relative_quantity` 重命名为 `Position.relative_qty`
- 将 `Position.peak_quantity` 重命名为 `Position.peak_qty`

### 增强功能

- 标准化纳秒时间戳
- 添加了时间单位转换函数，如 `nautilus_trader.core.datetime` 中所示
- 为 `Venue` 添加了可选的 `broker` 属性以协助路由
- 增强了来自 `LiveExecutionEngine` 和 `LiveExecutionClient` 的状态对账
- 添加了内部消息以协助状态对账

### 修复

- `DataCache` 错误地缓存 K 线

---

# NautilusTrader 1.111.0 Beta

此版本为平台添加了进一步的增强。

### 破坏性变更
无

### 增强功能

- 构建了 `RiskEngine`，包括配置选项钩子和 `LiveRiskEngine` 实现
- 添加了通用 `Throttler`
- 为 `instrument_id` 相关请求添加了详细信息 `dict` 以涵盖 IB 期货合约
- 添加了缺失的法定货币
- 添加了额外的加密货币
- 添加了 ISO 4217 代码
- 添加了货币名称

### 修复

- 当在 `maxlen` 处阻塞时，实时引擎中的队列 `put` 协程未在事件循环上创建任务

---

# NautilusTrader 1.110.0 Beta

此版本对标识符 API 应用了另一个重大更改。`Security` 已重命名为 `InstrumentId`，以更清楚地表明该对象是标识符，并将工具概念与其标识符分组。

框架中的数据对象已进一步抽象，为处理回测中的自定义数据做准备。

还搭建了 `RiskEngine` 基类。

### 破坏性变更

- `Security` 重命名为 `InstrumentId`
- `Instrument.security` 重命名为 `Instrument.id`
- `Data` 成为具有 `timestamp` 和 `unix_timestamp` 属性的抽象基类
- `Data` 和 `DataType` 移动到 `model.data`
- `on_data` 方法现在接受 `GenericData`

### 增强功能

- 添加了 `GenericData`
- 添加了 `Future` 工具

### 修复
无

---

# NautilusTrader 1.109.0 Beta

此版本的主要推动力是通过 `InstrumentId` 优化和进一步巩固标识符模型的更改，并修复一些错误。

上一个版本导致的 CCXT 客户端错误已得到解决。

### 破坏性变更

- `InstrumentId` 现在接受一等值对象 `Symbol`
- `InstrumentId` `asset_class` 和 `asset_type` 不再可选
- `SimulatedExchange.venue` 更改为 `SimulatedExchange.id`

### 增强功能

- 确保 `TestTimer` 单调递增推进
- 添加了 `AssetClass.BETTING`

### 修复

- 关于 `instrument_id` vs `symbol` 命名的 CCXT 数据和执行客户端
- `InstrumentId` 相等性和哈希
- 各种文档字符串

---

# NautilusTrader 1.108.0 Beta

此版本执行了对 `Symbol` 以及平台内证券通常如何标识的重大重构。这将允许与 Interactive Brokers 和其他交易所、经纪商和交易对手方的更顺畅集成。

以前 `Symbol` 标识符还包括场所，这混淆了概念。替换的 `Security` 标识符更清楚地表达了具有符号字符串、主要 `Venue`、`AssetClass` 和 `AssetType` 属性的领域。

### 破坏性变更

- 所有以前的序列化
- `Security` 用扩展属性替换 `Symbol`
- `AssetClass.EQUITY` 更改为 `AssetClass.STOCK`
- `from_serializable_string` 更改为 `from_serializable_str`
- `to_serializable_string` 更改为 `to_serializable_str`

### 增强功能

- 报告现在包含完整的 instrument_id 名称
- 添加了 `AssetType.WARRANT`

### 修复

- `StopLimitOrder` 序列化

---

# NautilusTrader 1.107.1 Beta

这是一个补丁版本，应用了各种修复和重构。

`StopLimitOrder` 的行为继续得到修复和优化。`SimulatedExchange` 进一步重构以降低复杂性。

### 破坏性变更
无

### 增强功能
无

### 修复

- 订单 FSM 中的 `TRIGGERED` 状态
- `StopLimitOrder` 触发行为
- `OrderFactory.stop_limit` 缺失的 `post_only` 和 `hidden`
- `Order` 和 `StopLimitOrder` `__repr__` 字符串（重复 id）

---

# NautilusTrader 1.107.0 Beta

此版本的主要推动力是优化与订单匹配和修改行为相关的一些细节，以提高真实性。这涉及对 `SimulatedExchange` 的相当大的重构以管理其复杂性，并支持扩展订单类型。

LIMIT 订单的 `post_only` 标志现在在订单放置和修改期间，当可市价的限价订单将成为流动性 `TAKER` 时，产生预期行为。

测试覆盖率适度增加。

### 破坏性变更
无

### 增强功能

- 重构了 `SimulatedExchange` 订单匹配和修改逻辑
- 添加了 `risk` 子包以分组风险组件

### 修复

- `StopLimitOrder` 触发行为
- 所有 flake8 警告

---

# NautilusTrader 1.106.0 Beta

此版本的主要推动力是引入 Interactive Brokers 集成，并开始添加平台功能以支持这一努力。

### 破坏性变更

- `from_serializable_string` 方法更改为 `from_serializable_str`

### 增强功能

- 在 `adapters/ib` 中搭建 Interactive Brokers 集成
- 添加了 `Future` 工具类型
- 添加了 `StopLimitOrder` 订单类型
- 添加了 `Data` 和 `DataType` 类型以支持自定义数据处理
- 添加了 `InstrumentId` 标识符类型初始实现以支持扩展平台功能

### 修复

- `BracketOrder` 正确性
- CCXT 精度解析错误
- 一些日志格式
