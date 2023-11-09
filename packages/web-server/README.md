# web-server

## Description

一个简单的多线程 web 服务器，可以处理请求并返回响应。

## Usage

```bash
cd packages/web-server

cargo run
```

## 简介

- 使用 `std::net` crate 监听 TCP 连接
- 使用 `std::thread` crate 创建线程
- 使用 `std::sync` crate 实现线程间的消息传递以及共享内存
- 使用 `std::io` crate 读写请求和响应
- 使用 `std::fs` crate 读取文件
- 封装 `ThreadPool` 结构体，用于存储线程池相关的逻辑，以及将对应的逻辑封装到 `thread_pool` 模块中，进行逻辑解耦
- 封装 `Worker` 结构体，用户生产线程，并且将对应的逻辑封装到 `worker` 模块中，进行用户逻辑执行

## 需要注意的点

- 使用 `std::net` crate 时，需要使用 `unwrap_or_else` 方法处理 `Result` 类型的返回值，并且支持传入一个闭包，当 `Result` 类型的返回值为 `Err` 时，会执行闭包中的逻辑-[闭包](https://course.rs/basic/closure/closure.html)
- 结构体以及方法的定义需要使用 `pub` 修饰，否则无法在其他包中使用-[模块](https://course.rs/basic/crate-module/module.html#%E6%A8%A1%E5%9D%97%E6%A0%91)
- 多线程中的消息传递，需要使用 `Arc` 和 `Mutex`，`Arc` 用于共享内存，`Mutex` 用于互斥访问-[多线程](https://course.rs/advance/concurrency/thread.html)
- 使用 `Box<dyn Error>` 作为 `Result` 类型的返回值，可以返回任何类型的错误-[Box](https://course.rs/basic/box/box.html)
- 自定义实现 `Drop` trait，用于在结构体被销毁时执行对应的逻辑-[Trait](https://course.rs/basic/trait/trait.html)
- TcpStream 本身是一个迭代器，可以使用 `read_to_string` 方法获取迭代器中的值，也可以使用 `write_all` 方法写回`buf`流-[迭代器](https://course.rs/advance/functional-programing/iterator.html)

## 实现

1.首先创建自定义的 `ThreadPool` 结构体，用于存储线程池相关的逻辑，以及将对应的逻辑封装到 `thread_pool` 模块中，进行逻辑解耦

```rust
let pool = ThreadPool::new(4); // 创建线程池
```

```rust
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
}
```

同时 `ThreadPool` 中会存储 `Vec<Worker>`，用于存储 `Worker`，以及 `Option<mpsc::Sender<Job>>`，用于存储 `mpsc::Sender<Job>`，用于发送请求到 `Worker` 中

```rust
let mut workers = Vec::with_capacity(size);
for id in 0..size {
  workers.push(Worker::new(id, Arc::clone(&receiver)));
}
```

2.连接到 `TcpListener`，并且监听 TCP 连接，当有请求时，将请求发送到 `ThreadPool` 中进行执行

```rust
pool.execute(|| {
  handle_connection(stream);
});
```

当有请求时，`sender` 将请求的闭包函数发送到 `Worker` 中

```rust
pub fn execute<F>(&self, f: F)
 where
     F: FnOnce() + Send + 'static,
 {
   let job = Box::new(f);
   self.sender.as_ref().unwrap().send(job).unwrap();
 }
```

`worker` 循环中接收到请求后，会执行对应的逻辑，也就是把请求中的 `job` 执行，而 `job` 本身是一个闭包，所以会执行闭包中的逻辑

```rust
let message = receiver.lock().unwrap().recv();
match message {
 Ok(job) => {
   println!("Worker {} got a job; executing.", id);
   job();
 },
 Err(_) => break,
}
```

3.线程池初始化的时候会创建多个线程，每个线程都会创建一个 `Worker`，并且创建一个 `mpsc::Receiver<Job>`，用于接收 `ThreadPool` 中的请求

```rust
let (sender, receiver) = mpsc::channel();
```

```rust
struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}
```

以及每个 worker 初始化的时候都会创建一个 `thread::JoinHandle<()>`，用于存储线程的句柄，以及线程的 id，并通过 `move` 关键字将 `receiver` 移动到线程中，用于接收 `ThreadPool` 中的请求，以及`loop`循环去接受消息请求

```rust
let thread = thread::spawn(move|| loop {
  let message = receiver.lock().unwrap().recv();
  match message {
    Ok(job) => {
      println!("Worker {} got a job; executing.", id);
      job();
    },
    Err(_) => break,
  }
});
```

4.用户处理每一个需要传入到 `ThreadPool` 中的请求，也就是 `job`，并且将请求中的 `TcpStream` 传入到 `handle_connection` 中，用于处理请求，读取请求中的数据，并且返回响应

```rust
fn handle_connection(mut stream: TcpStream) {
  // ...
}
```

## 精进

后续可以优化的点：

- 可以将 unwrap 调用替换为更加健壮的错误处理
- 为 ThreadPool 和其公有方法增加更多文档
- 为库的功能增加测试
