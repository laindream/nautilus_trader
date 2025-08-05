# 基础设施集成测试

此目录包含需要外部服务的基础设施集成测试。

## 服务要求
所有必需的服务都在 `.docker/docker-compose.yml` 中定义。

集成测试需要以下服务运行：

- PostgreSQL 在 `localhost:5432`
- Redis 在 `localhost:6379`

### 服务配置

- **PostgreSQL**: 用户名 `nautilus`，密码 `pass`，数据库 `nautilus`
- **Redis**: 默认配置，无身份验证
- **PgAdmin**（可选）: 可在 `http://localhost:5051` 访问（<admin@mail.com> / admin）

## 运行集成测试服务
使用以下 make 目标来管理服务：

### 初始设置

```bash
make init-services  # 启动容器并初始化数据库模式
```

### 管理服务

```bash
make stop-services   # 停止开发服务（保留数据）
make start-services  # 启动开发服务（不重新初始化数据库）
make purge-services  # 删除所有内容，包括数据卷
```

### 典型工作流程

1. 第一次：`make init-services`
2. 完成后停止：`make stop-services`
3. 恢复工作：`make start-services`
4. 重新开始：`make purge-services` 然后 `make init-services`

## 运行测试
一旦服务运行（并且 Nautilus Trader 已通过 `uv` 或 `make` 安装）：

### Python 基础设施集成测试

```bash
# 运行所有基础设施测试
uv run --no-sync pytest tests/integration_tests/infrastructure/

# 运行特定测试文件
uv run --no-sync pytest tests/integration_tests/infrastructure/test_cache_database_redis.py
uv run --no-sync pytest tests/integration_tests/infrastructure/test_cache_database_postgres.py
```

### Rust 基础设施集成测试
Rust 集成测试位于 `crates/infrastructure/tests/` 中，需要相同的服务。

```bash
# 运行所有 Rust 集成测试（包括 Redis 和 PostgreSQL 测试）
make cargo-test-crate-nautilus-infrastructure

# 直接使用 cargo nextest 和标准配置文件
# 运行所有基础设施测试，显示输出以便调试
cargo nextest run --lib --no-fail-fast --cargo-profile nextest -p nautilus-infrastructure --features redis,postgres --no-capture

# 仅运行 Redis 集成测试
cargo nextest run --lib --no-fail-fast --cargo-profile nextest -p nautilus-infrastructure --features redis,postgres -E 'test(test_cache_redis)'

# 仅运行 PostgreSQL 集成测试
cargo nextest run --lib --no-fail-fast --cargo-profile nextest -p nautilus-infrastructure --features redis,postgres -E 'test(test_cache_postgres) or test(test_cache_database_postgres)'

```

**注意**: 在给定示例中同时使用 redis 和 postgres 功能标志以避免重新构建。
Rust 基础设施集成测试标记为 `#[cfg(target_os = "linux")]`，只会在 Linux 上运行。
它们使用 `serial_test` 包来确保访问相同数据库的测试不会并发运行。 