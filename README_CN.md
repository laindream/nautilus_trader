# <img src="https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/nautilus-trader-logo.png" width="500">

[![codecov](https://codecov.io/gh/nautechsystems/nautilus_trader/branch/master/graph/badge.svg?token=DXO9QQI40H)](https://codecov.io/gh/nautechsystems/nautilus_trader)
[![codspeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/nautechsystems/nautilus_trader)
![pythons](https://img.shields.io/pypi/pyversions/nautilus_trader)
![pypi-version](https://img.shields.io/pypi/v/nautilus_trader)
![pypi-format](https://img.shields.io/pypi/format/nautilus_trader?color=blue)
[![Downloads](https://pepy.tech/badge/nautilus-trader)](https://pepy.tech/project/nautilus-trader)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/NautilusTrader)

| 分支      | 版本                                                                                                                                                                                                                     | 状态                                                                                                                                                                                            |
| :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `master`  | [![version](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnautechsystems%2Fnautilus_trader%2Fmaster%2Fversion.json)](https://packages.nautechsystems.io/simple/nautilus-trader/index.html)  | [![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=nightly)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml) |
| `nightly` | [![version](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnautechsystems%2Fnautilus_trader%2Fnightly%2Fversion.json)](https://packages.nautechsystems.io/simple/nautilus-trader/index.html) | [![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=nightly)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml) |
| `develop` | [![version](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnautechsystems%2Fnautilus_trader%2Fdevelop%2Fversion.json)](https://packages.nautechsystems.io/simple/nautilus-trader/index.html) | [![build](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml/badge.svg?branch=develop)](https://github.com/nautechsystems/nautilus_trader/actions/workflows/build.yml) |

| 平台               | Rust   | Python     |
| :----------------- | :----- | :--------- |
| `Linux (x86_64)`   | 1.88.0 | 3.11-3.13  |
| `Linux (ARM64)`    | 1.88.0 | 3.11-3.13  |
| `macOS (ARM64)`    | 1.88.0 | 3.11-3.13  |
| `Windows (x86_64)` | 1.88.0 | 3.11-3.13* |

\* Windows 构建当前固定在 CPython 3.13.2，请参阅[安装指南](https://github.com/nautechsystems/nautilus_trader/blob/develop/docs/getting_started/installation.md)。

- **文档**: <https://nautilustrader.io/docs/>
- **网站**: <https://nautilustrader.io>
- **支持**: [support@nautilustrader.io](mailto:support@nautilustrader.io)

## 简介

NautilusTrader 是一个开源、高性能、生产级的算法交易平台，
为量化交易者提供在事件驱动引擎上使用历史数据回测自动化交易策略组合的能力，
并且能够实时部署这些策略，无需修改代码。

该平台以 *AI 为先*，旨在在高性能和强大的 Python 原生环境中开发和部署算法交易策略。
这有助于解决保持 Python 研究/回测环境与生产实时交易环境一致性的挑战。

NautilusTrader 的设计、架构和实现理念将软件正确性和安全性置于最高优先级，
旨在支持 Python 原生、关键任务的交易系统回测和实时部署工作负载。

该平台也是通用的，不限于特定资产类别——任何 REST API 或 WebSocket 数据源都可以通过模块化适配器集成。
它支持跨多种资产类别和工具类型的高频交易，包括外汇、股票、期货、期权、加密货币和博彩，
实现跨多个交易场所的无缝同时操作。

![nautilus-trader](https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/nautilus-trader.png "nautilus-trader")

## 特性

- **快速**: 核心使用 Rust 编写，采用 [tokio](https://crates.io/crates/tokio) 异步网络。
- **可靠**: Rust 提供的类型安全和线程安全，支持可选的 Redis 状态持久化。
- **可移植**: 操作系统无关，运行在 Linux、macOS 和 Windows 上。支持 Docker 部署。
- **灵活**: 模块化适配器意味着任何 REST API 或 WebSocket 数据源都可以集成。
- **高级**: 支持时间有效性订单 `IOC`、`FOK`、`GTC`、`GTD`、`DAY`、`AT_THE_OPEN`、`AT_THE_CLOSE`，高级订单类型和条件触发器。执行指令 `post-only`、`reduce-only` 和冰山订单。条件订单包括 `OCO`、`OUO`、`OTO`。
- **可定制**: 添加用户定义的自定义组件，或利用[缓存](https://nautilustrader.io/docs/latest/concepts/cache)和[消息总线](https://nautilustrader.io/docs/latest/concepts/message_bus)从头组装整个系统。
- **回测**: 使用纳秒级分辨率的历史报价 tick、交易 tick、K线、订单簿和自定义数据，同时运行多个交易场所、工具和策略。
- **实时**: 在回测和实时部署之间使用相同的策略实现。
- **多场所**: 多场所功能促进做市和统计套利策略。
- **AI 训练**: 回测引擎足够快，可用于训练 AI 交易代理（强化学习/进化策略）。

![Alt text](https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/nautilus-art.png "nautilus")

> *nautilus - 来自古希腊语 'sailor'（水手）和 naus（船）。*
>
> *鹦鹉螺壳由模块化腔室组成，其生长因子近似对数螺旋。
> 这个概念可以转化为设计和架构的美学。*

## 为什么选择 NautilusTrader？

- **高性能事件驱动 Python**: 原生二进制核心组件。
- **回测与实时交易的一致性**: 相同的策略代码。
- **降低运营风险**: 增强的风险管理功能、逻辑准确性和类型安全。
- **高度可扩展**: 消息总线、自定义组件和执行器、自定义数据、自定义适配器。

传统上，交易策略研究和回测可能在 Python 中使用向量化方法进行，
然后需要使用 C++、C#、Java 或其他静态类型语言以更事件驱动的方式重新实现策略。
这里的原因是向量化回测代码无法表达实时交易的细粒度时间和事件依赖复杂性，
而编译语言由于其固有的更高性能和类型安全性被证明更适合。

NautilusTrader 的一个关键优势是现在绕过了这个重新实现步骤——因为平台的关键核心组件
都完全用 [Rust](https://www.rust-lang.org/) 或 [Cython](https://cython.org/) 编写。
这意味着我们在使用合适的工具，系统编程语言编译高性能二进制文件，
CPython C 扩展模块能够提供 Python 原生环境，适合专业量化交易者和交易公司。

## 为什么选择 Python？

Python 最初在几十年前被创建为一种简单的脚本语言，具有清晰直接的语法。
从那时起，它已经发展成为一种完全成熟的通用面向对象编程语言。
根据 TIOBE 指数，Python 目前是世界上最受欢迎的编程语言。
不仅如此，Python 已经成为数据科学、机器学习和人工智能的*事实上的通用语言*。

然而，Python 在大规模、延迟敏感系统方面存在性能和类型限制。Cython 通过在 Python 丰富的库和社区生态系统中引入静态类型来解决这些问题。

## 为什么选择 Rust？

[Rust](https://www.rust-lang.org/) 是一种多范式编程语言，专为性能和安全性设计，特别是安全并发。
Rust "极快"且内存高效（可与 C 和 C++ 媲美），没有垃圾收集器。
它可以为关键任务系统提供动力，在嵌入式设备上运行，并且易于与其他语言集成。

Rust 丰富的类型系统和所有权模型确定性地保证内存安全和线程安全——
在编译时消除许多类别的错误。

该项目越来越多地使用 Rust 来构建核心性能关键组件。Python 绑定通过 Cython 和 [PyO3](https://pyo3.rs) 实现——安装时不需要 Rust 工具链。

这个项目做出了[健全性承诺](https://raphlinus.github.io/rust/2020/01/18/soundness-pledge.html)：

> "这个项目的目标是没有健全性错误。
> 开发者将尽最大努力避免它们，并欢迎帮助分析和修复它们。"

> [!NOTE]
>
> **MSRV:** NautilusTrader 大量依赖 Rust 语言和编译器的改进。
> 因此，最低支持的 Rust 版本（MSRV）通常等于 Rust 的最新稳定版本。

## 集成

NautilusTrader 采用模块化设计，通过*适配器*工作，通过将原始 API 转换为统一接口和规范化域模型，
实现与交易场所和数据提供商的连接。

目前支持以下集成；详细信息请参见 [docs/integrations/](https://nautilustrader.io/docs/latest/integrations/)：

| 名称                                                                         | ID                    | 类型                    | 状态                                                  | 文档                                        |
| :--------------------------------------------------------------------------- | :-------------------- | :---------------------- | :------------------------------------------------------ | :------------------------------------------ |
| [Betfair](https://betfair.com)                                               | `BETFAIR`             | 体育博彩交易所 | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/betfair.md)       |
| [Binance](https://binance.com)                                               | `BINANCE`             | 加密货币交易所（CEX）   | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/binance.md)       |
| [Binance US](https://binance.us)                                             | `BINANCE`             | 加密货币交易所（CEX）   | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/binance.md)       |
| [Binance Futures](https://www.binance.com/en/futures)                        | `BINANCE`             | 加密货币交易所（CEX）   | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/binance.md)       |
| [Bybit](https://www.bybit.com)                                               | `BYBIT`               | 加密货币交易所（CEX）   | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/bybit.md)         |
| [Coinbase International](https://www.coinbase.com/en/international-exchange) | `COINBASE_INTX`       | 加密货币交易所（CEX）   | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/coinbase_intx.md) |
| [Databento](https://databento.com)                                           | `DATABENTO`           | 数据提供商           | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/databento.md)     |
| [dYdX](https://dydx.exchange/)                                               | `DYDX`                | 加密货币交易所（DEX）   | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/dydx.md)          |
| [Hyperliquid](https://hyperliquid.xyz)                                       | `HYPERLIQUID`         | 加密货币交易所（DEX）   | ![status](https://img.shields.io/badge/building-orange) | [指南](docs/integrations/hyperliquid.md)   |
| [Interactive Brokers](https://www.interactivebrokers.com)                    | `INTERACTIVE_BROKERS` | 经纪商（多场所） | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/ib.md)            |
| [OKX](https://okx.com)                                                       | `OKX`                 | 加密货币交易所（CEX）   | ![status](https://img.shields.io/badge/building-orange) | [指南](docs/integrations/okx.md)           |
| [Polymarket](https://polymarket.com)                                         | `POLYMARKET`          | 预测市场（DEX） | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/polymarket.md)    |
| [Tardis](https://tardis.dev)                                                 | `TARDIS`              | 加密数据提供商    | ![status](https://img.shields.io/badge/stable-green)    | [指南](docs/integrations/tardis.md)        |

- **ID**: 集成适配器客户端的默认客户端 ID。
- **类型**: 集成类型（通常是场所类型）。

### 状态

- `building`: 正在构建中，可能不处于可用状态。
- `beta`: 已完成到最小可工作状态，处于 beta 测试阶段。
- `stable`: 功能集和 API 已稳定，集成已经过开发者和用户的合理测试（可能仍有一些错误）。

更多详细信息请参见[集成](https://nautilustrader.io/docs/latest/integrations/index.html)文档。

## 版本控制和发布

**NautilusTrader 仍在积极开发中**。一些功能可能不完整，虽然
API 变得更加稳定，但在版本之间可能会发生破坏性变更。
我们努力在发布说明中记录这些变更，尽最大努力。

我们的目标是遵循**双周发布计划**，尽管实验性或较大的功能可能会导致延迟。

### 分支

我们的目标是在所有分支上保持稳定、通过的构建。

- `master`: 反映最新发布版本的源代码；推荐用于生产。
- `nightly`: `develop` 分支的每日快照，用于早期测试；在**14:00 UTC**合并或按需合并。
- `develop`: 贡献者和功能工作的活跃开发分支。

> [!NOTE]
>
> 我们的[路线图](/ROADMAP.md)旨在实现**版本 2.x 的稳定 API**（可能在 Rust 移植之后）。
> 一旦达到这个里程碑，我们计划为任何 API 变更实施正式的弃用过程。
> 这种方法允许我们现在保持快速的开发节奏。

## 精度模式

NautilusTrader 为其核心值类型（`Price`、`Quantity`、`Money`）支持两种精度模式，
它们在内部位宽和最大小数精度方面有所不同。

- **高精度**: 128 位整数，最多 16 位小数精度，更大的值范围。
- **标准精度**: 64 位整数，最多 9 位小数精度，更小的值范围。

> [!NOTE]
>
> 默认情况下，官方 Python wheels 在 Linux 和 macOS 上以高精度（128 位）模式**发布**。
> 在 Windows 上，由于缺乏原生 128 位整数支持，只有标准精度（64 位）可用。
> 对于 Rust crates，默认是标准精度，除非您明确启用 `high-precision` 功能标志。

更多详细信息请参见[安装指南](https://nautilustrader.io/docs/latest/getting_started/installation)。

**Rust 功能标志**: 要在 Rust 中启用高精度模式，请在您的 Cargo.toml 中添加 `high-precision` 功能：

```toml
[dependencies]
nautilus_model = { version = "*", features = ["high-precision"] }
```

## 安装

我们建议使用 Python 的最新支持版本，并在虚拟环境中安装 [nautilus_trader](https://pypi.org/project/nautilus_trader/) 以隔离依赖项。

**有两种受支持的安装方式**：

1. 从 PyPI 或 Nautech Systems 包索引获取预构建的二进制 wheel。
2. 从源代码构建。

> [!TIP]
>
> 我们强烈建议使用 [uv](https://docs.astral.sh/uv) 包管理器和"原生" CPython。
>
> Conda 和其他 Python 发行版*可能*可以工作，但不受官方支持。

### 从 PyPI

要使用 Python 的 pip 包管理器从 PyPI 安装最新的二进制 wheel（或 sdist 包）：

```bash
pip install -U nautilus_trader
```

### 从 Nautech Systems 包索引

Nautech Systems 包索引（`packages.nautechsystems.io`）符合 [PEP-503](https://peps.python.org/pep-0503/) 标准，
托管 `nautilus_trader` 的稳定和开发二进制 wheels。
这使用户能够安装最新的稳定版本或预发布版本进行测试。

#### 稳定 wheels

稳定 wheels 对应 `nautilus_trader` 在 PyPI 上的官方发布，使用标准版本控制。

要安装最新稳定版本：

```bash
pip install -U nautilus_trader --index-url=https://packages.nautechsystems.io/simple
```

#### 开发 wheels

开发 wheels 从 `nightly` 和 `develop` 分支发布，
允许用户在稳定发布之前测试功能和修复。

**注意**: 来自 `develop` 分支的 wheels 仅为 Linux x86_64 平台构建以节省时间和计算资源，
而 `nightly` wheels 支持下表所示的额外平台。

| 平台           | Nightly | Develop |
| :----------------- | :------ | :------ |
| `Linux (x86_64)`   | ✓       | ✓       |
| `Linux (ARM64)`    | ✓       | -       |
| `macOS (ARM64)`    | ✓       | -       |
| `Windows (x86_64)` | ✓       | -       |

这个过程还有助于保存计算资源，确保轻松访问在 CI 管道中测试的确切二进制文件，
同时遵循 [PEP-440](https://peps.python.org/pep-0440/) 版本控制标准：

- `develop` wheels 使用版本格式 `dev{date}+{build_number}`（例如，`1.208.0.dev20241212+7001`）。
- `nightly` wheels 使用版本格式 `a{date}`（alpha）（例如，`1.208.0a20241212`）。

> [!WARNING]
>
> 我们不建议在生产环境中使用开发 wheels，例如控制真实资本的实时交易。

#### 安装命令

默认情况下，pip 将安装最新的稳定版本。添加 `--pre` 标志确保考虑预发布版本，包括开发 wheels。

要安装最新的可用预发布版本（包括开发 wheels）：

```bash
pip install -U nautilus_trader --pre --index-url=https://packages.nautechsystems.io/simple
```

要安装特定的开发 wheel（例如，2024 年 12 月 12 日的 `1.208.0a20241212`）：

```bash
pip install nautilus_trader==1.208.0a20241212 --index-url=https://packages.nautechsystems.io/simple
```

#### 可用版本

您可以在[包索引](https://packages.nautechsystems.io/simple/nautilus-trader/index.html)上查看 `nautilus_trader` 的所有可用版本。

要以编程方式获取和列出可用版本：

```bash
curl -s https://packages.nautechsystems.io/simple/nautilus-trader/index.html | grep -oP '(?<=<a href=")[^"]+(?=")' | awk -F'#' '{print $1}' | sort
```

#### 分支更新

- `develop` 分支 wheels（`.dev`）：每次合并提交时持续构建和发布。
- `nightly` 分支 wheels（`a`）：当我们在**14:00 UTC**自动合并 `develop` 分支时（如果有变更）每天构建和发布。

#### 保留策略

- `develop` 分支 wheels（`.dev`）：我们只保留最新的 wheel 构建。
- `nightly` 分支 wheels（`a`）：我们只保留最近的 10 个 wheel 构建。

### 从源代码

如果您首先安装 `pyproject.toml` 中指定的构建依赖项，则可以使用 pip 从源代码安装。

1. 安装 [rustup](https://rustup.rs/)（Rust 工具链安装程序）：
   - Linux 和 macOS：

       ```bash
       curl https://sh.rustup.rs -sSf | sh
       ```

   - Windows：
       - 下载并安装 [`rustup-init.exe`](https://win.rustup.rs/x86_64)
       - 使用 [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16) 安装"使用 C++ 的桌面开发"
   - 验证（任何系统）：
       从终端会话运行：`rustc --version`

2. 在当前 shell 中启用 `cargo`：
   - Linux 和 macOS：

       ```bash
       source $HOME/.cargo/env
       ```

   - Windows：
     - 启动新的 PowerShell

3. 安装 [clang](https://clang.llvm.org/)（LLVM 的 C 语言前端）：
   - Linux：

       ```bash
       sudo apt-get install clang
       ```

   - Windows：
       1. 将 Clang 添加到您的 [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)：
          - 开始 | Visual Studio Installer | 修改 | C++ Clang tools for Windows (12.0.0 - x64…) = 选中 | 修改
       2. 在当前 shell 中启用 `clang`：

          ```powershell
          [System.Environment]::SetEnvironmentVariable('path', "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\Llvm\x64\bin\;" + $env:Path,"User")
          ```

   - 验证（任何系统）：
       从终端会话运行：`clang --version`

4. 安装 uv（更多详细信息请参见 [uv 安装指南](https://docs.astral.sh/uv/getting-started/installation)）：

    ```bash
    curl -LsSf https://astral.sh/uv/install.sh | sh
    ```

5. 使用 `git` 克隆源代码，并从项目根目录安装：

    ```bash
    git clone --branch develop --depth 1 https://github.com/nautechsystems/nautilus_trader
    cd nautilus_trader
    uv sync --all-extras
    ```

> [!NOTE]
>
> `--depth 1` 标志只获取最新提交，以实现更快、轻量的克隆。

6. 为 PyO3 编译设置环境变量（仅限 Linux 和 macOS）：

    ```bash
    # 为 Python 解释器设置库路径（在这个例子中是 Python 3.13.4）
    export LD_LIBRARY_PATH="$HOME/.local/share/uv/python/cpython-3.13.4-linux-x86_64-gnu/lib:$LD_LIBRARY_PATH"

    # 为 PyO3 设置 Python 可执行文件路径
    export PYO3_PYTHON=$(pwd)/.venv/bin/python
    ```

> [!NOTE]
>
> 调整 `LD_LIBRARY_PATH` 中的 Python 版本和架构以匹配您的系统。
> 使用 `uv python list` 找到您的 Python 安装的确切路径。

其他选项和更多详细信息请参见[安装指南](https://nautilustrader.io/docs/latest/getting_started/installation)。

## Redis

在 NautilusTrader 中使用 [Redis](https://redis.io) 是**可选的**，只有在配置为
[缓存](https://nautilustrader.io/docs/latest/concepts/cache)数据库或[消息总线](https://nautilustrader.io/docs/latest/concepts/message_bus)后端时才需要。
更多详细信息请参见[安装指南](https://nautilustrader.io/docs/latest/getting_started/installation#redis)的 **Redis** 部分。

## Makefile

提供了 `Makefile` 来自动化开发的大多数安装和构建任务。一些目标包括：

- `make install`: 以 `release` 构建模式安装，包含所有依赖组和额外项。
- `make install-debug`: 与 `make install` 相同，但使用 `debug` 构建模式。
- `make install-just-deps`: 只安装 `main`、`dev` 和 `test` 依赖项（不安装包）。
- `make build`: 以 `release` 构建模式运行构建脚本（默认）。
- `make build-debug`: 以 `debug` 构建模式运行构建脚本。
- `make build-wheel`: 以 `release` 模式运行 uv build 生成 wheel 格式。
- `make build-wheel-debug`: 以 `debug` 模式运行 uv build 生成 wheel 格式。
- `make cargo-test`: 使用 `cargo-nextest` 运行所有 Rust crate 测试。
- `make clean`: 删除所有构建结果，如 `.so` 或 `.dll` 文件。
- `make distclean`: **注意** 从仓库中删除所有不在 git 索引中的工件。这包括尚未 `git add` 的源文件。
- `make docs`: 使用 Sphinx 构建文档 HTML。
- `make pre-commit`: 对所有文件运行 pre-commit 检查。
- `make ruff`: 使用 `pyproject.toml` 配置对所有文件运行 ruff（带自动修复）。
- `make pytest`: 使用 `pytest` 运行所有测试。
- `make test-performance`: 使用 [codspeed](https://codspeed.io) 运行性能测试。

> [!TIP]
>
> 运行 `make help` 查看所有可用 make 目标的文档。

> [!TIP]
>
> 参见 [crates/infrastructure/TESTS.md](https://github.com/nautechsystems/nautilus_trader/blob/develop/crates/infrastructure/TESTS.md) 文件了解如何运行基础设施集成测试。

## 示例

指标和策略可以用 Python 和 Cython 开发。对于性能和延迟敏感的应用程序，我们建议使用 Cython。以下是一些示例：

- 用 Python 编写的[指标](/nautilus_trader/examples/indicators/ema_python.py)示例。
- 用 Cython 编写的[指标](/nautilus_trader/indicators/)示例。
- 用 Python 编写的[策略](/nautilus_trader/examples/strategies/)示例。
- 直接使用 `BacktestEngine` 的[回测](/examples/backtest/)示例。

## Docker

Docker 容器使用基础镜像 `python:3.12-slim` 构建，具有以下变体标签：

- `nautilus_trader:latest` 安装了最新发布版本。
- `nautilus_trader:nightly` 安装了 `nightly` 分支的 head。
- `jupyterlab:latest` 安装了最新发布版本以及 `jupyterlab` 和示例回测 notebook 及相关数据。
- `jupyterlab:nightly` 安装了 `nightly` 分支的 head 以及 `jupyterlab` 和示例回测 notebook 及相关数据。

您可以按如下方式拉取容器镜像：

```bash
docker pull ghcr.io/nautechsystems/<image_variant_tag> --platform linux/amd64
```

您可以通过运行以下命令启动回测示例容器：

```bash
docker pull ghcr.io/nautechsystems/jupyterlab:nightly --platform linux/amd64
docker run -p 8888:8888 ghcr.io/nautechsystems/jupyterlab:nightly
```

然后在以下地址打开您的浏览器：

```bash
http://127.0.0.1:8888/lab
```

> [!WARNING]
>
> NautilusTrader 当前超过了 Jupyter notebook 日志记录（stdout 输出）的速率限制。
> 因此，我们在示例中将 `log_level` 设置为 `ERROR`。降低此级别以查看更多
> 日志记录将导致 notebook 在单元格执行期间挂起。我们正在调查可能的修复方案，
> 可能涉及提高 Jupyter 的配置速率限制或限制 Nautilus 的日志刷新。
>
> - <https://github.com/jupyterlab/jupyterlab/issues/12845>
> - <https://github.com/deshaw/jupyterlab-limit-output>

## 开发

我们的目标是为这个 Python、Cython 和 Rust 的混合代码库提供最愉快的开发者体验。
有用的信息请参见[开发者指南](https://nautilustrader.io/docs/latest/developer_guide/index.html)。

> [!TIP]
>
> 在修改 Rust 或 Cython 代码后运行 `make build-debug` 进行编译，以获得最高效的开发工作流程。

### 使用 Rust 测试

[cargo-nextest](https://nexte.st) 是 NautilusTrader 的标准 Rust 测试运行器。
其主要优势是将每个测试隔离在自己的进程中，通过避免干扰来确保测试可靠性。

您可以通过运行以下命令安装 cargo-nextest：

```bash
cargo install cargo-nextest
```

> [!TIP]
>
> 使用 `make cargo-test` 运行 Rust 测试，它使用**cargo-nextest**的高效配置文件。

## 贡献

感谢您考虑为 NautilusTrader 做出贡献！我们欢迎任何和所有帮助来改进项目。
如果您有增强或错误修复的想法，第一步是在 GitHub 上开启一个[issue](https://github.com/nautechsystems/nautilus_trader/issues)
与团队讨论。这有助于确保您的贡献与项目目标保持一致，避免重复工作。

开始之前，请务必查看项目路线图中概述的[开源范围](/ROADMAP.md#open-source-scope)，了解什么在范围内和范围外。

准备好开始处理您的贡献时，请确保遵循
[CONTRIBUTING.md](https://github.com/nautechsystems/nautilus_trader/blob/develop/CONTRIBUTING.md) 文件中概述的指南。
这包括签署贡献者许可协议（CLA）以确保您的贡献可以包含在项目中。

> [!NOTE]
>
> Pull requests 应该针对 `develop` 分支（默认分支）。这是在发布之前集成新功能和改进的地方。

再次感谢您对 NautilusTrader 的兴趣！我们期待审查您的贡献并与您合作改进项目。

## 社区

加入我们在 [Discord](https://discord.gg/NautilusTrader) 上的用户和贡献者社区，聊天
并了解 NautilusTrader 的最新公告和功能。无论您是希望贡献的开发者还是只想了解更多平台信息的人，
我们的 Discord 服务器都欢迎所有人。

> [!WARNING]
>
> NautilusTrader 不发行、推广或认可任何加密货币代币。任何声称或暗示相反的说法都是未经授权和虚假的。
>
> 来自 NautilusTrader 的所有官方更新和通信将仅通过 <https://nautilustrader.io>、我们的 [Discord 服务器](https://discord.gg/NautilusTrader)
> 或我们的 X (Twitter) 账户：[@NautilusTrader](https://x.com/NautilusTrader) 分享。
>
> 如果您遇到任何可疑活动，请向相应平台举报并通过 <info@nautechsystems.io> 联系我们。

## 许可证

NautilusTrader 的源代码在 GitHub 上以 [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html) 许可证提供。
欢迎对项目做出贡献，需要完成标准的[贡献者许可协议（CLA）](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™ 由 Nautech Systems 开发和维护，这是一家专门从事
高性能交易系统开发的技术公司。
更多信息请访问 <https://nautilustrader.io>。

© 2015-2025 Nautech Systems Pty Ltd. 版权所有。

![nautechsystems](https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/ns-logo.png "nautechsystems")
<img src="https://github.com/nautechsystems/nautilus_trader/raw/develop/assets/ferris.png" width="128">
