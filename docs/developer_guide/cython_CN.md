# Cython

在这里您将找到使用Cython语言开发NautilusTrader的指导和技巧。
更多关于Cython语法和约定的信息可以通过阅读[Cython文档](https://cython.readthedocs.io/en/latest/index.html)获得。

## 什么是Cython？

Cython是Python的超集，编译为C扩展模块，支持可选的静态类型和优化性能。NautilusTrader依赖Cython提供Python绑定和性能关键组件。

## 函数和方法签名

确保所有返回`void`或原始C类型（如`bint`、`int`、`double`）的函数和方法在签名中包含`except *`关键字。

这将确保Python异常不被忽略，而是像预期的那样"冒泡"到调用者。

## 调试

### PyCharm

对Cython的改进调试支持多年来一直是PyCharm的高票功能。不幸的是，可以安全地假设PyCharm不会收到对Cython调试的一流支持
<https://youtrack.jetbrains.com/issue/PY-9476>。

### Cython文档

以下建议包含在Cython文档中：
<https://cython.readthedocs.io/en/latest/src/userguide/debugging.html>

总结是它涉及从命令行手动运行专门版本的`gdb`。
我们不推荐这种工作流程。

### 技巧

在调试并寻求理解像NautilusTrader这样的复杂系统时，使用调试器逐步执行代码可能非常有帮助。由于这对代码库的Cython部分不可用，有一些事情可以帮助：

- 确保为您正在调试的回测或实时系统配置`LogLevel.DEBUG`。
  这在`BacktestEngineConfig(logging=LoggingConfig(log_level="DEBUG"))`或`TradingNodeConfig(logging=LoggingConfig=log_level="DEBUG"))`上可用。
  激活`DEBUG`模式后，您将看到更细粒度和详细的日志跟踪，这可能是您理解流程所需的。
- 除此之外，如果您仍然需要系统某部分的更细粒度可见性，我们建议对组件记录器进行一些精心安排的调用（通常`self._log.debug(f"HERE {variable}"`就足够了）。 