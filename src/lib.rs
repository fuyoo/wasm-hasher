use js_sys::{Function, Uint8Array};
use wasm_bindgen::prelude::*;
use md5::{Md5};
use wasm_bindgen_futures::JsFuture;
use web_sys::Blob;
use sha1::Sha1;
use sha2::{Sha256, Sha512, Digest, Sha384, Sha224};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use sm3::Sm3;

#[wasm_bindgen]
pub async fn md5(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).md5().await
}

#[wasm_bindgen]
pub async fn sha2_224(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).sha224().await
}

#[wasm_bindgen]
pub async fn sha2_256(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).sha256().await
}

#[wasm_bindgen]
pub async fn sha2_384(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).sha384().await
}

#[wasm_bindgen]
pub async fn sha2_512(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).sha512().await
}

#[wasm_bindgen]
pub async fn sha1(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).sha1().await
}

#[wasm_bindgen]
pub async fn sha3_224(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).sha3_224().await
}

#[wasm_bindgen]
pub async fn sha3_256(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).sha3_256().await
}

#[wasm_bindgen]
pub async fn sha3_384(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).sha3_384().await
}

#[wasm_bindgen]
pub async fn sha3_512(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).sha3_512().await
}

#[wasm_bindgen]
pub async fn sm3(blob: Blob, cb: Function) -> Result<String, JsValue> {
    Hasher::new(blob, cb).sm3().await
}

#[derive(Debug, Clone)]
pub struct Hasher {
    blob: Blob,
    progress: Function,
}

impl Hasher {
    pub fn new(blob: Blob, cb: Function) -> Self {
        Hasher {
            blob,
            progress: cb,
        }
    }
    pub async fn md5(&mut self) -> Result<String, JsValue> {
        self.computed("md5").await
    }
    pub async fn sha224(&mut self) -> Result<String, JsValue> {
        self.computed("sha256").await
    }
    pub async fn sha256(&mut self) -> Result<String, JsValue> {
        self.computed("sha256").await
    }
    pub async fn sha384(&mut self) -> Result<String, JsValue> {
        self.computed("sha256").await
    }
    pub async fn sha512(&mut self) -> Result<String, JsValue> {
        self.computed("sha256").await
    }
    pub async fn sm3(&mut self) -> Result<String, JsValue> {
        self.computed("sm3").await
    }
    pub async fn sha1(&mut self) -> Result<String, JsValue> { self.computed("sha1").await }
    pub async fn sha3_224(&mut self) -> Result<String, JsValue> { self.computed("sha3_224").await }
    pub async fn sha3_256(&mut self) -> Result<String, JsValue> { self.computed("sha3_256").await }
    pub async fn sha3_384(&mut self) -> Result<String, JsValue> { self.computed("sha3_384").await }
    pub async fn sha3_512(&mut self) -> Result<String, JsValue> { self.computed("sha3_512").await }
    pub async fn computed(&self, flag: &str) -> Result<String, JsValue> {
        let hasher = match flag {
            "sha1" => HA::Sha1(Sha1::new()),
            "sha224" => HA::Sha224(Sha224::new()),
            "sha256" => HA::Sha256(Sha256::new()),
            "sha384" => HA::Sha384(Sha384::new()),
            "sha512" => HA::Sha512(Sha512::new()),
            "sm3" => HA::Sha512(Sha512::new()),
            "sha3_224" => HA::Sha3_224(Sha3_224::new()),
            "sha3_256" => HA::Sha3_256(Sha3_256::new()),
            "sha3_384" => HA::Sha3_384(Sha3_384::new()),
            "sha3_512" => HA::Sha3_512(Sha3_512::new()),
            _ => HA::Md5(Md5::new())
        };
        let step: f64 = 10485760.0;
        let size = self.blob.size();
        let chunks = (size as f64 / step as f64).ceil() as usize;
        let mut start: f64 = 0.0;
        for _ in 0..chunks {
            let mut end = start + step;
            end = if end >= size {
                size
            } else {
                end
            };
            let data = self.blob.slice_with_f64_and_f64(start, end).expect("slice blob failed!");
            let buffer: JsValue = JsFuture::from(data.array_buffer()).await.expect("get arraybuffer failed!").into();
            match hasher.clone() {
                HA::Md5(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
                HA::Sha224(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
                HA::Sha256(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
                HA::Sha384(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
                HA::Sha512(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
                HA::SM3(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
                HA::Sha1(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
                HA::Sha3_224(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
                HA::Sha3_256(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
                HA::Sha3_384(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
                HA::Sha3_512(mut hasher) => {
                    hasher.update(&Uint8Array::new(&buffer).to_vec());
                }
            }
            let _ = self.progress.call1(&JsValue::null(), &JsValue::from(start / size * 100.0));
            start += step;
        };
        return match hasher {
            HA::Md5(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }
            HA::Sha224(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }
            HA::Sha256(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }
            HA::Sha384(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }
            HA::Sha512(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }
            HA::SM3(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }
            HA::Sha1(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }
            HA::Sha3_224(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }
            HA::Sha3_256(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }
            HA::Sha3_384(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }
            HA::Sha3_512(hasher) => {
                Ok(format!("{:x}", hasher.finalize()))
            }

        };
    }
}

#[derive(Debug, Clone)]
pub enum HA {
    Md5(Md5),
    Sha1(Sha1),
    Sha224(Sha224),
    Sha256(Sha256),
    Sha384(Sha384),
    Sha512(Sha512),
    SM3(Sm3),
    Sha3_224(Sha3_224),
    Sha3_256(Sha3_256),
    Sha3_384(Sha3_384),
    Sha3_512(Sha3_512),
}