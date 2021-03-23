use std::io::prelude::*; // 引入io::prelude 库
use std::net::TcpListener; // 引入TcpListener 库
use std::net::TcpStream; // 引入 TcpStream 库

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // 绑定到本地地址和端口

    for stream in listener.incoming() { // 循环读取
        match stream {  
            Ok(stream) => {
                handle_connection(stream); // 调用函数handle_connection处理数据
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        stream.read(&mut buffer).unwrap(); // 读数据
        let str = String::from_utf8_lossy(&buffer[..]);
        if(str.starts_with("exit") == true) { // 命中exit退出
            break;
        }
        println!("Request: `{}`", str); // 打印
    }
    
}