# 教程

教程提供了一系列全面的分步指导学习体验。
每个教程针对特定功能或工作流程，实现动手学习。
从基本任务到更高级的操作，这些教程适合各种技能水平。

:::info
每个教程都是从位于docs [tutorials目录](https://github.com/nautechsystems/nautilus_trader/tree/develop/docs/tutorials)中的Jupyter笔记本生成的。这些笔记本作为有价值的学习辅助工具，让您可以交互式地执行代码。
:::

:::tip

- 确保您使用的教程文档与您的NautilusTrader版本匹配：
- **最新版本**：这些文档从`master`分支的HEAD构建，适用于最新稳定版本。请参阅<https://nautilustrader.io/docs/latest/tutorials/>。
- **夜间版本**：这些文档从`nightly`分支的HEAD构建，适用于前沿和实验性功能。请参阅<https://nautilustrader.io/docs/nightly/tutorials/>。

:::

## 在Docker中运行

或者，可以下载一个自包含的Docker化Jupyter笔记本服务器，无需设置或安装。这是启动并运行以试用NautilusTrader的最快方法。请注意，删除容器也会删除任何数据。

- 首先，安装docker：
  - 转到[Docker安装指南](https://docs.docker.com/get-docker/)并按照说明操作。
- 从终端下载最新镜像：
  - `docker pull ghcr.io/nautechsystems/jupyterlab:nightly --platform linux/amd64`
- 运行docker容器，暴露Jupyter端口：
  - `docker run -p 8888:8888 ghcr.io/nautechsystems/jupyterlab:nightly`
- 当容器启动时，终端中会打印带有访问令牌的URL。复制该URL并在浏览器中打开，例如：
  - <http://localhost:8888>

:::info
NautilusTrader目前超过了Jupyter笔记本日志记录（stdout输出）的速率限制，因此我们在示例中将`log_level`设置为`ERROR`。降低此级别以查看更多日志记录会导致笔记本在单元格执行期间挂起。我们目前正在调查一个修复方案，该方案涉及提高Jupyter的配置速率限制，或者限制Nautilus的日志刷新。

- <https://github.com/jupyterlab/jupyterlab/issues/12845>
- <https://github.com/deshaw/jupyterlab-limit-output>

:::
