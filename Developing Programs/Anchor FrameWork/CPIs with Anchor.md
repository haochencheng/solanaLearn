# [CPIs with Anchor](https://solana.com/docs/programs/anchor/cpi)

跨程序调用（CPI）是指一个程序调用另一个程序的指令的过程，这使得 Solana 上程序的可组合性成为可能。

本节将介绍在 Anchor 程序中实现 CPI 的基础知识，并使用简单的 SOL 转移指令作为实际示例。

一旦您了解了如何实施 CPI 的基础知识，您就可以将相同的概念应用于任何指令。

## Cross Program Invocations

让我们检查一个将 CPI 实现到系统程序的传输指令的程序。这是 Solana Playground 上的示例程序。

lib.rs 文件包含单个 sol_transfer 指令。当调用 Anchor 程序上的 sol_transfer 指令时，程序内部会调用 System Program 的 transfer 指令。

```
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
 
declare_id!("9AvUNHjxscdkiKQ8tUn12QCMXtcnbR9BVGq3ULNzFMRi");
 
#[program]
pub mod cpi {
    use super::*;
 
    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();
 
        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        );
 
        transfer(cpi_context, amount)?;
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}
```

