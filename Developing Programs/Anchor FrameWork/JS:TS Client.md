# [JS/TS Client](https://solana.com/docs/programs/anchor/client-typescript)

Anchor 提供了一个 Typescript 客户端库 (@coral-xyz/anchor)，简化了使用 JavaScript 或 TypeScript 从客户端与 Solana 程序交互的过程。

## Client Program

要使用客户端库，首先使用 Anchor 生成的 IDL 文件创建一个程序实例。

创建 Program 的实例需要程序的 IDL 和 AnchorProvider。AnchorProvider 是结合了两件事的抽象：

- 连接 - 与 Solana 集群的连接（即本地主机、开发网、主网） 
- 钱包 -（可选）用于支付和签署交易的默认钱包

当使用钱包适配器与前端集成时，您需要设置 AnchorProvider 和 Program。

Anchor 会自动在新项目的默认测试文件中设置 Program 实例。但是，此设置与在 Anchor 工作区外初始化 Program 的方式不同，例如在 React 或 Node.js 应用程序中。

```
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloAnchor } from "../target/types/hello_anchor";
 
describe("hello_anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
 
  const program = anchor.workspace.HelloAnchor as Program<HelloAnchor>;
 
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
```

## Invoke Instructions

使用程序 IDL 设置程序后，您可以使用 Anchor MethodsBuilder 来： 构建单个指令 构建交易 构建和发送交易 基本格式如下：

- 建立个人指令 
- 建立交易 
- 建立并发送交易 基本格式如下

基本格式如下:

program.methods - 这是用于从程序的 IDL 创建指令调用的构建器 API

```
await program.methods
  .instructionName(instructionData)
  .accounts({})
  .signers([])
  .rpc();
```

在 .methods 之后，指定程序 IDL 中的指令名称，并将任何所需的参数作为逗号分隔的值传递。

```
await program.methods
  .instructionName(instructionData1, instructionData2)
  .accounts({})
  .signers([])
  .rpc();
```

.accounts - 按照 IDL 中指定的方式传入指令所需的账户地址

```
await program.methods
  .instructionName(instructionData)
  .accounts({})
  .signers([])
  .rpc();
```

.signers - 可选地传入指令所需的作为附加签名者的密钥对数组。这通常在创建新帐户时使用，其中帐户地址是新生成的密钥对的公钥。

请注意，.signers 只应在使用 .rpc() 时使用。使用 .transaction() 或 .instruction() 时，应在发送之前将签名者添加到交易中。

Anchor 提供了多种构建程序指令的方法：

rpc() 方法使用指定的指令发送已签名的交易并返回 TransactionSignature。 使用 .rpc 时，提供商的钱包将自动作为签名者包含在内。

```
// Generate keypair for the new account
const newAccountKp = new Keypair();
 
const data = new BN(42);
const transactionSignature = await program.methods
  .initialize(data)
  .accounts({
    newAccount: newAccountKp.publicKey,
    signer: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([newAccountKp])
  .rpc();
```

transaction() 方法使用指定的指令构建交易，但不发送交易。

```
// Generate keypair for the new account
const newAccountKp = new Keypair();
 
const data = new BN(42);
const transaction = await program.methods
  .initialize(data)
  .accounts({
    newAccount: newAccountKp.publicKey,
    signer: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .transaction();
 
const transactionSignature = await connection.sendTransaction(transaction, [
  wallet.payer,
  newAccountKp,
]);
```

Instruction() 方法使用指定的指令构建 TransactionInstruction。如果您想要手动将指令添加到交易并将其与其他指令相结合，则此方法非常有用。

```
// Generate keypair for the new account
const newAccountKp = new Keypair();
 
const data = new BN(42);
const instruction = await program.methods
  .initialize(data)
  .accounts({
    newAccount: newAccountKp.publicKey,
    signer: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .instruction();
 
const transaction = new Transaction().add(instruction);
 
const transactionSignature = await connection.sendTransaction(transaction, [
  wallet.payer,
  newAccountKp,
]);
```

## Fetch Accounts

程序客户端简化了获取和反序列化由 Anchor 程序创建的帐户的过程。

使用 program.account 后跟 IDL 中定义的帐户类型的名称。Anchor 提供了多种获取帐户的方法。

使用 all() 获取特定帐户类型的所有现有帐户。

```
const accounts = await program.account.newAccount.all();
```

使用 memcmp（内存比较）来筛选与特定偏移量处的特定值匹配的帐户数据。使用 memcmp 需要您了解要获取的帐户类型的数据字段的字节布局。

计算偏移量时，请记住 Anchor 程序创建的账户中的前 8 个字节是为账户鉴别器保留的。

```
const accounts = await program.account.newAccount.all([
  {
    memcmp: {
      offset: 8,
      bytes: "",
    },
  },
]);
```

使用 fetch() 获取单个账户的账户数据

```
const account = await program.account.newAccount.fetch(ACCOUNT_ADDRESS);
```

Use [`fetchMultiple()`](https://github.com/coral-xyz/anchor/blob/v0.30.1/ts/packages/anchor/src/program/namespace/account.ts#L200) to fetch the account data for multiple accounts by passing in an array of account addresses

```
const accounts = await program.account.newAccount.fetchMultiple([
  ACCOUNT_ADDRESS_ONE,
  ACCOUNT_ADDRESS_TWO,
]);
```





