# 安装

NautilusTrader官方支持在以下64位平台上运行Python 3.11-3.13*：

| 操作系统       | 支持版本    | CPU架构  |
|------------------------|-----------------------|-------------------|
| Linux (Ubuntu)         | 22.04及更高版本       | x86_64            |
| Linux (Ubuntu)         | 22.04及更高版本       | ARM64             |
| macOS                  | 14.7及更高版本        | ARM64             |
| Windows Server         | 2022及更高版本        | x86_64            |

\* Windows构建目前固定在CPython 3.13.2，因为后续的3.13.x Windows二进制文件是在启用了每解释器GIL的情况下构建的，但没有导出一些私有C-API符号。这些缺失的导出会破坏我们Cython扩展的链接。如果/当上游CPython发布恢复导出时，可以移除此固定。

:::note
NautilusTrader可能在其他平台上运行，但只有上面列出的平台会被开发者定期使用并在CI中测试。
:::

我们建议使用最新支持的Python版本，并在虚拟环境中安装[nautilus_trader](https://pypi.org/project/nautilus_trader/)以隔离依赖项。

**有两种支持的安装方式**：

1. 从PyPI或Nautech Systems包索引获取预构建的二进制wheel。
2. 从源码构建。

:::tip
我们强烈建议使用[uv](https://docs.astral.sh/uv)包管理器配合"原版"CPython进行安装。

Conda和其他Python发行版*可能*有效，但不是官方支持的。
:::

## 从PyPI安装

要使用Python的pip包管理器从PyPI安装最新的[nautilus_trader](https://pypi.org/project/nautilus_trader/)二进制wheel（或sdist包）：

```bash
pip install -U nautilus_trader
```

## 额外功能

安装特定集成的可选依赖项作为'extras'：

- `betfair`: Betfair适配器（集成）依赖项。
- `docker`: 使用IB网关时需要Docker（配合Interactive Brokers适配器）。
- `dydx`: dYdX适配器（集成）依赖项。
- `ib`: Interactive Brokers适配器（集成）依赖项。
- `polymarket`: Polymarket适配器（集成）依赖项。

使用pip安装特定额外功能：

```bash
pip install -U "nautilus_trader[docker,ib]"
```

## 从Nautech Systems包索引安装

Nautech Systems包索引（`packages.nautechsystems.io`）符合[PEP-503](https://peps.python.org/pep-0503/)标准，托管`nautilus_trader`的稳定版和开发版二进制wheel。
这使用户能够安装最新稳定版本或预发布版本进行测试。

### 稳定版wheel

稳定版wheel对应PyPI上`nautilus_trader`的官方发布，使用标准版本控制。

安装最新稳定版本：

```bash
pip install -U nautilus_trader --index-url=https://packages.nautechsystems.io/simple
```

### 开发版wheel

开发版wheel从`nightly`和`develop`分支发布，允许用户在稳定版本发布前测试功能和修复。

**注意**：`develop`分支的wheel仅为Linux x86_64平台构建以节省时间和计算资源，而`nightly` wheel支持如下所示的额外平台。

| 平台           | Nightly | Develop |
| :----------------- | :------ | :------ |
| `Linux (x86_64)`   | ✓       | ✓       |
| `Linux (ARM64)`    | ✓       | -       |
| `macOS (ARM64)`    | ✓       | -       |
| `Windows (x86_64)` | ✓       | -       |

这个过程还有助于保护计算资源，确保轻松访问CI管道中测试的确切二进制文件，同时遵守[PEP-440](https://peps.python.org/pep-0440/)版本控制标准：

- `develop` wheel使用版本格式`dev{date}+{build_number}`（例如：`1.208.0.dev20241212+7001`）。
- `nightly` wheel使用版本格式`a{date}`（alpha）（例如：`1.208.0a20241212`）。

:::warning
我们不建议在生产环境中使用开发版wheel，例如控制真实资本的实时交易。
:::

### 安装命令

默认情况下，pip会安装最新稳定版本。添加`--pre`标志确保考虑预发布版本，包括开发版wheel。

安装最新可用预发布版本（包括开发版wheel）：

```bash
pip install -U nautilus_trader --pre --index-url=https://packages.nautechsystems.io/simple
```

安装特定开发版wheel（例如，2024年12月12日的`1.208.0a20241212`）：

```bash
pip install nautilus_trader==1.208.0a20241212 --index-url=https://packages.nautechsystems.io/simple
```

### 可用版本

您可以在[包索引](https://packages.nautechsystems.io/simple/nautilus-trader/index.html)上查看`nautilus_trader`的所有可用版本。

通过编程方式请求和列出可用版本：

```bash
curl -s https://packages.nautechsystems.io/simple/nautilus-trader/index.html | grep -oP '(?<=<a href="))[^"]+(?=")' | awk -F'#' '{print $1}' | sort
```

### 分支更新

- `develop`分支wheel（`.dev`）：每次提交合并时持续构建和发布。
- `nightly`分支wheel（`a`）：当我们在**14:00 UTC**自动合并`develop`分支时每日构建和发布（如果有更改）。

### 保留策略

- `develop`分支wheel（`.dev`）：我们只保留最新的wheel构建。
- `nightly`分支wheel（`a`）：我们只保留最近的10个wheel构建。

## 从源码安装

如果您首先安装`pyproject.toml`中指定的构建依赖项，可以使用pip从源码安装。

1. 安装[rustup](https://rustup.rs/)（Rust工具链安装器）：
   - Linux和macOS：

```bash
curl https://sh.rustup.rs -sSf | sh
```

- Windows：
  - 下载并安装[`rustup-init.exe`](https://win.rustup.rs/x86_64)
  - 使用[Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)安装"Desktop development with C++"
- 验证（任何系统）：
    从终端会话运行：`rustc --version`

2. 在当前shell中启用`cargo`：
   - Linux和macOS：

```bash
source $HOME/.cargo/env
```

- Windows：
  - 启动新的PowerShell

     1. 安装[clang](https://clang.llvm.org/)（LLVM的C语言前端）：
- Linux：

```bash
sudo apt-get install clang
```

- Windows：

1. 将Clang添加到您的[Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)：

- 开始 | Visual Studio Installer | 修改 | C++ Clang tools for Windows (12.0.0 - x64…) = 选中 | 修改

2. 在当前shell中启用`clang`：

```powershell
[System.Environment]::SetEnvironmentVariable('path', "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\Llvm\x64\bin\;" + $env:Path,"User")
```

- 验证（任何系统）：
  从终端会话运行：

```bash
clang --version
```

3. 安装uv（有关更多详细信息，请参阅[uv安装指南](https://docs.astral.sh/uv/getting-started/installation)）：

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

4. 使用`git`克隆源码，并从项目根目录安装：

```bash
git clone --branch develop --depth 1 https://github.com/nautechsystems/nautilus_trader
cd nautilus_trader
uv sync --all-extras
```

:::note
`--depth 1`标志只获取最新提交，实现更快的轻量级克隆。
:::

5. 为PyO3编译设置环境变量（仅限Linux和macOS）：

```bash
# 为Python解释器设置库路径（在此情况下为Python 3.13.4）
export LD_LIBRARY_PATH="$HOME/.local/share/uv/python/cpython-3.13.4-linux-x86_64-gnu/lib:$LD_LIBRARY_PATH"

# 为PyO3设置Python可执行文件路径
export PYO3_PYTHON=$(pwd)/.venv/bin/python
```

:::note
调整`LD_LIBRARY_PATH`中的Python版本和架构以匹配您的系统。
使用`uv python list`查找您Python安装的确切路径。
:::

## 从GitHub发布安装

要从GitHub安装二进制wheel，首先导航到[最新发布](https://github.com/nautechsystems/nautilus_trader/releases/latest)。
下载适合您操作系统和Python版本的`.whl`文件，然后运行：

```bash
pip install <file-name>.whl
```

## 版本控制和发布

NautilusTrader仍在积极开发中。某些功能可能不完整，虽然API变得越来越稳定，但在发布之间可能会发生破坏性更改。
我们努力在发布说明中记录这些更改，**尽最大努力**。

我们旨在遵循**每周发布计划**，尽管实验性或大型功能可能会导致延迟。

只有在您准备适应这些更改时才使用NautilusTrader。

## Redis

在NautilusTrader中使用[Redis](https://redis.io)是**可选的**，只有在配置为缓存数据库或[消息总线](../concepts/message_bus_CN.md)的后端时才需要。

:::info
支持的最低Redis版本是6.2（[streams](https://redis.io/docs/latest/develop/data-types/streams/)功能所需）。
:::

为了快速设置，我们建议使用[Redis Docker容器](https://hub.docker.com/_/redis/)。您可以在`.docker`目录中找到示例设置，或运行以下命令启动容器：

```bash
docker run -d --name redis -p 6379:6379 redis:latest
```

此命令将：

- 如果尚未下载，则从Docker Hub拉取最新版本的Redis。
- 在分离模式（`-d`）下运行容器。
- 将容器命名为`redis`以便于引用。
- 在默认端口6379上公开Redis，使您机器上的NautilusTrader可以访问它。

管理Redis容器：

- 使用`docker start redis`启动
- 使用`docker stop redis`停止

:::tip
我们建议使用[Redis Insight](https://redis.io/insight/)作为GUI来高效地可视化和调试Redis数据。
:::

## 精度模式

NautilusTrader为其核心值类型（`Price`、`Quantity`、`Money`）支持两种精度模式，它们在内部位宽和最大十进制精度方面有所不同。

- **高精度**：128位整数，最多16位小数精度，更大的值范围。
- **标准精度**：64位整数，最多9位小数精度，较小的值范围。

:::note
默认情况下，官方Python wheel在Linux和macOS上以高精度（128位）模式**发布**。
在Windows上，由于缺乏本机128位整数支持，只有标准精度（64位）可用。

对于Rust crates，默认是标准精度，除非您明确启用`high-precision`功能标志。
:::

性能权衡是标准精度在典型回测中快约3-5%，但具有较低的十进制精度和较小的可表示值范围。

:::note
比较这些模式的性能基准测试尚待完成。
:::

### 构建配置

精度模式由以下确定：

- 在编译期间设置`HIGH_PRECISION`环境变量，**和/或**
- 明确启用`high-precision` Rust功能标志。

#### 高精度模式（128位）

```bash
export HIGH_PRECISION=true
make install-debug
```

#### 标准精度模式（64位）

```bash
export HIGH_PRECISION=false
make install-debug
```

### Rust功能标志

要在Rust中启用高精度（128位）模式，请将`high-precision`功能添加到您的`Cargo.toml`：

```toml
[dependencies]
nautilus_core = { version = "*", features = ["high-precision"] }
```

:::info
有关更多详细信息，请参阅[值类型](../concepts/overview_CN.md#value-types)规范。
:::
