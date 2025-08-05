# 投资组合

:::info
我们目前正在编写这个概念指南。
:::

投资组合作为管理和跟踪交易节点或回测中所有活跃策略头寸的中央枢纽。它整合来自多个工具的头寸数据，提供您持仓、风险敞口和整体表现的统一视图。探索本节以了解NautilusTrader如何聚合和更新投资组合状态，以支持有效的交易和风险管理。

## 投资组合统计

有各种[内置投资组合统计](https://github.com/nautechsystems/nautilus_trader/tree/develop/nautilus_trader/analysis/statistics)用于分析回测和实时交易的交易投资组合表现。

统计通常分类如下：

- 基于PnL的统计（按货币）
- 基于收益的统计
- 基于头寸的统计
- 基于订单的统计

也可以调用交易者的`PortfolioAnalyzer`并在任意时间计算统计，包括*在*回测或实时交易会话期间。

## 自定义统计

可以通过继承`PortfolioStatistic`基类并实现任何`calculate_`方法来定义自定义投资组合统计。

例如，以下是内置`WinRate`统计的实现：

```python
import pandas as pd
from typing import Any
from nautilus_trader.analysis.statistic import PortfolioStatistic


class WinRate(PortfolioStatistic):
    """
    从已实现PnL系列计算胜率。
    """

    def calculate_from_realized_pnls(self, realized_pnls: pd.Series) -> Any | None:
        # 前置条件
        if realized_pnls is None or realized_pnls.empty:
            return 0.0

        # 计算统计
        winners = [x for x in realized_pnls if x > 0.0]
        losers = [x for x in realized_pnls if x <= 0.0]

        return len(winners) / float(max(1, (len(winners) + len(losers))))
```

然后可以将这些统计注册到交易者的`PortfolioAnalyzer`中。

```python
stat = WinRate()

# 注册到投资组合分析器
engine.portfolio.analyzer.register_statistic(stat)

:::info
有关可用方法的详细信息，请参阅`PortfolioAnalyzer` [API参考](../api_reference/analysis_CN.md#class-portfolioanalyzer)。
:::
```

:::tip
确保您的统计对于退化输入（如`None`、空系列或数据不足）是稳健的。

期望是您返回`None`、NaN或合理的默认值。
:::

## 回测分析

在回测运行后，将通过将已实现PnL、收益、头寸和订单数据依次传递给每个注册的统计来进行性能分析，计算其值（使用默认配置）。然后任何输出都会显示在`投资组合表现`标题下的撕页中，分组为：

- 已实现PnL统计（按货币）
- 收益统计（整个投资组合）
- 从头寸和订单数据派生的一般统计（整个投资组合） 