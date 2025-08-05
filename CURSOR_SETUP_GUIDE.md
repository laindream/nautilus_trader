# NautilusTrader Cursor IDE 开发指南

这是一份专为 **NautilusTrader** 项目定制的 Cursor IDE 配置指南，帮助新手快速建立高效的开发环境。

## 📋 项目概况

### 🎯 什么是 NautilusTrader？

NautilusTrader 是一个**开源、高性能、生产级**的算法交易平台：

- **🚀 双重能力**：同时支持历史数据回测和实时交易（代码零修改）
- **⚡ 高性能架构**：Rust 核心 + Python 前端，性能与易用性兼具
- **🌍 通用平台**：支持外汇、股票、期货、期权、加密货币、体育博彩等
- **🤖 AI 友好**：专为量化研究和人工智能训练优化

### 🏗️ 技术架构

```
┌─────────────────┐
│   Python 层     │  ← 策略开发、API 接口、易用性
│   (应用逻辑)     │
├─────────────────┤
│   Cython 层     │  ← Python-Rust 桥接、类型安全
│   (绑定层)      │
├─────────────────┤
│    Rust 层      │  ← 核心引擎、网络、高性能计算
│   (核心引擎)     │
└─────────────────┘
```

## 🔧 环境要求

### 必需软件

- **Python**: 3.11-3.13
- **Rust**: 1.88.0+
- **Visual Studio Build Tools** (Windows) 或 **GCC/Clang** (Linux/macOS)

### 推荐工具

- **uv**: 现代 Python 包管理器
- **Git**: 版本控制
- **Docker**: 容器化部署（可选）

## 🚀 快速开始

### 1. 自动环境设置

我们为您准备了自动化设置脚本：

**Windows 用户：**

```powershell
PowerShell -ExecutionPolicy Bypass -File setup_dev_env.ps1
```

**Linux/macOS 用户：**

```bash
chmod +x setup_dev_env.sh && ./setup_dev_env.sh
```

### 2. 手动设置（如果需要）

```bash
# 1. 创建虚拟环境
python -m venv venv

# 2. 激活虚拟环境
# Windows:
venv\Scripts\activate
# Linux/macOS:
source venv/bin/activate

# 3. 安装依赖
pip install uv
uv sync --group dev --group test

# 4. 构建项目
python build.py
```

## 📁 项目结构解析

```
nautilus_trader/
├── 🦀 crates/              # Rust 工作区（核心实现）
│   ├── adapters/           # 交易所适配器
│   ├── common/             # 通用组件  
│   ├── core/               # 核心引擎
│   ├── model/              # 数据模型
│   └── indicators/         # 技术指标
├── 🐍 nautilus_trader/     # Python 包
│   ├── adapters/           # Python 适配器层
│   ├── backtest/           # 回测引擎
│   ├── live/               # 实时交易
│   └── indicators/         # Python 指标接口
├── 📚 examples/            # 示例代码
│   ├── backtest/           # 回测示例
│   └── live/               # 实时交易示例
├── 📖 docs/                # 详细文档
└── 🧪 tests/               # 测试套件
```

## 🎮 Cursor IDE 使用指南

### 核心快捷键

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `Ctrl+Shift+P` | 命令面板 | 访问所有功能 |
| `Ctrl+Shift+B` | 构建任务 | 快速构建项目 |
| `F5` | 开始调试 | 运行调试器 |
| `Ctrl+Shift+\`` | 新建终端 | 打开集成终端 |
| `Ctrl+P` | 快速打开文件 | 模糊文件搜索 |
| `Ctrl+Shift+F` | 全局搜索 | 在项目中搜索 |

### 预配置构建任务

通过 `Ctrl+Shift+P` → "Tasks: Run Task" 可以运行：

- **Build Rust (Debug)**: 构建 Rust 调试版本
- **Build Rust (Release)**: 构建 Rust 发布版本
- **Build Python Package**: 构建 Python 扩展
- **Test Rust**: 运行 Rust 测试
- **Run Python Tests**: 运行 Python 测试
- **Clippy Check**: Rust 代码检查
- **Run Ruff Format**: Python 代码格式化

### 调试配置

项目预配置了多种调试模式：

- **Python: Current File**: 调试当前 Python 文件
- **Python: Backtest Example**: 调试回测示例
- **Python: Pytest**: 调试测试文件
- **Rust: Debug Current Crate**: 调试 Rust 代码

## 💡 开发最佳实践

### 1. 代码结构指导

**Python 策略开发：**

```python
# examples/backtest/my_strategy.py
from nautilus_trader.trading.strategy import Strategy

class MyStrategy(Strategy):
    def on_start(self):
        # 策略初始化
        pass
    
    def on_data(self, data):
        # 处理市场数据
        pass
```

**Rust 核心组件：**

```rust
// crates/my_component/src/lib.rs
use nautilus_core::prelude::*;

pub struct MyComponent {
    // 组件字段
}

impl MyComponent {
    pub fn new() -> Self {
        // 构造函数
    }
}
```

### 2. 测试策略

**Python 测试：**

```bash
# 运行所有测试
python -m pytest tests/ -v

# 运行特定测试
python -m pytest tests/unit_tests/test_strategy.py -v

# 运行性能测试
python -m pytest tests/performance_tests/ -v
```

**Rust 测试：**

```bash
# 运行所有 Rust 测试
cargo test

# 运行特定 crate 测试
cargo test -p nautilus-model

# 运行基准测试
cargo bench
```

### 3. 代码质量

**Python 代码检查：**

```bash
# 格式化代码
ruff format .

# 检查代码质量
ruff check . --fix

# 类型检查
mypy nautilus_trader/
```

**Rust 代码检查：**

```bash
# 格式化代码
cargo fmt

# 检查代码质量
cargo clippy --all-targets --all-features
```

## 🎯 学习路径（新手推荐）

### 第1周：了解项目结构

1. 🔍 **浏览项目结构**：了解 `crates/` 和 `nautilus_trader/` 目录
2. 📖 **阅读文档**：查看 `docs/` 中的概念和架构文档
3. 🚀 **运行示例**：执行 `examples/backtest/` 中的简单示例

### 第2周：环境熟悉

1. 🔧 **配置开发环境**：使用我们提供的配置文件
2. 🧪 **运行测试**：熟悉测试框架和调试工具
3. 📝 **修改示例**：尝试修改现有的策略示例

### 第3周：核心概念

1. 📚 **学习核心概念**：理解 Engine、Cache、MessageBus 等
2. 🔍 **代码跟踪**：使用调试器跟踪代码执行流程
3. 🎨 **创建简单策略**：编写你的第一个交易策略

### 第4周：深入开发

1. 🦀 **学习 Rust 基础**：了解 Rust 语法和所有权模型
2. 🔗 **理解绑定机制**：学习 Python-Rust 交互
3. 🚀 **贡献代码**：开始为项目贡献小的改进

## 🆘 常见问题解决

### Python 相关

**问题：导入错误**

```python
# 错误：ModuleNotFoundError: No module named 'nautilus_trader'
```

**解决：**

```bash
# 确保在虚拟环境中且已构建项目
source venv/bin/activate  # Linux/macOS
# 或
venv\Scripts\activate     # Windows
python build.py
```

**问题：Cython 编译错误**

```
# 错误：Microsoft Visual C++ 14.0 is required
```

**解决：** 在 Windows 上安装 Visual Studio Build Tools

### Rust 相关

**问题：编译缓慢**

```
# 编译时间过长
```

**解决：**

```bash
# 使用更多并行任务
export CARGO_BUILD_JOBS=8

# 使用 mold 链接器（Linux）
sudo apt install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
```

### IDE 相关

**问题：语法高亮不工作**

- 确保安装了推荐扩展
- 重启 Cursor IDE
- 检查 Python 解释器路径

**问题：调试器无法启动**

- 检查虚拟环境路径
- 确保项目已正确构建
- 查看调试控制台的错误信息

## 📚 有用资源

### 官方文档

- 🌐 [NautilusTrader 官网](https://nautilustrader.io)
- 📖 [API 文档](https://nautilustrader.io/docs)
- 🚀 [入门指南](https://nautilustrader.io/docs/getting_started/installation)

### 社区支持

- 💬 [Discord 社区](https://discord.gg/NautilusTrader)
- 🐙 [GitHub 仓库](https://github.com/nautechsystems/nautilus_trader)
- 🐛 [问题报告](https://github.com/nautechsystems/nautilus_trader/issues)

### 学习资源

- 🦀 [Rust 官方教程](https://doc.rust-lang.org/book/)
- 🐍 [Python 金融分析](https://github.com/yhilpisch/py4fi2nd)
- 📊 [量化交易基础](https://www.quantstart.com/)

## 🎉 祝您开发愉快

通过这个配置，您已经拥有了一个专业的 NautilusTrader 开发环境。记住：

- 🔄 **迭代学习**：从简单示例开始，逐步深入
- 🤝 **积极参与**：在社区中提问和分享经验
- 🧪 **实验探索**：不要害怕尝试和犯错误
- 📖 **阅读源码**：这是学习最好的方式之一

Happy Trading! 🚀📈
