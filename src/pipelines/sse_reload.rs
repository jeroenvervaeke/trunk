use crate::pipelines::TrunkLinkPipelineOutput;
use anyhow::Result;
use async_std::task::{spawn, JoinHandle};
use nipper::Document;

pub struct SSEReloadScript;

impl SSEReloadScript {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn(self) -> JoinHandle<Result<TrunkLinkPipelineOutput>> {
        spawn(self.build())
    }

    async fn build(self) -> Result<TrunkLinkPipelineOutput> {
        Ok(TrunkLinkPipelineOutput::SSEReload(self))
    }

    pub async fn finalize(self, dom: &mut Document) -> Result<()> {
        let script = format!(r#"<script>{}</script>"#, include_str!("sse_reload.js"));

        dom.select("html head").append_html(script);

        Ok(())
    }
}
