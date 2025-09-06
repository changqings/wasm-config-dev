use std::collections::HashMap;

use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::{Deserialize, Serialize};

struct MyHttp {
    config: MyConfig,
}

#[derive(Serialize, Deserialize, Clone)]
struct MyConfig {
    cdn_info: HashMap<String, String>,
}

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(MyHttp::new())
    });
}}

impl MyHttp {
    fn new() -> Self {
        MyHttp {
            config: MyConfig {
                cdn_info: HashMap::new(),
            },
        }
    }
}

// impl httpContext
impl Context for MyHttp {}
impl HttpContext for MyHttp {
    fn on_http_request_headers(&mut self, _num_headers: usize, _ok: bool) -> Action {
        for (key, value) in self.config.cdn_info.iter() {
            self.add_http_request_header(key, value);
        }
        Action::Continue
    }
}

// impl RootContext
impl RootContext for MyHttp {
    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        info!("Plugin is configuring");
        if let Some(config) = self.get_plugin_configuration() {
            if let Some(config) = serde_json::from_slice(&config).ok() {
                self.config = config;
            }
        }
        true
    }
    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MyHttp {
            config: self.config.clone(),
        }))
    }
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
