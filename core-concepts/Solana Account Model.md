在Solana上，所有数据都存储在所谓的“账户”中。Solana上数据的组织方式类似于键值存储，数据库中的每个条目都被称为“账户”。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/accounts/accounts.svg">



## Key Points 

账户可以存储最多10MB的数据，这些数据可以是可执行程序代码或程序状态。

账户需要按存储的数据量支付以SOL计的租金押金，关闭账户时该押金是全额可退的。

每个账户都有一个程序“所有者”。只有拥有该账户的程序才能修改其数据或扣除其lamport余额。不过，任何人都可以增加余额。

程序（智能合约）是无状态账户，用于存储可执行代码。

数据账户由程序创建，用于存储和管理程序状态。

原生程序是与Solana运行时一起提供的内置程序。

Sysvar账户是存储网络集群状态的特殊账户。



## Account

每个账户都有一个独特的地址，可以用32个字节表示，格式为Ed25519公钥。你可以把这个地址看作是账户的唯一标识。

互联网中的ipv4，linux中的pwd，sol中的pda

<img src="https://solana-developer-content.vercel.app/assets/docs/core/accounts/account-address.svg">



账户和地址之间的关系可以看作是一对键值对，其中地址作为键来定位账户对应的链上数据。



### AccountInfo

账户的最大大小为 10MB（10 兆字节），Solana 上每个账户上存储的数据都具有以下称为 AccountInfo 的结构。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/accounts/accountinfo.svg">

每个账户的AccountInfo包含以下字段：

数据：一个字节数组，用于存储账户的状态。如果账户是程序（智能合约），则存储可执行的程序代码。这个字段通常被称为“账户数据”。
可执行：一个布尔标志，指示账户是否为程序。
lamports：账户余额的数字表示，单位是lamports，lamports是SOL的最小单位（1 SOL = 10亿lamports）。
所有者：指定拥有该账户的程序的公钥（程序ID）。



作为Solana账户模型的关键部分，Solana上的每个账户都有一个指定的“拥有者”，特指一个程序。只有被指定为账户拥有者的程序才能修改账户中存储的数据或扣除lamport余额。需要注意的是，虽然只有拥有者可以扣减余额，但任何人都可以增加余额。



要在链上存储数据，必须将一定数量的SOL转移到账户中。转移的金额与账户上存储的数据大小成正比。这个概念通常被称为“租金”。不过，你可以把“租金”更看作是一种“押金”，因为分配给账户的SOL在账户关闭时可以全部回收。



## Native Programs

Solana 有一些本地程序，它们是验证者实现的一部分，提供网络的各种核心功能。完整的本地程序列表可以在这里找到。

在 Solana 上开发自定义程序时，您通常会与两个本地程序进行交互，分别是系统程序和 BPF 加载器。



### System Program

默认情况下，所有新账户由系统程序拥有。系统程序执行几个关键任务，例如：

新账户创建：仅系统程序可以创建新账户。
空间分配：设置每个账户数据字段的字节容量。
分配程序所有权：一旦系统程序创建了一个账户，它可以将指定的程序所有者重新分配给另一个程序账户。这就是自定义程序如何接管系统程序创建的新账户的方式。

在 Solana 上，“钱包”只是系统程序拥有的一个账户。钱包的 lampor 余额就是该账户拥有的 SOL 数量。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/accounts/system-account.svg">

只有系统程序拥有的账户才可以作为交易费用的支付者。

### BPFLoader Program

BPF Loader 是网络上所有其他程序（不包括 Native 程序）的“所有者”。它负责部署、升级和执行自定义程序。

## Sysvar Accounts

Sysvar 帐户是位于预定义地址的特殊帐户，可用于访问集群状态数据。这些帐户会使用有关网络集群的数据进行动态更新。您可以在此处找到 Sysvar 帐户的完整列表.用于集群同步

## Custom Programs

在Solana上，“智能合约”被称为程序。程序是一个包含可执行代码的账户，它的“可执行”标志被设置为true。

有关程序部署过程的详细说明，请参阅本文件中的“部署程序”页面。

### Program Account

在Solana上部署新程序时，实际上会创建三个独立的账户：

程序账户：代表链上程序的主要账户。这个账户存储一个可执行数据账户的地址（该账户存储编译后的程序代码）和程序的更新权限（被授权对程序进行更改的地址）。
程序可执行数据账户：一个包含程序可执行字节码的账户。
缓冲账户：一个临时账户，用于在程序被积极部署或升级时存储字节码。一旦过程完成，数据会转移到程序可执行数据账户，缓冲账户也会被关闭。

例如，这里是代币扩展程序账户及其对应程序可执行数据账户的 Solana Explorer 链接。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/accounts/program-account-expanded.svg">

为了简单起见，您可以将“程序帐户”视为程序本身。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/accounts/program-account-simple.svg">

“程序账户”的地址，也就是我们通常说的“程序ID”，用于调用该程序。

### Data Account

Solana 程序是“无状态的”，这意味着程序账户仅包含程序的可执行字节码。要存储和修改其他数据，必须创建新账户。这些账户通常称为“数据账户”。

数据账户可以存储所有者程序代码中定义的任意数据。

<img src="https://solana-developer-content.vercel.app/assets/docs/core/accounts/data-account.svg">

请注意，只有系统程序才能创建新帐户。系统程序创建帐户后，即可将新帐户的所有权转让给其他程序。

换句话说，为自定义程序创建数据帐户需要两个步骤：

调用系统程序创建一个帐户，然后将所有权转移给自定义程序 调用现在拥有该帐户的自定义程序，然后按照程序代码中的定义初始化帐户数据

这个数据账户创建过程通常被抽象为一个步骤，但了解底层过程是有帮助的。



















































