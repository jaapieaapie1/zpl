//! A tiny HTTP/1.1 mock server on std threads, usable from async and blocking tests.

#![allow(dead_code)]

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub const LABEL: &str = "^XA^FO50,50^A0N,40^FDhi^FS^XZ";

/// A request as the server received it.
#[derive(Debug, Clone)]
pub struct Recorded {
    pub method: String,
    pub target: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Recorded {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

/// A canned response.
#[derive(Debug, Clone)]
pub struct Reply {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub delay: Duration,
}

impl Reply {
    pub fn new(status: u16, body: impl Into<Vec<u8>>) -> Self {
        Reply {
            status,
            headers: Vec::new(),
            body: body.into(),
            delay: Duration::ZERO,
        }
    }

    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// A 200 PDF with the full set of success headers.
    pub fn pdf() -> Self {
        Reply::new(200, "%PDF-1.7 fake")
            .header("Content-Type", "application/pdf")
            .header("X-Page-Count", "3")
            .header("X-File-Page-Counts", "2, 1")
            .header("X-Labels-Billed", "3")
            .header("RateLimit-Limit", "200")
            .header("RateLimit-Remaining", "199")
            .header(
                "Server-Timing",
                "read;dur=0.1, parse;dur=0.2, total;dur=1.0",
            )
    }
}

/// Serves the replies in order, one per request; the last one repeats.
pub struct MockServer {
    url: String,
    requests: Arc<Mutex<Vec<Recorded>>>,
}

impl MockServer {
    pub fn start(replies: Vec<Reply>) -> Self {
        assert!(!replies.is_empty());
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded = requests.clone();
        thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else { continue };
                let recorded = recorded.clone();
                let replies = replies.clone();
                thread::spawn(move || handle(stream, &recorded, &replies));
            }
        });
        MockServer { url, requests }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn requests(&self) -> Vec<Recorded> {
        self.requests.lock().unwrap().clone()
    }
}

/// A URL where nothing listens, for connection errors.
pub fn closed_url() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    drop(listener);
    url
}

fn handle(stream: TcpStream, recorded: &Mutex<Vec<Recorded>>, replies: &[Reply]) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();
    if reader.read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_owned();
    let target = parts.next().unwrap_or_default().to_owned();
    let mut headers = Vec::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        let (name, value) = line.split_once(':').unwrap();
        headers.push((name.trim().to_ascii_lowercase(), value.trim().to_owned()));
    }
    let length = headers
        .iter()
        .find(|(n, _)| n == "content-length")
        .map(|(_, v)| v.parse::<usize>().unwrap())
        .unwrap_or(0);
    let mut body = vec![0; length];
    reader.read_exact(&mut body).unwrap();

    let index = {
        let mut requests = recorded.lock().unwrap();
        requests.push(Recorded {
            method,
            target,
            headers,
            body,
        });
        requests.len() - 1
    };
    let reply = &replies[index.min(replies.len() - 1)];
    thread::sleep(reply.delay);

    let mut out = format!(
        "HTTP/1.1 {} Mock\r\nContent-Length: {}\r\nConnection: close\r\n",
        reply.status,
        reply.body.len()
    );
    for (name, value) in &reply.headers {
        out.push_str(&format!("{name}: {value}\r\n"));
    }
    out.push_str("\r\n");
    let mut stream = stream;
    let _ = stream.write_all(out.as_bytes());
    let _ = stream.write_all(&reply.body);
    let _ = stream.flush();
}

/// Gunzips a request body.
pub fn gunzip(body: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    flate2::read::GzDecoder::new(body)
        .read_to_end(&mut out)
        .unwrap();
    out
}
