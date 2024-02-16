use anyhow::{anyhow, Result};
use async_trait::async_trait;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf};

pub struct RacketLanguageServer;

#[async_trait]
impl LspAdapter for RacketLanguageServer {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("racket-langserver".into())
    }

    fn short_name(&self) -> &'static str {
        "racket-langserver"
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        _version: Box<dyn 'static + Send + Any>,
        _container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        Err(anyhow!(
            "racket-langserver (Racket language server) must be installed via raco"
        ))
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "racket-langserver".into(),
            arguments: vec![],
        })
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        None
    }
}
