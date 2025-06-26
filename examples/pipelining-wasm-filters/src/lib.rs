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
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde_json::{Result, Value};

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpBodyRoot) });
}}

struct HttpBodyRoot;

impl Context for HttpBodyRoot {}

impl RootContext for HttpBodyRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpBody { context_id }))
    }
}

struct HttpBody {
    context_id: u32,
}

impl Context for HttpBody {}

impl HttpContext for HttpBody {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        info!("#{} -> on request headers", self.context_id);
        for (name, value) in &self.get_http_request_headers() {
            info!("#{} -> {}: {}", self.context_id, name, value);
        }

        Action::Continue
    }

    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        info!("#{} -> on request body", self.context_id);
        if !end_of_stream {
            // Wait -- we'll be called again when the complete body is buffered
            // at the host side.
            return Action::Pause;
        }

        info!(
            "#{} -> request body: end of stream {body_size}",
            self.context_id
        );

        match self.get_http_request_body(0, body_size) {
            Some(body_bytes) => {
                let body_str = String::from_utf8(body_bytes).unwrap();
                info!("#{} -> request body: {:?}", self.context_id, &body_str);
                let res: Result<Value> = serde_json::from_str(body_str.as_str());
                match res {
                    Err(e) => {
                        self.send_http_response(
                            500,
                            vec![("Powered-By", "proxy-wasm")],
                            Some(format!("error:{e:?}\n").as_bytes()),
                        );
                        Action::Pause
                    }
                    Ok(_) => Action::Continue,
                }
            }
            None => {
                self.send_http_response(
                    500,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(("No bytes\n").as_bytes()),
                );
                Action::Pause
            }
        }
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        info!("#{} -> on response headers", self.context_id);
        Action::Continue
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("#{} -> on response body", self.context_id);
        Action::Continue
    }
}
