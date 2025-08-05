use std::{any::Any, fmt::Debug, sync::Arc};

mod imp_vulkan;

pub fn test() {
    ApiArc::create(&ApiParams {  }).unwrap().create_device(&DeviceParams {  }).unwrap();
}

/// Just a newtype to enhance an Arc<Api> with better methods.
/// The definition of this type will not ever change, it will always be an Arc<Api>.
#[derive(Debug, Clone)]
pub struct ApiArc(Arc<ApiInner>);


#[derive(Debug)]
struct ApiInner {
    imp: Box<dyn ApiImpl>,
}

pub struct ApiParams {

}

trait ApiImpl: Debug {
    fn as_any(&self) -> &dyn Any;
    fn create_device(&self, api_arc: &ApiArc, params: &DeviceParams) -> Result<Box<dyn DeviceImpl>, std::io::Error>;
}

impl ApiArc {
    pub fn create(params: &ApiParams) -> Result<Self, std::io::Error> {
        let imp = Box::new(imp_vulkan::VulkanApi::create(params)?);
        Ok(Self(Arc::new(ApiInner { imp })))
    }
    pub fn create_device(&self, params: &DeviceParams) -> Result<DeviceArc, std::io::Error> {
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

pub struct DeviceParams {

}

pub trait DeviceImpl: Debug {
    fn as_any(&self) -> &dyn Any;
}

