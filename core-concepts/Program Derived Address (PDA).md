# [Program Derived Address (PDA)](https://solana.com/docs/core/pda)

程序派生地址 (PDA) 为 Solana 上的开发人员提供了两种主要用例：

- 确定性账户地址：PDA 提供一种机制，通过可选“种子”（预定义输入）和特定程序 ID 的组合来确定性地派生地址。
- 启用程序签名：Solana 运行时允许程序为源自其程序 ID 的 PDA 进行“签名”。

您可以将 PDA 视为一种根据预定义的一组输入（例如字符串、数字和其他帐户地址）在链上创建类似哈希图的结构的方法。

这种方法的好处是，它消除了记住确切地址的需要。相反，你只需要记住用于推导该地址的特定输入。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/pda/pda.svg">

重要的是要理解，简单地派生一个程序派生地址（PDA）不会自动在该地址创建一个链上账户

以 PDA 作为链上地址的账户必须通过用于导出地址的程序明确创建。你可以将导出 PDA 视为在地图上查找地址

仅仅有地址并不意味着该位置建有任何东西。

本节将介绍派生 PDA 的细节。关于程序如何使用 PDA 进行签名的细节将在跨程序调用 (CPI) 部分中讨论，因为它需要这两个概念的上下文。



## Key Points

- PDA 是使用用户定义的种子、碰撞种子和程序的 ID 的组合确定性地得出的地址。
- PDA 是不符合 Ed25519 曲线的地址，没有相应的私钥。
- Solana 程序可以以编程方式“签名”使用其程序 ID 派生的 PDA。
- 派生 PDA 不会自动创建链上账户。
- 使用 PDA 作为地址的帐户必须通过 Solana 程序内的专用指令明确创建。

## What is a PDA

PDA 是确定性派生的地址，看起来像标准公钥，但没有关联的私钥。这意味着没有外部用户可以为该地址生成有效的签名。

但是，Solana 运行时允许程序以编程方式为 PDA“签名”，而无需私钥。

就上下文而言，Solana 密钥对是 Ed25519 曲线（椭圆曲线密码学）上的点，具有公钥和相应的私钥。

我们经常使用公钥作为新的链上账户的唯一ID，使用私钥进行签名。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/pda/address-on-curve.svg">

PDA 是使用一组预定义的输入有意得出的偏离 Ed25519 曲线的点。

不在Ed25519曲线上的点没有有效的对应私钥，不能用于加密操作（签名）。

然后，PDA 可用作链上帐户的地址（唯一标识符），从而提供一种轻松存储、映射和获取程序状态的方法。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/pda/address-off-curve.svg">

## How to derive a PDA

派生 PDA 需要 3 个输入。 

- 可选种子：用于派生 PDA 的预定义输入（例如字符串、数字、其他帐户地址）。这些输入将转换为字节缓冲区。
- 碰撞种子：一个额外的输入（值在 255-0 之间），用于保证生成有效的 PDA（偏离曲线）。在生成 PDA 以“碰撞” Ed25519 曲线上的点时，此碰撞种子（以 255 开头）会附加到可选种子中。碰撞种子有时被称为“随机数”。
- 程序 ID：PDA 所源自的程序的地址。这也是可以代表 PDA“签名”的程序

<img src="https://solana-developer-content.vercel.app/assets/docs/core/pda/pda-derivation.svg">



下面的示例包含 Solana Playground 的链接，您可以在浏览器内编辑器中运行示例。

To derive a PDA, we can use the [`findProgramAddressSync`](https://github.com/solana-labs/solana-web3.js/blob/ca9da583a39cdf8fd874a2e03fccdc849e29de34/packages/library-legacy/src/publickey.ts#L212) method from [`@solana/web3.js`](https://www.npmjs.com/package/@solana/web3.js). There are equivalents of this function in other programming languages (e.g. [Rust](https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/sdk/program/src/pubkey.rs#L484)), but in this section, we will walk through examples using Javascript.

使用 findProgramAddressSync 方法时，我们传入：

-  转换为字节缓冲区的预定义可选种子，以及 
- 用于派生 PDA 的程序 ID（地址）

一旦找到有效的 PDA，findProgramAddressSync 将返回用于派生 PDA 的地址（PDA）和碰撞种子。

下面的示例在不提供任何可选种子的情况下派生出 PDA。

```
import { PublicKey } from "@solana/web3.js";
 
const programId = new PublicKey("11111111111111111111111111111111");
 
const [PDA, bump] = PublicKey.findProgramAddressSync([], programId);
 
console.log(`PDA: ${PDA}`);
console.log(`Bump: ${bump}`);
```

You can run this example on [Solana Playground](https://beta.solpg.io/66031e5acffcf4b13384cfef). The PDA and bump seed output will always be the same:

```
PDA: Cu7NwqCXSmsR5vgGA3Vw9uYVViPi3kQvkbKByVQ8nPY9Bump: 255
```

The next example below adds an optional seed "helloWorld".

请注意，碰撞种子为 254。这意味着 255 在 Ed25519 曲线上派生出一个点，并且不是有效的 PDA。

findProgramAddressSync 返回的 bump seed 是可选种子和程序 ID 的给定组合派生出有效 PDA 的第一个值（255-0 之间）。

### CreateProgramAddress

在底层，findProgramAddressSync 会迭代地将额外的种子（nonce）附加到种子缓冲区并调用 createProgramAddressSync 方法。

碰撞种子的起始值为 255，然后逐个减少 1，直到找到有效的 PDA（曲线外）。 您可以使用 createProgramAddressSync 复制上一个示例，并明确传入碰撞种子 254。

```
import { PublicKey } from "@solana/web3.js";
 
const programId = new PublicKey("11111111111111111111111111111111");
const string = "helloWorld";
const bump = 254;
 
const PDA = PublicKey.createProgramAddressSync(
  [Buffer.from(string), Buffer.from([bump])],
  programId,
);
 
console.log(`PDA: ${PDA}`);
```

Run this example above on [Solana Playground](https://beta.solpg.io/66031f8ecffcf4b13384cff1). Given the same seeds and program ID, the PDA output will match the previous one:

```
PDA: 46GZzzetjCURsdFPb7rcnspbEMnCBXe9kpjrsZAkKb6X
```



### Canonical Bump 

“规范碰撞”是指第一个生成有效 PDA 的碰撞种子（从 255 开始，以 1 为单位递减）。为了程序安全，建议仅使用从规范碰撞生成的 PDA。

使用前面的例子作为参考，下面的例子尝试使用从 255-0 的每个碰撞种子来派生一个 PDA。

```
import { PublicKey } from "@solana/web3.js";
 
const programId = new PublicKey("11111111111111111111111111111111");
const string = "helloWorld";
 
// Loop through all bump seeds for demonstration
for (let bump = 255; bump >= 0; bump--) {
  try {
    const PDA = PublicKey.createProgramAddressSync(
      [Buffer.from(string), Buffer.from([bump])],
      programId,
    );
    console.log("bump " + bump + ": " + PDA);
  } catch (error) {
    console.log("bump " + bump + ": " + error);
  }
}
```

Run the example on [Solana Playground](https://beta.solpg.io/66032009cffcf4b13384cff2) and you should see the following output:

```
解释

bump 255: Error: Invalid seeds, address must fall off the curvebump 254: 46GZzzetjCURsdFPb7rcnspbEMnCBXe9kpjrsZAkKb6Xbump 253: GBNWBGxKmdcd7JrMnBdZke9Fumj9sir4rpbruwEGmR4ybump 252: THfBMgduMonjaNsCisKa7Qz2cBoG1VCUYHyso7UXYHHbump 251: EuRrNqJAofo7y3Jy6MGvF7eZAYegqYTwH2dnLCwDDGdPbump 250: Error: Invalid seeds, address must fall off the curve...// remaining bump outputs
```

正如预期的那样，碰撞种子 255 会引发错误，而第一个得出有效 PDA 的碰撞种子是 254。

但是，请注意，碰撞种子 253-251 都派生出具有不同地址的有效 PDA。这意味着，给定相同的可选种子和 programId，具有不同值的碰撞种子仍然可以派生出有效的 PDA。

**警告** 

在构建 Solana 程序时，建议包含安全检查，以验证传递给程序的 PDA 是否使用规范的 bump 派生

如果不这样做，可能会引入漏洞，从而允许向程序提供意外的帐户。



## Create PDA Accounts

This example program on [Solana Playground](https://beta.solpg.io/github.com/ZYJLiu/doc-examples/tree/main/pda-account) demonstrates how to create an account using a PDA as the address of the new account. The example program is written using the Anchor framework.

In the `lib.rs` file, you will find the following program which includes a single instruction to create a new account using a PDA as the address of the account. The new account stores the address of the `user` and the `bump` seed used to derive the PDA.

```
use anchor_lang::prelude::*;
 
declare_id!("75GJVCJNhaukaa2vCCqhreY31gaphv7XTScBChmr1ueR");
 
#[program]
pub mod pda_account {
    use super::*;
 
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account_data = &mut ctx.accounts.pda_account;
        // store the address of the `user`
        account_data.user = *ctx.accounts.user.key;
        // store the canonical bump
        account_data.bump = ctx.bumps.pda_account;
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
 
    #[account(
        init,
        // set the seeds to derive the PDA
        seeds = [b"data", user.key().as_ref()],
        // use the canonical bump
        bump,
        payer = user,
        space = 8 + DataAccount::INIT_SPACE
    )]
    pub pda_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}
 
#[account]
 
#[derive(InitSpace)]
pub struct DataAccount {
    pub user: Pubkey,
    pub bump: u8,
}
```

用于派生 PDA 的种子包括硬编码的字符串数据和指令中提供的用户账户地址。Anchor 框架会自动派生出规范的 bump 种子。

```
#[account(
    init,
    seeds = [b"data", user.key().as_ref()],
    bump,
    payer = user,
    space = 8 + DataAccount::INIT_SPACE
)]
pub pda_account: Account<'info, DataAccount>,
```

初始化约束指示 Anchor 调用系统程序，使用 PDA 作为地址创建一个新帐户。在底层，这是通过 CPI 完成的。

```
#[account(
    init,
    seeds = [b"data", user.key().as_ref()],
    bump,
    payer = user,
    space = 8 + DataAccount::INIT_SPACE
)]
pub pda_account: Account<'info, DataAccount>,
```

在上面提供的 Solana Playground 链接内的测试文件 (pda-account.test.ts) 中，您将找到派生 PDA 的 Javascript 等效代码。

```
const [PDA] = PublicKey.findProgramAddressSync(
  [Buffer.from("data"), user.publicKey.toBuffer()],
  program.programId,
);
```

然后发送交易以调用初始化指令，使用 PDA 作为地址创建一个新的链上账户。发送交易后，PDA 用于获取在该地址创建的链上账户。

```
it("Is initialized!", async () => {
  const transactionSignature = await program.methods
    .initialize()
    .accounts({
      user: user.publicKey,
      pdaAccount: PDA,
    })
    .rpc();
 
  console.log("Transaction Signature:", transactionSignature);
});
 
it("Fetch Account", async () => {
  const pdaAccount = await program.account.dataAccount.fetch(PDA);
  console.log(JSON.stringify(pdaAccount, null, 2));
});
```

请注意，如果您使用同一个用户地址作为种子多次调用初始化指令，则交易将失败。这是因为派生地址上已经存在一个帐户。









