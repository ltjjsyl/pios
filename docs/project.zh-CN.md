# Raspberry Pi 5 学习型操作系统项目

这是一个面向 Raspberry Pi 5 的学习型操作系统项目。项目目标不是替代 Linux，而是从裸机启动开始，逐步实现一个可观察、可调试、便于学习操作系统核心概念的小内核。

## 项目目标

第一阶段目标：

- 使用 Rust `no_std` 编写 AArch64 裸机内核
- 使用少量汇编完成启动入口、栈初始化和 `.bss` 清零
- 在 Raspberry Pi 5 真机上启动 `kernel_2712.img`
- 通过 Raspberry Pi 5 板载 DEBUG UART 输出早期启动日志
- 建立后续异常、中断、内存管理、调度和 shell 的基础结构

当前第一阶段的期望串口输出包含：

```text
raspi5-os: Rust learning kernel
Hello from kernel
board: Raspberry Pi 5 / BCM2712 / AArch64
stage: bring-up
console: Raspberry Pi 5 DEBUG UART
```

## 当前目录结构

```text
.
├── boot/
│   └── config.txt              # Raspberry Pi 5 启动配置
├── docs/
│   ├── bringup.md              # 英文 bring-up 说明
│   └── project.zh-CN.md        # 中文项目说明
├── kernel/
│   ├── Cargo.toml              # 内核 crate 配置
│   └── src/
│       ├── main.rs             # 裸机入口 kernel_main
│       ├── lib.rs              # 可测试的内核公共逻辑
│       ├── console.rs          # 控制台抽象和行缓冲
│       ├── panic.rs            # panic 串口输出
│       ├── arch/aarch64/       # AArch64 启动、链接脚本和架构代码
│       └── board/raspi5/       # Raspberry Pi 5 板级代码
├── tools/
│   └── build.ps1               # 构建 ELF 并生成 kernel 镜像
├── .cargo/config.toml          # AArch64 裸机 target 的链接参数
└── Cargo.toml                  # workspace 配置
```

## 构建方式

需要安装 Rust 工具链组件：

```powershell
rustup target add aarch64-unknown-none
rustup component add llvm-tools-preview
```

运行 host 单元测试：

```powershell
cargo test -p kernel --lib
```

构建 Raspberry Pi 5 内核镜像：

```powershell
powershell -ExecutionPolicy Bypass -File .\tools\build.ps1
```

成功后会生成：

```text
target\raspi5\kernel_2712.img
target\raspi5\kernel8.img
```

`.cargo/config.toml` 不再把 `aarch64-unknown-none` 设为整个 workspace 的默认 target。这样 RustRover 和普通 host 测试会默认使用 Windows host 环境，避免把 `#[test]`、`assert!`、`assert_eq!` 等测试符号误判为裸机环境缺失。

`tools/build.ps1` 会在构建镜像时显式传入：

```powershell
cargo build -p kernel --release --target aarch64-unknown-none
```

因此裸机镜像仍然使用 AArch64 target、链接脚本和裸机 panic handler。`kernel/src/panic.rs` 中的 panic handler 只在非测试构建中启用，避免 RustRover 或 host 测试环境加载 `std` 时出现重复 `panic_impl` 诊断。

## 启动文件

Raspberry Pi 5 的 boot 分区需要包含官方固件文件、项目里的 `boot/config.txt`，以及生成的内核镜像：

```text
kernel_2712.img
```

当前 `boot/config.txt` 内容：

```ini
arm_64bit=1
kernel=kernel_2712.img
enable_uart=1
uart_2ndstage=1
```

`kernel8.img` 目前作为备用镜像生成，方便后续调试和兼容不同启动配置。

## 串口验证

当前项目以 Raspberry Pi 5 板载 3-pin DEBUG UART 作为第一阶段唯一调试接口。

推荐串口参数：

```text
115200 8N1
```

真机启动后，如果 DEBUG UART 接线正确，应能先看到 bootloader 日志，然后看到早期内核日志。当前 DEBUG UART MMIO 基址为：

```text
0x107d001000
```

40-pin GPIO14/GPIO15 上的 RP1 UART 暂不作为第一阶段控制台，后续会作为独立板级驱动再接入。

## 当前阶段

已经完成：

- Rust workspace 和裸机 kernel crate
- AArch64 启动汇编
- 链接脚本
- panic handler
- DEBUG UART 控制台输出路径
- `kernel_2712.img` / `kernel8.img` 构建脚本
- 基础 host 单元测试

尚未完成：

- 异常向量表
- IRQ 和定时器中断
- 物理页分配器
- 堆分配器
- MMU 和虚拟内存
- 任务调度
- UART shell

## 后续路线

建议按以下顺序继续推进：

1. 在真机上验证 UART 输出。
2. 增加异常向量表，能打印异常类型和关键寄存器。
3. 接入定时器中断，建立稳定 tick。
4. 实现简单物理页分配器和内存统计。
5. 实现协作式任务调度。
6. 增加 UART shell，支持 `help`、`mem`、`tasks`、`panic` 等命令。
7. 在核心纯 Rust 模块上持续补充 host 单元测试。

这个项目会优先保持每个阶段都有可观察结果：串口日志、测试输出或 shell 命令反馈。这样即使底层实现还很小，也能持续确认内核是真的在硬件上往前走。
