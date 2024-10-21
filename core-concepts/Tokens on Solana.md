# [Tokens on Solana](https://solana.com/docs/core/tokens)

代币是代表不同类别资产所有权的数字资产。代币化使产权数字化，成为管理可替代资产和非可替代资产的基本组成部分。

同质化代币代表相同类型和价值的可互换和可分割资产（例如 USDC）。 非同质化代币 (NFT) 代表不可分割资产（例如艺术品）的所有权。

本节将介绍 Solana 上代币的基本表示方法。这些代币被称为 SPL（Solana 程序库）代币。

- 代币程序包含与网络上的代币（可替代和不可替代）交互的所有指令逻辑。

- 铸币账户代表一种特定类型的代币，并存储有关代币的全局元数据，例如总供应量和铸币权限（授权创建代币新单位的地址）。

- 代币账户跟踪个人所有权，即特定地址拥有多少特定类型的代币（铸币账户）单位。

**Info**

​	目前有两个版本的代币计划。原始代币计划和代币扩展计划（Token2022）。代币扩展计划的功能与原始代币计划相同，

​	但具有附加功能和改进。代币扩展程序是用于创建新代币（铸造账户）的推荐版本。

## Key Points

- 代币代表对可替代（可互换）或不可替代（独特）资产的所有权。
- 代币程序包含与网络上的可替代代币和不可替代代币进行交互的所有指令。
- 代币扩展程序是代币程序的新版本，它在保持相同核心功能的同时包含附加功能。
- Mint 账户代表网络上的唯一代币，并存储总供应量等全局元数据。
- 代币账户追踪特定铸币账户的代币个人所有权
- 关联代币账户是使用源自所有者和铸币账户地址的地址创建的代币账户。



## Token Program

代币程序包含与网络上的代币（可替代和不可替代）交互的所有指令逻辑。 Solana 上的所有代币实际上都是代币程序拥有的数据账户。

您可以在此处找到代币计划说明的完整列表。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/tokens/token-program.svg">

一些常用的指令包括：

- InitializeMint：创建一个新的铸币账户来代表一种新类型的代币。

- InitializeAccount：创建一个新的代币账户来持有特定类型代币（铸币）的单位。
- InitializeMint：创建一个新的铸币账户来代表一种新类型的代币。 InitializeAccount：创建一个新的代币账户来持有特定类型代币（铸币）的单位。
- 转移：将特定类型的代币单位从一个代币账户转移到另一个代币账户。

### Mint Account

Solana 上的代币由代币计划拥有的 Mint 账户地址唯一标识。此帐户实际上是特定代币的全局计数器，并存储以下数据：

- Supply: 代币的总供应量
- Decimals: 令牌的小数精度
- Mint authority: 该账户有权创建新的代币单位，从而增加供应量
- Freeze authority: 授权冻结从“代币账户”转移代币的账户

<img src="https://solana-developer-content.vercel.app/assets/docs/core/tokens/mint-account.svg">

每个 Mint 帐户中存储的完整详细信息包括以下内容：

```
pub struct Mint {
    /// Optional authority used to mint new tokens. The mint authority may only
    /// be provided during mint creation. If no mint authority is present
    /// then the mint has a fixed supply and no further tokens may be
    /// minted.
    pub mint_authority: COption<Pubkey>,
    /// Total supply of tokens.
    pub supply: u64,
    /// Number of base 10 digits to the right of the decimal place.
    pub decimals: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Optional authority to freeze token accounts.
    pub freeze_authority: COption<Pubkey>,
}
```



For reference, here is a Solana Explorer link to the [USDC Mint Account](https://explorer.solana.com/address/EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v).

### Token Account

为了追踪特定代币的每个单位的个人所有权，必须创建代币计划拥有的另一种数据账户。此账户称为代币账户。

代币账户中存储的最常见引用数据包括以下内容：

- Mint：代币账户持有的代币类型
- Owner: 有权将代币从代币账户转出的账户
- Amount: 代币账户当前持有的代币单位

<img src="https://solana-developer-content.vercel.app/assets/docs/core/tokens/token-account.svg">



每个代币账户上存储的完整详细信息包括以下内容：

```
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u64,
    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate
    pub delegate: COption<Pubkey>,
    /// The account's state
    pub state: AccountState,
    /// If is_native.is_some, this is a native token, and the value logs the
    /// rent-exempt reserve. An Account is required to be rent-exempt, so
    /// the value is used by the Processor to ensure that wrapped SOL
    /// accounts do not drop below this threshold.
    pub is_native: COption<u64>,
    /// The amount delegated
    pub delegated_amount: u64,
    /// Optional authority to close the account.
    pub close_authority: COption<Pubkey>,
}
```

一个钱包要想拥有某种代币的单位，就需要为特定类型的代币（mint）创建一个代币账户，该账户将钱包指定为代币账户的所有者。

一个钱包可以为同一类型的代币创建多个代币账户，但每个代币账户只能由一个钱包拥有并持有一种代币的单位。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/tokens/token-account-relationship.svg">



请注意，每个代币账户的数据都包含一个所有者字段，用于标识谁对该特定代币账户拥有权限。这与 AccountInfo 中指定的程序所有者不同，后者是所有代币账户的代币程序。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/tokens/associated-token-account.svg">

这引入了 Solana 开发中的一个关键概念：程序派生地址 (PDA)。从概念上讲，PDA 提供了一种使用一些预定义输入生成地址的确定性方法。这使得我们以后可以轻松找到帐户的地址。

以下是 Solana Playground 的示例，它导出了 USDC 关联代币账户地址和所有者。它将始终为同一铸币厂和所有者生成相同的地址。

```
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
 
const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
  USDC_MINT_ADDRESS,
  OWNER_ADDRESS,
);
```



具体来说，关联代币账户的地址是使用以下输入得出的。这是一个 Solana Playground 示例，它生成与上一个示例相同的地址。

```
import { PublicKey } from "@solana/web3.js";
 
const [PDA, bump] = PublicKey.findProgramAddressSync(
  [
    OWNER_ADDRESS.toBuffer(),
    TOKEN_PROGRAM_ID.toBuffer(),
    USDC_MINT_ADDRESS.toBuffer(),
  ],
  ASSOCIATED_TOKEN_PROGRAM_ID,
);
```

如果两个钱包要存放相同类型的代币，则每个钱包都需要有自己的代币账户来存放特定的铸币账户。下图展示了这种账户关系。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/tokens/token-account-relationship-ata.svg">



## Token Examples

spl-token CLI 可用于试验 SPL 令牌。在下面的示例中，我们将使用 Solana Playground 终端直接在浏览器中运行 CLI 命令，而无需在本地安装 CLI。

创建代币和账户需要 SOL 用于账户租金押金和交易费用。如果这是您第一次使用 Solana Playground，请创建一个 Playground 钱包并在 Playground 终端中运行 solana 空投命令。您还可以使用公共网络水龙头获取 devnet SOL。

```
solana airdrop 2
```

运行 spl-token --help 获取可用命令的完整描述。

```
spl-token --help
```



在以下部分中，运行 CLI 命令时显示的帐户地址将与下面显示的示例输出不同。请在操作过程中使用 Playground 终端中显示的地址。例如，create-token 输出的地址是您的 Playground 钱包设置为铸币机构的铸币账户。

### Create a New Token

要创建新的代币（mint 账户），请在 Solana Playground 终端中运行以下命令。

```
spl-token create-token
```

您应该会看到类似下面的输出。您可以使用地址和签名在 Solana Explorer 上检查代币和交易详细信息。

在下面的示例输出中，新代币的唯一标识符（地址）是 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg。

```
Creating token 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg
 
Address:  99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg
Decimals:  9
 
Signature: 44fvKfT1ezBUwdzrCys3fvCdFxbLMnNvBstds76QZyE6cXag5NupBprSXwxPTzzjrC3cA6nvUZaLFTvmcKyzxrm1
```



新代币最初没有供应。您可以使用以下命令检查代币的当前供应量：

```
spl-token supply <TOKEN_ADDRESS>

```

对新创建的代币运行 supply 命令将返回值 0：

```
spl-token supply 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg
0

```

在底层，创建新的 Mint 帐户需要发送包含两个指令的交易。以下是 Solana Playground 上的 Javascript 示例。

- 调用系统程序创建一个具有足够空间容纳 Mint 账户数据的新账户，然后将所有权转移给代币程序。 
- 调用代币程序将新账户的数据初始化为 Mint 账户

### Create Token Account 

要持有特定代币的单位，您必须首先创建一个代币账户。要创建新的代币账户，请使用以下命令：

```
spl-token create-account [OPTIONS] <TOKEN_ADDRESS>
```

例如，在 Solana Playground 终端中运行以下命令：

```
spl-token create-account 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg
```

Returns the following output:

- `AfB7uwBEsGtrrBqPTVqEgzWed5XdYfM1psPNLmf7EeX9` is the address of the token account created to hold units of the token specified in the `create-account` command.

```
Creating account AfB7uwBEsGtrrBqPTVqEgzWed5XdYfM1psPNLmf7EeX9

Signature: 2BtrynuCLX9CNofFiaw6Yzbx6hit66pup9Sk7aFjwU2NEbFz7NCHD9w9sWhrCfEd73XveAGK1DxFpJoQZPXU9tS1
```

默认情况下，create-account 命令会创建一个关联的代币账户，并以您的钱包地址作为代币账户所有者。 您可以使用以下命令创建具有不同所有者的代币账户：

```
spl-token create-account --owner <OWNER_ADDRESS> <TOKEN_ADDRESS>
```

For example, running the following command:

```
spl-token create-account --owner 2i3KvjDCZWxBsqcxBHpdEaZYQwQSYE6LXUMx5VjY5XrR 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg
```



返回以下输出： Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt 是为保存 create-account 命令中指定的代币单位而创建的代币账户的地址 (99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg)并由 --owner 标志 (2i3KvjDCZWxBsqcxBHpdEaZYQwQSYE6LXUMx5VjY5XrR) 后指定的地址拥有。当您需要为其他用户创建代币账户时，这很有用。

```
Creating account Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt
 
Signature: 44vqKdfzspT592REDPY4goaRJH3uJ3Ce13G4BCuUHg35dVUbHuGTHvqn4ZjYF9BGe9QrjMfe9GmuLkQhSZCBQuEt
```

在底层，创建关联代币账户需要一条调用关联代币程序的指令。这是 Solana Playground 上的 Javascript 示例。

- [Invoking the System Program](https://github.com/solana-labs/solana-program-library/blob/b1c44c171bc95e6ee74af12365cb9cbab68be76c/associated-token-account/program/src/tools/account.rs#L19) to create a new account using the provided PDA as the address of the new account

- [Invoking the Token Program](https://github.com/solana-labs/solana-program-library/blob/b1c44c171bc95e6ee74af12365cb9cbab68be76c/associated-token-account/program/src/processor.rs#L138-L161) to initialize the Token Account data for the new account.

或者，使用随机生成的密钥对（不是关联代币账户）创建新代币账户需要发送包含两个指令的交易。以下是 Solana Playground 上的 Javascript 示例。

- 调用系统程序创建一个具有足够空间容纳代币账户数据的新账户，然后将所有权转移给代币程序。 
- 调用代币程序将新账户的数据初始化为代币账户

### Mint Tokens

To create new units of a token, use the following command:

```
spl-token mint [OPTIONS] <TOKEN_ADDRESS> <TOKEN_AMOUNT> [--] [RECIPIENT_TOKEN_ACCOUNT_ADDRESS]
```

For example, running the following command:

```
spl-token mint 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg 100
```

返回以下输出： 

```
Minting 100 tokens
  Token: 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg
  Recipient: AfB7uwBEsGtrrBqPTVqEgzWed5XdYfM1psPNLmf7EeX9
 
Signature: 2NJ1m7qCraPSBAVxbr2ssmWZmBU9Jc8pDtJAnyZsZJRcaYCYMqq1oRY1gqA4ddQno3g3xcnny5fzr1dvsnFKMEqG
```

- 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg 是正在为其铸造代币的铸币账户的地址（增加总供应量）。
- AfB7uwBEsGtrrBqPTVqEgzWed5XdYfM1psPNLmf7EeX9 是您的钱包的代币账户的地址，代币单位正在被铸造到该地址（数量增加）。

要将代币铸造到不同的代币账户，请指定预期接收者代币账户的地址。例如，运行以下命令：

```
spl-token mint 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg 100 -- Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt
```

返回以下输出： 

```
Minting 100 tokens
  Token: 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg
  Recipient: Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt
 
Signature: 3SQvNM3o9DsTiLwcEkSPT1Edr14RgE2wC54TEjonEP2swyVCp2jPWYWdD6RwXUGpvDNUkKWzVBZVFShn5yntxVd7
```

- 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg 是正在为其铸造代币的铸币账户的地址（增加总供应量）。

- Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt 是代币账户的地址，代币单位正在被铸造到该账户（数量增加）。

在底层，创建新的代币单位需要调用代币程序上的 MintTo 指令。该指令必须由铸币机构签署

该指令将新的代币单位铸造到代币账户，并增加铸造账户的总供应量。以下是 Solana Playground 上的 Javascript 示例。

### Transfer Tokens

要在两个代币账户之间转移代币单位，请使用以下命令：

```
spl-token transfer [OPTIONS] <TOKEN_ADDRESS> <TOKEN_AMOUNT> <RECIPIENT_ADDRESS
or RECIPIENT_TOKEN_ACCOUNT_ADDRESS>
```



For example, running the following command:

```
spl-token transfer 99zqUzQGohamfYxyo8ykTEbi91iom3CLmwCA75FK5zTg 100 Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt
```



返回以下输出： 

- AfB7uwBEsGtrrBqPTVqEgzWed5XdYfM1psPNLmf7EeX9 是代币从中转移的代币账户的地址。这将是您转移的指定代币的代币账户的地址。
- Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt 是代币正在转移到的代币账户的地址。

```
Transfer 100 tokens
  Sender: AfB7uwBEsGtrrBqPTVqEgzWed5XdYfM1psPNLmf7EeX9
  Recipient: Hmyk3FSw4cfsuAes7sanp2oxSkE9ivaH6pMzDzbacqmt
 
Signature: 5y6HVwV8V2hHGLTVmTmdySRiEUCZnWmkasAvJ7J6m7JR46obbGKCBqUFgLpZu5zQGwM4Xy6GZ4M5LKd1h6Padx3o
```

在底层，转移代币需要调用代币程序上的转移指令。此指令必须由发送者的代币账户所有者签名。

该指令将代币单位从一个代币账户转移到另一个代币账户。以下是 Solana Playground 上的 Javascript 示例。

重要的是要理解，发送者和接收者都必须拥有针对要传输的特定类型的代币的现有代币账户。

发送者可以在交易中包含额外的指令来创建接收者的代币账户，通常是关联代币账户

### Create Token Metadata

代币扩展程序允许将额外的可定制元数据（如名称、符号、图像链接）直接存储在 Mint 帐户中。

要使用 Token Extensions CLI 标志，请确保您已在本地安装 CLI，版本 3.4.0 或更高版本： cargo install --version 3.4.0 spl-token-cli



要创建启用元数据扩展的新令牌，请使用以下命令：

```
spl-token create-token --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
--enable-metadata
```



该命令返回以下输出： BdhzpzhTD1MFqBiwNdrRy4jFo2FHFufw3n9e8sVjJczP 是启用元数据扩展后创建的新代币的地址。



```
Creating token BdhzpzhTD1MFqBiwNdrRy4jFo2FHFufw3n9e8sVjJczP under program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
To initialize metadata inside the mint, please run `spl-token initialize-metadata BdhzpzhTD1MFqBiwNdrRy4jFo2FHFufw3n9e8sVjJczP <YOUR_TOKEN_NAME> <YOUR_TOKEN_SYMBOL> <YOUR_TOKEN_URI>`, and sign with the mint authority.
 
Address:  BdhzpzhTD1MFqBiwNdrRy4jFo2FHFufw3n9e8sVjJczP
Decimals:  9
 
Signature: 5iQofFeXdYhMi9uTzZghcq8stAaa6CY6saUwcdnELST13eNSifiuLbvR5DnRt311frkCTUh5oecj8YEvZSB3wfai
```

一旦创建了启用元数据扩展的新令牌，请使用以下命令初始化元数据。

```
spl-token initialize-metadata <TOKEN_MINT_ADDRESS> <YOUR_TOKEN_NAME>
<YOUR_TOKEN_SYMBOL> <YOUR_TOKEN_URI>
```

代币 URI 通常是指向您想要与代币关联的链下元数据的链接。您可以在此处找到 JSON 格式的示例。

例如，运行以下命令将把附加元数据直接存储在指定的 mint 帐户上：

```
spl-token initialize-metadata BdhzpzhTD1MFqBiwNdrRy4jFo2FHFufw3n9e8sVjJczP "TokenName" "TokenSymbol" "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json"
```

然后，您可以在浏览器上查找铸币账户的地址以检查元数据。例如，这是在 SolanaFm 浏览器上启用元数据扩展创建的代币。

您可以在元数据扩展指南中了解更多信息。有关各种 Token 扩展的更多详细信息，请参阅 Token 扩展入门指南和 SPL 文档。





























