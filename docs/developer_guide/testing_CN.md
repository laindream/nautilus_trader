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

- 断言调用抽象方法时引发`NotImplementedError`。
- 当无法测试时断言if-else块的最终条件检查（如上所述）。

这些测试维护成本高（因为它们必须与任何重构保持一致），回报很少或没有好处。
意图是让所有抽象方法实现都被测试完全覆盖。
因此，当不再适当时应该明智地移除`pragma: no cover`，并将其使用*限制*在上述情况下。

## 调试Rust测试

Rust测试可以使用默认测试配置进行调试。

如果您想在编译时使用调试符号运行所有测试以便以后单独调试某些测试，
请运行`make cargo-test-debug`而不是`make cargo-test`。

在IntelliJ IDEA中，要调试以`#[rstest]`开头且在测试头部定义参数的参数化测试，
您需要修改测试的运行配置，使其看起来像`test --package nautilus-model --lib data::bar::tests::test_get_time_bar_start::case_1`
（删除字符串末尾的`-- --exact`并附加`::case_n`，其中`n`是对应于从1开始的第n个参数化测试的整数）。
这样做的原因在[这里](https://github.com/rust-lang/rust-analyzer/issues/8964#issuecomment-871592851)有文档记录（测试被扩展为一个包含几个名为`case_n`的函数的模块）。

在VS Code中，可以直接选择要调试的测试用例。
