use mio_extras::timer::Timeout;
use std::time;

use ws::util::Token;
use ws::Frame;
use ws::{CloseCode, Handler, Handshake, Message, OpCode, Sender};
use ws::{Error, ErrorKind};
// use tungstenite::Error;

const PING: Token = Token(1);
const EXPIRE: Token = Token(2);

type Result<T> = std::result::Result<T, Error>;

pub struct Transport {
    out: Sender,
    ping_timeout: Option<Timeout>,
    expire_timeout: Option<Timeout>,
}

impl Handler for Transport {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // schedule a timeout to send a ping every 60 seconds
        self.out.timeout(60_000, PING)?;
        // schedule a timeout to close the connection if there is no activity for 3 minutes
        self.out.timeout(180_000, EXPIRE)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);

        // todo: check NOTE: This code demonstrates cleaning up timeouts
        if let Some(t) = self.ping_timeout.take() {
            self.out.cancel(t).unwrap();
        }
        if let Some(t) = self.expire_timeout.take() {
            self.out.cancel(t).unwrap();
        }
        println!("Shutting down server after first connection closes.");
        self.out.shutdown().unwrap();
    }

    fn on_error(&mut self, err: Error) {
        // Shutdown on any error
        println!("Shutting down server for error: {}", err);
        self.out.shutdown().unwrap();
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        match event {
            // PING timeout has occured, send a ping and reschedule
            PING => {
                self.out.ping(now_ns().to_string().into_bytes()).expect("Cannot parse ws-response");
                self.ping_timeout.take();
                self.out.timeout(5_000, PING)
            }
            // EXPIRE timeout has occured, this means that the connection is inactive, let's close
            EXPIRE => self.out.close(CloseCode::Away),
            // No other timeouts are possible
            _ => Err(Error::new(
                ErrorKind::Internal,
                "Invalid timeout token encountered!",
            )),
        }
    }

    fn on_new_timeout(&mut self, event: Token, timeout: Timeout) -> Result<()> {
        // Cancel the old timeout and replace.
        if event == EXPIRE {
            if let Some(t) = self.expire_timeout.take() {
                self.out.cancel(t)?
            }
            self.expire_timeout = Some(timeout)
        } else {
            // This ensures there is only one ping timeout at a time
            if let Some(t) = self.ping_timeout.take() {
                self.out.cancel(t)?
            }
            self.ping_timeout = Some(timeout)
        }

        Ok(())
    }

    fn on_frame(&mut self, frame: Frame) -> Result<Option<Frame>> {
        // If the frame is a pong, print the round-trip time.
        // The pong should contain data from out ping, but it isn't guaranteed to.
        if frame.opcode() == OpCode::Pong {
            if let Ok(pong) = std::str::from_utf8(frame.payload())?.parse::<u128>() {
                let now = now_ns();
                println!("RTT is {:.3}ms.", (now - pong) as u128 / 1_000_000u128);
            } else {
                println!("Received bad pong.");
            }
        }

        // Some activity has occured, so reset the expiration
        self.out.timeout(30_000, EXPIRE)?;

        // Run default frame validation
        DefaultHandler.on_frame(frame)
    }
}

fn now_ns() -> u128 {
    time::SystemTime::now().elapsed().unwrap().as_nanos()
}

// For accessing the default handler implementation
struct DefaultHandler;

impl Handler for DefaultHandler {}
