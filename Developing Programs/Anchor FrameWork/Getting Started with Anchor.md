# [Getting Started with Anchor](https://solana.com/docs/programs/anchor)

Anchor 框架是一种简化 Solana 程序构建过程的工具。无论您是区块链开发新手还是经验丰富的程序员，Anchor 都可以简化编写、测试和部署 Solana 程序的过程。

在本节中，我们将介绍： 

- 创建新的 Anchor 项目 
- 构建和测试您的程序 
- 部署到 Solana 集群 
- 了解项目文件结构



## Prerequisites

有关详细的安装说明，请访问安装页面。

在开始之前，请确保您已安装以下内容：

- Rust：用于构建Solana程序的编程语言。
- Solana CLI：Solana开发的命令行工具。
- Anchor CLI：Anchor框架的命令行工具。

要验证 Anchor CLI 安装，请打开终端并运行：

```
anchor --version
```

Expected output:

```
anchor-cli 0.30.1
```



## Getting Started

本节介绍创建、构建和测试您的第一个本地 Anchor 程序的基本步骤。

### Create a new Project 

要启动新项目，请使用 anchor init 命令，后跟项目名称。此命令会创建一个具有指定名称的新目录，并设置默认程序和测试文件。

```
anchor init my-program
```

导航到新的项目目录并在代码编辑器中打开它。

```
cd my-project
```



The default Anchor program is located at `/programs/my-project/src/lib.rs`.

declared_id! 宏中的值是程序 ID，即程序的唯一标识符。 默认情况下，它是 /target/deploy/my_project-keypair.json 中生成的密钥对的公钥。

```
use anchor_lang::prelude::*;
 
declare_id!("3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg");
 
#[program]
pub mod my_project {
    use super::*;
 
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct Initialize {}
```

默认的 Typescript 测试文件位于

this file demonstrates how to invoke the default program's `initialize` instruction in Typescript.

my-project.ts

```
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyProject } from "../target/types/my_project";
 
describe("my-project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
 
  const program = anchor.workspace.MyProject as Program<MyProject>;
 
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
```

如果您更喜欢使用 Rust 进行测试，请使用 --test-template rust 标志初始化您的项目。

```
anchor init --test-template rust my-program

```

Rust 测试文件位于

/tests/src/test_initialize.rs

```
use std::str::FromStr;
 
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};
 
#[test]
fn test_initialize() {
    let program_id = "3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
 
    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();
 
    let tx = program
        .request()
        .accounts(my_program::accounts::Initialize {})
        .args(my_program::instruction::Initialize {})
        .send()
        .expect("");
 
    println!("Your transaction signature {}", tx);
}
```

### Build the Program

Build the program by running `anchor build`.

```
anchor build
```

编译后的程序位于 /target/deploy/my_project.so。部署程序时，此文件的内容将存储在 Solana 网络上（作为可执行帐户）。

### Test the Program

为了测试程序，运行锚测试。

```
anchor test
```

默认情况下，Anchor.toml 配置文件指定了 localnet 集群。在 localnet 上开发时，anchor test 会自动执行以下操作：

- 启动一个本地的 Solana 验证者
- 构建并部署你的程序到本地集群
- 运行 tests 文件夹中的测试
- 停止本地的 Solana 验证者

另外，你可以手动启动一个本地的 Solana 验证者，并在其上运行测试。这在你想在开发程序的同时保持验证者运行时非常有用。这样你可以在本地开发时查看 Solana Explorer 上的账户和交易日志。

打开一个新的终端，并通过运行 solana-test-validator 命令来启动一个本地的 Solana 验证者。

```
solana-test-validator
```

在单独的终端中，针对本地集群运行测试。使用 --skip-local-validator 标志跳过启动本地验证器，因为它已在运行。

```
anchor test --skip-local-validator
```

### Deploy to Devnet [#](https://solana.com/docs/programs/anchor#deploy-to-devnet)

默认情况下，Anchor 项目中的 Anchor.toml 配置文件指定 localnet 集群。

```
[toolchain]
 
[features]
resolution = true
skip-lint = false
 
[programs.localnet]
my_program = "3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg"
 
[registry]
url = "https://api.apr.dev"
 
[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"
 
[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

要将您的程序部署到 devnet，请将集群值更改为 Devnet。请注意，这要求您的钱包在 Devnet 上有足够的 SOL 来支付部署成本。

```
-cluster = "Localnet"
+cluster = "Devnet"
```

```
[provider]
cluster = "Devnet"
wallet = "~/.config/solana/id.json"
```

现在，当您运行 anchor deploy 时，您的程序将被部署到 devnet 集群。anchor test 命令也将使用 Anchor.toml 文件中指定的集群。

### Update the Program

可以通过将程序重新部署到相同的程序 ID 来更新 Solana 程序。 要更新程序，只需更改程序的代码并运行 anchor build 命令即可生成更新的 .so 文件。

```
anchor build
```

Then run the `anchor deploy` command to redeploy the updated program.

```
anchor deploy
```

### Close the Program

要收回分配给程序帐户的 SOL，您可以关闭 Solana 程序。 要关闭程序，请使用 solana program close <PROGRAM_ID> 命令。例如：

```
solana program close 3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg --bypass-warning
```

请注意，一旦关闭某个程序，则不能重新使用该程序 ID 来部署新程序。

## Project File Structure

以下是 Anchor 工作区中默认文件结构的概述：

```
.
├── .anchor
│   └── program-logs
├── app
├── migrations
├── programs
│   └── [project-name]
│       └── src
│           ├── lib.rs
│           ├── Cargo.toml
│           └── Xargo.toml
├── target
│   ├── deploy
│   │   └── [project-name]-keypair.json
│   ├── idl
│   │   └── [project-name].json
│   └── types
│       └── [project-name].ts
├── tests
│   └── [project-name].ts
├── Anchor.toml
├── Cargo.toml
└── package.json
```

### Programs Folder

/programs 文件夹包含项目的 Anchor 程序。单个工作区可以包含多个程序。

### Tests Folder

/tests 文件夹包含项目的测试文件。创建项目时会为您创建一个默认测试文件。

### Target Folder

/target 文件夹包含构建输出。

- 主要子文件夹包括： /deploy：包含程序的密钥对和程序二进制文件。 
- /idl：包含程序的 JSON IDL。 
- /types：包含 IDL 的 TypeScript 类型。

### Anchor.toml File 

Anchor.toml 文件为您的项目配置工作区设置。

### .anchor Folder [#](https://solana.com/docs/programs/anchor#anchor-folder)

包括一个程序日志文件，其中包含上次运行测试文件的事务日志。

### App Folder [#](https://solana.com/docs/programs/anchor#app-folder)

/app 文件夹是一个空文件夹，您可以选择将其用于前端代码。





