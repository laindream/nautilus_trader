# Rust风格指南

[Rust](https://www.rust-lang.org/learn)编程语言是实现平台和系统关键核心的理想选择。其强类型系统、所有权模型和编译时检查在构造上消除了内存错误和数据竞争，同时零成本抽象和没有垃圾收集器提供了类C的性能——这对高频交易工作负载至关重要。

## Python绑定

通过Cython和[PyO3](https://pyo3.rs)提供Python绑定，允许用户直接在Python中导入NautilusTrader crate，无需Rust工具链。

## 代码风格和约定

### 文件头要求

所有Rust文件必须包含标准化的版权头：

```rust
// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------
```

### 代码格式化

运行`make format`时，导入格式由rustfmt自动处理。
该工具将导入组织成组（标准库、外部crate、本地导入）并在每组内按字母顺序排序。

#### 函数间距

- 在函数间（包括测试）**留一个空行**——这提高了可读性并反映了`rustfmt`的默认行为。
- 在每个文档注释（`///`或`//!`）上方**留一个空行**，使注释明显与前面的代码块分离。

#### 字符串格式化

优先使用内联格式字符串而不是位置参数：

```rust
// 推荐 - 带变量名的内联格式
anyhow::bail!("Failed to subtract {n} months from {datetime}");

// 替代 - 位置参数
anyhow::bail!("Failed to subtract {} months from {}", n, datetime);
```

这使消息更可读且自文档化，特别是当有多个变量时。

#### PyO3命名约定

当**通过PyO3**向Python暴露Rust函数时：

1. Rust符号**必须**以`py_*`为前缀，以在Rust代码库中明确其目的。
2. 使用`#[pyo3(name = "…")]`属性发布**不带**`py_`前缀的*Python*名称，以保持Python API的清洁。

```rust
#[pyo3(name = "do_something")]
pub fn py_do_something() -> PyResult<()> {
    // …
}
```

### 错误处理

一致地使用结构化错误处理模式：

1. **主要模式**：对可能失败的函数使用`anyhow::Result<T>`：

   ```rust
   pub fn calculate_balance(&mut self) -> anyhow::Result<Money> {
       // 实现
   }
   ```

2. **自定义错误类型**：对特定领域错误使用`thiserror`：

   ```rust
   #[derive(Error, Debug)]
   pub enum NetworkError {
       #[error("Connection failed: {0}")]
       ConnectionFailed(String),
       #[error("Timeout occurred")]
       Timeout,
   }
   ```

3. **错误传播**：使用`?`操作符进行清洁的错误传播。

4. **错误创建**：优先使用`anyhow::bail!`进行带错误的早期返回：

   ```rust
   // 推荐 - 使用bail!进行早期返回
   pub fn process_value(value: i32) -> anyhow::Result<i32> {
       if value < 0 {
           anyhow::bail!("Value cannot be negative: {value}");
       }
       Ok(value * 2)
   }

   // 替代 - 冗长的返回语句
   if value < 0 {
       return Err(anyhow::anyhow!("Value cannot be negative: {value}"));
   }
   ```

   **注意**：在闭包上下文中，当早期返回不可能时，使用`anyhow::bail!`进行早期返回，但使用`anyhow::anyhow!`，如`ok_or_else()`。

### 属性模式

一致的属性使用和排序：

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.model")
)]
pub struct Symbol(Ustr);
```

对于具有广泛derive属性的枚举：

```rust
#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    AsRefStr,
    FromRepr,
    EnumIter,
    EnumString,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.model.enums")
)]
pub enum AccountType {
    /// 仅包含无杠杆现金资产的账户。
    Cash = 1,
    /// 便于使用账户资产作为抵押品进行保证金交易的账户。
    Margin = 2,
}
```

### 构造函数模式

一致地使用`new()`与`new_checked()`约定：

```rust
/// 使用正确性检查创建新的[`Symbol`]实例。
///
/// # 错误
///
/// 如果`value`不是有效字符串，则返回错误。
///
/// # 注释
///
/// PyO3需要`Result`类型以在Python中正确处理错误和堆栈跟踪打印。
pub fn new_checked<T: AsRef<str>>(value: T) -> anyhow::Result<Self> {
    // 实现
}

/// 创建新的[`Symbol`]实例。
///
/// # Panics
///
/// 如果`value`不是有效字符串则panic。
pub fn new<T: AsRef<str>>(value: T) -> Self {
    Self::new_checked(value).expect(FAILED)
}
```

始终对与正确性检查相关的`.expect()`消息使用`FAILED`常量：

```rust
use nautilus_core::correctness::FAILED;
```

### 常量和命名约定

对常量使用SCREAMING_SNAKE_CASE并使用描述性名称：

```rust
/// 一秒中的纳秒数。
pub const NANOSECONDS_IN_SECOND: u64 = 1_000_000_000;

/// 1分钟最新价格bar的Bar规范。
pub const BAR_SPEC_1_MINUTE_LAST: BarSpecification = BarSpecification {
    step: NonZero::new(1).unwrap(),
    aggregation: BarAggregation::Minute,
    price_type: PriceType::Last,
};
```

### 重新导出模式

按字母顺序组织重新导出并放在lib.rs文件的末尾：

```rust
// 重新导出
pub use crate::{
    nanos::UnixNanos,
    time::AtomicTime,
    uuid::UUID4,
};

// 模块级重新导出
pub use crate::identifiers::{
    account_id::AccountId,
    actor_id::ActorId,
    client_id::ClientId,
};
```

### 文档标准

#### 模块级文档

所有模块必须有以简要描述开头的模块级文档：

```rust
//! 类似于*按合同设计*哲学的正确性检查函数。
//!
//! 此模块提供函数或方法条件的验证检查。
//!
//! 条件是在执行某些代码段之前必须为真的谓词——
//! 用于根据设计规范的正确行为。
```

对于具有特性标志的模块，清楚地记录它们：

```rust
//! # 特性标志
//!
//! 此crate提供特性标志以在编译期间控制源代码包含，
//! 取决于预期的用例：
//!
//! - `ffi`：启用来自[cbindgen](https://github.com/mozilla/cbindgen)的C外部函数接口(FFI)。
//! - `python`：启用来自[PyO3](https://pyo3.rs)的Python绑定。
//! - `stubs`：启用在测试场景中使用的类型存根。
```

#### 字段文档

所有结构体和枚举字段必须有带结束句号的文档：

```rust
pub struct Currency {
    /// 货币代码作为alpha-3字符串（例如，"USD"，"EUR"）。
    pub code: Ustr,
    /// 货币小数精度。
    pub precision: u8,
    /// ISO 4217货币代码。
    pub iso4217: u16,
    /// 货币的全名。
    pub name: Ustr,
    /// 货币类型，指示其类别（例如法币、加密货币）。
    pub currency_type: CurrencyType,
}
```

#### 函数文档

为所有公共函数文档包含：

- 目的和行为
- 输入参数使用的解释
- 错误条件（如果适用）
- Panic条件（如果适用）

```rust
/// 返回指定货币的`AccountBalance`引用，如果不存在则返回`None`。
///
/// # Panics
///
/// 如果`currency`为`None`且`self.base_currency`为`None`则panic。
pub fn base_balance(&self, currency: Option<Currency>) -> Option<&AccountBalance> {
    // 实现
}
```

#### 错误和Panic文档格式

对于单行错误和panic文档，使用句子大小写并遵循以下约定：

```rust
/// 返回指定货币的`AccountBalance`引用，如果不存在则返回`None`。
///
/// # 错误
///
/// 如果货币转换失败则返回错误。
///
/// # Panics
///
/// 如果`currency`为`None`且`self.base_currency`为`None`则panic。
pub fn base_balance(&self, currency: Option<Currency>) -> anyhow::Result<Option<&AccountBalance>> {
    // 实现
}
```

对于多行错误和panic文档，使用句子大小写并使用项目符号和结束句号：

```rust
/// 计算仓位的未实现盈亏。
///
/// # 错误
///
/// 如果出现以下情况，此函数将返回错误：
/// - 无法找到工具的市场价格。
/// - 转换率计算失败。
/// - 遇到无效的仓位状态。
///
/// # Panics
///
/// 如果出现以下情况，此函数将panic：
/// - 工具ID无效或未初始化。
/// - 缓存中缺少所需的市场数据。
/// - 内部状态一致性检查失败。
pub fn calculate_unrealized_pnl(&self, market_price: Price) -> anyhow::Result<Money> {
    // 实现
}
```

#### 安全文档格式

对于安全文档，使用`SAFETY:`前缀，后跟解释为什么不安全操作有效的简短描述：

```rust
/// 从原始组件创建新实例而不进行验证。
///
/// # 安全
///
/// 调用者必须确保所有输入参数都有效且正确初始化。
pub unsafe fn from_raw_parts(ptr: *const u8, len: usize) -> Self {
    // SAFETY: 调用者保证ptr有效且len正确
    Self {
        data: std::slice::from_raw_parts(ptr, len),
    }
}
```

对于内联不安全块，直接在不安全代码上方使用`SAFETY:`注释：

```rust
impl Send for MessageBus {
    fn send(&self) {
        // SAFETY: 消息总线不应在线程间传递
        unsafe {
            // 这里的不安全操作
        }
    }
}
```

### 测试约定

- 使用`mod tests`作为标准测试模块名，除非您需要特别区分。
- 一致地使用`#[rstest]`属性，这种标准化减少了认知开销。
- 在Rust测试中*不要*使用Arrange、Act、Assert分隔符注释。

#### 测试组织

使用一致的测试模块结构和节分隔符：

```rust
////////////////////////////////////////////////////////////////////////////////
// 测试
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;
    use crate::identifiers::{Symbol, stubs::*};

    #[rstest]
    fn test_string_reprs(symbol_eth_perp: Symbol) {
        assert_eq!(symbol_eth_perp.as_str(), "ETH-PERP");
        assert_eq!(format!("{symbol_eth_perp}"), "ETH-PERP");
    }
}
```

#### 参数化测试

一致地使用`rstest`属性，对于参数化测试：

```rust
#[rstest]
#[case("AUDUSD", false)]
#[case("AUD/USD", false)]
#[case("CL.FUT", true)]
fn test_symbol_is_composite(#[case] input: &str, #[case] expected: bool) {
    let symbol = Symbol::new(input);
    assert_eq!(symbol.is_composite(), expected);
}
```

#### 测试命名

使用解释场景的描述性测试名称：

```rust
fn test_sma_with_no_inputs()
fn test_sma_with_single_input()
fn test_symbol_is_composite()
```

## Rust-Python内存管理

在使用PyO3绑定时，理解并避免Rust的`Arc`引用计数和Python垃圾收集器之间的引用循环至关重要。
本节记录了在回调持有结构中处理Python对象的最佳实践。

### 引用循环问题

**问题**：在回调持有结构中使用`Arc<PyObject>`会创建循环引用：

1. **Rust `Arc`持有Python对象** → 增加Python引用计数。
2. **Python对象可能引用Rust对象** → 创建循环。
3. **两边都无法被垃圾收集** → 内存泄漏。

**有问题模式的示例**：

```rust
// 避免：这会创建引用循环
struct CallbackHolder {
    handler: Option<Arc<PyObject>>,  // ❌ Arc包装器导致循环
}
```

### 解决方案：基于GIL的克隆

**解决方案**：通过`clone_py_object()`使用带适当GIL基础克隆的普通`PyObject`：

```rust
use nautilus_core::python::clone_py_object;

// 正确：使用普通PyObject不带Arc包装器
struct CallbackHolder {
    handler: Option<PyObject>,  // ✅ 无Arc包装器
}

// 使用clone_py_object的手动Clone实现
impl Clone for CallbackHolder {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.as_ref().map(clone_py_object),
        }
    }
}
```

### 最佳实践

#### 1. 对Python对象克隆使用`clone_py_object()`

```rust
// 克隆Python回调时
let cloned_callback = clone_py_object(&original_callback);

// 在手动Clone实现中
self.py_handler.as_ref().map(clone_py_object)
```

#### 2. 从回调持有结构中移除`#[derive(Clone)]`

```rust
// 之前：自动derive在PyObject上引起问题
#[derive(Clone)]  // ❌ 移除这个
struct Config {
    handler: Option<PyObject>,
}

// 之后：带适当克隆的手动实现
struct Config {
    handler: Option<PyObject>,
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            // 正常克隆常规字段
            url: self.url.clone(),
            // 对Python对象使用clone_py_object
            handler: self.handler.as_ref().map(clone_py_object),
        }
    }
}
```

#### 3. 更新函数签名以接受`PyObject`

```rust
// 之前：函数签名中的Arc包装器
fn spawn_task(handler: Arc<PyObject>) { ... }  // ❌

// 之后：普通PyObject
fn spawn_task(handler: PyObject) { ... }  // ✅
```

#### 4. 创建Python回调时避免`Arc::new()`

```rust
// 之前：在Arc中包装
let callback = Arc::new(py_function);  // ❌

// 之后：直接使用
let callback = py_function;  // ✅
```

### 为什么这有效

`clone_py_object()`函数：

- **在执行克隆操作前获取Python GIL**。
- **通过`clone_ref()`使用Python的原生引用计数**。
- **避免干扰Python GC的Rust Arc包装器**。
- **通过适当的GIL管理维护线程安全**。

这种方法允许Rust和Python垃圾收集器正确工作，消除了引用循环的内存泄漏。

## 不安全Rust

为了能够在Cython和Rust之间实现互操作的价值，需要编写`unsafe` Rust代码。能够超出安全Rust边界的能力使得实现Rust语言本身的许多最基本特性成为可能，正如C和C++用于实现它们自己的标准库一样。

我们将非常小心地使用Rust的`unsafe`功能——它只启用一小组额外的语言特性，从而改变接口和调用者之间的合同，将保证正确性的一些责任从Rust编译器转移到我们身上。目标是实现`unsafe`功能的优势，同时避免*任何*未定义行为。Rust语言设计者认为什么是未定义行为的定义可以在[语言参考](https://doc.rust-lang.org/stable/reference/behavior-considered-undefined.html)中找到。

### 安全策略

为保持正确性，任何`unsafe` Rust的使用都必须遵循我们的策略：

- 如果函数`unsafe`调用，文档中*必须*有`Safety`部分解释为什么函数是`unsafe`的，并涵盖函数期望调用者维护的不变量，以及如何在该合同中履行其义务。
- 在其文档注释的Safety部分记录为什么每个函数是`unsafe`的，并用单元测试覆盖所有`unsafe`块。
- 始终包含`SAFETY:`注释解释为什么不安全操作有效：

```rust
// SAFETY: 消息总线不应在线程间传递
#[allow(unsafe_code)]

unsafe impl Send for MessageBus {}
```

- **Crate级lint** – 每个暴露FFI符号的crate启用`#![deny(unsafe_op_in_unsafe_fn)]`。即使在`unsafe fn`内部，每个指针解引用或其他危险操作都必须包装在自己的`unsafe { … }`块中。

- **CVec合同** – 对于跨越FFI边界的原始向量，请阅读[FFI内存合同](ffi.md)。外部代码成为分配的所有者，**必须**恰好调用一次匹配的`vec_drop_*`函数。

## 工具配置

项目使用多种代码质量工具：

- **rustfmt**：自动代码格式化（参见`rustfmt.toml`）。
- **clippy**：Linting和最佳实践（参见`clippy.toml`）。
- **cbindgen**：FFI的C头文件生成。

## 资源

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) – 不安全Rust的黑魔法。
- [The Rust Reference – Unsafety](https://doc.rust-lang.org/stable/reference/unsafety.html)。
- [Safe Bindings in Rust – Russell Johnston](https://www.abubalay.com/blog/2020/08/22/safe-bindings-in-rust)。
- [Google – Rust and C interoperability](https://www.chromium.org/Home/chromium-security/memory-safety/rust-and-c-interoperability/)。

## Python + Rust混合调试指南

这种方法允许从VS Code内的Jupyter notebook同时调试Python和Rust代码。

### 设置

安装VS Code扩展：Rust Analyzer、CodeLLDB、Python、Jupyter

### 步骤0：使用调试符号编译nautilus_trader

   ```bash
   cd nautilus_trader && make build-debug-pyo3
   ```

### 步骤1：设置调试配置

```python
from nautilus_trader.test_kit.debug_helpers import setup_debugging

setup_debugging()
```

这创建了必要的VS Code调试配置并启动Python调试器可以连接的debugpy服务器。

注意：默认情况下，包含调试配置的.vscode文件夹假设位于`nautilus_trader`根目录上一级文件夹中。如果需要，您可以调整这个。

### 步骤2：设置断点

- **Python断点：**在VS Code中的Python源文件中设置。
- **Rust断点：**在VS Code中的Rust源文件中设置。

### 步骤3：开始混合调试

1. 在VS Code中：选择**"Debug Jupyter + Rust (Mixed)"**配置。
2. 开始调试（F5）或按右箭头绿色按钮。
3. Python和Rust调试器都将附加到您的Jupyter会话。

### 步骤4：执行代码

运行调用rust函数的Jupyter notebook单元格。调试器将在Python和Rust代码中的断点处停止。

### 可用配置

`setup_debugging()`创建这些VS Code配置：

- **`Debug Jupyter + Rust (Mixed)`** - Jupyter notebook的混合调试。
- **`Jupyter Mixed Debugging (Python)`** - notebook的仅Python调试。
- **`Rust Debugger (for jupyter debugging)`** - notebook的仅Rust调试。

### 示例

打开并运行示例notebook：`debug_mixed_jupyter.ipynb`

### 参考

- [PyO3调试](https://pyo3.rs/v0.25.1/debugging.html?highlight=deb#debugging-from-jupyter-notebooks)
