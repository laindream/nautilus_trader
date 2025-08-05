# Greeks 计算器与 Actor 系统集成

本文档说明如何在 Nautilus actor 系统中使用 `GreeksCalculator`。

## 概述

`GreeksCalculator` 是一个用于计算期权和期货希腊字母（价格变动相对于市场数据变动的敏感性）的工具。它已与 actor 系统集成，可以在 actor（包括策略）中轻松使用。

## 核心组件

1. **Clock**：`GreeksCalculator` 使用与 actor 系统相同的 `Clock` 实例。
2. **消息总线**：`GreeksCalculator` 使用消息交换板来发布和订阅消息。

## 在 Actor 中使用 GreeksCalculator

### 基本设置

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use nautilus_common::{
    actor::{
        data_actor::{DataActor, DataActorConfig, DataActorCore},
        Actor,
    },
    cache::Cache,
    clock::LiveClock,
    greeks::GreeksCalculator,
    msgbus::MessagingSwitchboard,
};

struct MyActor {
    core: DataActorCore,
    greeks_calculator: GreeksCalculator,
}

impl MyActor {
    pub fn new(
        config: DataActorConfig,
        cache: Rc<RefCell<Cache>>,
        clock: Rc<RefCell<LiveClock>>,
        switchboard: Arc<MessagingSwitchboard>,
    ) -> Self {
        let core = DataActorCore::new(config, cache.clone(), clock.clone(), switchboard.clone());

        // 使用相同的时钟和缓存创建 GreeksCalculator
        let greeks_calculator = GreeksCalculator::new(
            cache,
            clock,
        );

        Self {
            core,
            greeks_calculator,
        }
    }
}
```

### 计算希腊字母

```rust
use nautilus_model::{
    data::greeks::GreeksData,
    identifiers::InstrumentId,
};

impl MyActor {
    pub fn calculate_greeks(&self, instrument_id: InstrumentId) -> anyhow::Result<GreeksData> {
        // 示例参数
        let flat_interest_rate = 0.0425;
        let flat_dividend_yield = None;
        let spot_shock = 0.0;
        let vol_shock = 0.0;
        let time_to_expiry_shock = 0.0;
        let use_cached_greeks = false;
        let cache_greeks = true;
        let publish_greeks = true;
        let ts_event = self.core.clock.borrow().timestamp_ns();
        let position = None;
        let percent_greeks = false;
        let index_instrument_id = None;
        let beta_weights = None;

        // 计算希腊字母
        self.greeks_calculator.instrument_greeks(
            instrument_id,
            Some(flat_interest_rate),
            flat_dividend_yield,
            Some(spot_shock),
            Some(vol_shock),
            Some(time_to_expiry_shock),
            Some(use_cached_greeks),
            Some(cache_greeks),
            Some(publish_greeks),
            Some(ts_event),
            position,
            Some(percent_greeks),
            index_instrument_id,
            beta_weights,
        )
    }
}
```

### 订阅希腊字母数据

```rust
impl MyActor {
    pub fn subscribe_to_greeks(&self, underlying: &str) {
        // 订阅希腊字母数据
        self.greeks_calculator.subscribe_greeks(underlying, None);
    }
}

impl DataActor for MyActor {
    fn on_start(&mut self) -> anyhow::Result<()> {
        // 订阅 SPY 的希腊字母数据
        self.subscribe_to_greeks("SPY");
        Ok(())
    }

    fn on_data(&mut self, data: &dyn std::any::Any) -> anyhow::Result<()> {
        // 处理接收到的数据
        if let Some(greeks_data) = data.downcast_ref::<GreeksData>() {
            println!("接收到希腊字母数据: {:?}", greeks_data);
        }
        Ok(())
    }
}
```

## 完整示例

请参阅 `crates/common/examples/greeks_actor_example.rs` 中的完整示例以获得工作实现。

## 主要功能

1. **与 Actor 系统集成**：`GreeksCalculator` 使用与 actor 系统相同的时钟和消息总线。
2. **消息总线集成**：希腊字母数据可以通过消息总线发布和订阅。
3. **缓存**：希腊字母计算可以被缓存以提高性能。
4. **投资组合希腊字母**：计算整个持仓组合的希腊字母。

## 注意事项

- 当将 `publish_greeks` 设置为 `true` 时，计算器将把希腊字母数据发布到消息总线，主题格式为 `data.GreeksData.instrument_id={symbol}`。
- 订阅希腊字母数据时，您可以提供自定义处理程序或使用默认处理程序，默认处理程序会缓存接收到的希腊字母数据。
