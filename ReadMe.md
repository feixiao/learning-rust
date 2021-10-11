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

- Clion + Rust

### 内容概述

- [《Rust 权威指南》代码](./TheRustProgrammingLanguage)
