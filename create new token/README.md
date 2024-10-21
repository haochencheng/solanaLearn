 spl-token create-token

```
Creating token gLqGAe5jaJPbQCvb7oKusNT6S1D9yjtEsf9KyHy6qTj under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Address:  gLqGAe5jaJPbQCvb7oKusNT6S1D9yjtEsf9KyHy6qTj
Decimals:  9

Signature: 4J7sA93Gfrbd6BJ1tvuCKd1GdDgemQtfyjnigPf1EgTMYHV6ZM7MXEqUKG9m6cjJ1Ah8hDgZZvqWmRJfyaUcGe1k
```

新token的唯一标识 : gLqGAe5jaJPbQCvb7oKusNT6S1D9yjtEsf9KyHy6qTj



新代币最初没有供应。您可以使用以下命令检查代币的当前供应量：

```
spl-token supply <TOKEN_ADDRESS>

```



```
spl-token supply 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg
0
```



### Create Token Account 

要持有特定代币的单位，您必须首先创建一个代币账户。要创建新的代币账户，请使用以下命令：

```
spl-token create-account [OPTIONS] <TOKEN_ADDRESS>
```

例如，在 Solana Playground 终端中运行以下命令：

```
spl-token create-account gLqGAe5jaJPbQCvb7oKusNT6S1D9yjtEsf9KyHy6qTj
```



```
spl-token create-account gLqGAe5jaJPbQCvb7oKusNT6S1D9yjtEsf9KyHy6qTj
Creating account JA9qtV2hQcqDf975LBUQ31EyZPbDpmvMPuM5iqLg57nm

Signature: 3Q2q2geHQC6L1Sa1eLAkNToUWT2PRXjXWjMj4Z4xrSVwGV9WKwhZL4kSiZcJ9ASr2E3uXPDhwchJDJBhTsXrzpAb
```



账户地址：

JA9qtV2hQcqDf975LBUQ31EyZPbDpmvMPuM5iqLg57nm

### Mint Tokens

To create new units of a token, use the following command:

```
spl-token mint [OPTIONS] <TOKEN_ADDRESS> <TOKEN_AMOUNT> [--] [RECIPIENT_TOKEN_ACCOUNT_ADDRESS]
```

For example, running the following command:

```
 spl-token mint gLqGAe5jaJPbQCvb7oKusNT6S1D9yjtEsf9KyHy6qTj  100

```

输出结果

```
Minting 100 tokens
  Token: gLqGAe5jaJPbQCvb7oKusNT6S1D9yjtEsf9KyHy6qTj
  Recipient: JA9qtV2hQcqDf975LBUQ31EyZPbDpmvMPuM5iqLg57nm

Signature: 33e8qcZmzWFs7W6GctvtptVyRawGoGRDJuqaWTqbADzrYsP9quEPYUJohpfgSSbwvXNmCNeZCnDQWRuyuCVQtXrC

```

返回以下输出： 

- gLqGAe5jaJPbQCvb7oKusNT6S1D9yjtEsf9KyHy6qTj 是正在为其铸造代币的铸币账户的地址（增加总供应量）。
- JA9qtV2hQcqDf975LBUQ31EyZPbDpmvMPuM5iqLg57nm 是您的钱包的代币账户的地址，代币单位正在被铸造到该地址（数量增加）。

要将代币铸造到不同的代币账户，请指定预期接收者代币账户的地址。例如，运行以下命令：

```
spl-token mint gLqGAe5jaJPbQCvb7oKusNT6S1D9yjtEsf9KyHy6qTj 100 -- Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt
```

要将代币铸造到不同的代币账户，请指定预期接收者代币账户的地址。例如，运行以下命令：

```
spl-token mint gLqGAe5jaJPbQCvb7oKusNT6S1D9yjtEsf9KyHy6qTj 100 -- Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt
```

返回以下输出： 

- 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg 是正在为其铸造代币的铸币账户的地址（增加总供应量）。

- Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt 是代币账户的地址，代币单位正在被铸造到该账户（数量增加）。

在底层，创建新的代币单位需要调用代币程序上的 MintTo 指令。该指令必须由铸币机构签署

该指令将新的代币单位铸造到代币账户，并增加铸造账户的总供应量。以下是 Solana Playground 上的 Javascript 示例。