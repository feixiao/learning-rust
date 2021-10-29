## Rust 学习资料

### 常用操作

#### 安装

- 国内源加速

  ```shell
  echo '''
    export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
    export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
    export PATH="$HOME/.cargo/bin:$PATH"
    ''' >> ~/.profile
  ```

- WSL

  ```shell
  sudo apt-get install build-essential
  source ~/.profile
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  # 切换stable版本
  rustup update stable
  ```
  
+ Windows
  ```shell
  #用于更新 toolchain
  
  set RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
  
  #用于更新 rustup
  
  set RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
  
  # 运行rust-init.exe
  ```

#### 更新和卸载

```shell
# 更新
rustup update
# 卸载
rustup self uninstall
```

#### cargo

- 创建项目

```shell
cargo new hello_cargo
```

- 编译和运行

```shell
cd hello_cargo

# 编译
cargo build

# 运行
cargo run
```

#### 本地文档

```shell
rustup doc
```

#### IDE

- Clion + Rust插件

### 内容概述

- [《Rust 权威指南》代码](./TheRustProgrammingLanguage)
- [rust-by-example](https://doc.bccnsoft.com/docs/rust-1.36.0-docs-html/rust-by-example/index.html)
- [bindgen](https://rust-lang.github.io/rust-bindgen/)    Rust调用C/C++.


#### Rust写操作系统
+ [《rCore-Tutorial-Book-v3》](https://rcore-os.github.io/rCore-Tutorial-Book-v3)

### Rust in Android
+ [《Rust/C++ interop in the Android Platform》](https://security.googleblog.com/2021/06/rustc-interop-in-android-platform.html)
+ [《cross-platform-rust》](https://github.com/feixiao/cross-platform-rust)