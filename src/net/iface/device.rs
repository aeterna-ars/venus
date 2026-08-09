use tun_rs::{SyncDevice, AsyncDevice};
use tun_rs::DeviceBuilder;

use crate::error::IoError;

pub use super::configs::IfaceConfig;

use bytes::*;

pub trait DeviceMode {
    type Device: DeviceIO;
    fn build(config: &IfaceConfig) -> Result<Self::Device, IoError>;
}

pub trait DeviceIO: Send + Sync {
    fn recv_io(&mut self, buf: &mut [u8]) -> impl std::future::Future<Output = Result<usize, IoError>> + Send;
    fn send_io(&mut self, buf: &[u8]) -> impl std::future::Future<Output = Result<usize, IoError>> + Send;
    fn iface_id(&self) -> Result<u32, IoError>;
}

pub struct Device<M: DeviceMode> {
    _config: IfaceConfig,
    inner: M::Device,
    read_buf: BytesMut,
    write_buf: BytesMut,
}

impl<M: DeviceMode> Device<M> {
    pub fn new(config: IfaceConfig) -> Result<Self, IoError> {
        let inner = M::build(&config)?;
        let capacity = config.buf_capacity;

        Ok(Self {
            read_buf: BytesMut::with_capacity(capacity),
            write_buf: BytesMut::with_capacity(capacity),
            _config: config,
            inner,
        })
    }

    pub async fn recv(&mut self) -> Result<BytesMut, IoError> {
        let buf = &mut self.read_buf;

        let dst = buf.chunk_mut();
        let dst =
            unsafe { &mut *(dst as *mut _ as *mut [std::mem::MaybeUninit<u8>] as *mut [u8]) };

        let n = self.inner.recv_io(dst).await?;

        unsafe {
            buf.advance_mut(n);
        }

        let data = self.read(n);

        Ok(data)
    }

    fn read(&mut self, len: usize) -> BytesMut {
        self.read_buf.split_to(len)
    }

    pub async fn send(&mut self, data: &[u8]) -> Result<usize, IoError> {
        self.write(data);

        let n = self.inner.send_io(&self.write_buf).await?;

        Ok(n)
    }

    pub fn write(&mut self, data: &[u8]) {
        self.write_buf.clear();
        self.write_buf.put(data)
    }

    pub fn get_iface_id(&self) -> Result<u32, IoError> {
        let id = self.inner.iface_id()?;
        Ok(id)
    }
}

pub struct SyncMode;

impl DeviceMode for SyncMode {
    type Device = SyncDevice;

    fn build(config: &IfaceConfig) -> Result<Self::Device, IoError> {
        DeviceBuilder::new()
            .name(&config.name)
            .ipv4(config.addr, config.netmask, config.destination)
            .mtu(config.mtu)
            .offload(config.offload)
            .enable(true)
            .build_sync()
            .map_err(|e| IoError(format!("build error: {e}")))
    }
}

impl DeviceIO for SyncDevice {
    async fn recv_io(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        let n = self.recv(buf)
            .map_err(|e| IoError(format!("recv error: {e}")))?;

        Ok(n)
    }

    async fn send_io(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        let n = self.send(buf)
            .map_err(|e| IoError(format!("recv error: {e}")))?;

        Ok(n)
    }

    fn iface_id(&self) -> Result<u32, IoError> {
        self.if_index().map_err(|e| IoError(format!("get iface index error: {e}")))
    }
}

pub struct AsyncMode;

impl DeviceMode for AsyncMode {
    type Device = AsyncDevice;

    fn build(config: &IfaceConfig) -> Result<Self::Device, IoError> {
        DeviceBuilder::new()
            .name(&config.name)
            .ipv4(config.addr, config.netmask, config.destination)
            .mtu(config.mtu)
            .offload(config.offload)
            .enable(true)
            .build_async()
            .map_err(|e| IoError(format!("build error: {e}")))
    }
}

impl DeviceIO for AsyncDevice {
    async fn recv_io(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        let n = self.recv(buf)
            .await
            .map_err(|e| IoError(format!("recv error: {e}")))?;

        Ok(n)
    }

    async fn send_io(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        let n = self.send(buf)
            .await
            .map_err(|e| IoError(format!("send error: {e}")))?;

        Ok(n)
    }

    fn iface_id(&self) -> Result<u32, IoError> {
        self.if_index().map_err(|e| IoError(format!("get iface index error: {e}")))
    }
}

#[cfg(unix)]
pub struct UnixMode;

#[cfg(unix)]
impl DeviceMode for UnixMode {
    type Device = SyncDevice;
    
    fn build(config: &IfaceConfig) -> Result<Self::Device, IoError> {
        let fd = config.fd.ok_or_else(|| IoError("unix fd required".to_string()))?;
        unsafe {
            SyncDevice::from_fd(fd).map_err(|e| IoError(format!("build unix device error: {e}")))
        }
    }
}