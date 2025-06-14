# 使用 Rust 从零开始编写操作系统指南

本指南旨在为您提供使用 Rust 语言从零开始编写操作系统的详细步骤和学习路径。Rust 语言因其内存安全、并发性和高性能特性，成为操作系统开发领域的理想选择。

## 1. 环境准备

在开始之前，您需要准备以下开发环境：

*   **Rust nightly 版本**：操作系统开发通常需要使用 Rust 的 `nightly` 版本，因为它提供了最新的实验性功能和工具，如 `no_std` 和自定义目标。安装 `rustup` 后，可以使用 `rustup install nightly` 和 `rustup default nightly` 来切换到 `nightly` 版本。
*   **Cargo**：Rust 的包管理器和构建系统，用于管理项目依赖和编译代码。
*   **QEMU**：一个开源的机器模拟器和虚拟器，我们将使用它来模拟裸机环境，运行我们编写的操作系统。
*   **`bootimage` 工具**：一个 Cargo 子命令，用于将 Rust 内核编译成一个可引导的磁盘镜像。您可以使用 `cargo install bootimage` 进行安装。
*   **交叉编译工具链**：为了将 Rust 代码编译成特定架构（如 `x86_64`）的裸机二进制文件，您需要安装相应的 `rust-src` 和 `rust-std` 组件，例如 `rustup component add rust-src --toolchain nightly`。

## 2. 创建独立式可执行程序

标准的 Rust 程序依赖于操作系统的标准库。然而，在操作系统开发中，我们无法依赖这些库，因为我们正在构建自己的操作系统。因此，我们需要创建一个独立式（freestanding）的可执行程序。

1.  **创建新的 Rust 项目**：
    ```bash
    cargo new myos --bin
    cd myos
    ```

2.  **禁用标准库**：在 `src/main.rs` 的顶部添加 `#![no_std]` 属性。这会告诉 Rust 编译器不链接标准库。
    ```rust
    #![no_std]
    #![no_main] // 禁用 Rust 入口点，我们将自己定义入口点

    use core::panic::PanicInfo;

    /// 这个函数会在 panic 时被调用
    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }

    // ... 更多代码 ...
    ```

3.  **定义 `_start` 入口点**：由于禁用了标准库，Rust 不再提供默认的 `main` 函数作为程序入口。我们需要自己定义一个 `_start` 函数作为裸机程序的入口点。这个函数必须被标记为 `no_mangle` 以防止 Rust 编译器对其进行名称修饰，并且是 `extern "C"` 以确保其 C 语言调用约定。
    ```rust
    #[no_mangle]
    pub extern "C" fn _start() -> ! {
        // 这是操作系统的入口点
        loop {}
    }
    ```

## 3. 配置链接器脚本

为了让我们的裸机程序能够正确运行，我们需要告诉链接器如何组织生成的二进制文件。这通常通过一个自定义的链接器脚本来实现，它定义了代码和数据在内存中的布局。

例如，为 `x86_64` 架构创建一个 `linker.ld` 文件：

```linker.ld
ENTRY(_start)

SECTIONS
{
    . = 0x100000; /* 内核加载的内存地址 */

    .text : {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
    }
}
```

在 `Cargo.toml` 中配置 `bootimage` 使用这个链接器脚本：

```toml
[package]
name = "myos"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[build-dependencies]

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

[package.metadata.bootimage]
# 定义用于构建可引导镜像的 target triple
target = "x86_64-unknown-none"

# 传递给链接器的额外参数，用于指定链接器脚本
linker-args = [
    "-Tlinker.ld"
]
```

## 4. 裸机运行与 QEMU 模拟

1.  **添加自定义 Target**：创建一个 JSON 文件来定义我们的裸机目标。例如，`x86_64-unknown-none.json`：
    ```json
    {
      "llvm-target": "x86_64-unknown-none",
      "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
      "arch": "x86_64",
      "os": "none",
      "cpu": "x86-64",
      "target-endian": "little",
      "target-pointer-width": "64",
      "executables": true,
      "features": "+sse,+sse2",
      "disable-redzone": true,
      "code-model": "kernel",
      "relocation-model": "static",
      "panic-strategy": "abort"
    }
    ```
    在 `Cargo.toml` 中，修改 `bootimage` 配置以使用这个目标：
    ```toml
    [package.metadata.bootimage]
target = "x86_64-unknown-none"
    ```

2.  **构建可引导镜像**：使用 `bootimage` 工具构建可引导的 OS 镜像。
    ```bash
    cargo bootimage --target x86_64-unknown-none.json
    ```
    这将在 `target/x86_64-unknown-none/debug/` （或 `release`）目录下生成 `myos.elf` 和 `bootimage-myos.bin` 文件。

3.  **在 QEMU 中运行**：使用 QEMU 模拟器运行生成的镜像。
    ```bash
    qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-myos.bin
    ```
    如果一切正常，QEMU 将启动并显示一个黑屏，这表明您的操作系统已成功启动。

## 5. 基本 I/O：在屏幕上打印

为了让操作系统能够与用户交互，我们需要实现基本的输入/输出功能，例如在屏幕上打印字符。这通常通过直接写入显存（VGA 文本模式）或通过串口来实现。

*   **VGA 文本模式**：直接写入 `0xb8000` 地址的显存来显示字符。每个字符由一个 ASCII 码和一个颜色字节组成。
*   **串口通信**：通过与串行端口控制器交互来发送和接收数据。这通常需要使用 `x86_64` 架构的 I/O 端口指令。

您可以开始实现一个简单的 `println!` 宏，用于在屏幕上打印消息。

## 6. 内存管理

操作系统的一个核心功能是管理内存。这包括：

*   **物理内存管理**：跟踪哪些物理页是空闲的，哪些正在被使用。这通常通过位图或链表来实现。
*   **虚拟内存管理**：为每个进程提供独立的虚拟地址空间，并通过页表将虚拟地址映射到物理地址。
*   **堆分配器**：实现一个内核堆，允许内核在运行时动态分配内存。

## 7. 中断和异常处理

操作系统必须能够处理硬件中断（如定时器中断、键盘输入）和 CPU 异常（如除零错误、页错误）。这需要：

*   **中断描述符表 (IDT)**：设置一个 IDT 来告诉 CPU 在发生中断或异常时应该跳转到哪个处理函数。
*   **中断处理程序**：为每种中断或异常编写相应的处理函数，并在其中保存上下文、执行处理逻辑并恢复上下文。

## 8. 设备驱动

为了与硬件设备（如键盘、硬盘、网络适配器）交互，操作系统需要相应的设备驱动程序。这通常涉及：

*   **PCI/PCIe 总线枚举**：发现连接到总线上的设备。
*   **MMIO/端口 I/O**：通过内存映射 I/O 或端口 I/O 与设备寄存器进行通信。

## 9. 多任务处理

实现多任务处理是操作系统的另一个关键功能，它允许同时运行多个程序。这包括：

*   **进程和线程管理**：创建、调度和销毁进程/线程。
*   **上下文切换**：在不同任务之间切换 CPU 上下文。
*   **调度器**：决定哪个任务在何时运行。

## 学习资源

*   **`Writing an OS in Rust`**：一个非常受欢迎的在线书籍，详细介绍了使用 Rust 编写操作系统的过程。
*   **`Rustonomicon`**：Rust 语言的“黑暗宝典”，深入介绍了 Rust 的底层细节和不安全代码。
*   **操作系统原理书籍**：如《操作系统导论》 (Operating Systems: Three Easy Pieces) 或 《现代操作系统》 (Modern Operating Systems)。
*   **`OSDev Wiki`**：一个丰富的操作系统开发资源库，包含各种架构和技术的详细信息。
*   **社区和论坛**：参与 Rust 和操作系统开发社区，寻求帮助和分享经验。

祝您在 Rust 操作系统开发之旅中一切顺利！