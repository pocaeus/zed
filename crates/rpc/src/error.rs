use crate::proto;
pub use proto::ErrorCode;

/// error_simple creates a new RPCError where the message is the name of the code
pub fn error_simple(c: proto::ErrorCode) -> anyhow::Error {
    RPCError {
        request: None,
        code: c,
        msg: format!("{:?}", c).to_string(),
    }
    .into()
}

/// error_detail creats a new RPCError with a code and custom message
pub fn error_detail(c: proto::ErrorCode, msg: String) -> anyhow::Error {
    RPCError {
        request: None,
        code: c,
        msg,
    }
    .into()
}

/// received_error creates a new RPCError tagged with the request that caused it
pub fn received_error(c: proto::ErrorCode, request: &str, msg: &str) -> anyhow::Error {
    RPCError {
        request: Some(request.to_string()),
        code: c,
        msg: msg.to_string(),
    }
    .into()
}

/// error_code returns the code of an RPCError, or Internal if the error is not an RPCError
pub fn error_code(e: &anyhow::Error) -> proto::ErrorCode {
    e.downcast_ref::<RPCError>()
        .map(|e| e.code)
        .unwrap_or(proto::ErrorCode::Internal)
}

#[derive(Clone, Debug)]
struct RPCError {
    request: Option<String>,
    msg: String,
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
            write!(f, "RPC request {:?} failed: {}", request, self.msg)
        } else {
            write!(f, "{}", self.msg)
        }
    }
}
