use log::error;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
};

fn main() {
    //监听端口
    let port = 8090;
    //初始化监听器
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .expect(format!("port '{}' aleady used", port).as_str());
    //等待新客户端连接
    for (_id, conn) in listener.incoming().enumerate() {
        //提取steam，如果有错误则直接返回
        match conn {
            Ok(stream) => {
                //spawn新的线程处理新连接，后期可以优化为线程池
                thread::spawn(move || {
                    //生成writer，后续可优化错误处理
                    let mut writer = stream.try_clone().unwrap();
                    //封装stream为buf reader
                    let mut reader = BufReader::new(stream);
                    //通过buf reader按行读取客户端发来的数据
                    let mut line = String::new();
                    loop {
                        let result = reader.read_line(&mut line);
                        match result {
                            Ok(size) => {
                                //如果读到EOF，退出线程关闭socket
                                if size == 0 {
                                    error!("connection closed");
                                    break;
                                }
                                //原样输出到客户端，后续优化错误处理
                                writer.write_all(line.as_bytes()).unwrap();
                                //清除line中的数据
                                line.clear();
                            }
                            //遇到错误，退出线程
                            Err(e) => {
                                error!("reading error: {:?}", e);
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                error!("connecting error: {:?}", e);
            }
        }
    }
}
