# 课程二

介绍一些solana-cli基本使用命令

#### 查看solana配置

```
solana config get
```

选择开发网络

RPC URL 和 Websocket URL 指定了 CLI 将请求的 Solana 集群。默认情况下，这将是 mainnet-beta。

```
solana config set --url mainnet-beta
solana config set --url devnet
solana config set --url localhost
solana config set --url testnet
```

也可以使用简短命令

```
solana config set -um    # For mainnet-beta
solana config set -ud    # For devnet
solana config set -ul    # For localhost
solana config set -ut    # For testnet
```

这里我们选择本地开发网络

```
solana config set -ul 
```



#### 创建钱包

```
solana-keygen new
```

会生成私钥

```
/Users/haochencheng/.config/solana/id.json
```

一旦生成密钥对，您可以使用以下命令获取密钥对的地址（公钥）：

```
solana address
```

#### Airdrop SOL

设置本地钱包后，请求 SOL 空投来为您的钱包提供资金。您需要 SOL 来支付交易费用和部署程序。 将您的集群设置为开发网络：

本地启动solana环境

```
mkdir validator
cd validator
solana-test-validator
solana config set -ul
```

#### 空投

```
solana airdrop 2
```

#### 查询余额

```
solana balance
```



