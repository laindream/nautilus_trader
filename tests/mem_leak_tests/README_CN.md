# 性能测试

这个子包提供了一套性能测试工具，包括可以运行的脚本来分析内存和线程资源使用情况。

内存分析使用 [memray](https://github.com/bloomberg/memray) 进行。
这个包不是开发依赖项，因为它目前不支持Windows系统。

您可以通过PyPI安装这个包：

```bash
pip install memray
```

要使用memray进行分析，首先使用memray CLI调用脚本：

```bash
memray run --live-port 8100 --live-remote tests/mem_leak_tests/memray_backtest.py
```

然后从另一个shell连接到memray分析器仪表板：

```bash
memray live 8100
```
