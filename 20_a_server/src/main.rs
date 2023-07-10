use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use a_server::ThreadPool;

fn main() {
    // bind 绑定一个ip、port， unwrap表示失败直接停止程序
    let listener = TcpListener::bind("127.0.0.1:7891").unwrap();

    // 创建线程池
    let pool = ThreadPool::new(4);
    // incoming 返回了一个迭代器，提供一系列的TCP流
    for stream in listener.incoming() {
        // 每次从浏览器中获得链接的时候：
        let stream = stream.unwrap();

        println!("connect established!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new((&mut stream));

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {:#?}", http_request);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let loc = "D:/workspace/Rust/20_a_server/src/".to_owned();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", loc + "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", loc + "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", loc + "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
