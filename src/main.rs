mod balance;
mod grpc;
mod utils;

use grpc::{create_server, AsyncBalanceMessage};
use utils::MessageProcessor;
use crossbeam_channel;
use std::thread;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting High-Performance Lightning Balance Service...");

    // 创建高性能channel
    let (message_sender, message_receiver) = crossbeam_channel::unbounded::<AsyncBalanceMessage>();

    // 启动高性能消息处理器
    let processor = MessageProcessor::new(message_receiver);
    let processor_handle = thread::spawn(move || {
        processor.run();
    });

    // 创建高性能gRPC服务
    let grpc_service = create_server(message_sender);

    // 配置高性能服务器
    let addr = "0.0.0.0:50051".parse()?;
    println!("High-performance gRPC server listening on {}", addr);

    // 使用tokio的并发运行时
    let server = Server::builder()
        .add_service(grpc_service)
        .serve(addr);

    // 启动服务器
    tokio::select! {
        result = server => {
            if let Err(e) = result {
                eprintln!("Server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!("Shutting down server...");
        }
    }

    // 等待处理器线程结束
    processor_handle.join().unwrap();

    Ok(())
}
