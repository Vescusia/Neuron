use base64::Engine;

#[derive(Debug)]
pub struct PuuidDecoder<const LEN: usize> {
    buf: [u8; LEN]
}

impl<const LEN: usize> PuuidDecoder<LEN> {
    pub const fn new() -> Self {
        Self{
            buf: [0u8; LEN]
        }
    }

    pub fn decode<S: AsRef<[u8]>>(&mut self, input: S) -> anyhow::Result<&[u8]> {
        let read = base64::prelude::BASE64_URL_SAFE_NO_PAD.decode_slice(input, &mut self.buf)?;
        if read != self.buf.len() {
            anyhow::bail!("Invalid decode buffer len!")
        }
        Ok(&self.buf)
    }
}

impl<const LEN: usize> Default for PuuidDecoder<LEN> {
    fn default() -> Self {
        Self::new()
    }
}


pub struct PuuidEncoder<const LEN: usize> {
    buf: [u8; LEN]
}

impl<const LEN: usize> PuuidEncoder<LEN> {
    pub const fn new() -> Self {
        Self{
            buf: [0u8; LEN]
        }
    }

    pub fn encode<S: AsRef<[u8]>>(&mut self, input: S) -> anyhow::Result<&str> {
        let written = base64::prelude::BASE64_URL_SAFE_NO_PAD.encode_slice(input, &mut self.buf)?;
        if written != self.buf.len() {
            anyhow::bail!("Invalid encode buffer len!")
        }
        Ok(std::str::from_utf8(&self.buf).expect("Invalid UTF-8 in input!"))
    }
}

impl<const LEN: usize> Default for PuuidEncoder<LEN> {
    fn default() -> Self {
        Self::new()
    }
}