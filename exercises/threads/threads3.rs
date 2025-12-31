// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


// 导入多生产者单消费者通道模块
use std::sync::mpsc;
// 导入原子引用计数智能指针，用于多线程间共享所有权
use std::sync::Arc;
// 导入线程模块
use std::thread;
// 导入时间持续时间模块
use std::time::Duration;

// 定义一个队列结构体，包含长度和两个半部分
struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    // 创建新的队列实例
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// 发送数据到通道的函数
// q: 要发送的队列
// tx: 通道的发送端
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    // 将队列包装在Arc中，允许多个线程共享所有权
    let qc = Arc::new(q);
    // 克隆Arc引用，增加引用计数，供第一个线程使用
    let qc1 = Arc::clone(&qc);
    // 再次克隆Arc引用，增加引用计数，供第二个线程使用
    let qc2 = Arc::clone(&qc);
    // 克隆通道发送端，供第二个线程使用
    let tx2 = tx.clone();

    // 生成第一个线程，处理队列的前半部分
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            // 通过通道发送值
            tx.send(*val).unwrap();
            // 线程休眠1秒
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 生成第二个线程，处理队列的后半部分
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            // 通过通道发送值
            tx2.send(*val).unwrap();
            // 线程休眠1秒
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    // 创建一个多生产者单消费者通道
    let (tx, rx) = mpsc::channel();
    // 创建新的队列
    let queue = Queue::new();
    // 保存队列长度用于后续验证
    let queue_length = queue.length;

    // 调用send_tx函数，启动两个线程发送数据
    send_tx(queue, tx);

    // 接收并统计从通道接收到的数据
    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    // 打印接收到的总数并验证是否等于队列长度
    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
