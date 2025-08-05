# 打包数据

各种数据内部包含在`tests/test_kit/data`文件夹中。

## Libor利率

1个月USD的libor利率可以通过从<https://fred.stlouisfed.org/series/USD1MTD156N>下载CSV数据来更新。

确保为时间窗口选择`Max`。

## 短期利率

银行间短期利率可以通过在<https://data.oecd.org/interest/short-term-interest-rates.htm>下载CSV数据来更新。

## 经济事件

经济事件可以通过从fxstreet <https://www.fxstreet.com/economic-calendar>下载CSV数据来更新。

确保时区设置为GMT。

最多可以过滤3个月的范围，因此必须手动下载年度季度并将其拼接到单个CSV中。
使用日历图标按以下方式过滤数据；

- 01/01/xx - 31/03/xx
- 01/04/xx - 30/06/xx
- 01/07/xx - 30/09/xx
- 01/10/xx - 31/12/xx

下载每个CSV。 