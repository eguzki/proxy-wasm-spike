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
use std::time::Duration;

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
        Some(Box::new(HttpBody {
            context_id,
            request_phase: false,
        }))
    }
}

struct HttpBody {
    context_id: u32,
    request_phase: bool,
}

impl HttpContext for HttpBody {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        info!("on request headers yeah!");
        for (name, value) in &self.get_http_request_headers() {
            info!("#{} -> {}: {}", self.context_id, name, value);
        }

        Action::Continue
    }

    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        info!("on request body");
        if !end_of_stream {
            // Wait -- we'll be called again when the complete body is buffered
            // at the host side.
            return Action::Pause;
        }
        info!("request body: end of stream {body_size}");

        if let Some(body_bytes) = self.get_http_request_body(0, body_size) {
            let body_str = String::from_utf8(body_bytes).unwrap();
            info!("request body: {:}", body_str);
        }

        info!("request body: run grpc call");
        self.request_phase = true;
        self.dispatch_grpc_call(
            "rlsbin",
            "envoy.service.ratelimit.v3.RateLimitService",
            "ShouldRateLimit",
            vec![],
            None,
            Duration::from_secs(1),
        )
        .unwrap();
        Action::Pause
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        info!("on response headers");
        self.request_phase = false;
        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        info!("on response body");
        if !end_of_stream {
            // Wait -- we'll be called again when the complete body is buffered
            // at the host side.
            return Action::Pause;
        }

        if let Some(body_bytes) = self.get_http_response_body(0, body_size) {
            let body_str = String::from_utf8(body_bytes).unwrap();
            info!("response body: {:?}", body_str);
        }

        info!("on response body: run grpc call");
        self.dispatch_grpc_call(
            "rlsbin",
            "envoy.service.ratelimit.v3.RateLimitService",
            "ShouldRateLimit",
            vec![],
            None,
            Duration::from_secs(1),
        )
        .unwrap();
        Action::Pause
    }
}

impl Context for HttpBody {
    fn on_grpc_call_response(&mut self, token_id: u32, status_code: u32, _: usize) {
        info!("on_grpc_call_response: token_id: {token_id}, status_code:{status_code}, request_phase: {:?}", self.request_phase);
        match self.request_phase {
            true => self.resume_http_request(),
            false => self.resume_http_response(),
        }
    }
}
