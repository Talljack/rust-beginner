# Todo Task

## Description

一个简单的 todo 程序，可以添加、修改、查询 todo 事项。

## Usage

```bash
cd packages/todo

cargo run
```

## 简介

- 使用 `struct` 定义 `Task` 结构体，包含 `name`、`completed` 两个字段
- 为 `Task` 结构体实习关联函数 `new`，用于创建 `Task` 实例，以及实现 task 处理的一些方法
- 在 `main` 包中读取 task 包并导入 task 包中的 `Task` 结构体以及相关方法
- 使用 `std::io` crate 读取用户输入
- 对用户的数据进行处理，分别允许输入 1-5 的数字，对应不同的操作
- 继续读取用户输入，然后根据用户内容进行相应的操作

## 使用到的 crate 包

- std::io 读取用户输入
- colored 输出彩色文字(需要在 Cargo.toml 中添加依赖)

## 需要注意的点

- 结构体的字段需要使用 `pub` 修饰，否则无法在其他包中使用-[模块](https://course.rs/basic/crate-module/module.html#%E6%A8%A1%E5%9D%97%E6%A0%91)
- 结构体方法的定义使用 `impl`并且内部的方法需要使用 `pub` 修饰，否则无法在其他包中使用-[use 包]()https://course.rs/basic/crate-module/use.html
- 使用 `std::io` crate 读取用户输入的时候，需要使用 `trim()` 方法去除空格
- 使用 `Vec` 存储 `Task` 实例，使用 `push` 方法添加 Task 实例，使用 `index` 获取 `Task` 实例-[Vector](https://course.rs/basic/collections/vector.html)
- 使用 `iter` 方法遍历 `Vec` 中的 `Task` 实例，使用 `enumerate` 方法获取 `Task` 实例的索引和实例 -[迭代器](https://course.rs/advance/functional-programing/iterator.html)
- 函数的复用性，可以将一些公共的逻辑抽取出来，作为一个函数，然后在需要的地方调用，可以增加泛型以及特征约束，提高复用性-[泛型](https://course.rs/basic/generic/generic.html)
- 使用 `match` 表达式处理 `Result` 类型的返回值-[match](https://course.rs/basic/match-pattern/match-if-let.html)

## 精进

后续可以优化的点：

- 错误处理：更健全的错误处理，例如处理枚举分类。
- 任务编辑和删除：允许用户编辑或删除任务。
- 日期和时间：记录任务的创建和完成日期。
