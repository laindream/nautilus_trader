# 基准测试

本指南解释了NautilusTrader如何测量Rust性能，何时使用每种工具以及在添加新基准测试时应遵循的约定。

---

## 工具概述

Nautilus Trader依赖于**两个互补的基准测试框架**：

| 框架 | 它是什么？ | 它测量什么 | 何时优先使用 |
|-----------|-------------|------------------|-------------------|
| [**Criterion**](https://docs.rs/criterion/latest/criterion/) | 产生详细HTML报告并执行离群值检测的统计基准测试工具。 | 带置信区间的挂钟运行时间。 | 端到端场景，任何慢于≈100 ns的内容，视觉比较。 |
| [**iai**](https://docs.rs/iai/latest/iai/) | 通过硬件计数器计算退役CPU指令的确定性微基准测试工具。 | 精确指令计数（无噪声）。 | 超快函数，通过指令差异进行CI门控。 |

大多数热代码路径都能从**两种**测量方式中受益。

---

## 目录布局

每个crate将其性能测试保存在本地`benches/`文件夹中：

```text
crates/<crate_name>/
└── benches/
    ├── foo_criterion.rs   # Criterion组
    └── foo_iai.rs         # iai微基准测试
```

`Cargo.toml`必须明确列出每个基准测试，以便`cargo bench`发现它们：

```toml
[[bench]]
name = "foo_criterion"             # benches/中的文件词干
path = "benches/foo_criterion.rs"
harness = false                    # 禁用默认libtest工具
```

---

## 编写Criterion基准测试

1. 在计时循环（`b.iter`）**外部执行所有昂贵的设置**。
2. 用`black_box`包装输入/输出以防止优化器移除工作。
3. 用`benchmark_group!`将相关案例分组，当默认值不理想时设置`throughput`或`sample_size`。

```rust
use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn bench_my_algo(c: &mut Criterion) {
    let data = prepare_data(); // 重设置只做一次

    c.bench_function("my_algo", |b| {
        b.iter(|| my_algo(black_box(&data)));
    });
}

criterion_group!(benches, bench_my_algo);
criterion_main!(benches);
```

---

## 编写iai基准测试

`iai`需要**不接受参数**并返回值（可以被忽略）的函数。保持它们尽可能小，以便测量的指令计数有意义。

```rust
use std::hint::black_box;

fn bench_add() -> i64 {
    let a = black_box(123);
    let b = black_box(456);
    a + b
}

iai::main!(bench_add);
```

---

## 在本地运行基准测试

- **单个crate**：`cargo bench -p nautilus-core`。
- **单个基准测试模块**：`cargo bench -p nautilus-core --bench time`。
- **CI性能基准测试**：`make cargo-ci-benches`（逐个运行CI性能工作流程中包含的crate以避免混合panic策略链接器问题）。

Criterion将HTML报告写入`target/criterion/`；在浏览器中打开`target/criterion/report/index.html`。

### 生成火焰图

`cargo-flamegraph`让您看到单个基准测试的采样调用栈配置文件。在Linux上它使用`perf`，在macOS上它使用`DTrace`。

1. 每台机器安装一次`cargo-flamegraph`（它会自动安装`cargo flamegraph`子命令）。

   ```bash
   cargo install flamegraph
   ```

2. 使用符号丰富的`bench`配置文件运行特定基准测试。

   ```bash
   # 示例：nautilus-common中的匹配基准测试
   cargo flamegraph --bench matching -p nautilus-common --profile bench
   ```

3. 在浏览器中打开生成的`flamegraph.svg`并放大热路径。

#### Linux

在Linux上，必须可用`perf`。在Debian/Ubuntu上，您可以使用以下命令安装它：

```bash
sudo apt install linux-tools-common linux-tools-$(uname -r)
```

如果您看到提到`perf_event_paranoid`的错误，您需要为当前会话放松内核的perf限制（需要root权限）：

```bash
sudo sh -c 'echo 1 > /proc/sys/kernel/perf_event_paranoid'
```

值`1`通常就足够了；如果需要，可以将其设置回`2`（默认值）或通过`/etc/sysctl.conf`使更改永久生效。

#### macOS

在macOS上，`DTrace`需要root权限，因此您必须使用`sudo`运行`cargo flamegraph`：

```bash
# 注意使用sudo
sudo cargo flamegraph --bench matching -p nautilus-common --profile bench
```

> **警告**
> 使用`sudo`运行会在您的`target/`目录中创建由`root`用户拥有的文件。这可能会在后续的`cargo`命令中导致权限错误。
>
> 要解决这个问题，您可能需要手动删除root拥有的文件，或者简单地运行`sudo cargo clean`来删除整个`target/`目录。

因为`[profile.bench]`保留完整的调试符号，SVG将显示可读的函数名，而不会膨胀生产二进制文件（仍然使用`panic = "abort"`并通过`[profile.release]`构建）。

> **注意** 基准测试二进制文件使用工作空间`Cargo.toml`中定义的自定义`[profile.bench]`编译。该配置文件继承自`release-debugging`，保留完整优化*和*调试符号，以便`cargo flamegraph`或`perf`等工具产生人类可读的堆栈跟踪。

---

## 模板

即用即拷贝的启动文件位于`docs/dev_templates/`中。

- **Criterion**：`criterion_template.rs`
- **iai**：`iai_template.rs`

将模板复制到`benches/`中，调整导入和名称，然后开始测量！
