/*
파일 자체가 모듈이 되므로 mod 키워드로 감쌀 필요가 없습니다
파일명이 모듈명이 됩니다 (server.rs → server 모듈)
*/
pub struct Server {
    addr: String,
}

// impl에는 pub 불필요
impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        // run method get ownership
        println!("listen on : {}", self.addr);
    }
}
