---
 name: Bug 报告
 about: Bug – 与平台文档或预期设计相矛盾的行为
labels:
- bug
---

# Bug 报告

仅对符合**Bug**定义的问题使用此模板。

| 术语                          | 定义 |
|-------------------------------|------------|
| **Bug**                       | 与平台的文档或预期设计（按代码、文档或规范）相矛盾的行为。（即，实现是不正确的。） |
| **期望不匹配** | 遵循平台文档或预期设计但与您期望不同的行为。（即，设计/规范可能是问题所在。） |
| **增强请求**       | 对现有设计未暗示的新功能或行为的请求。（即，*"如果平台能够..."*就太好了） |

**注意：**

- 提交此问题会自动应用 `bug` 标签。
- 带有 `bug` 标签的问题会被高优先级分类，因为它们需要纠正性实现工作。
- **期望不匹配**和设计级问题应该作为[讨论](https://github.com/nautechsystems/nautilus_trader/discussions)或 RFC 打开，在那里可以在安排任何工作之前验证和讨论达成共识。
- 缺少功能通常不是期望不匹配，应该作为增强请求提交。

## 确认

**在打开 bug 报告之前，请确认：**

- [ ] 我已经重新阅读了文档的相关部分。
- [ ] 我已经搜索了现有的问题和讨论以避免重复。
- [ ] 我已经审查或浏览了源代码（或示例）以确认该行为不是设计使然。
- [ ] 我已经使用最近的*开发*版本（`dev` develop 或 `a` nightly）测试了此问题，仍然可以重现。

检查最近的开发版本可以节省时间，因为问题可能已经被修复。
您可以通过运行以下命令安装开发版本：

```bash
pip install -U nautilus_trader --pre --index-url https://packages.nautechsystems.io/simple
```

有关更多详细信息，请参阅[开发版本](https://github.com/nautechsystems/nautilus_trader#development-wheels)部分。

### 预期行为

在此添加...

### 实际行为

在此添加...

### 重现问题的步骤

1.
2.
3.

### 代码片段或日志

<!-- 如果适用，请提供相关的代码片段、错误日志或堆栈跟踪。使用代码块以确保清晰度。 -->

<!-- 考虑从我们的最小可重现示例模板开始： -->
<!-- https://github.com/nautechsystems/nautilus_trader/tree/develop/examples/other/minimal_reproducible_example -->

### 规格

- 操作系统平台：
- Python 版本：
- `nautilus_trader` 版本：
