<!--
  .github 目录的 README：组合操作和工作流定义。
-->
# GitHub Actions 概述

此目录包含用于 NautilusTrader 仓库中 CI/CD、测试、发布和自动化的可重用组合操作和工作流定义。

## 组合操作 (`.github/actions`)

- **common-setup**: 准备环境（操作系统包、Rust 工具链、Python、sccache、pre-commit）。
- **common-test-data**: 缓存 `tests/test_data/large` 下的大型测试数据。
- **common-wheel-build**: 在 Linux、macOS 和 Windows 上为多个 Python 版本构建和安装 Python wheels。
- **publish-wheels**: 将构建的 wheels 发布到 Cloudflare R2，管理旧 wheel 清理和索引生成。
- **upload-artifact-wheel**: 将最新的 wheel 工件上传到 GitHub Actions。

## 工作流 (`.github/workflows`)

- **build.yml**: 运行 pre-commit、Rust 测试、Python 测试，在多个平台上构建 wheels，并上传 wheel 工件。
- **build-docs.yml**: 分发仓库事件以在 `master` 和 `nightly` 推送时触发文档构建。
- **codeql-analysis.yml**: 在拉取请求上调度和运行 CodeQL 安全扫描，并通过 cron 定期运行。
- **coverage.yml**: （可选）为 `nightly` 分支生成覆盖率报告。
- **docker.yml**: 使用 Buildx 和 QEMU 为 `master` 和 `nightly` 分支构建和推送 Docker 镜像（`nautilus_trader`、`jupyterlab`）。
- **nightly-merge.yml**: 当最新的 `develop` 工作流成功时，自动将 `develop` 合并到 `nightly`。
- **performance.yml**: 在 `nightly` 分支上运行 Rust/Python 性能基准测试并报告给 CodSpeed。

## 安全性

- **不可变操作固定**: 所有第三方操作都固定到特定的提交 SHA，以保证不可变性和可重现性。
- **加固运行器**: 大多数工作流采用 `step-security/harden-runner` 和 `egress-policy: audit` 来减少攻击面并监控出站流量。
- **密钥管理**: 仓库中不存储任何密钥或凭据。AWS、PyPI 和其他凭据通过 GitHub Secrets 提供并在运行时注入。
- **代码扫描**: 启用 CodeQL 进行持续安全分析。
- **依赖固定**: 关键工具（pre-commit、Python 版本、Rust 工具链、cargo-nextest）锁定到固定版本或 SHA。
- **最小权限令牌**: 工作流将 `GITHUB_TOKEN` 默认为
  `contents: read, actions: read` 并选择性地提升范围（例如
  `contents: write`）仅用于需要标记发布或上传
  资产的作业。这遵循最小权限原则并限制
  作业受到威胁时的影响范围。
- **缓存**: sccache、pip/site-packages、pre-commit 和测试数据的缓存加速工作流，同时保持封闭式构建。

对于操作或工作流的更新或更改，请遵守仓库的
CONTRIBUTING 指南并维护这些安全最佳实践。
