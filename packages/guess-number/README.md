# guess-number

## Description

一个简单的猜数字游戏，数字范围在 1 到 1000 之间。

## Usage

```bash
cd packages/2-guess-number

cargo run
```

## 简介

- main 函数是 Rust 程序的入口函数
- 使用 `rand` crate 生成随机数
- 使用 `std::io` crate 读取用户输入
- 将用户输入转换为数字并进行比较
- 如果用户输入的数字大于随机数，则提示用户输入的数字太大，太小则提示太小,并重复上述步骤
- 直到用户输入的数字等于随机数，提示用户猜对了，并退出程序

## 使用到的 crate 包

- rand 生成随机数(需要在 Cargo.toml 中添加依赖)
- std::io 读取用户输入
- std::cmp::Ordering 比较用户输入和随机数的大小
- colored 输出彩色文字(需要在 Cargo.toml 中添加依赖)

## 需要注意的点

- 需要使用`rand::thread_rng()` 生成随机数生成器，然后使用`gen_range()`方法生成随机数
- guess_number 转化成数字的时候，需要使用`trim()`方法去除空格，然后使用`parse()`方法转化成数字，并且返回的是`Result`类型，需要使用`match`表达式处理错误
- 使用`std::cmp::Ordering`比较用户输入和随机数的大小，需要使用`match`表达式处理`Ordering`的三种情况，分别是`Less`、`Greater`、`Equal`
- 使用`colored` crate 输出彩色文字，需要使用`use colored::*;`导入所有的颜色，然后使用`println!("{}", "文字".red());`输出红色的文字
- 使用`loop`循环，直到用户输入的数字等于随机数，使用`break`退出循环

## 精进

可以去读一下 colored crate 的源码，相对来说源码不多，可以了解一下颜色输出的原理，如果有兴趣可以自己实现一个类似的 crate 包。