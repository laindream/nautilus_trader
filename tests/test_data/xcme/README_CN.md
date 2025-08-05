# 文件：`6EH4.XCME_1min_bars_20240101_20240131.csv.gz`

- 金融工具：6E
- 到期日：H4（2024年3月）
- 交易所：XCME（MIC代码）
- 时间段：2024-01-01 --> 2024-01-31（UTC时间戳，此期间内没有合约展期）
- 柱状图类型：1分钟柱状图

# 压缩格式

我们使用了压缩数据，因为它们比原始CSV文件小9倍，并且可以直接被 [pandas](https://pandas.pydata.org/) 读取，
使用如下代码：

```python
import pandas as pd

df = pd.read_csv(
    "6EH4.XCME_1min_bars_20240101_20240131.csv.gz",  # 根据需要更新路径
    header=0,
    index_col=False,
)
```
