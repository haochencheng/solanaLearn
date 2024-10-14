### [Core Concepts](https://solana.com/docs/core)

深入理解使Solana与其他区块链不同的核心概念。通过这些核心概念理解“Solana编程模型”对于最大化你作为Solana区块链开发者的成功至关重要。

#### Solana Account Model [](https://solana.com/docs/core#solana-account-model)

在Solana上，所有数据都存储在被称为“账户”的地方。Solana区块链上的数据组织方式类似于键值存储，数据库中的每个条目被称为“账户”。

类似linux一切皆文件

#### Transactions and Instructions [](https://solana.com/docs/core#transactions-and-instructions)

在Solana上，我们发送交易来与网络互动。交易可以包含一个或多个指令，每个指令代表一个特定的操作。指令的执行逻辑存储在部署到Solana网络的程序中，每个程序都有自己的一套指令。

#### Fees on Solana

Solana 区块链有几种不同的费用和成本，用于使用这个无权限的网络。这些费用可以分为几种具体类型：

交易费用 - 让验证者处理交易/指令的费用
优先费用 - 一种可选的费用，用于提高交易处理的优先级
租金 - 为了保持数据在链上存储而扣留的余额

#### Programs on Solana [](https://solana.com/docs/core#programs-on-solana)

在Solana生态系统中，“智能合约”被称为程序。每个程序都是一个链上的账户，存储可执行的逻辑，组织成特定的函数，称为指令，并通过各自已部署程序中的指令处理函数调用。

## Program Derived Address [](https://solana.com/docs/core#program-derived-address)

程序派生地址（PDA）为Solana上的开发者提供了两个主要用例：

确定性账户地址：PDA提供了一种机制，可以使用一组可选的“种子”（预定义输入）和特定的程序ID来确定性地派生地址。
启用程序签名：Solana运行时允许程序为从其程序ID派生的PDA进行“签名”。
你可以把PDA想象成一种在链上从预定义输入（例如字符串、数字和其他账户地址）创建类似哈希映射结构的方法。



## Cross Program Invocation

跨程序调用（CPI）是指一个程序调用另一个程序的指令。这个机制允许Solana程序的组合性。

你可以把指令看作是一个程序向网络公开的API端点，而CPI则是一个API内部调用另一个API。

类似rpc，微服务调用

## Tokens on Solana [](https://solana.com/docs/core#tokens-on-solana)

代币是代表对不同类别资产所有权的数字资产。代币化使得财产权的数字化成为可能，是管理可替代和不可替代资产的基本组成部分。

可替代代币代表同类型、同价值的可互换和可分割资产（例如USDC）。
不可替代代币（NFT）则代表对不可分割资产的所有权（例如艺术品）。

## Clusters and Endpoints

Solana 区块链有几个不同的验证者组，称为集群（Clusters）。每个集群在整个生态系统中承担不同的角色，并且包含专门的 API 节点，以满足各自集群的 JSON-RPC 请求。

集群中的各个节点由第三方拥有和运营，每个节点都有一个公开的端点。

在 Solana 网络上有三个主要集群，每个集群都有不同的公开端点：

主网 - [https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com/)
开发网 - [https://api.devnet.solana.com](https://api.devnet.solana.com/)
测试网 - [https://api.testnet.solana.com](https://api.testnet.solana.com/)









