use crate::proto;

pub fn error(c: proto::ErrorCode) -> anyhow::Error {
    RPCError {
        request: None,
        code: c,
    }
    .into()
}

pub fn received_error(c: proto::ErrorCode, request: &str) -> anyhow::Error {
    RPCError {
        request: Some(request.to_string()),
        code: c,
    }
    .into()
}

pub fn cause(e: &anyhow::Error) -> proto::ErrorCode {
    e.downcast_ref::<RPCError>()
        .map(|e| e.code)
        .unwrap_or(proto::ErrorCode::Internal)
}

#[derive(Clone, Debug)]
struct RPCError {
    request: Option<String>,
    code: proto::ErrorCode,
}

impl std::error::Error for RPCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for RPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(request) = &self.request {
            write!(f, "RPC request {:?} failed: {:?}", request, self.code)
        } else {
            write!(f, "{:?}", self.code)
        }
    }
}
