extern crate hostname;
extern crate rand;

use hostname::get_hostname;
use rand::prelude::*;
use std::net::*;
use std::sync::mpsc::*;
use std::thread;

#[derive(Clone)]
pub struct Vapor {
    hostname: String,
    sender: Sender<Vec<u8>>,
}

impl Vapor {
    pub fn new(host_name: &str, port: u16) -> Vapor {
        let address = format!("{}:{}", host_name, port);
        let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();

        thread::spawn(move || {
            let mut socket = None;

            loop {
                match rx.recv() {
                    Ok(data) => {
                        if socket.is_none() {
                            socket = UdpSocket::bind("0.0.0.0:0").ok();
                        }

                        if let Some(s) = &socket {
                            match s.send_to(data.as_slice(), &address) {
                                Ok(_) => {},
                                Err(_) => {}
                            }
                        }
                    },

                    Err(err) => {
                        eprintln!("vapor thread crashed: {:?}", err);
                    }
                }
            }
        });

        Vapor {
            hostname: get_hostname().unwrap_or_else(|| "unknown".to_string()),
            sender: tx,
        }
    }

    pub fn event<S: AsRef<str>>(&self, name: S) {
        let data = format!("e.{}.0", self.calc_name(name));
        let _ = self.sender.send(data.into_bytes());
    }

    pub fn gauge<S: AsRef<str>>(&self, name: S, value: i64) {
        let data = format!("g/{}/{}", self.calc_name(name), value);
        let _ = self.sender.send(data.into_bytes());
    }

    pub fn sample_event<S: AsRef<str>>(&self, name: S, sample_rate: f64) {
        if Vapor::should_send(sample_rate) {
            self.event(name);
        }
    }

    pub fn sample_gauge<S: AsRef<str>>(&self, name: S, value: i64, sample_rate: f64) {
        if Vapor::should_send(sample_rate) {
            self.gauge(name, value);
        }
    }

    fn calc_name<S: AsRef<str>>(&self, name: S) -> String {
        name.as_ref()
            .replace("%h", &self.hostname)
            .replace("/", "")
    }

    fn should_send(sample_rate: f64) -> bool {
        let mut rng = thread_rng();
        let value: f64 = rng.gen();

        value < sample_rate
    }
}

