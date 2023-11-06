# Word Count

## Description

一个简单的统计单词数量的程序，对于用户输入的字符串，统计其中单词的数量。

## Usage

```bash
cd packages/4-word-count

cargo run
```

## 简介

- 使用 `std::io` crate 读取用户输入
- 对用户的数据进行处理，统计单词的数量，并将结果存放在 `HashMap` 中
- 使用 `HashMap` 存储单词以及单词的数量
- 使用 `regex` crate 过滤用户输入的字符串，去除标点符号
- 使用 `colored` crate 输出彩色文字


## 使用到的 crate 包

- std::io 读取用户输入
- regex 过滤用户输入的字符串，去除标点符号(需要在 Cargo.toml 中添加依赖)
- colored 输出彩色文字(需要在 Cargo.toml 中添加依赖)
- HashMap 存储单词以及单词的数量


## 需要注意的点

- 使用 `std::io` crate 读取用户输入的时候，需要使用 `trim()` 方法去除空格
- 注意 `HashMap` 的引用和所有权的问题，需要使用 `entry` 方法获取 `HashMap` 中的值- [HashMap](https://course.rs/basic/collections/hashmap.html)
- 使用 `regex` crate 过滤用户输入的字符串，去除标点符号-[正则表达式](https://github.com/rust-lang/regex)


## 精进

后续可以优化的点：

- 错误处理：更健全的错误处理，例如处理枚举分类。
- 单词大小写：统计单词的时候，可以将单词转换为小写，然后再进行统计。
- 支持中文：支持中文的单词统计。

