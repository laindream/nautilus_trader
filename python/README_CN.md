# NautilusTrader v2

欢迎来到新一代NautilusTrader！

> [!WARNING]
>
> **正在积极开发中，目前尚不能实际使用。**

此目录包含`nautilus_trader` v2的纯Python包，完全通过PyO3绑定构建。

这种方法移除了旧的Cython层，提供了更清洁的架构、与Rust核心的直接集成，以及更流畅的开发体验。

**为了减少混淆，我们遵循一些简单的规则：**

- `python/`目录是自包含的，v2版本的所有Python相关内容都将在这里。
- 当我们完全从v1过渡后，此目录将保留（顶级的`nautilus_trader/`将被移除）。
- 目前，此目录外的任何内容都不应引用此目录内的任何内容。

## 项目结构

v2包的结构设计为在公共Python API和内部编译核心之间提供清晰的分离。
这通过类型存根启用了一流的IDE/编辑器支持，并允许混合使用纯Python和高性能Rust模块。

```
python/
├── README.md                   # 此文件
├── generate_stubs.py           # 自动生成Python类型存根
├── pyproject.toml              # 基于Maturin的v2包构建配置
├── uv.lock                     # v2包的UV锁定文件
└── nautilus_trader/
    ├── __init__.py             # 主包入口点，从_libnautilus重新导出
    ├── _libnautilus.so         # *单一*编译的Rust扩展（由构建创建）
    ├── core/
    │   ├── __init__.py         # 从`_libnautilus.core`重新导出
    │   └── __init__.pyi        # 核心模块的类型存根（进行中）
    ├── model/
    │   ├── __init__.py         # 从`_libnautilus.model`重新导出
    │   └── __init__.pyi        # 模型模块的类型存根（进行中）
    └── ...                     # 其他子模块遵循相同模式
```

## 开发设置

所有命令都应在此`python/`目录内运行。

### 先决条件

- 在项目根目录激活的虚拟环境（例如`.venv`）。
- Python 3.11-3.13
- Rust工具链（通过`rustup`）。

### 快速开始

要设置开发环境，请运行以下命令。它将编译Rust扩展，以"可编辑"模式安装它，并安装所有必要的开发和测试依赖项。

```bash
uv run --active maturin develop --extras dev,test
```

这是您开始所需的唯一命令。如果您对Rust代码进行更改，只需再次运行它即可重新编译。

## 工作原理

`nautilus_trader` Python包充当已编译Rust核心的"外观"。

1.  **构建**：`maturin develop`将所有Rust代码编译成单个本机库`nautilus_trader/_libnautilus.so`。
2.  **外观**：`nautilus_trader/__init__.py`文件从`_libnautilus.so`文件导入所有功能。
3.  **子模块**：每个子目录（例如`nautilus_trader/model/`）使用其`__init__.py`从`_libnautilus`重新导出相关组件，创建一个清洁、有组织的公共API。
4.  **类型提示**：`.pyi`存根文件为您的IDE和`mypy`等工具提供完整的类型信息，即使对于编译的Rust代码，也能为您提供自动完成和静态分析。

## 使用方法

包安装后，您可以像任何其他Python库一样导入和使用它。底层的Rust实现是完全透明的。

```python
from nautilus_trader.core import UUID4

# 直接使用绑定
UUID4()
# ...
```

## 安装

### 从源码安装

要从源码构建和安装，您需要安装Rust和Python 3.11+。您可以使用uv或Poetry：

**使用uv：**

```bash
# 克隆仓库
git clone https://github.com/nautechsystems/nautilus_trader.git
cd nautilus_trader/python

# 安装依赖并构建
uv run --active maturin develop --extras dev,test
```

### 开发轮子（预发布版）

虽然v2仍在大量开发中，但从`develop`或`nightly`分支的每次成功构建都会将轮子发布到我们的私有v2包索引。

```bash
# 添加Nautilus v2索引
pip install --index-url https://packages.nautechsystems.io/v2/simple/ --pre nautilus-trader
```

要点：

- 需要`--pre`标志，因为轮子被标记为开发版本（例如`0.2.0.dev20250601`）。
- 轮子是为x86-64 Linux和ARM64 macOS上的CPython 3.13构建的。

## Python类型存根

自动生成类型存根是一项高优先级的进行中工作...
