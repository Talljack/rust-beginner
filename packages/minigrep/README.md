# Minigrep

## Description

一个简单的 `grep` 程序，可以查询文件中包含特定字符串的行。

## Usage

```bash
cd packages/minigrep

cargo run <query> <filename>
```

## 简介

- 使用 `std::env` crate 获取命令行参数
- 使用 `std::fs` crate 读取文件
- 封装 `Config` 结构体，用于存储命令行参数，以及将对应的逻辑封装到 `minigrep crate`中，进行逻辑解耦
- 使用 `Result` 类型处理错误，使用 `unwrap_or_else` 方法处理 `Result` 类型的返回值

## 使用到的 crate 包

- std::env 获取命令行参数
- std::fs 读取文件
- colored 输出彩色文字(需要在 Cargo.toml 中添加依赖)

## 需要注意的点

- 使用 `std::env` crate 时，需要使用 `unwrap_or_else` 方法处理 `Result` 类型的返回值，并且支持传入一个闭包，当 `Result` 类型的返回值为 `Err` 时，会执行闭包中的逻辑-[闭包](https://course.rs/basic/closure/closure.html)
- 结构体以及方法的定义需要使用 `pub` 修饰，否则无法在其他包中使用-[模块](https://course.rs/basic/crate-module/module.html#%E6%A8%A1%E5%9D%97%E6%A0%91)
- 使用 `Vec` 存储 `grep_result`，然后将结果输出-[Vector](https://course.rs/basic/collections/vector.html)
- 使用 `Err(e)` 语法将错误转换为 `Result` 类型的返回值-[错误处理](https://course.rs/basic/error-handling/error-handling.html)
- 使用 Box<dyn Error> 作为 `Result` 类型的返回值，可以返回任何类型的错误-[Box](https://course.rs/basic/box/box.html)
- 使用 `'a`和`'static` 作为生命周期参数，可以将`search_case_insensitive`函数中返回的值确定其生命周期-[生命周期](https://course.rs/basic/lifetime/lifetime.html)
- 读取的 `env::args` 本身是一个迭代器，可以使用 `next` 方法获取迭代器中的值-[迭代器](https://course.rs/advance/functional-programing/iterator.html)

## 精进

后续可以优化的点：

- 更加明确的用户输入，例如使用 `--query xxx`来处理用户的输入。
- 支持自定义的文本查询，例如支持正则表达式。
- 文本进行存储，例如将文本存储到数据库中。
