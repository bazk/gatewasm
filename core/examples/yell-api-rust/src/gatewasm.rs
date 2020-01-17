use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::Read;
use std::slice;

struct PtrReader(usize);

impl Read for PtrReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes = unsafe { slice::from_raw_parts(self.0 as _, 1) };
        buf[0] = bytes[0];
        self.0 += 1;
        Ok(1)
    }
}

pub fn get_request<O>(request: i32) -> Result<O, rmp_serde::decode::Error>
where
    O: DeserializeOwned,
{
    rmp_serde::decode::from_read(PtrReader(request as usize))
}

pub fn build_response<I>(response: &I) -> Result<i32, rmp_serde::encode::Error>
where
    I: Serialize + ?Sized,
{
    Ok(rmp_serde::encode::to_vec(response)?.as_ptr() as i32)
}
