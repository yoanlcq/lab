#[derive(Debug)]
#[repr(C)]
pub struct ShardMetadata {
    pub name: &'static str,
    pub description: &'static str,
}

impl ShardMetadata {
    pub const DEFAULT: Self = Self {
        name: "",
        description: "",
    };
}

impl Default for ShardMetadata {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[macro_export]
macro_rules! shard {
    (|$s:ident| $load:expr) => {
        #[no_mangle]
        pub fn on_load_shard($s: &$crate::OnLoad) -> $crate::ShardMetadata {
            $load
        }
    };
}


