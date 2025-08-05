# 测试

测试套件分为广泛的测试类别，包括：

- 单元测试
- 集成测试
- 验收测试
- 性能测试
- 内存泄漏测试

性能测试的存在是为了帮助开发性能关键组件。

测试可以使用[pytest](https://docs.pytest.org)运行，这是我们的主要测试运行器。
我们建议使用参数化测试和固件（例如，`@pytest.mark.parametrize`）来避免重复代码并提高清晰度。

## 运行测试

### Python测试

从仓库根目录：

```bash
make pytest
# 或
uv run --active --no-sync pytest --new-first --failed-first
# 或简单地
pytest
```

对于性能测试：

```bash
make test-performance
# 或
uv run --active --no-sync pytest tests/performance_tests --benchmark-disable-gc --codspeed
```

### Rust测试

```bash
make cargo-test
# 或
cargo nextest run --workspace --features "python,ffi,high-precision,defi" --cargo-profile nextest
```

### IDE集成

- **PyCharm**：右键点击tests文件夹或文件 → "运行pytest"
- **VS Code**：使用Python Test Explorer扩展

## 测试风格

- 测试函数命名应该描述正在测试的内容；不必在函数名称中包含预期的断言。
- 测试函数*可以*有文档字符串，这对于详细说明测试设置、场景和预期断言很有用。
- 偏好pytest风格的自由函数用于Python测试，而不是带有设置方法的测试类。
- **分组断言** - 在可能的情况下先执行所有设置/操作步骤，然后在测试结束时一起断言期望，以避免*操作-断言-操作*的坏味道。
- 在**测试**中使用`unwrap`、`expect`或直接`panic!`/`assert`调用是可接受的。测试套件的清晰性和简洁性胜过生产代码中所需的防御性错误处理。

## 模拟

单元测试通常包括其他充当模拟的组件。这样做的目的是简化测试套件，避免广泛使用模拟框架，尽管`MagicMock`对象目前在特定情况下使用。

## 代码覆盖率

代码覆盖率输出使用`coverage`生成，并使用[codecov](https://about.codecov.io/)报告。

高测试覆盖率是项目的目标，但不能以适当的错误处理为代价，或导致架构的"测试诱导损害"。

目前代码库中有一些区域在不更改生产代码的情况下无法测试。
例如，if-else块的最后条件检查会捕获无法识别的值；
这些应该保留在原位，以防生产代码发生更改，这些检查可以捕获到。

其他设计时异常也可能无法测试，因此100%测试覆盖率不是最终目标。

## 排除的代码覆盖率

整个代码库中找到的`pragma: no cover`注释[从测试覆盖率中排除代码](https://coverage.readthedocs.io/en/coverage-4.3.3/excluding.html)。
使用它们的原因是为了减少为了保持高覆盖率而进行的冗余/不必要的测试，例如： 