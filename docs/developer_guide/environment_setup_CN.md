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

1. 按照[安装指南](../getting_started/installation.md)设置项目，修改最后的命令以安装开发和测试依赖项：

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

在工作空间`Cargo.toml`中激活nightly功能并对开发和测试配置文件使用"cranelift"后端。您可以使用`git apply <patch>`应用下面的补丁。您可以在推送更改之前使用`git apply -R <patch>`删除它。

```
diff --git a/Cargo.toml b/Cargo.toml
index 62b78cd8d0..beb0800211 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -1,3 +1,6 @@
+# This line needs to come before anything else in Cargo.toml
+cargo-features = ["codegen-backend"]
+
 [workspace]
 resolver = "2"
 members = [
@@ -140,6 +143,7 @@ lto = false
 panic = "unwind"
 incremental = true
 codegen-units = 256
+codegen-backend = "cranelift"

 [profile.test]
 opt-level = 0
@@ -150,11 +154,13 @@ strip = false
 lto = false
 incremental = true
 codegen-units = 256
+codegen-backend = "cranelift"

 [profile.nextest]
 inherits = "test"
 debug = false # Improves compile times
 strip = "debuginfo" # Improves compile times
+codegen-backend = "cranelift"

 [profile.release]
 opt-level = 3
```

运行`make build-debug`等命令时传递`RUSTUP_TOOLCHAIN=nightly`，并将其包含在所有[rust analyzer设置](#rust-analyzer设置)中，以实现更快的构建和IDE检查。

## 服务

您可以使用位于`.docker`目录中的`docker-compose.yml`文件来引导Nautilus工作环境。这将启动以下服务：

```bash
docker-compose up -d
```

如果您只想运行特定服务（例如`postgres`），您可以使用命令启动它们：

```bash
docker-compose up -d postgres
```

使用的服务有：

- `postgres`：Postgres数据库，root用户`POSTRES_USER`默认为`postgres`，`POSTGRES_PASSWORD`默认为`pass`，`POSTGRES_DB`默认为`postgres`。
- `redis`：Redis服务器。
- `pgadmin`：PgAdmin4用于数据库管理和管理。

:::info
请仅将此用作开发环境。对于生产环境，请使用适当且更安全的设置。
:::

服务启动后，您必须使用`psql` cli登录以创建`nautilus` Postgres数据库。
为此，您可以运行并输入来自docker服务设置的`POSTGRES_PASSWORD`

```bash
psql -h localhost -p 5432 -U postgres
```

以`postgres`管理员身份登录后，使用目标数据库名称（我们使用`nautilus`）运行`CREATE DATABASE`命令：

```
psql (16.2, server 15.2 (Debian 15.2-1.pgdg110+1))
Type "help" for help.

postgres=# CREATE DATABASE nautilus;
CREATE DATABASE

```

## Nautilus CLI开发者指南

## 介绍

Nautilus CLI是用于与NautilusTrader生态系统交互的命令行界面工具。
它提供用于管理PostgreSQL数据库和处理各种交易操作的命令。

:::note
Nautilus CLI命令仅在类UNIX系统上受支持。
:::

## 安装

您可以使用下面的Makefile目标安装Nautilus CLI，它在底层利用`cargo install`。
这将把nautilus二进制文件放置在您系统的PATH中，假设Rust的`cargo`已正确配置。

```bash
make install-cli
```

## 命令

您可以运行`nautilus --help`来查看CLI结构和可用的命令组：

### 数据库

这些命令处理PostgreSQL数据库的引导。
要使用它们，您需要提供正确的连接配置，
通过命令行参数或位于根目录或当前工作目录中的`.env`文件。

- `--host`或`POSTGRES_HOST`用于数据库主机
- `--port`或`POSTGRES_PORT`用于数据库端口
- `--user`或`POSTGRES_USER`用于根管理员（通常是postgres用户）
- `--password`或`POSTGRES_PASSWORD`用于根管理员的密码
- `--database`或`POSTGRES_DATABASE`用于**数据库名称和具有该数据库权限的新用户**
    （例如，如果您提供`nautilus`作为值，将创建一个名为nautilus的新用户，密码来自`POSTGRES_PASSWORD`，并且`nautilus`数据库将使用此用户作为所有者进行引导）。

`.env`文件示例

```
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_USERNAME=postgres
POSTGRES_PASSWORD=pass
POSTGRES_DATABASE=nautilus
```

命令列表有：

1. `nautilus database init`：将引导模式、角色和位于`schema`根目录中的所有sql文件（如`tables.sql`）。
2. `nautilus database drop`：将删除目标Postgres数据库中的所有表、角色和数据。

## Rust analyzer设置

Rust analyzer是Rust的流行语言服务器，与许多IDE集成。建议配置rust analyzer具有与`make build-debug`相同的环境变量，以实现更快的编译时间。下面提供了VSCode和Astro Nvim的测试配置。有关更多信息，请参见[PR](https://github.com/nautechsystems/nautilus_trader/pull/2524)或rust analyzer [配置文档](https://rust-analyzer.github.io/book/configuration.html)。

### VSCode

您可以将以下设置添加到VSCode的`settings.json`文件中：

```
    "rust-analyzer.restartServerOnConfigChange": true,
    "rust-analyzer.linkedProjects": [
        "Cargo.toml"
    ],
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.check.workspace": false,
    "rust-analyzer.check.extraEnv": {
        "VIRTUAL_ENV": "<path-to-your-virtual-environment>/.venv",
        "CC": "clang",
        "CXX": "clang++"
    },
    "rust-analyzer.cargo.extraEnv": {
        "VIRTUAL_ENV": "<path-to-your-virtual-environment>/.venv",
        "CC": "clang",
        "CXX": "clang++"
    },
    "rust-analyzer.runnables.extraEnv": {
        "VIRTUAL_ENV": "<path-to-your-virtual-environment>/.venv",
        "CC": "clang",
        "CXX": "clang++"
    },
    "rust-analyzer.check.features": "all",
    "rust-analyzer.testExplorer": true
```

### Astro Nvim (Neovim + AstroLSP)

您可以将以下内容添加到您的astro lsp配置文件中：

```
    config = {
      rust_analyzer = {
        settings = {
          ["rust-analyzer"] = {
            restartServerOnConfigChange = true,
            linkedProjects = { "Cargo.toml" },
            cargo = {
              features = "all",
              extraEnv = {
                VIRTUAL_ENV = "<path-to-your-virtual-environment>/.venv",
                CC = "clang",
                CXX = "clang++",
              },
            },
            check = {
              workspace = false,
              command = "check",
              features = "all",
              extraEnv = {
                VIRTUAL_ENV = "<path-to-your-virtual-environment>/.venv",
                CC = "clang",
                CXX = "clang++",
              },
            },
            runnables = {
              extraEnv = {
                VIRTUAL_ENV = "<path-to-your-virtual-environment>/.venv",
                CC = "clang",
                CXX = "clang++",
              },
            },
            testExplorer = true,
          },
        },
      },
```
