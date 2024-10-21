# [PDAs with Anchor](https://solana.com/docs/programs/anchor/pda)

程序派生地址 (PDA) 是指 Solana 开发的一项功能，允许您创建一个从预定义输入（种子）和程序 ID 确定性派生的唯一地址。

本节将介绍如何在 Anchor 程序中使用 PDA 的基本示例。

## Anchor PDA Constraints

在 Anchor 程序中使用 PDA 时，您通常使用 Anchor 的帐户约束来定义用于派生 PDA 的种子。这些约束可作为安全检查，以确保派生正确的地址。

用于定义PDA种子的约束包括：

- seeds：用于派生 PDA 的可选种子数组。种子可以是静态值，也可以是帐户数据的动态引用。

- bump：用于派生 PDA 的 bump 种子。用于确保地址符合 Ed25519 曲线并且是有效的 PDA。
- `seeds::program` - （可选）用于派生 PDA 地址的程序 ID。此约束仅用于派生程序 ID 不是当前程序的 PDA。

种子和碰撞约束需要一起使用。

### Usage Examples

下面是演示如何在 Anchor 程序中使用 PDA 约束的示例。

#### No Optional Seeds [#](https://solana.com/docs/programs/anchor/pda#no-optional-seeds)

- Use an empty array `[]` to define a PDA without optional seeds.

```
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(
        seeds = [],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```



#### Single Static Seed [#](https://solana.com/docs/programs/anchor/pda#single-static-seed)

在种子约束中指定可选种子。

```
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(
        seeds = [b"hello_world"],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```

#### Multiple Seeds and Account References [#](https://solana.com/docs/programs/anchor/pda#multiple-seeds-and-account-references)

seeds 约束中可以指定多个种子。seeds 约束还可以引用其他账户地址或账户数据。

```
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"hello_world", signer.key().as_ref()],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```

上面的例子既使用了静态种子（b“hello_world”），又使用了动态种子（签名者的公钥）。

## PDA seeds in the IDL 

种子约束中定义的程序派生地址 (PDA) 种子包含在程序的 IDL 文件中。这允许 Anchor 客户端在构建指令时使用这些种子自动解析帐户。

下面的示例展示了程序、IDL 和客户端之间的关系。

下面的程序使用静态种子（b“hello_world”）和签名者的公钥作为动态种子定义了一个 pda_account。

```
use anchor_lang::prelude::*;
 
declare_id!("BZLiJ62bzRryYp9mRobz47uA66WDgtfTXhhgM25tJyx5");
 
#[program]
mod hello_anchor {
    use super::*;
    pub fn test_instruction(ctx: Context<InstructionAccounts>) -> Result<()> {
        msg!("PDA: {}", ctx.accounts.pda_account.key());
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"hello_world", signer.key().as_ref()],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```

程序的 IDL 文件包含种子约束中定义的 PDA 种子。 静态种子 b“hello_world”转换为字节值。 动态种子作为对签名者帐户的引用包含在内。

```
{
  "address": "BZLiJ62bzRryYp9mRobz47uA66WDgtfTXhhgM25tJyx5",
  "metadata": {
    "name": "hello_anchor",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "test_instruction",
      "discriminator": [33, 223, 61, 208, 32, 193, 201, 79],
      "accounts": [
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "pda_account",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        }
      ],
      "args": []
    }
  ]
}
```

Anchor 客户端可以使用 IDL 文件自动解析 PDA 地址。

在下面的示例中，Anchor 使用提供商钱包作为签名者，并将其公钥作为 PDA 派生的动态种子来自动解析 PDA 地址。这样就无需在构建指令时明确派生 PDA。

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
    const tx = await program.methods.testInstruction().rpc();
    console.log("Your transaction signature", tx);
  });
});
```

当调用该指令时，PDA 会按照程序指令中的定义打印到程序日志中。

```
Program BZLiJ62bzRryYp9mRobz47uA66WDgtfTXhhgM25tJyx5 invoke [1]
Program log: Instruction: TestInstruction
Program log: PDA: 3Hikt5mpKaSS4UNA5Du1TZJ8tp4o8VC8YWW6X9vtfVnJ
Program BZLiJ62bzRryYp9mRobz47uA66WDgtfTXhhgM25tJyx5 consumed 18505 of 200000 compute units
Program BZLiJ62bzRryYp9mRobz47uA66WDgtfTXhhgM25tJyx5 success
```





