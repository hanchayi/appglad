起因想要系统的看一些浏览器的实现源码，这边选了`Rust`开发的`Servo`。而作为一个前端，对于GDB调试这些并不擅长，所以有了此文章。

`Servo`是一个巨大的工程。 我已经帮你统计了代码行数了。 `Servo`项目中有近十万行代码。 对于开发这么大的项目，了解如何以正确的方式进行调试非常重要，因为您希望快速有效地找到瓶颈。

在本文中，我将教您一些在 `Servo` 项目中使用 `GDB` 开发和调试 `Rust` 代码的技巧。

## 如何调试？

我假设你不熟悉软件开发，但你有一些编写代码的技能。

因此，当您想了解框中的某些内容时，您可以添加一些行，例如：

``` rust
println!("{:?}", SOMETHING);
```
这是一个简单的方法。 够直白了吧！ 它确实可以帮助您弄清楚代码发生了什么。

然而，每次当你想知道程序中另一个变量的值时，都需要编译。 此外，当您的程序崩溃或导致内存泄漏时，很难追踪潜在的问题。

简单的方法不够有用，这意味着您需要更强大的工具。 它可以是 `Linux` 中的 `GDB`，也可以是 `macOS` 中的 `LLDB`。 （在Windows平台上，VS调试器也是一个非常强大的工具，但本文不讨论。）

接下来我会讲一下如何使用`GDB`。 `LLDB` 与 `GDB` 非常相似。 基本上，他们的命令几乎是一样的，所以我只介绍如何使用GDB来调试Rust和Servo。

## GDB介绍

> “GDB，GNU 项目调试器，允许您查看另一个程序在执行时‘内部’发生了什么，或者另一个程序在崩溃时正在做什么。” ——来自 gnu.org

换句话说，`GDB` 允许您控制程序运行并从代码内部获取更多信息。

例如，您可以在文件中的某一行停止程序，这称为“断点”。 当程序在断点处停止时，您可以打印它们以查看断点范围内变量的值。

您还可以从断点开始回溯代码。 `Backtrace` 的意思是打印断点之前调用的所有函数。 有时，程序崩溃并不是因为崩溃所在的代码。 它可能会更早发生，并传递无效参数导致崩溃。

还有一些其他的用法，我会在下面的段落中提到。

## GDB调试Rust

首先，我将创建一个简单的 `Hello World` 来演示如何在 `Rust` 项目中使用 `GDB`。 您需要确保已经安装了 `Rust` 和 `Cargo`。

请按照“Rust Book”中的[步骤](https://doc.rust-lang.org/book/second-edition/ch01-03-hello-cargo.html#creating-a-project-with-cargo)创建一个 `Hello World`。 确保您可以编译、运行代码并了解 `Cargo` 在做什么。

创建项目
``` bash
cargo new hello_cargo --bin
cd hello_cargo
```

接着我们开始

为了展示如何使用 `GDB`，我设计了一个示例代码。请将以下代码复制到您的 `./src/main.rs` ：

``` rust
fn main() {
    let name = "Tiger";
    let msg = make_hello_string(&name);
    println!("{}", msg);
}

fn make_hello_string(name: &str) -> String {
    let hello_str = format!("Hi {}, how are you?", name);
    hello_str
}
```

要打包此代码，只需运行 `cargo build` 。

build成功后会有一个可执行文件 `./target/debug/hello_cargo `。

默认情况下，构建设置为调试模式，我们可以使用调试构建与 `GDB` 一起运行。但是，如果是发布版本，则无法使用 GDB 运行，因为调试信息会丢失。

要使用 `GDB` 运行程序，请执行以下操作：

``` bash
gdb target/debug/hello_cargo
```
就是这样。您会在 GDB 中看到如下所示的界面：

``` bash
gdb (Ubuntu 8.1-0ubuntu3) 8.1.0.20180409-git
Copyright (C) 2018 Free Software Foundation, Inc.
...
(gdb)
```

现在您可以在 `GDB` 中输入一些命令！

## GDB 命令

`GDB` 中有很多命令，但我只为您介绍最重要的部分。对于我的个人情况，我通常也只使用这些命令。

### 1. break

正如我之前提到的，断点允许您在某个位置停止程序。有两种方法可以设置断点。

使用 `break` 或者 `b` 作为命令设置断点。

在第一种情况下，您可以在函数上中断。（您需要输入整个路径，就像在一个大项目中一样 mod::mod::function ）

``` bash
(gdb) break make_hello_world
Breakpoint 1 at 0x55555555beca: file src/main.rs, line 8.
```
> lldb使用`b make_hello_string`

或者，您可以添加带有行号的文件路径，以定义停止位置。

``` bash
(gdb) b src/main.rs:9
Breakpoint 2 at 0x55555555bf6a: file src/main.rs, line 9.
```

让我们用`info break`看看我们是否设置成功。
> lldb使用`br l`

``` bash
(gdb) info break
Num     Type           Disp Enb Address            What
1       breakpoint     keep y   0x000055555555beca in hello_cargo::make_hello_string at src/main.rs:8
2       breakpoint     keep y   0x000055555555bf6a in hello_cargo::make_hello_string at src/main.rs:9
```

### 2. del

但我只想要一个断点，所以我可以通过命令删除 `del` 第一个断点。

> lldb使用 `br del 1`

``` bash
(gdb) del 1
(gdb) info break
Num     Type           Disp Enb Address            What
2       breakpoint     keep y   0x000055555555bf6a in hello_cargo::make_hello_string at src/main.rs:9
```

### 3. run

用于 `run` 开始启动程序。

``` bash
(gdb) run
Starting program: /home/tigercosmos/Desktop/hello_cargo/target/debug/hello_cargo
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 2, hello_cargo::make_hello_string (name=...) at src/main.rs:9
9	    hello_str
```

如您所见，程序已停在我们想要的位置。然后，我们可以在这个断点做点什么！


### 4. backtrace

如果您想知道在运行到此断点之前调用了哪些上游函数，可以使用 `bt` .

现在 GDB 告诉你它是 `main.rs` 第 3 行 ，它 `let msg = make_hello_string(&name);` 被称为 `main.rs` 第 9 行 ，属于 `make_hello_string` 。

你可能会说，这很明显，不是吗？

是的！但是，如果您正在调试一个大型开源项目，例如 Servo 项目，并且需要找出模块的回溯，该怎么办？一般来说，模块中断点之前大约有三十到四十个函数调用。通过读取代码来查找回溯是非常困难的，但现在我们可以使用 GDB 来做到这一点。

### 5. frame

帧是回溯中的程序状态之一。我们可以切换到我们想要的帧，并检查该帧中的一些信息。

现在我想检查帧 `#1` ，看看 `name` 在帧`#1` 范围内，在 的值 `src/main.rs:2` 是多少。

因此， `frame` 让您在 `frame` `#1` 中输入作用域，然后您可以使用 打印 `print` 变量的值，或者您可以使用 `call` 在该范围内调用函数。


您可以使用 `frame` 或 `f` 使用此命令。

```
(gdb) frame 0
#0  hello_cargo::make_hello_string (name=...) at src/main.rs:9
9	    hello_str
(gdb) print hello_str
$2 = alloc::string::String {vec: alloc::vec::Vec<u8> {buf: alloc::raw_vec::RawVec<u8, alloc::heap::Heap> {ptr: core::ptr::Unique<u8> {pointer: core::nonzero::NonZero<*const u8> (0x7ffff6c22060 "Hi Tiger, how are you?\000"), _marker: core::marker::PhantomData<u8>}, cap: 34, a: alloc::heap::Heap}, len: 22}}

```

### 6. continue

在检查了一些我们想知道的信息后，我们可能希望程序继续。使用 `continue`或者`c` 继续运行代码。程序将继续运行，直到遇到另一个断点或完成执行。

``` bash
(gdb) c
Continuing.
Hi Tiger, how are you?
```

由于此示例中只有一个断点，因此程序将运行到最后。

### 7. step

在断点处停止后，可以使用命令逐 `step` 行运行代码。

在断点处停止后，可以使用 `up` 和 `down` 切换帧，而不是直接使用 frame <number> 。


## 调试Servo

在调试模式下构建：

``` bash
./mach build -d
```

构建完成后，我们要调试 Servo：

``` bash
./mach run -d https://google.com --debug
Reading symbols from /home/tigercosmos/servo/target/debug/servo...done.
(gbd)
```

您现在可以调试`Servo`啦！

## 结论

本文中的概念不仅可以应用于 Rust 项目，还可以应用于C++项目。如果你想调试 `Firefox` 或 `Chromium`，你可以使用相同的方法来调试它们。

我不会讨论一些细节，例如如何为 `GDB` 应用复杂的设置，因此您可能需要搜索其他高级文章以学习更多技能。

## 原文链接

- [programming-servo-the-debug-way](https://medium.com/coding-neutrino-blog/programming-servo-the-debug-way-5db01f09b7f4)
