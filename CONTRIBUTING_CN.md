# 为 NautilusTrader 贡献

我们高度重视交易社区的参与，非常感谢所有贡献，它们帮助我们持续改进 NautilusTrader！

## 步骤

要贡献代码，请遵循以下步骤：

1. 在 GitHub 上开启一个 issue 来讨论您建议的更改或增强功能。

2. 一旦大家达成一致，fork `develop` 分支，并通过定期合并任何上游更改来确保您的 fork 是最新的。

3. 在您的本地机器上安装和配置 [pre-commit](https://pre-commit.com/)，以在每次提交前自动运行代码检查、格式化程序和代码检查器。
   您可以通过以下命令安装 pre-commit：

    ```bash
    pip install pre-commit
    pre-commit install
    ```

4. 在 `develop` 分支上打开一个拉取请求（PR），并提供摘要评论和对任何相关 GitHub issue 的引用。

5. CI 系统将在您的代码上运行完整的测试套件，包括所有单元测试和集成测试，因此请在 PR 中包含适当的测试。

6. 阅读并理解贡献者许可协议（CLA），可在 <https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md> 获取。

7. 您还需要签署 CLA，这通过 [CLA Assistant](https://cla-assistant.io/) 自动管理。

8. 我们将尽快审查您的代码，并在合并前提供反馈（如果需要任何更改）。

## 提示

- 遵循 [开发者指南](https://nautilustrader.io/docs/developer_guide/index.html) 中建立的编码实践。
- 对于文档更改，请遵循 `docs/developer_guide/docs.md` 中的样式指南（H2 及以下标题使用句子大小写）。
- 保持 PR 小而专注，以便于审查。
- 在您的 PR 评论中引用相关的 GitHub issue。
