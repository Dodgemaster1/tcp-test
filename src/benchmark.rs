use anyhow::{Context, Result, bail};
use criterion::Criterion;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;

const MODEM_ID: u32 = 666666;
const REQUEST: &[u8] = b"get_more_data()";
const ANSWER: &[u8] = &[0x14; 432];

pub fn benchmark(host: String, modem_port: u16, program_port: u16) -> Result<()> {
    let mut modem = Client::connect(host.clone(), modem_port)?;
    let mut program = Client::connect(host.clone(), program_port)?;

    modem.handshake(MODEM_ID)?;
    program.handshake(MODEM_ID)?;

    for _ in 0..10 {
        program.send(REQUEST)?;
        if modem.recv()? != REQUEST {
            bail!("Sent and received messages don't match")
        }
    }

    Ok(())
}

pub fn bench_with_criterion<F>(func: F)
where
    F: Fn() -> Result<()>,
{
    let mut c = Criterion::default()
        .with_output_color(true)
        .sample_size(50);

    c.bench_function("connect+send+receive", |b| {
        b.iter_custom(|iters| {
            let start = std::time::Instant::now();

            for _ in 0..iters {
                if let Err(e) = func() {
                    println!("{:-<88}", "");
                    println!("Benchmark failed:");
                    println!("{:#}", e);
                    exit(-1)
                }
            }

            start.elapsed()
        });
    });
}

struct Client {
    _port: u16,
    stream: TcpStream,
}

impl Client {
    fn connect(host: String, port: u16) -> Result<Self> {
        let address = format!("{host}:{port}");
        let stream = TcpStream::connect(address.clone())
            .with_context(|| format!("Can't connect to server <{address}>"))?;
        Ok(Client {
            _port: port,
            stream,
        })
    }

    fn handshake(&mut self, modem_id: u32) -> Result<()> {
        self.stream
            .write_all(format!("Modem={modem_id}").as_bytes())?;
        let mut buf = [0u8; 4];
        self.stream.read(&mut buf)?;
        if buf == "OK\r\n".as_bytes() {
            Ok(())
        } else {
            bail!("Non-ok response from server during hanshake")
        }
    }

    fn send(&mut self, msg: &[u8]) -> Result<()> {
        Ok(self.stream.write_all(msg)?)
    }

    fn recv(&mut self) -> Result<Vec<u8>> {
        let mut vec = vec![0; 1024];
        let len = self.stream.read(&mut vec)?;
        vec.truncate(len);
        Ok(vec)
    }
}
