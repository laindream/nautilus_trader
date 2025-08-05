# 环境设置

对于开发，我们建议使用PyCharm *Professional*版本IDE，因为它能解释Cython语法。或者，您可以使用带有Cython扩展的Visual Studio Code。

[uv](https://docs.astral.sh/uv)是处理所有Python虚拟环境和依赖项的首选工具。

[pre-commit](https://pre-commit.com/)用于在提交时自动运行各种检查、自动格式化程序和linting工具。

NautilusTrader越来越多地使用[Rust](https://www.rust-lang.org)，因此您的系统上也应该安装Rust（[安装指南](https://www.rust-lang.org/tools/install)）。

:::info
NautilusTrader *必须*在**Linux、macOS和Windows**上编译和运行。请记住可移植性（使用`std::path::Path`，避免在shell脚本中使用Bash特定语法等）。
:::

## 设置

以下步骤适用于类UNIX系统，只需完成一次。

1. 按照[安装指南](../getting_started/installation_CN.md)设置项目，修改最后的命令以安装开发和测试依赖项：

```bash
uv sync --active --all-groups --all-extras
```

或

```bash
make install
```

如果您经常开发和迭代，那么在调试模式下编译通常就足够了，而且比完全优化的构建*显著*更快。
要在调试模式下安装，请使用：

```bash
make install-debug
```

2. 设置pre-commit钩子，它将在提交时自动运行：

```bash
pre-commit install
```

在打开拉取请求之前，请在本地运行格式化和lint套件，以便CI在第一次尝试时通过：

```bash
make format
make pre-commit
```

确保Rust编译器报告**零错误** - 损坏的构建会拖慢每个人的速度。

3. **可选**：对于频繁的Rust开发，在`.cargo/config.toml`中配置`PYO3_PYTHON`变量，指向Python解释器的路径。这有助于减少IDE/rust-analyzer基于`cargo check`的重新编译时间：

```bash
PYTHON_PATH=$(which python)
echo -e "\n[env]\nPYO3_PYTHON = \"$PYTHON_PATH\"" >> .cargo/config.toml
```

由于`.cargo/config.toml`被跟踪，配置git跳过任何本地修改：

```bash
git update-index --skip-worktree .cargo/config.toml
```

要恢复跟踪：`git update-index --no-skip-worktree .cargo/config.toml`

## 构建

在对`.rs`、`.pyx`或`.pxd`文件进行任何更改后，您可以通过运行以下命令重新编译：

```bash
uv run --no-sync python build.py
```

或

```bash
make build
```

如果您经常开发和迭代，那么在调试模式下编译通常就足够了，而且比完全优化的构建*显著*更快。
要在调试模式下编译，请使用：

```bash
make build-debug
```

## 更快的构建

cranelift后端显著减少了开发、测试和IDE检查的构建时间。但是，cranelift在nightly工具链上可用，需要额外配置。安装nightly工具链：

```
rustup install nightly
rustup override set nightly
rustup component add rust-analyzer # 安装nightly lsp
rustup override set stable # 重置为stable
``` 