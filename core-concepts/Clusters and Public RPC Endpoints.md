# [Clusters and Public RPC Endpoints](https://solana.com/docs/core/clusters)

Solana 区块链有几组不同的验证器，称为集群。每个集群在整个生态系统中服务于不同的目的，并包含专用的 api 节点来满足各自集群的 JSON-RPC 请求。

集群中的各个节点由第三方拥有和运营，每个节点都有一个公共端点。

## Solana public RPC endpoints [#](https://solana.com/docs/core/clusters#solana-public-rpc-endpoints)

Solana Labs 组织为每个集群运营一个公共 RPC 端点。每个公共端点都受到速率限制，但可供用户和开发人员与 Solana 区块链交互。

公共端点速率限制可能会发生变化。本文档中列出的具体速率限制不保证是最新的。

### Using explorers with different Clusters [#](https://solana.com/docs/core/clusters#using-explorers-with-different-clusters)

许多流行的 Solana 区块链浏览器都支持选择任何集群，通常还允许高级用户添加自定义/私有 RPC 端点。

An example of some of these Solana blockchain explorers include:

- [http://explorer.solana.com/](https://explorer.solana.com/).
- [http://solana.fm/](https://solana.fm/).
- [http://solscan.io/](https://solscan.io/).
- http://solanabeach.io/.
- http://validators.app/.

## Devnet

对于任何想要试用 Solana 的人来说，Devnet 都是一个游乐场，无论是用户、代币持有者、应用程序开发者还是验证者。

- 应用程序开发人员应以 Devnet 为目标。 
- 潜在验证者应首先以 Devnet 为目标。 
- Devnet 和 Mainnet Beta 之间的主要区别： Devnet 代币不是真实的
- Devnet 包含一个用于应用程序测试的空投代币水龙头 
- Devnet 可能会受到账本重置的影响 
- Devnet 通常运行与 Mainnet Beta 相同的软件发布分支版本，但可能会运行比 Mainnet Beta 更新的次要发布版本。

- Gossip entrypoint for Devnet: `entrypoint.devnet.solana.com:8001`

### Devnet endpoint [#](https://solana.com/docs/core/clusters#devnet-endpoint)

- `https://api.devnet.solana.com` - single Solana Labs hosted api node; rate-limited



#### Example `solana` command-line configuration [#](https://solana.com/docs/core/clusters#example-object-object-command-line-configuration)

To connect to the `devnet` Cluster using the Solana CLI:

```
solana config set --url https://api.devnet.solana.com
```



### Devnet rate limits [#](https://solana.com/docs/core/clusters#devnet-rate-limits)

- 每个 IP 每 10 秒最大请求数：100 

- 每个 IP 单次 RPC 每 10 秒最大请求数：40 

- 每个 IP 最大并发连接数：40

- 每 IP 每 10 秒最大连接速率：40 
- 每 30 秒最大数据量：100 MB

## Testnet [#](https://solana.com/docs/core/clusters#testnet)

测试网是 Solana 核心贡献者在实时集群上对最近发布的功能进行压力测试的地方，特别关注网络性能、稳定性和验证器行为。

- Testnet 代币并非真实代币 Testnet 可能会重置账本。 
- Testnet 包含用于应用程序测试的空投代币水龙头 
- Testnet 通常运行比 Devnet 和 Mainnet Beta 更新的软件发布分支
- 测试网的八卦入口点：entrypoint.testnet.solana.com:8001

### Testnet endpoint [#](https://solana.com/docs/core/clusters#testnet-endpoint)

- `https://api.testnet.solana.com` - single Solana Labs api node; rate-limited

#### Example `solana` command-line configuration [#](https://solana.com/docs/core/clusters#example-object-object-command-line-configuration)

```
solana config set --url https://api.testnet.solana.com
```

### Testnet rate limits [#](https://solana.com/docs/core/clusters#testnet-rate-limits)

- 每个 IP 每 10 秒最大请求数：100 
- 每个 IP 单次 RPC 每 10 秒最大请求数：40 
- 每个 IP 最大并发连接数：40

- 每 IP 每 10 秒最大连接速率：40 
- 每 30 秒最大数据量：100 MB

## Mainnet beta

面向 Solana 用户、构建者、验证者和代币持有者的无需许可的持久集群。 

- 在 Mainnet Beta 上发行的代币是真正的 SOL Mainnet Beta 的

-  Gossip 入口点：entrypoint.mainnet-beta.solana.com:8001



### Mainnet beta endpoint [#](https://solana.com/docs/core/clusters#mainnet-beta-endpoint)

- `https://api.mainnet-beta.solana.com` - Solana Labs hosted api node cluster, backed by a load balancer; rate-limited

#### Example `solana` command-line configuration [#](https://solana.com/docs/core/clusters#example-object-object-command-line-configuration)

```
solana config set --url https://api.mainnet-beta.solana.com
```



### Mainnet beta rate limits [#](https://solana.com/docs/core/clusters#mainnet-beta-rate-limits)

- 每个 IP 每 10 秒最大请求数：100 

- 每个 IP 单次 RPC 每 10 秒最大请求数：40 
- 每个 IP 最大并发连接数：40

- 每 IP 每 10 秒最大连接速率：40 每 30 秒最大数据量：100 MB

公共 RPC 端点不适用于生产应用程序。启动应用程序、投放 NFT 等时，请使用专用/私有 RPC 服务器

公共服务容易被滥用，速率限制可能会在未事先通知的情况下发生变化。同样，高流量网站可能会在未事先通知的情况下被屏蔽。



## Common HTTP Error Codes

403 —— 您的 IP 地址或网站已被封锁。是时候运行您自己的 RPC 服务器或寻找私人服务了

429 —— 您的 IP 地址超出了速率限制。请放慢速度！使用 Retry-After HTTP 响应标头来确定发出另一个请求之前要等待多长时间。