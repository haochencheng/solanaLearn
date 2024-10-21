课程一

### 安装solana

https://solana.com/docs/intro/installation

mac系统

安装solana开发需要的依赖

#### 安装rust 

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

验证安装

```
. "$HOME/.cargo/env"

 rustc --version
```



#### 安装solana

```
 sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

写入环境变量

```
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.zshrc
```



```
source ~/.zshrc
```

验证安装

```
solana --version
```

显示

```
solana-cli 1.18.25 (src:92ddaa20; feat:3241752014, client:Agave)
```

####	升级solana-cli

```
agave-install update
```



### 安装Anchor-cli

Anchor是一个用于开发Solana程序的框架。Anchor框架利用Rust宏来简化编写Solana程序的过程。

安装Anchor CLI和工具有两种方法：

使用Anchor版本管理器（AVM）- 这是推荐的安装方法，因为它简化了将来更新Anchor版本的过程。
不使用AVM - 这种方法需要手动更新Anchor版本。

### 安装avm

```
cargo install --git https://github.com/coral-xyz/anchor avm --force
```

#### 验证avm安装

```
avm --version
```

#### 安装最新版本 Anchor CLI 

```
avm install latest
avm use latest
```

安装问题

```
error[E0282]: type annotations needed for `Box<_>`
  --> /Users/haochencheng/.cargo/registry/src/index.crates.io-6f17d22bba15001f/time-0.3.29/src/format_description/parse/mod.rs:83:9
   |
83 |     let items = format_items
   |         ^^^^^
...
86 |     Ok(items.into())
   |              ---- type must be known at this point
   |
   = note: this is an inference error on crate `time` caused by an API change in Rust 1.80.0; update `time` to version `>=0.3.35` by calling `cargo update`

   Compiling solana-security-txt v1.1.1
   Compiling unicode-segmentation v1.10.1
   Compiling heck v0.3.3
   Compiling console v0.15.8
For more information about this error, try `rustc --explain E0282`.
error: could not compile `time` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
error: failed to compile `anchor-cli v0.30.1 (https://github.com/coral-xyz/anchor?tag=v0.30.1#e6d7dafe)`, intermediate artifacts can be found at `/var/folders/dh/lkz1zdyd611dn_4lmn5dpz840000gn/T/cargo-installu8lnF0`.
To reuse those artifacts with a future compilation, set the environment variable `CARGO_TARGET_DIR` to that path.
Error: Failed to install 0.30.1, is it a valid version?
```

解决

```
rustup default 1.79.0
```



验证安装

```
anchor --version
```



###	安装Node.js and Yarn

yarn是nodejs 依赖包管理器

#### 安装nvm 

nvm可以选择版本安装nodejs

```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash
```

验证安装

```
command -v nvm
```



#### 安装node

```
nvm install node
```

验证安装

```
node --version
```









