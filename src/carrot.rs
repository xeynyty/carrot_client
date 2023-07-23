use anyhow::Error;
use async_trait::async_trait;
use carrot_sdk::{Request, SetData};
use carrot_sdk::result::Response;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const SIZE: usize = 1024;

#[derive(Debug)]
pub struct Carrot {
    server_url: String,
    connection: TcpStream
}

impl Carrot {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        Ok(Self {
            server_url: String::from(url),
            connection: TcpStream::connect(url).await?
        })
    }

    // pub async fn request(&mut self, request: Request) -> anyhow::Result<Response> {
    //     let mut buf = [0; 1024];
    //     self.connection.write_all(<Request as TryInto<Vec<u8>>>::try_into(request)?.as_slice()).await?;
    //     let _x = self.connection.read(&mut buf).await?;
    //
    //     Ok(buf.as_slice().try_into()?)
    // }

    pub async fn read(&mut self, key: u32) -> anyhow::Result<Response> {
        let mut write: Vec<u8> = vec![0u8; SIZE];
        let req: Vec<u8> = Request::new()
            .set_key(Some(key))
            .read()
            .try_into()?;

        write = req;
        let res = self.tcp_request(&mut write).await?;

        Ok(res.as_slice().try_into()?)
    }

    async fn reconnect(&mut self) -> anyhow::Result<()> {
        for i in 0..10 {
            let x = TcpStream::connect(&self.server_url).await;
            if let Ok(stream) = x {
                self.connection = stream
            }
            eprintln!("{}. Try reconnect to carrot...", i + 1);
        }
        Ok(())
    }

    async fn tcp_request(&mut self, mut write: &mut [u8]) -> anyhow::Result<[u8; SIZE]> {
        self.connection.write_all(write).await?;

        let mut result = [0u8; SIZE];

        let _n = match self.connection.read(&mut result).await {
            Ok(n) if n == 0 => return Err(Error::msg("N == 0")),
            Ok(n) => {
                // println!("N = {}", n);
                n
            },
            Err(e) => {
                eprintln!("{}", e);
                return Err(Error::msg("N > SIZE"))
            }
        };

        Ok(result)
    }
}

#[async_trait]
pub trait Write<T> {
    async fn write(&mut self, key: u32, data: T, iat: u64) -> anyhow::Result<Response>;
}

#[async_trait]
impl Write<u64> for Carrot {
    async fn write(&mut self, key: u32, data: u64, iat: u64) -> anyhow::Result<Response> {
        let mut buf: Vec<u8> = vec![0u8; SIZE];
        buf = Request::new()
            .set_key(Some(key))
            .set_data(data)
            .set_iat(iat)
            .write()
            .try_into()?;

        let res = self.tcp_request(&mut buf).await?;

        Ok(res.as_slice().try_into()?)
    }
}

#[async_trait]
impl Write<i64> for Carrot {
    async fn write(&mut self, key: u32, data: i64, iat: u64) -> anyhow::Result<Response> {
        let mut buf: Vec<u8> = vec![0u8; SIZE];
        buf = Request::new()
            .set_key(Some(key))
            .set_data(data)
            .set_iat(iat)
            .write()
            .try_into()?;

        let res = self.tcp_request(&mut buf).await?;

        Ok(res.as_slice().try_into()?)
    }
}

#[async_trait]
impl Write<String> for Carrot {
    async fn write(&mut self, key: u32, data: String, iat: u64) -> anyhow::Result<Response> {
        let mut buf: Vec<u8> = vec![0u8; SIZE];
        buf = Request::new()
            .set_key(Some(key))
            .set_data(data)
            .set_iat(iat)
            .write()
            .try_into()?;

        let res = self.tcp_request(&mut buf).await?;

        Ok(res.as_slice().try_into()?)
    }
}

#[async_trait]
impl Write<Vec<u8>> for Carrot {
    async fn write(&mut self, key: u32, data: Vec<u8>, iat: u64) -> anyhow::Result<Response> {
        let mut buf: Vec<u8> = vec![0u8; SIZE];
        buf = Request::new()
            .set_key(Some(key))
            .set_data(data)
            .set_iat(iat)
            .write()
            .try_into()?;

        let res = self.tcp_request(&mut buf).await?;

        Ok(res.as_slice().try_into()?)
    }
}