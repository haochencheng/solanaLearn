# [Fees on Solana](https://solana.com/docs/core/fees)

Solana 区块链在使用无需许可的网络时会产生几种不同类型的费用和成本。这些费用和成本可以分为几种特定的类型：

- 交易费 - 让验证者处理交易/指令的费用 优先费 
- 提高交易处理顺序的可选费用 
- 租金 - 保留余额以将数据存储在链上

## Transaction Fees

在 Solana 区块链上的链上程序中处理逻辑（指令）所支付的小额费用称为“交易费”。

当每个交易（包含一个或多个指令）通过网络发送时，它会由当前验证器领导者处理。

一旦确认为全局状态交易，该交易费将支付给网络，以帮助支持 Solana 区块链的经济设计。

交易费用与租金的账户数据存储押金费用不同。

在支付交易费用来处理 Solana 网络上的指令时，会将租金押金保留在一个账户中，以将其数据存储在区块链上并可回收。

目前，Solana 的基本交易费用设定为每个签名 5k lampors 的静态值。在此基本费用的基础上，可以添加任何额外的优先费用。

### Why pay transaction fees?

交易费在 Solana 经济设计中提供了许多好处，主要是：

- 为验证器网络处理交易所需的 CPU/GPU 计算资源提供补偿
- reduce network spam by introducing a real cost to transactions
- provide long-term economic stability to the network through a protocol-captured minimum fee amount per transaction

### Basic economic design

许多区块链网络（包括比特币和以太坊）依靠通胀协议奖励来在短期内保护网络。从长远来看，这些网络将越来越依赖交易费来维持安全。

Solana 也是如此。具体来说： 每笔交易费用的固定比例（最初为 50%）被烧毁（销毁），剩余部分归处理交易的当前领导者所有。

预定的全球通货膨胀率为分配给 Solana 验证者的奖励提供了来源。

### Fee collection

交易需要至少有一个已签署交易且可写入的账户。这些可写入的签名者账户在账户列表中首先被序列化，其中第一个账户始终用作“费用支付者”。

在处理任何交易指令之前，费用支付者账户余额将被扣除以支付交易费用。

如果费用支付者的余额不足以支付交易费用，交易处理将停止并导致交易失败

如果余额足够，费用将被扣除并且交易指令将开始执行。

如果任何指令导致错误，交易处理将停止，并最终在 Solana 分类账中记录为失败交易。运行时仍会对这些失败的交易收取费用。

如果任何指令返回错误或违反运行时限制，则除交易费扣除之外的所有账户更改都将被回滚。

这是因为验证器网络已经消耗了计算资源来收集交易并开始初始处理。

### Fee distribution 

交易费用部分被烧毁，剩余费用由生成包含相应交易的区块的验证者收取。

具体来说，50％被烧毁，50％分配给生成区块的验证者。

### Why burn some fees

如上所述，每笔交易费用的固定比例会被销毁。这是为了巩固 SOL 的经济价值，从而维持网络的安全。

与交易费用完全被烧毁的方案不同，领导者仍然有动力在他们的位置（创建区块的机会）中包含尽可能多的交易。

销毁费用还可以通过在分叉选择中加以考虑，帮助防止恶意验证者审查交易。

#### Example of an attack

如果出现恶意或审查领导者的历史证明（PoH）分叉：

- 由于审查费用的损失，我们预计总费用的销毁量将低于类似的诚实分叉

- 如果审查领导者要补偿这些丢失的协议费用，他们就必须自己在分叉上替换已烧毁的费用

- 从而潜在地减少了审查的动机

### Calculating transaction fees

特定交易的完整费用基于两个主要部分计算：

- 每个签名静态设定的基本费用，
- 以及 交易过程中使用的计算资源，以“计算单位”衡量

由于每笔交易可能需要不同数量的计算资源，因此每笔交易都会分配一定数量的计算单元作为计算预算的一部分。

## Compute Budget

为了防止滥用计算资源，每个交易都分配有一个“计算预算”。此预算指定了有关计算单元的详细信息，包括：

- 与交易可能执行的不同类型的操作相关的计算成本（每个操作消耗的计算单元），
- 交易可以消耗的最大计算单元数（计算单元限制），
- 以及交易必须遵守的操作界限（如账户数据大小限制）

当交易消耗了整个计算预算（计算预算耗尽），或者超出了某个界限（例如试图超出最大调用堆栈深度或最大加载帐户数据大小限制）时，运行时会暂停交易处理并返回错误。导致交易失败且状态不会发生变化（除了收取交易费）。

### Accounts data size limit

交易可以通过包含 SetLoadedAccountsDataSizeLimit 指令来指定允许加载的帐户数据的最大字节数（不超过运行时的绝对最大值）。

如果没有提供 SetLoadedAccountsDataSizeLimit，则事务默认使用运行时的 MAX_LOADED_ACCOUNTS_DATA_SIZE_BYTES 值。

ComputeBudgetInstruction::set_loaded_accounts_data_size_limit 函数可用于创建此指令：

```
let instruction = ComputeBudgetInstruction::set_loaded_accounts_data_size_limit(100_000);
```



### Compute units

交易中在链上执行的所有操作都需要验证者在处理时消耗不同数量的计算资源（计算成本）。

这些资源消耗的最小计量单位称为“计算单位”。

在处理交易时，链上执行的每个指令都会逐步消耗计算单元（消耗预算）。

由于每条指令执行不同的逻辑（写入账户、cpi、执行系统调用等），因此每条指令可能消耗不同数量的计算单元。

程序可以记录有关其计算使用情况的详细信息，包括其分配的计算预算中剩余的金额。您还可以在本指南中找到有关优化计算使用情况的更多信息。

每个事务都分配有一个计算单元限制，可以是运行时设置的默认限制，也可以是明确请求更高的限制。

当交易超出其计算单元限制时，其处理将停止，从而导致交易失败。 以下是一些会产生计算成本的常见操作：

- executing instructions
- passing data between programs
- performing syscalls
- using sysvars
- logging with the `msg!` macro
- logging pubkeys
- creating program addresses (PDAs)
- cross-program invocations (CPI)
- cryptographic operations

对于跨程序调用，调用的指令将继承其父级的计算预算和限制。如果调用的指令消耗了交易的剩余预算，或者超出了界限，整个调用链和顶层交易处理就会停止。

您可以在 Solana 运行时的 ComputeBudget 中找到有关消耗计算单元的所有操作的更多详细信息。

### Compute unit limit

每个交易都有一个它可以消耗的最大计算单元 (CU) 数量，称为“计算单元限制”。对于每个交易，Solana 运行时的绝对最大计算单元限制为 140 万 CU，并设置每个指令的默认请求最大限制为 20 万 CU。

事务可以通过包含单个 SetComputeUnitLimit 指令来请求更具体、更优化的计算单元限制。可以是更高或更低的限制。但它绝不能请求高于每个事务的绝对最大限制。

虽然事务的默认计算单元限制在大多数情况下适用于简单事务，但它们通常不是最佳的（对于运行时和用户而言）。

对于更复杂的交易，例如调用执行多个 CPI 的程序，您可能需要为交易请求更高的计算单元限制。

为您的交易请求最佳计算单元限制对于帮助您减少交易费用以及帮助您在网络上更好地安排交易至关重要。

钱包、dApp 和其他服务应确保其计算单元请求是最佳的，以便为其用户提供最佳体验。

### Compute unit price

当交易希望支付更高的费用来提高其处理优先级时，它可以设置“计算单位价格”。此价格与计算单位限制结合使用，将用于确定交易的优先级费用。

默认情况下，没有设置计算单价，因此没有额外的优先费用。

## Prioritization Fees

作为计算预算的一部分，运行时支持支付可选费用（称为“优先级费用”）的交易。支付这笔额外费用有助于提高交易在处理时的优先级，从而缩短执行时间。

### How the prioritization fee is calculated

交易的优先费用通过将其计算单元限制乘以计算单元价格（以微单位衡量）来计算。这些值可以在每笔交易中设置一次，方法是添加以下计算预算指令：

- SetComputeUnitLimit - 设置交易可以消耗的最大计算单元数 

- SetComputeUnitPrice - 设置交易愿意支付的额外费用，以提高其优先级

如果没有提供 SetComputeUnitLimit 指令，则将使用默认计算单元限制。

如果没有提供 SetComputeUnitPrice 指令，交易将默认不收取额外提升费用且优先级最低（即无优先费用）。

### How to set the prioritization fee

交易的优先费用是通过包含 SetComputeUnitPrice 指令和可选的 SetComputeUnitLimit 指令来设置的。

运行时将使用这些值来计算优先级费用，该费用将用于对块内的给定交易进行优先级排序。

您可以通过 Rust 或 @solana/web3.js 函数来编写每条指令。然后可以将每条指令包含在交易中并像平常一样发送到集群。另请参阅下面的最佳实践。

与 Solana 交易中的其他指令不同，计算预算指令不需要任何账户。包含多个上述指令的交易将失败。

交易只能包含每种类型的计算预算指令之一。重复的指令类型将导致 TransactionError::DuplicateInstruction 错误，并最终导致交易失败。

#### Rust [#](https://solana.com/docs/core/fees#rust)

The rust `solana-sdk` crate includes functions within [`ComputeBudgetInstruction`](https://docs.rs/solana-sdk/latest/solana_sdk/compute_budget/enum.ComputeBudgetInstruction.html) to craft instructions for setting the *compute unit limit* and *compute unit price*:



```
let instruction = ComputeBudgetInstruction::set_compute_unit_limit(300_000);
```



```
let instruction = ComputeBudgetInstruction::set_compute_unit_price(1);
```

### Prioritization fee best practices

以下是有关优先费用最佳实践的一般信息。您还可以在本指南中找到有关如何请求最佳计算的更多详细信息，包括如何模拟交易以确定其大致的计算使用情况。

#### Request the minimum compute units [#](https://solana.com/docs/core/fees#request-the-minimum-compute-units)

交易应该请求执行所需的最少计算单位以尽量减少费用。另请注意，当请求的计算单位数量超过执行交易实际消耗的计算单位数量时，费用不会调整。

#### Get recent prioritization fees [#](https://solana.com/docs/core/fees#get-recent-prioritization-fees)

在向集群发送交易之前，您可以使用 getRecentPrioritizationFees RPC 方法获取节点处理的最近块中最近支付的优先费用列表。

然后，您可以使用这些数据来估算您的交易的适当优先费用，以便 (a) 更好地确保它被集群处理，以及 (b) 最大限度地减少支付的费用。

## Rent

为保持其相关数据在链上可用而存入每个 Solana 帐户的费用称为“租金”。此费用在每个帐户的正常 Lamport 余额中扣除，并在帐户关闭时退还。

租金与交易费不同。租金是“支付的”（保留在账户中），用于保留存储在 Solana 区块链上的数据，并且可以回收。而交易费是支付的，用于处理网络上的指令。

所有账户都必须维持足够高的 Lamport 余额（相对于其分配的空间）才能免除租金并保留在 Solana 区块链上。

任何试图将账户余额减少到低于其各自的租金免除最低余额的交易都将失败（除非余额减少到正好为零）。

当账户所有者不再希望将这些数据保留在链上并在全球状态中可用时，所有者可以关闭该账户并收回租金押金。

这是通过将账户的全部 Lamport 余额提取（转移）到另一个账户（即您的钱包）来实现的。通过将账户余额减少到 0，

运行时将在“垃圾收集”过程中从网络中删除该帐户及其相关数据。

### Rent rate

Solana 租金率是在整个网络范围内设置的，主要基于运行时设置“每年每字节的 lamports”。目前，租金率是一个静态金额，存储在 Rent 系统变量中。

此租金率用于计算账户中分配给该账户的空间所需要预扣的确切租金金额，（即账户可存储的数据量）。账户分配的空间越大，预扣的租金押金就越高。

### Rent exempt

账户必须维持大于链上存储其各自数据所需的最低金额的 Lamport 余额。这称为“免租”，该余额称为“免租最低余额”。

Solana 上的新帐户（和程序）必须使用足够的 Lamport 进行初始化才能免租。但情况并非总是如此。

此前，运行时会定期自动从每个低于最低余额的账户中收取免租费用。

最终将这些账户的余额减少到零，并从全局状态中收集它们（除非手动充值）。

在创建新帐户的过程中，您必须确保存入足够的资金以高于此最低余额。低于此最低阈值的任何值都将导致交易失败

每次账户余额减少时，运行时都会检查账户是否仍高于免租最低余额。除非他们将最终余额减少到恰好 0

（关闭账户），导致账户余额低于免租门槛的交易将会失败。

账户免租的具体最低余额取决于区块链的当前租金率和账户想要分配的存储空间量（账户大小）。因此，建议使用

因此，建议使用 getMinimumBalanceForRentExemption RPC 端点来计算给定账户大小的具体余额。 也可以通过 solana rent CLI 子命令估算所需的租金押金金额：

当交易成功将账户余额减至 0 后，运行时会自动进行垃圾回收，垃圾收集由运行时自动进行。任何试图将账户余额减少到租金豁免最低余额以下（非零）的交易都将失败。

即使帐户已从网络中删除（通过垃圾收集），它仍可能具有与其地址相关的交易（过去的历史或将来的）。

即使 Solana 区块浏览器可能显示“未找到帐户”类型的消息，您仍然可以查看与该帐户相关的交易历史记录。

您可以阅读验证器实施的垃圾收集提案以了解更多信息。



