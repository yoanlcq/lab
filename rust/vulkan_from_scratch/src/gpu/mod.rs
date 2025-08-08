use alloc::sync::Arc;
use core::fmt::Debug;
use alloc::sync::Weak;

use crate::{as_any::AsAny, windowing::WindowArc};

mod imp_vulkan;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Vulkan")]
    Vulkan(#[from] ash::vk::Result),
    #[error("IO")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

/// Just a newtype to enhance an Arc<Api> with better methods.
/// The definition of this type will not ever change, it will always be an Arc<Api>.
#[derive(Debug, Clone)]
pub struct ApiArc(Arc<ApiInner>);

#[derive(Debug)]
struct ApiInner {
    imp: Box<dyn ApiImpl>,
}

#[derive(Debug)]
pub enum ApiSpec {
    Vulkan, // TODO: add related params: version, extensions, features, etc?
}

#[derive(Debug)]
pub struct ApiParams {
    pub spec: ApiSpec,
}

trait ApiImpl: Debug + Send + Sync + AsAny {
    fn post_create(&mut self, weak_self: Weak<ApiInner>);
    fn create_device(&self, params: &DeviceParams) -> Result<Box<dyn DeviceImpl>>;
}

impl ApiArc {
    pub fn create(params: &ApiParams) -> Result<Self> {
        let imp = Box::new(match params.spec {
            ApiSpec::Vulkan => imp_vulkan::VulkanApi::create(params),
        }?);
        let mut arc = Arc::new(ApiInner { imp });
        let weak = Arc::downgrade(&arc);
        // SAFETY: We are obviously the only owner
        unsafe { Arc::get_mut(&mut arc).unwrap_unchecked() }.imp.post_create(weak);
        Ok(Self(arc))
    }
    pub fn create_device(&self, params: &DeviceParams) -> Result<DeviceArc> {
        let imp = self.0.imp.create_device(params)?;
        Ok(DeviceArc(Arc::new(DeviceInner { imp })))
    }
}

#[derive(Debug, Clone)]
pub struct DeviceArc(Arc<DeviceInner>);

#[derive(Debug)]
struct DeviceInner {
    imp: Box<dyn DeviceImpl>,
}

impl DeviceArc {
    pub fn create_swap_chain(&self, params: &SwapChainParams) -> Result<SwapChainArc> {
        let imp = self.0.imp.create_swap_chain(params)?;
        Ok(SwapChainArc(Arc::new(SwapChainInner { imp })))
    }
}

#[expect(
    clippy::empty_structs_with_brackets,
    reason = "This will surely have fields later but it's too early"
)]
pub struct DeviceParams {}

trait DeviceImpl: Debug + Send + Sync + AsAny {
    fn create_swap_chain(&self, params: &SwapChainParams) -> Result<Box<dyn SwapChainImpl>>;
}

#[derive(Debug, Clone)]
pub struct SwapChainParams<'a> {
    pub window: &'a WindowArc,
}

#[expect(dead_code, reason="WIP")]
#[derive(Debug, Clone)]
pub struct SwapChainArc(Arc<SwapChainInner>);

#[expect(dead_code, reason="WIP")]
#[derive(Debug)]
struct SwapChainInner {
    imp: Box<dyn SwapChainImpl>,
}

trait SwapChainImpl: Debug + Send + Sync + AsAny {
}