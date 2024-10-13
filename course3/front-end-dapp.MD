### 安装solana-dapp 

安装npnm

```
https://pnpm.io/installation
```

```
升级node到带有corepack的版本
开启corepack 
corepack enable
查看时候有corepack
npm -g list
```



#### 安装[create-solana-dapp](https://github.com/solana-developers/create-solana-dapp)

```
https://github.com/solana-developers/create-solana-dapp?tab=readme-ov-file#create-solana-dapp
```

开启pnpm设置镜像

```
npm install -g pnpm
corepack enable pnpm

export COREPACK_NPM_REGISTRY=https://registry.npmmirror.com

```

```
git clone https://github.com/solana-developers/create-solana-dapp.git
cd create-solana-dapp

设置npm镜像 
npm config set registry https://registry.npmmirror.com

构建
pnpm install
pnpm build
```

构建成功生成静态文件

```
✔ Build succeeded for create-solana-dapp                                                  09:33:39
  dist/bin/index.cjs (total size: 31.2 kB, chunk size: 437 B)                              09:33:39
  └─ dist/index.cjs (30.8 kB)

  dist/index.cjs (total size: 30.8 kB, chunk size: 30.8 kB, exports: main)                 09:33:39

  dist/bin/index.mjs (total size: 30.1 kB, chunk size: 381 B)                              09:33:39
  └─ dist/index.mjs (29.8 kB)

  dist/index.mjs (total size: 29.8 kB, chunk size: 29.8 kB, exports: main)
```

nginx代理静态文件

```

```

