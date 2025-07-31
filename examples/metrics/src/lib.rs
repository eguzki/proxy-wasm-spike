// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use log::info;
use log::warn;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(MyProxyRoot{ok_metric_id: 0}) });
}}

struct MyProxyRoot {
    pub ok_metric_id: u32,
}

impl Context for MyProxyRoot {}

impl RootContext for MyProxyRoot {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        self.ok_metric_id = match proxy_wasm::hostcalls::define_metric(
            MetricType::Counter,
            "mynamespace.metric.ok",
        ) {
            Ok(id) => id,
            Err(e) => panic!("Error: {:?}", e),
        };
        true
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MyProxy {
            context_id,
            ok_metric_id: self.ok_metric_id,
        }))
    }
}

struct MyProxy {
    context_id: u32,
    ok_metric_id: u32,
}

impl HttpContext for MyProxy {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        info!("on request headers");
        for (name, value) in &self.get_http_request_headers() {
            info!("#{} -> {}: {}", self.context_id, name, value);
        }
        if let Err(e) = proxy_wasm::hostcalls::increment_metric(self.ok_metric_id, 1) {
            warn!(
                "proxy_wasm::hostcalls::increment_metric metric {}, offset 1, {e:?}",
                self.ok_metric_id
            );
        }

        Action::Continue
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("on request body");
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        info!("on response headers");
        Action::Continue
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("on response body");
        Action::Continue
    }
}

impl Context for MyProxy {}
