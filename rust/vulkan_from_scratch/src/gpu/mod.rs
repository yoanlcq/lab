mod imp_vulkan;

pub fn test() {
    Api::create(&ApiParams {  }).unwrap();
}

pub struct Api {
    imp: Box<dyn ApiImpl>,
}

pub struct ApiParams {

}

pub trait ApiImpl {

}

impl Api {
    pub fn create(params: &ApiParams) -> Result<Self, std::io::Error> {
        Ok(Self {
            imp: Box::new(imp_vulkan::Api::create(params)?),
        })
    }
}

pub struct Device {
    imp: Box<dyn DeviceImpl>,
}

pub trait DeviceImpl {
    
}

