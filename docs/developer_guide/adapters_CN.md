# 适配器

## 介绍

本开发者指南提供了如何为NautilusTrader平台开发集成适配器的说明。
适配器提供与交易场所和数据提供商的连接——将原始场所API转换为Nautilus的统一接口和标准化领域模型。

## 适配器的结构

适配器通常由几个组件组成：

1. **工具提供商**：提供工具定义。
2. **数据客户端**：处理实时市场数据源和历史数据请求。
3. **执行客户端**：处理订单执行和管理。
4. **配置**：配置客户端设置。

## 适配器实现步骤

1. 为您的适配器创建一个新的Python子包。
2. 通过继承`InstrumentProvider`并实现加载工具的必要方法来实现工具提供商。
3. 通过继承适用的`LiveDataClient`和`LiveMarketDataClient`类，并为所需方法提供实现来实现数据客户端。
4. 通过继承`LiveExecutionClient`并为所需方法提供实现来实现执行客户端。
5. 创建配置类来保存您的适配器设置。
6. 彻底测试您的适配器以确保所有方法都正确实现且适配器按预期工作（参见[测试指南](testing.md)）。

## 构建适配器的模板

以下是使用提供的模板为新数据提供商构建适配器的逐步指南。

### 工具提供商

`InstrumentProvider`提供场所上可用的工具定义。这包括加载所有可用工具、按ID加载特定工具，以及对工具列表应用过滤器。

```python
from nautilus_trader.common.providers import InstrumentProvider
from nautilus_trader.model import InstrumentId

class TemplateInstrumentProvider(InstrumentProvider):
    """
    一个``InstrumentProvider``的示例模板，显示集成完成所必须实现的最小方法。
    """

    async def load_all_async(self, filters: dict | None = None) -> None:
        raise NotImplementedError("在您的适配器子类中实现`load_all_async`")

    async def load_ids_async(self, instrument_ids: list[InstrumentId], filters: dict | None = None) -> None:
        raise NotImplementedError("在您的适配器子类中实现`load_ids_async`")

    async def load_async(self, instrument_id: InstrumentId, filters: dict | None = None) -> None:
        raise NotImplementedError("在您的适配器子类中实现`load_async`")
```

**关键方法**：

- `load_all_async`：异步加载所有工具，可选择性地应用过滤器
- `load_ids_async`：通过其ID加载特定工具
- `load_async`：通过其ID加载单个工具

### 数据客户端

`LiveDataClient`处理与市场数据不直接相关的数据源的订阅和管理。这可能包括新闻源、自定义数据流或增强交易策略但不直接代表市场活动的其他数据源。

```python
from nautilus_trader.live.data_client import LiveDataClient
from nautilus_trader.model import DataType
from nautilus_trader.core import UUID4

class TemplateLiveDataClient(LiveDataClient):
    """
    ``LiveDataClient``的示例，突出显示可重写的抽象方法。
    """

    async def _connect(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_connect`")

    async def _disconnect(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_disconnect`")

    def reset(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`reset`")

    def dispose(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`dispose`")

    async def _subscribe(self, data_type: DataType) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_subscribe`")

    async def _unsubscribe(self, data_type: DataType) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_unsubscribe`")

    async def _request(self, data_type: DataType, correlation_id: UUID4) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_request`")
```

**关键方法**：

- `_connect`：建立与数据提供商的连接。
- `_disconnect`：关闭与数据提供商的连接。
- `reset`：重置客户端的状态。
- `dispose`：释放客户端持有的任何资源。
- `_subscribe`：订阅特定数据类型。
- `_unsubscribe`：取消订阅特定数据类型。
- `_request`：从提供商请求数据。

### 市场数据客户端

`MarketDataClient`处理市场特定数据，如订单簿、最佳报价和交易，以及工具状态更新。它专注于提供对交易操作至关重要的历史和实时市场数据。

```python
from nautilus_trader.live.data_client import LiveMarketDataClient
from nautilus_trader.model import BarType, DataType
from nautilus_trader.model import InstrumentId
from nautilus_trader.model.enums import BookType

class TemplateLiveMarketDataClient(LiveMarketDataClient):
    """
    ``LiveMarketDataClient``的示例，突出显示可重写的抽象方法。
    """

    async def _connect(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_connect`")

    async def _disconnect(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_disconnect`")

    def reset(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`reset`")

    def dispose(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`dispose`")

    async def _subscribe_instruments(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_subscribe_instruments`")

    async def _unsubscribe_instruments(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_unsubscribe_instruments`")

    async def _subscribe_order_book_deltas(self, instrument_id: InstrumentId, book_type: BookType, depth: int | None = None, kwargs: dict | None = None) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_subscribe_order_book_deltas`")

    async def _unsubscribe_order_book_deltas(self, instrument_id: InstrumentId) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_unsubscribe_order_book_deltas`")
```

**关键方法**：

- `_connect`：建立与场所API的连接。
- `_disconnect`：关闭与场所API的连接。
- `reset`：重置客户端的状态。
- `dispose`：释放客户端持有的任何资源。
- `_subscribe_instruments`：订阅多个工具的市场数据。
- `_unsubscribe_instruments`：取消订阅多个工具的市场数据。
- `_subscribe_order_book_deltas`：订阅订单簿增量更新。
- `_unsubscribe_order_book_deltas`：取消订阅订单簿增量更新。

---

## REST API字段映射指南

当将场所的REST载荷转换为我们的领域模型时，**避免重命名**上游字段，除非有令人信服的理由（例如与保留关键字冲突）。我们默认应用的唯一转换是**camelCase → snake_case**。

保持外部名称不变使得调试载荷、将捕获与Rust结构进行比较变得轻而易举，并加快了对并排打开场所API参考的新贡献者的入门速度。

### 执行客户端

`ExecutionClient`负责订单管理，包括订单的提交、修改和取消。它是适配器的关键组件，与场所交易系统交互以管理和执行交易。

```python
from nautilus_trader.execution.messages import BatchCancelOrders, CancelAllOrders, CancelOrder, ModifyOrder, SubmitOrder
from nautilus_trader.execution.reports import FillReport, OrderStatusReport, PositionStatusReport
from nautilus_trader.live.execution_client import LiveExecutionClient
from nautilus_trader.model import ClientOrderId, InstrumentId, VenueOrderId

class TemplateLiveExecutionClient(LiveExecutionClient):
    """
    ``LiveExecutionClient``的示例，突出显示方法要求。
    """

    async def _connect(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_connect`")

    async def _disconnect(self) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_disconnect`")

    async def _submit_order(self, command: SubmitOrder) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_submit_order`")

    async def _modify_order(self, command: ModifyOrder) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_modify_order`")

    async def _cancel_order(self, command: CancelOrder) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_cancel_order`")

    async def _cancel_all_orders(self, command: CancelAllOrders) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_cancel_all_orders`")

    async def _batch_cancel_orders(self, command: BatchCancelOrders) -> None:
        raise NotImplementedError("在您的适配器子类中实现`_batch_cancel_orders`")

    async def generate_order_status_report(
        self, instrument_id: InstrumentId, client_order_id: ClientOrderId | None = None, venue_order_id: VenueOrderId | None = None
    ) -> OrderStatusReport | None:
        raise NotImplementedError("方法`generate_order_status_report`必须在子类中实现")

    async def generate_order_status_reports(
        self, instrument_id: InstrumentId | None = None, start: pd.Timestamp | None = None, end: pd.Timestamp | None = None, open_only: bool = False
    ) -> list[OrderStatusReport]:
        raise NotImplementedError("方法`generate_order_status_reports`必须在子类中实现")

    async def generate_fill_reports(
        self, instrument_id: InstrumentId | None = None, venue_order_id: VenueOrderId | None = None, start: pd.Timestamp | None = None, end: pd.Timestamp | None = None
    ) -> list[FillReport]:
        raise NotImplementedError("方法`generate_fill_reports`必须在子类中实现")

    async def generate_position_status_reports(
        self, instrument_id: InstrumentId | None = None, start: pd.Timestamp | None = None, end: pd.Timestamp | None = None
    ) -> list[PositionStatusReport]:
        raise NotImplementedError("方法`generate_position_status_reports`必须在子类中实现")
```

**关键方法**：

- `_connect`：建立与场所API的连接。
- `_disconnect`：关闭与场所API的连接。
- `_submit_order`：向场所提交新订单。
- `_modify_order`：修改场所上的现有订单。
- `_cancel_order`：取消场所上的特定订单。
- `_cancel_all_orders`：取消场所上工具的所有订单。
- `_batch_cancel_orders`：批量取消场所上工具的订单。
- `generate_order_status_report`：为场所上的特定订单生成报告。
- `generate_order_status_reports`：为场所上的所有订单生成报告。
- `generate_fill_reports`：为场所上的已成交订单生成报告。
- `generate_position_status_reports`：为场所上的仓位状态生成报告。

### 配置

配置文件定义适配器特定的设置，如API密钥和连接详细信息。这些设置对于初始化和管理适配器与数据提供商的连接至关重要。

```python
from nautilus_trader.config import LiveDataClientConfig, LiveExecClientConfig

class TemplateDataClientConfig(LiveDataClientConfig):
    """
    ``TemplateDataClient``实例的配置。
    """

    api_key: str
    api_secret: str
    base_url: str

class TemplateExecClientConfig(LiveExecClientConfig):
    """
    ``TemplateExecClient``实例的配置。
    """

    api_key: str
    api_secret: str
    base_url: str
```

**关键属性**：

- `api_key`：用于与数据提供商进行身份验证的API密钥。
- `api_secret`：用于与数据提供商进行身份验证的API密钥。
- `base_url`：连接到数据提供商API的基本URL。
