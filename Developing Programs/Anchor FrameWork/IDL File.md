# [IDL File](https://solana.com/docs/programs/anchor/idl)



接口描述语言 (IDL) 文件提供了描述程序指令和帐户的标准化 JSON 文件。此文件简化了链上程序与客户端应用程序集成的过程。

IDL 的主要优点： 标准化：提供用于描述程序指令和帐户的一致格式 客户端生成：用于生成与程序交互的客户端代码



锚点构建命令生成位于 /target/idl/<program-name>.json 的 IDL 文件。 以下代码片段突出显示了程序、IDL 和客户端之间的关系。

## Program Instructions

IDL 中的指令数组直接对应于程序中定义的指令。它指定每条指令所需的帐户和参数。

下面的程序包括一个初始化指令，指定它所需的账户和参数。

```
use anchor_lang::prelude::*;
 
declare_id!("BYFW1vhC1ohxwRbYoLbAWs86STa25i9sD5uEusVjTYNd");
 
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
 
#[account]
pub struct NewAccount {
    data: u64,
}
```

生成的 IDL 文件包含标准化 JSON 格式的指令，包括其名称、帐户、参数和鉴别器。

```
{
  "address": "BYFW1vhC1ohxwRbYoLbAWs86STa25i9sD5uEusVjTYNd",
  "metadata": {
    "name": "hello_anchor",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [175, 175, 109, 31, 13, 152, 155, 237],
      "accounts": [
        {
          "name": "new_account",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "system_program",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "data",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "NewAccount",
      "discriminator": [176, 95, 4, 118, 91, 177, 125, 232]
    }
  ],
  "types": [
    {
      "name": "NewAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "data",
            "type": "u64"
          }
        ]
      }
    }
  ]
}
```



然后使用IDL文件生成与程序交互的客户端，简化调用程序指令的过程。

```
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { HelloAnchor } from "../target/types/hello_anchor";
import { Keypair } from "@solana/web3.js";
import assert from "assert";
 
describe("hello_anchor", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.HelloAnchor as Program<HelloAnchor>;
 
  it("initialize", async () => {
    // Generate keypair for the new account
    const newAccountKp = new Keypair();
 
    // Send transaction
    const data = new BN(42);
    const transactionSignature = await program.methods
      .initialize(data)
      .accounts({
        newAccount: newAccountKp.publicKey,
        signer: wallet.publicKey,
      })
      .signers([newAccountKp])
      .rpc();
 
    // Fetch the created account
    const newAccount = await program.account.newAccount.fetch(
      newAccountKp.publicKey,
    );
 
    console.log("Transaction signature: ", transactionSignature);
    console.log("On-chain data is:", newAccount.data.toString());
    assert(data.eq(newAccount.data));
  });
});
```



## Program Accounts

IDL 中的 accounts 数组对应于程序中用 #[account] 宏注释的结构体。这些结构体定义了程序创建的帐户中存储的数据。

下面的程序定义了一个 NewAccount 结构，它有一个 u64 类型的数据字段。

```
use anchor_lang::prelude::*;
 
declare_id!("BYFW1vhC1ohxwRbYoLbAWs86STa25i9sD5uEusVjTYNd");
 
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
 
#[account]
pub struct NewAccount {
    data: u64,
}
```



```
{
  "address": "BYFW1vhC1ohxwRbYoLbAWs86STa25i9sD5uEusVjTYNd",
  "metadata": {
    "name": "hello_anchor",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [175, 175, 109, 31, 13, 152, 155, 237],
      "accounts": [
        {
          "name": "new_account",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "system_program",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "data",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "NewAccount",
      "discriminator": [176, 95, 4, 118, 91, 177, 125, 232]
    }
  ],
  "types": [
    {
      "name": "NewAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "data",
            "type": "u64"
          }
        ]
      }
    }
  ]
}
```



然后使用 IDL 文件生成与程序交互的客户端，简化获取和反序列化帐户数据的过程。

```
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { HelloAnchor } from "../target/types/hello_anchor";
import { Keypair } from "@solana/web3.js";
import assert from "assert";
 
describe("hello_anchor", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.HelloAnchor as Program<HelloAnchor>;
 
  it("initialize", async () => {
    // Generate keypair for the new account
    const newAccountKp = new Keypair();
 
    // Send transaction
    const data = new BN(42);
    const transactionSignature = await program.methods
      .initialize(data)
      .accounts({
        newAccount: newAccountKp.publicKey,
        signer: wallet.publicKey,
      })
      .signers([newAccountKp])
      .rpc();
 
    // Fetch the created account
    const newAccount = await program.account.newAccount.fetch(
      newAccountKp.publicKey,
    );
 
    console.log("Transaction signature: ", transactionSignature);
    console.log("On-chain data is:", newAccount.data.toString());
    assert(data.eq(newAccount.data));
  });
});
```

## Discriminators

Anchor 为程序中的每个指令和账户类型分配一个唯一的 8 字节标识符。这些标识符用于区分不同的指令或账户类型。该标识符是通过将前缀与指令或账户名称结合，然后取其 Sha256 哈希的前 8 个字节生成的。从 Anchor v0.30 开始，这些标识符会包含在 IDL 文件中。请注意，在使用 Anchor 时，通常不需要直接与这些标识符交互。本节主要提供有关标识符生成和使用的背景信息。

指令鉴别器被程序用来决定在调用时要执行哪条具体指令。

当调用 Anchor 程序指令时，鉴别符将作为指令数据的前 8 个字节包含在内。此操作由 Anchor 客户端自动完成。

```
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [175, 175, 109, 31, 13, 152, 155, 237],
       ...
    }
  ]
```

指令的鉴别器是前缀全局加上指令名称的 Sha256 哈希的前 8 个字节。 例如：

For example:

```
sha256("global:initialize")
```

Hexadecimal output:

```
af af 6d 1f 0d 98 9b ed d4 6a 95 07 32 81 ad c2 1b b5 e0 e1 d7 73 b2 fb bd 7a b5 04 cd d4 aa 30
```

The first 8 bytes are used as the discriminator for the instruction.

```
af = 175af = 1756d = 1091f = 310d = 1398 = 1529b = 155ed = 237
```

You can find the implementation of the discriminator generation in the Anchor codebase [here](https://github.com/coral-xyz/anchor/blob/v0.30.1/lang/syn/src/codegen/program/common.rs#L5-L19), which is used [here](https://github.com/coral-xyz/anchor/blob/v0.30.1/lang/syn/src/codegen/program/instruction.rs#L27).





