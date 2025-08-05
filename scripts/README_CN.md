# Scripts 目录

该目录包含NautilusTrader开发工具和CI流水线使用的各种辅助脚本。其中只有一个脚本（`curate-dataset.sh`）需要简单说明，因为它需要在整理测试夹具数据集时手动执行。

---

## `curate-dataset.sh` – 为测试数据存储桶打包外部数据集

`curate-dataset.sh` 自动化了将第三方文件导入NautilusTrader *测试数据* 存储桶时所需的小而重复的任务：

- 从原始URL下载原始文件（带重试机制）
- 创建版本化目录（`v1/<slug>/`）
- 将文件复制到该目录中
- 编写包含SPDX标识符或许可证URL的`LICENSE.txt`文件
- 计算大小和SHA-256校验和并将其存储在`metadata.json`中

结果是一个独立的目录，可以一对一上传到S3存储桶（或者如果数据大小较小，可以提交到仓库中）。

### 用法

```bash
scripts/curate-dataset.sh <slug> <filename> <download-url> <licence>
```

- **`slug`** – 子目录名称（例如 `fi2010_all`）
- **`filename`** – 目录内你想要的基本文件名（例如 `Fi2010.zip`）
- **`download-url`** – 文件的原始公共URL
- **`licence`** – 短ID或完整URL（例如 `CC-BY-SA-4.0`）

示例 – 从Dropbox镜像整理完整的FI-2010限价订单簿数据集（所有10个交易日）：

```bash
scripts/curate-dataset.sh fi2010_all Fi2010.zip \
  "https://www.dropbox.com/s/6ywf3td7zdrp1n5/Fi2010.zip?dl=1" \
  CC-BY-SA-4.0
```

脚本完成后，你将拥有以下结构，可以提交或上传：

```
v1/fi2010_all/
 ├── Fi2010.zip          # ≈230 MB，包含 day_1 … day_10
 ├── LICENSE.txt         # CC-BY-SA-4.0
 └── metadata.json       # 大小、sha256、出处
```

你现在可以从测试或示例代码中引用`v1/fi2010_all/Fi2010.zip`，下游工具可以验证校验和。

### 注意事项

- 脚本使用`curl -L --fail --retry 3`，因此可以自动处理暂时的网络故障。
- 使用相同参数重新运行脚本会简单地覆盖现有文件 – 当上游文件更新且你想要更新校验和时很有用。
- 只执行基本验证；请确保你指定的许可证确实允许重新分发。

---

关于其他辅助脚本的详细信息，请使用`-h`参数运行它们或阅读内联注释；它们主要由CI调用，很少需要手动执行。 