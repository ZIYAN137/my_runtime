use std::{io, task::Poll, thread};

use futures::future::poll_fn;
use my_runtime::{block_on, Executor, Proactor, Stdin}; // 使用 Proactor 代替 Reactor

async fn yield_now() {
    let mut x = false;
    poll_fn(|cx| {
        if x {
            Poll::Ready(())
        } else {
            x = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
        .await;
}

fn main() -> io::Result<()> {
    // 初始化 Proactor
    let proactor = Proactor::new(256)?;
    let ex = Executor::new();

    thread::scope(|s| {
        // 起8个线程作为线程池，来并发执行 task
        for _ in 0..8 {
            s.spawn(|| {
                block_on(ex.execute()); // 执行任务队列
            });
        }

        // 创建异步任务，丢到 Executor 中执行
        ex.spawn(async {
            let mut buf = [0; 1000];
            let mut buf = &mut buf[..];
            let stdin = Stdin::new(&proactor).unwrap(); // 使用 Proactor 创建 Stdin

            while buf.len() > 0 {
                let x = stdin.read(buf).await.unwrap(); // 使用 io_uring 完成异步读取
                println!("from stdin: {:?}", String::from_utf8_lossy(&buf[..x]));

                buf = &mut buf[x..];
            }
        })
            .detach();

        // 第二个异步任务，验证调度器
        ex.spawn(async {
            yield_now().await;
            println!("yield 1");
            yield_now().await;
            println!("yield 2");
        })
            .detach();
    });

    Ok(())
}
