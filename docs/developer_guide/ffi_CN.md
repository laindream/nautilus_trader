# FFI内存合约

NautilusTrader暴露了几种**C兼容**类型，以便编译的Rust代码可以被Cython生成的C扩展或其他本地语言使用。其中最重要的是`CVec`——一个围绕Rust `Vec<T>`的*薄*包装器，**按值**传递跨FFI边界。

下面的规则是*严格的*；违反它们会导致未定义行为（通常是双重释放或内存泄漏）。

## CVec生命周期

| 步骤  | 所有者                         | 操作 |
|-------|-------------------------------|--------|
| **1** | Rust                          | 构建一个`Vec<T>`并用`into()`转换它——这会*泄漏*向量并将原始分配的所有权转移给外部代码。 |
| **2** | 外部（Python / Cython / C） | 在`CVec`值在作用域内时使用数据。**不要修改字段`ptr`、`len`、`cap`。** |
| **3** | 外部                       | 恰好一次，调用Rust导出的*类型特定*drop助手（例如`vec_drop_book_levels`、`vec_drop_book_orders`、`vec_time_event_handlers_drop`）。助手使用`Vec::from_raw_parts`重构原始`Vec<T>`并让它drop，释放内存。 |

:::warning
如果忘记步骤**3**，分配将在进程的剩余时间内泄漏；如果执行**两次**，程序将双重释放并可能崩溃。
:::

## 在Python端创建的胶囊

几个Cython助手使用`PyMem_Malloc`分配临时C缓冲区，将它们包装到`CVec`中，并返回`PyCapsule`内的地址。**每个这样的胶囊都使用析构函数创建**（`capsule_destructor`或`capsule_destructor_deltas`），它释放缓冲区和`CVec`。因此调用者*不得*手动释放内存——这样做会导致双重释放。

## 在Rust端创建的胶囊*（PyO3绑定）*

当Rust代码将堆分配的值推送到Python中时，它**必须**使用`PyCapsule::new_with_destructor`，以便Python知道如何在胶囊变得不可达时释放分配。闭包/析构函数负责重构原始`Box<T>`或`Vec<T>`并让它drop。

```rust
Python::with_gil(|py| {
    // 在堆上分配值
    let my_data = MyStruct::new();

    // 将其移动到胶囊中并注册析构函数
    let capsule = pyo3::types::PyCapsule::new_with_destructor(py, my_data, None, |_, _| {})
        .expect("胶囊创建失败");

    // ... 将`capsule`传递回Python ...
});
```

**不要**使用`PyCapsule::new(…, None)`；该变体不注册*任何*析构函数，除非接收者手动提取并释放指针（我们从不依赖的东西），否则会泄漏内存。代码库已经在各处更新以遵循此规则——添加新的FFI模块必须遵循相同的模式。

## 为什么不再有通用的`cvec_drop`

代码库的早期版本提供了一个通用的`cvec_drop`函数，它总是将缓冲区视为`Vec<u8>`。将其用于任何其他元素类型会在释放期间导致大小不匹配，并破坏分配器的簿记。因为该助手在项目内部没有被引用，所以已将其删除以避免意外误用。

## 由Box支持的`*_API`包装器（拥有的Rust对象）

当Rust核心需要将*复杂*值（例如`OrderBook`、`SyntheticInstrument`或`TimeEventAccumulator`）交给外部代码时，它使用`Box::new`在堆上分配值，并返回一个小的`repr(C)`包装器，其唯一字段是该`Box`。

```rust
#[repr(C)]
pub struct OrderBook_API(Box<OrderBook>);

#[unsafe(no_mangle)]
pub extern "C" fn orderbook_new(id: InstrumentId, book_type: BookType) -> OrderBook_API {
    OrderBook_API(Box::new(OrderBook::new(id, book_type)))
}

#[unsafe(no_mangle)]
pub extern "C" fn orderbook_drop(book: OrderBook_API) {
    drop(book); // 释放堆分配
}
```

因此内存安全要求是：

1.  每个构造函数（`*_new`）**必须**有一个匹配的`*_drop`导出在其旁边。
2.  *Python/Cython*绑定必须保证`*_drop`被恰好调用一次。接受两种方法：

    • 将指针包装在使用`PyCapsule::new_with_destructor`创建的`PyCapsule`中，传递调用drop助手的析构函数。

    • 在Python端的`__del__`/`__dealloc__`中显式调用助手。这是大多数v1 Cython模块的历史模式：

      ```cython
      cdef class OrderBook:
          cdef OrderBook_API _mem

          def __cinit__(self, ...):
              self._mem = orderbook_new(...)

          def __dealloc__(self):
              if self._mem._0 != NULL:
                  orderbook_drop(self._mem)
      ```

无论使用哪种风格，请记住：**忘记drop调用会泄漏整个结构**，而调用它两次会双重释放并崩溃。

新的FFI代码必须在合并之前遵循此模板。
