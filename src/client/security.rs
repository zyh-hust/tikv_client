use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ptr;

use grpcio::{Channel, ChannelBuilder, ChannelCredentialsBuilder};

#[derive(Clone, Debug, PartialEq)]
pub struct SecurityConfig {
    pub ca_path: String,
    pub cert_path: String,
    pub key_path: String,
}

impl Default for SecurityConfig {
    fn default() -> SecurityConfig {
        SecurityConfig {
            ca_path: String::new(),
            cert_path: String::new(),
            key_path: String::new(),
        }
    }
}

fn check_key_file(tag: &str, path: &str) -> Result<Option<File>, Box<dyn Error>> {
    if path.is_empty() {
        return Ok(None);
    }
    match File::open(path) {
        Err(e) => Err(format!("failed to open {} to load {}: {:?}", path, tag, e).into()),
        Ok(f) => Ok(Some(f)),
    }
}

fn load_key(tag: &str, path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut key = vec![];
    let f = check_key_file(tag, path)?;
    match f {
        None => return Ok(vec![]),
        Some(mut f) => {
            if let Err(e) = f.read_to_end(&mut key) {
                return Err(format!("failed to load {} from path {}: {:?}", tag, path, e).into());
            }
        }
    }
    Ok(key)
}

#[derive(Default)]
pub struct SecurityManager {
    ca: Vec<u8>,
    cert: Vec<u8>,
    key: Vec<u8>,
}

impl Drop for SecurityManager {
    fn drop(&mut self) {
        unsafe {
            for b in &mut self.key {
                ptr::write_volatile(b, 0);
            }
        }
    }
}

impl SecurityManager {
    pub fn new(cfg: &SecurityConfig) -> Result<SecurityManager, Box<dyn Error>> {
        Ok(SecurityManager {
            ca: load_key("CA", &cfg.ca_path)?,
            cert: load_key("certificate", &cfg.cert_path)?,
            key: load_key("private key", &cfg.key_path)?,
        })
    }

    pub fn connect(&self, cb: ChannelBuilder, addr: &str) -> Channel {
        if self.ca.is_empty() {
            cb.connect(addr)
        } else {
            let cred = ChannelCredentialsBuilder::new()
                .root_cert(self.ca.clone())
                .cert(self.cert.clone(), self.key.clone())
                .build();
            cb.secure_connect(addr, cred)
        }
    }
}
