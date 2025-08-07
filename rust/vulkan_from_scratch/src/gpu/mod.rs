use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

mod imp_vulkan;

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::expect_used)]
pub fn test() {
    let api = ApiArc::create(&ApiParams {}).expect("Failed to create Api");
    let device = api.create_device(&DeviceParams {}).expect("Failed to create device");
    device.test();
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Vulkan")]
    Vulkan(#[from] ash::vk::Result),
    #[error("IO")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Just a newtype to enhance an Arc<Api> with better methods.
/// The definition of this type will not ever change, it will always be an Arc<Api>.
#[derive(Debug, Clone)]
pub struct ApiArc(Arc<ApiInner>);

#[derive(Debug)]
struct ApiInner {
    imp: Box<dyn ApiImpl>,
}

pub struct ApiParams {}

trait ApiImpl: Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn create_device(&self, api_arc: &ApiArc, params: &DeviceParams) -> Result<Box<dyn DeviceImpl>>;
}

impl ApiArc {
    pub fn create(params: &ApiParams) -> Result<Self> {
        let imp = Box::new(imp_vulkan::VulkanApi::create(params)?);
        Ok(Self(Arc::new(ApiInner { imp })))
    }
    pub fn create_device(&self, params: &DeviceParams) -> Result<DeviceArc> {
        let imp = self.0.imp.create_device(self, params)?;
        Ok(DeviceArc(Arc::new(DeviceInner { imp })))
    }
}

#[derive(Debug, Clone)]
pub struct DeviceArc(Arc<DeviceInner>);

#[derive(Debug)]
pub struct DeviceInner {
    imp: Box<dyn DeviceImpl>,
}

impl DeviceArc {
    pub fn test(&self) {
        // TODO: remove this function soon, of course
        _ = self.0.imp;
    }
}

pub struct DeviceParams {}

pub trait DeviceImpl: Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}
