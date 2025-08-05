# 开发者指南

欢迎来到NautilusTrader开发者指南！

在这里，您将找到有关开发和扩展NautilusTrader以满足您的交易需求或为项目贡献改进的指导。

:::info
本指南的结构使得自动化工具可以与人类读者一起使用它。
:::

我们相信为正确的工作使用正确的工具。整体设计哲学是充分利用Python的高级功能，包括其丰富的框架和库生态系统，同时克服其在性能方面的一些固有缺陷和缺乏内置类型安全性（因为它是一种解释型动态语言）。

Cython的优势之一是内存的分配和释放由C代码生成器在构建的'cythonization'步骤中处理（除非您特别使用其一些较低级别的功能）。

这种方法通过编译扩展将Python的简单性与接近原生C性能相结合。

我们工作的主要开发和运行时环境是Python。随着在`.pyx`和`.pxd`文件的生产代码库中引入Cython，了解Python的CPython实现如何与底层CPython API以及Cython生成的NautilusTrader C扩展模块交互是很重要的。

我们建议彻底查看[Cython文档](https://cython.readthedocs.io/en/latest/)，以熟悉其一些核心概念，以及C类型的使用位置。

虽然不必成为C语言专家，但了解在函数和方法定义、本地代码块中如何使用Cython C语法，以及常见的原始C类型及其如何映射到相应的`PyObject`类型是有帮助的。

## 内容

- [环境设置](environment_setup_CN.md)
- [编码标准](coding_standards_CN.md)
- [Cython](cython_CN.md)
- [Rust](rust_CN.md)
- [文档](docs_CN.md)
- [测试](testing_CN.md)
- [适配器](adapters_CN.md)
- [基准测试](benchmarking_CN.md)
- [打包数据](packaged_data_CN.md)
- [FFI内存契约](ffi_CN.md)
