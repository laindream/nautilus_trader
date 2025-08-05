# nautilus-blockchain

一个高性能、通用、可扩展的适配器，用于从去中心化交易所(DEX)、流动性池和链上事件中获取DeFi数据。它使您能够利用实时和历史链上数据为分析管道和交易策略提供动力。

## 平台

[NautilusTrader](http://nautilustrader.io)是一个开源、高性能、生产级的算法交易平台，为量化交易者提供使用事件驱动引擎在历史数据上回测自动化交易策略组合的能力，并且可以实时部署这些相同的策略，无需任何代码更改。

NautilusTrader的设计、架构和实现理念将软件正确性和安全性置于最高优先级，旨在支持关键任务、交易系统回测和实时部署工作负载。

## 脚本

您可以运行一些示例脚本并提供目标RPC环境变量。这些示例演示了如何连接到区块链节点并订阅各种事件。

您可以通过两种方式配置所需的环境变量：

1. **在项目根目录中使用`.env`文件：**
   在项目根目录中创建名为`.env`的文件，内容如下：

   ```
   CHAIN=Ethereum
   RPC_WSS_URL=wss://mainnet.infura.io/ws/v3/YOUR_INFURA_API_KEY
   RPC_HTTP_URL=https://mainnet.infura.io/v3/YOUR_INFURA_API_KEY
   ```

2. **直接在命令行中提供变量：**

   ```
   CHAIN=Ethereum RPC_WSS_URL=wss://your-node-endpoint cargo run --bin live_blocks_rpc
   ```

### 观察实时区块

脚本将连接到指定的区块链，并为RPC版本和仅Hypersync记录接收到的每个新区块的信息。

```
cargo run --bin live_blocks_rpc --features hypersync
```

```
cargo run --bin live_blocks_hypersync --features hypersync
```

对于RPC示例，输出应该是：

```
Running `target/debug/live_blocks_rpc`
2025-04-25T14:54:41.394620000Z [INFO] TRADER-001.nautilus_blockchain::rpc::core: Subscribing to new blocks on chain Ethereum
2025-04-25T14:54:48.951608000Z [INFO] TRADER-001.nautilus_blockchain::data: Block(chain=Ethereum, number=22346765, timestamp=2025-04-25T14:54:47+00:00, hash=0x18a3c9f1e3eec06b45edc1f632565e5c23089dc4ad0892b00fda9e4ffcc9bf91)
2025-04-25T14:55:00.646992000Z [INFO] TRADER-001.nautilus_blockchain::data: Block(chain=Ethereum, number=22346766, timestamp=2025-04-25T14:54:59+00:00, hash=0x110436e41463daeacd1501fe53d38c310573abc136672a12054e1f33797ffeb9)
2025-04-25T14:55:14.369337000Z [INFO] TRADER-001.nautilus_blockchain::data: Block(chain=Ethereum, number=22346767, timestamp=2025-04-25T14:55:11+00:00, hash=0x54e7dbcfc14c058e22c70cbacabe4872e84bd6d3b976258f0d364ae99226b314)
2025-04-25T14:55:38.314022000Z [INFO] TRADER-001.live_blocks: Shutdown signal received, shutting down...

```

### 同步以太坊上Uniswap V3的dex、代币和池

此脚本演示了如何使用区块链数据客户端发现并缓存Uniswap V3池及其关联代币。它查询以太坊区块链中Uniswap V3工厂合约发出的池创建事件，通过智能合约调用检索池中每个代币的元数据（名称、符号、小数位），并将所有内容存储在本地Postgres数据库中。

```
cargo run --bin sync_tokens_pools --features hypersync
```

## 许可证

NautilusTrader的源代码在GitHub上可用，采用[GNU较宽通用公共许可证v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html)。欢迎为项目做出贡献，需要完成标准的[贡献者许可协议(CLA)](https://github.com/nautechsystems/nautilus_trader/blob/develop/CLA.md)。

---

NautilusTrader™由Nautech Systems开发和维护，这是一家专门开发高性能交易系统的技术公司。更多信息请访问<https://nautilustrader.io>。

<img src="https://nautilustrader.io/nautilus-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. 保留所有权利。</span> 