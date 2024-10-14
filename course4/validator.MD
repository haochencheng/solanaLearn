### 创建Authorized Withdrawer Account

授权提款人账号可以操作你的账户，有最终权限。该密钥对将有权从您的投票账户中提取资金，并有权更改您的投票账户的所有其他方面。

请安全保管

```
solana-keygen new -o ~/authorized-withdrawer-keypair.json
```



#### 创建Vote Account



```
solana-keygen new -o ~/vote-account-keypair.json
```



```
solana create-vote-account ~/vote-account-keypair.json ~/validator-keypair.json ~/authorized-withdrawer-keypair.json
```



#### 连接验证者Connect Your Validator

```
solana-validator \
  --identity ~/validator-keypair.json \
  --vote-account ~/vote-account-keypair.json \
  --rpc-port 8899 \
  --entrypoint entrypoint.devnet.solana.com:8001 \
  --limit-ledger-size \
  --log ~/solana-validator.log
```



#### 确认是否已连接

```
solana gossip
```



#### 管理本地端口分配

验证器默认会选择 8000-10000 范围内的端口

```
--dynamic-port-range.
For example, 
solana-validator --dynamic-port-range 11000-11020
```

#### 限制leader账本

如果没有次参数，验证器将保留所有收到的分类账数据，直到磁盘空间耗尽。默认值尝试将块存储（rocksdb 目录中的数据）磁盘使用量保持在 500 GB 以下,注意这500G不包括以下项目：

- Persistent accounts data
- Persistent accounts index
- Snapshots

```
--limit-ledger-size 100000
```



