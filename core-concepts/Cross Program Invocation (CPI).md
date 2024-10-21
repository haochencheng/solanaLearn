# [Cross Program Invocation (CPI)](https://solana.com/docs/core/cpi)

跨程序调用 (CPI) 是指一个程序调用另一个程序的指令。此机制允许 Solana 程序具有可组合性。

您可以将指令视为程序向网络公开的 API 端点，将 CPI 视为内部调用另一个 API 的一个 API。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/cpi/cpi.svg">

​															Cross Program Invocation

当一个程序向另一个程序发起跨程序调用 (CPI) 时：

- 签名者权限从调用调用程序 (A) 的初始交易扩展到被调用程序 (B) 

- 被调用程序 (B) 可以对其他程序进行进一步的 CPI，最大深度为 4（例如 B->C、C->D）
- 这些程序可以根据其程序 ID 为 PDA 进行“签名”

Solana 程序运行时定义了一个常量，名为 max_invoke_stack_height，其值设置为 5。这表示程序指令调用堆栈的最大高度。

对于事务指令，堆栈高度从 1 开始，每次程序调用另一条指令时，堆栈高度都会增加 1。此设置有效地将 CPI 的调用深度限制为 4。

## Key Points

- CPI 使 Solana 程序指令能够直接调用另一个程序上的指令。 

- 调用程序的签名者权限扩展到被调用程序。
- 在进行 CPI 时，程序可以代表从其自身程序 ID 派生的 PDA“签名”。 
- 被调用程序可以向其他程序进行额外的 CPI，最大深度为 4。

## How to write a CPI

编写 CPI 指令遵循与构建添加到交易的指令相同的模式。在底层，每个 CPI 指令必须指定以下信息：

- 程序地址：指定被调用的程序 

- 帐户：列出指令读取或写入的每个帐户，包括其他程序
- 指令数据：指定要调用程序中的哪条指令，以及该指令所需的任何其他数据（函数参数）

根据您调用的程序，可能会有带有辅助函数的包可用于构建指令。然后，程序使用 solana_program 包中的以下任一函数执行 CPI：

- invoke - 当没有 PDA 签名者时使用 

- invoke_signed - 当调用者程序需要使用从其程序 ID 派生的 PDA 进行签名时使用

### Basic CPI 

当制作不需要 PDA 签名者的 CPI 时，会使用 Invoke 函数。制作 CPI 时，提供给调用者程序的签名者会自动扩展到被调用者程序。

```
pub fn invoke(
    instruction: &Instruction,
    account_infos: &[AccountInfo<'_>]
) -> Result<(), ProgramError>
```

Here is an example program on [Solana Playground](https://beta.solpg.io/github.com/ZYJLiu/doc-examples/tree/main/cpi-invoke) that makes a CPI using the `invoke` function to call the transfer instruction on the System Program. You can also reference the [Basic CPI guide](https://solana.com/developers/guides/getstarted/how-to-cpi) for further details.

### CPI with PDA Signer 

当制作需要 PDA 签名者的 CPI 时，会使用invoke_signed 函数。用于派生签名者 PDA 的种子会作为 signer_seeds 传递到invoke_signed 函数中。

您可以参考程序派生地址页面来了解有关如何派生 PDA 的详细信息。

```
pub fn invoke_signed(
    instruction: &Instruction,
    account_infos: &[AccountInfo<'_>],
    signers_seeds: &[&[&[u8]]]
) -> Result<(), ProgramError>
```

运行时使用授予调用程序的权限来确定可以向被调用程序扩展哪些权限。此处的权限是指签名者和可写帐户。

例如，如果调用者正在处理的指令包含签名者或可写帐户，则调用者可以调用也包含该签名者和/或可写帐户的指令。

尽管 PDA 没有私钥，但它们仍可通过 CPI 充当指令中的签名者。要验证 PDA 是否来自调用程序，必须将用于生成 PDA 的种子作为 signers_seeds 包含在内。

处理 CPI 时，Solana 运行时会使用 signers_seeds 和调用程序的 program_id 在内部调用 create_program_address。如果找到有效的 PDA，则该地址将添加为有效签名者。

Here is an example program on [Solana Playground](https://beta.solpg.io/github.com/ZYJLiu/doc-examples/tree/main/cpi-invoke-signed) that makes a CPI using the `invoke_signed` function to call the transfer instruction on the System Program with a PDA signer. You can reference the [CPI with PDA Signer guide](https://solana.com/developers/guides/getstarted/how-to-cpi-with-signer) for further details.





