# [Anchor Program Structure](https://solana.com/docs/programs/anchor/program-structure)

Anchor 框架使用 Rust 宏来减少样板代码并简化编写 Solana 程序所需的常见安全检查的实现。

在一个 Anchor 项目中，主要的宏包括：

- declare_id：指定程序的链上地址
- [program]：指定包含程序指令逻辑的模块
- [derive(Accounts)]：用于结构体，表示指令所需的账户列表
- [account]：用于结构体，创建程序的自定义账户类型

## Example Program

让我们检查一个简单的程序，该程序演示了上述宏的用法，以了解 Anchor 程序的基本结构。

下面的示例程序创建了一个新的账户（NewAccount），它存储了传递给初始化指令的 u64 值。

```
use anchor_lang::prelude::*;
 
declare_id!("11111111111111111111111111111111");
 
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



## declare_id! macro

declared_id 宏指定程序的链上地址，即程序ID。

```
use anchor_lang::prelude::*;
 
declare_id!("11111111111111111111111111111111");
```

默认情况下，程序 ID 是在 /target/deploy/your_program_name.json 生成的密钥对的公钥。

要使用 /target/deploy/your_program_name.json 文件中密钥对的公钥更新 declare_id 宏中的程序 ID 值，请运行以下命令：

```
anchor keys sync
```

当克隆存储库时，运行 anchor keys sync 命令很有用，其中克隆的存储库的 declared_id 宏中的程序 ID 值与您在本地运行 anchor build 时生成的值不匹配。



## #[program] macro

\#[program] 宏定义包含程序所有指令处理程序的模块。此模块中的每个公共函数都对应一条可调用的指令。

```
use anchor_lang::prelude::*;
 
declare_id!("11111111111111111111111111111111");
 
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

### Instruction Context

指令处理程序是定义当调用指令时执行的逻辑的函数。每个处理程序的第一个参数是一个 Context<T> 类型，其中 T 是实现 Accounts 特征的结构体，并且指定了该指令所需的账户。

Context 类型为指令提供了对以下非参数输入的访问：

```
pub struct Context<'a, 'b, 'c, 'info, T> {
    /// Currently executing program id.
    pub program_id: &'a Pubkey,
    /// Deserialized accounts.
    pub accounts: &'b mut T,
    /// Remaining accounts given but not deserialized or validated.
    /// Be very careful when using this directly.
    pub remaining_accounts: &'c [AccountInfo<'info>],
    /// Bump seeds found during constraint validation. This is provided as a
    /// convenience so that handlers don't have to recalculate bump seeds or
    /// pass them in as arguments.
    pub bumps: BTreeMap<String, u8>,
}
```

上下文字段可以通过点符号在指令中访问：

- ctx.accounts: 指令所需的账户
- ctx.program_id: 程序的公钥（地址）
- ctx.remaining_accounts: 在账户结构中未指定的附加账户
- ctx.bumps: 账户结构中指定的任何程序导出地址（PDA）账户的增量种子

附加参数是可选的，可以包括在内以指定调用指令时必须提供的参数。

```
pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
    ctx.accounts.new_account.data = data;
    msg!("Changed data to: {}!", data);
    Ok(())
}
```



```
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
```



## #[derive(Accounts)] macro

\#[derive(Accounts)] 宏应用于一个结构体，以指定在调用指令时必须提供的账户。这个宏实现了 Accounts 特征，从而简化了账户验证以及账户数据的序列化和反序列化。

```
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

结构体中的每个字段代表一条指令所需的账户。每个字段的命名是任意的，但建议使用一个描述性的名称来表明账户的用途。

### Account Validation

为了防止安全漏洞，验证提供给指令的账户是否是预期账户非常重要。Anchor 程序通过两种通常一起使用的方式验证账户：

- 账户约束：约束定义了账户必须满足的额外条件，才能被视为有效的指令。约束通过 #[account(..)] 属性应用，该属性放置在实现 Accounts 特性的结构体字段上方。

您可以在这里找到约束的实现。

```
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

- 账户类型：Anchor 提供各种账户类型，以帮助确保客户提供的账户与程序期望的一致。 您可以在此处找到账户类型的实现。

```
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

当调用 Anchor 程序中的指令时，程序首先会验证所提供的账户，然后再执行指令的逻辑。验证后，可以使用 ctx.accounts 语法在指令中访问这些账户。

## #[account] macro

\#[account] 宏应用于定义程序创建的自定义帐户中存储的数据的结构。

此宏实现了这里详细说明的各种特性。#[account]宏的主要功能包括：

- 指定程序拥有者：在创建账户时，账户的程序拥有者会自动设置为在declare_id中指定的程序。
- 设置区分符：一个独特的8字节区分符，特定于账户类型，作为账户数据初始化时的前8个字节添加。这有助于区分账户类型，并用于账户验证。
- 数据序列化和反序列化：账户数据会根据账户类型自动进行序列化和反序列化。

### Account Discriminator

在Anchor程序中，账户识别符是一个8字节的标识符，每种账户类型都有独特的标识。它是由字符串account:<AccountName>的SHA256哈希的前8个字节生成的。当创建账户时，这个识别符会作为账户数据的前8个字节存储。

在Anchor程序中创建账户时，必须为识别符预留8个字节。

```
#[account(init, payer = signer, space = 8 + 8)]
pub new_account: Account<'info, NewAccount>,
```

鉴别器在以下两种情况下使用： 

- 初始化：创建帐户时，将鉴别器设置为帐户数据的前 8 个字节。
- 反序列化：当账户数据被反序列化时，会根据预期账户类型的鉴别器检查账户数据的前 8 个字节。

如果不匹配，则表明客户端提供了意外的账户。此机制用作 Anchor 程序中的账户验证检查。

