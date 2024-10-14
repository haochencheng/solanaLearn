# [Transactions and Instructions](https://solana.com/docs/core/transactions)

在 Solana 上，我们发送交易来与网络交互。交易包括一条或多条指令，每条指令代表要处理的特定操作。

指令的执行逻辑存储在部署到 Solana 网络的程序中，每个程序都存储自己的一组指令。

以下是有关如何执行交易的关键细节：

执行顺序：如果一笔交易包含多条指令，则这些指令将按照添加到交易的顺序进行处理。

原子性：事务具有原子性，这意味着它要么完全完成并成功处理所有指令，要么全部失败。如果事务中的任何指令失败，则不会执行任何指令。

为简单起见，交易可以被视为处理一个或多个指令的请求。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/transaction-simple.svg">



您可以将交易想象成一个信封，其中每个指令都是您填写并放入信封内的文档。

然后，我们邮寄出信封来处理文件，就像在网络上发送交易来处理我们的指令一样。

## Key Points

Solana 交易由与网络上各种程序交互的指令组成，其中每个指令代表一个特定的操作。

每条指令都指定了执行该指令的程序，指令所需的账户以及执行该指令所需的数据。

交易中的指令按照列出的顺序进行处理

交易是原子的，这意味着要么所有指令都成功处理，要么整个交易失败。

交易的最大大小为 1232 字节。

## Basic Example

下面的图表表示一条指令即可将 SOL 从发送方转移到接收方的交易。

Solana 上的个人“钱包”是系统程序拥有的账户。

作为 Solana 账户模型的一部分，只有拥有账户的程序才被允许修改账户上的数据。

因此，从“钱包”账户转移 SOL 需要发送交易来调用系统程序上的转账指令。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/sol-transfer.svg">



发送者账户必须作为交易的签名者（is_signer）才能批准扣除他们的 lampor 余额。

发送者和接收者帐户都必须是可变的（is_writable），因为该指令会修改两个帐户的 Lamport 余额。

一旦交易发送，就会调用系统程序来处理转账指令。

然后，系统程序会相应地更新发送者和接收者账户的 Lamport 余额。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/sol-transfer-process.svg">

### Simple SOL Transfer

以下是 Solana Playground 示例，说明如何使用 SystemProgram.transfer 方法构建 SOL 转移指令：

```
// Define the amount to transfer
const transferAmount = 0.01; // 0.01 SOL

// Create a transfer instruction for transferring SOL from wallet_1 to wallet_2
const transferInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: receiver.publicKey,
  lamports: transferAmount * LAMPORTS_PER_SOL, // Convert transferAmount to lamports
});

// Add the transfer instruction to a new transaction
const transaction = new Transaction().add(transferInstruction);
```

运行脚本并检查控制台中记录的交易详细信息。在下面的部分中，我们将详细介绍幕后发生的事情。

## Transaction

Solana 交易包括：

签名：交易中包含的签名数组。 消息：要原子处理的指令列表。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/tx_format.png">



交易消息的结构包括：

消息头：指定签名者和只读账户的数量。

 账户地址：交易指令所需的账户地址数组

最近的区块哈希：作为交易的时间戳。 

指令：要执行的指令数组。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/legacy_message.png">

### Transaction Size

Solana 网络遵循 1280 字节的最大传输单元 (MTU) 大小，与 IPv6 MTU 大小限制一致，以确保通过 UDP 快速可靠地传输集群信息

考虑到必要的报头（IPv6 为 40 字节，片段报头为 8 字节），

1232 字节仍可用于数据包数据（例如序列化交易）。 这意味着 Solana 交易的总大小限制为 1232 字节。签名和消息的组合不能超过此限制。

签名：每个签名需要 64 个字节。签名的数量可能有所不同，具体取决于交易的要求。

消息：消息包括指令、账户和其他元数据，每个账户需要 32 个字节。账户和元数据的总大小可能有所不同，具体取决于交易中包含的指令。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/issues_with_legacy_txs.png">

### Message Header

消息头指定了交易的账户地址数组中包含的账户的权限。它由三个字节组成，每个字节包含一个 u8 整数，它们共同指定：

交易所需签名的数量。 

需要签名的只读账户地址的数量。

 不需要签名的只读账户地址的数量。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/message_header.png">



### Compact-Array Format 

交易消息上下文中的紧凑数组是指按以下格式序列化的数组：

数组的长度，编码为 compact-u16。 数组的各个项按编码长度后的顺序列出。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/compact_array_format.png">



此编码方法用于指定交易消息中的帐户地址和指令数组的长度。

账户地址数组

### Array of Account Addresses

交易消息包括一个数组，其中包含交易内指令所需的所有账户地址。

该数组以 compact-u16 编码的帐户地址数量开始，然后是按帐户权限排序的地址。消息头中的元数据用于确定每个部分中的帐户数量。

- 可写且为签名者的帐户 
- 只读且为签名者的帐户 
- 可写且为非签名者的帐户 
- 只读且为非签名者的帐户

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/compat_array_of_account_addresses.png">



### Recent Blockhash

所有交易都包含一个最近的区块哈希，作为交易的时间戳。区块哈希用于防止重复并消除陈旧交易。

交易的区块哈希的最大年龄为 150 个区块（假设区块时间为 400 毫秒，则约为 1 分钟）。如果交易的区块哈希比最新区块哈希早 150 个区块，则视为已过期。

这意味着未在特定时间范围内处理的交易将永远不会被执行。

您可以使用 getLatestBlockhash RPC 方法获取当前区块哈希和区块哈希有效的最后一个区块高度。以下是 Solana Playground 上的一个示例。



### Array of Instructions

交易消息中包含了所有需要处理的指令的数组，交易消息中的指令格式为CompiledInstruction。

与帐户地址数组非常相似，此紧凑数组以指令数量的 compact-u16 编码开始，后跟指令数组。数组中的每条指令指定以下信息：

1. 程序 ID：标识将处理指令的链上程序。这表示为指向帐户地址数组中的帐户地址的 u8 索引。
2. 帐户地址索引的紧凑数组：指向指令所需的每个帐户的帐户地址数组的 u8 索引数组。
3. 不透明 u8 数据的紧凑数组：特定于所调用程序的 u8 字节数组。此数据指定要在程序上调用的指令以及该指令所需的任何其他数据（例如函数参数）。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/compact_array_of_ixs.png">



### Example Transaction Structure

下面是包含单个 SOL 转账指令的交易结构示例。它显示了消息详细信息，包括标题、帐户密钥、区块哈希和指令，以及交易的签名。

- header：包含用于指定 accountKeys 数组中的读/写和签名者权限的数据。 
- accountKeys：包含交易中所有指令的账户地址的数组。
- recentBlockhash：创建交易时，交易中包含的区块哈希。
- instructions：包含交易的所有指令的数组。指令中的每个帐户和 programIdIndex 通过索引引用 accountKeys 数组。
- signatures：包含交易指令要求作为签名者的所有账户的签名的数组。签名是通过使用账户的相应私钥对交易消息进行签名而创建的。

```
"transaction": {
    "message": {
      "header": {
        "numReadonlySignedAccounts": 0,
        "numReadonlyUnsignedAccounts": 1,
        "numRequiredSignatures": 1
      },
      "accountKeys": [
        "3z9vL1zjN6qyAFHhHQdWYRTFAcy69pJydkZmSFBKHg1R",
        "5snoUseZG8s8CDFHrXY2ZHaCrJYsW457piktDmhyb5Jd",
        "11111111111111111111111111111111"
      ],
      "recentBlockhash": "DzfXchZJoLMG3cNftcf2sw7qatkkuwQf4xH15N5wkKAb",
      "instructions": [
        {
          "accounts": [
            0,
            1
          ],
          "data": "3Bxs4NN8M2Yn4TLb",
          "programIdIndex": 2,
          "stackHeight": null
        }
      ],
      "indexToProgramIds": {}
    },
    "signatures": [
      "5LrcE2f6uvydKRquEJ8xp19heGxSvqsVbcqUeFoiWbXe8JNip7ftPQNTAVPyTK7ijVdpkzmKKaAQR7MWMmujAhXD"
    ]
  }
```



## Instruction

指令是处理链上特定动作的请求，是程序中执行逻辑的最小连续单元。

在构建添加到交易的指令时，每个指令必须包含以下信息：

- **Program address**：指定被调用的程序。 
- **Accounts**：使用 AccountMeta 结构列出指令读取或写入的每个帐户，包括其他程序。
- **Instruction Data**：一个字节数组，指定调用程序上的哪个指令处理程序，以及指令处理程序所需的任何其他数据（函数参数）。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/instruction.svg">



### AccountMeta

对于指令所需的每个帐户，必须指定以下信息：

- pubkey：账户的链上地址 
- is_signer：指定账户是否需要作为交易的签名者 
- is_writable：指定账户数据是否会被修改

此信息被称为 AccountMeta。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/transactions/accountmeta.svg">



通过指定指令所需的所有账户以及每个账户是否可写，可以并行处理交易。

例如，两个不包含任何写入同一状态的账户的交易可以同时执行。

### Example Instruction Structure [#](https://solana.com/docs/core/transactions#example-instruction-structure)

下面是 SOL 转账指令结构的示例，其中详细说明了账户密钥、程序 ID 和指令所需的数据。

- keys：包含指令所需的每个帐户的 AccountMeta。 
- programId：包含所调用指令的执行逻辑的程序的地址。 
- data：指令的指令数据，以字节缓冲区的形式

```
{
  "keys": [
    {
      "pubkey": "3z9vL1zjN6qyAFHhHQdWYRTFAcy69pJydkZmSFBKHg1R",
      "isSigner": true,
      "isWritable": true
    },
    {
      "pubkey": "BpvxsLYKQZTH42jjtWHZpsVSa7s6JVwLKwBptPSHXuZc",
      "isSigner": false,
      "isWritable": true
    }
  ],
  "programId": "11111111111111111111111111111111",
  "data": [2,0,0,0,128,150,152,0,0,0,0,0]
}
```

## Expanded Example

构建程序指令的细节通常由客户端库抽象出来。但是，如果没有可用的指令，您可以随时手动构建指令。

### Manual SOL Transfer

以下是 Solana Playground 有关如何手动构建 SOL 转移指令的示例：

```
// Define the amount to transfer
const transferAmount = 0.01; // 0.01 SOL
 
// Instruction index for the SystemProgram transfer instruction
const transferInstructionIndex = 2;
 
// Create a buffer for the data to be passed to the transfer instruction
const instructionData = Buffer.alloc(4 + 8); // uint32 + uint64
// Write the instruction index to the buffer
instructionData.writeUInt32LE(transferInstructionIndex, 0);
// Write the transfer amount to the buffer
instructionData.writeBigUInt64LE(BigInt(transferAmount * LAMPORTS_PER_SOL), 4);
 
// Manually create a transfer instruction for transferring SOL from sender to receiver
const transferInstruction = new TransactionInstruction({
  keys: [
    { pubkey: sender.publicKey, isSigner: true, isWritable: true },
    { pubkey: receiver.publicKey, isSigner: false, isWritable: true },
  ],
  programId: SystemProgram.programId,
  data: instructionData,
});
 
// Add the transfer instruction to a new transaction
const transaction = new Transaction().add(transferInstruction);
```

从本质上讲，使用 SystemProgram.transfer 方法的简单示例在功能上等同于上面更详细的示例

SystemProgram.transfer 方法只是抽象出了为指令所需的每个帐户创建指令数据缓冲区和 AccountMeta 的细节。



