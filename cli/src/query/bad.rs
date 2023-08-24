use crate::query::QueryConfig;
use colored_json::ToColoredJson;
use guac::client::GuacClient;
use std::process::ExitCode;

#[derive(clap::Args, Debug)]
#[command(
    about = "Run the query to find CertifyBad information of the package (purl)",
    args_conflicts_with_subcommands = true
)]
pub struct BadCommand {
    #[command(flatten)]
    pub(crate) config: QueryConfig,
}

impl BadCommand {
    pub async fn run(self) -> anyhow::Result<ExitCode> {
        /*
        let guac = GuacClient::new(self.config.guac_url);
        let good = guac.certify_bad(&self.config.purl).await?;
        let out =
            serde_json::to_string(&good)?.to_colored_json(crate::color_mode(self.config.color))?;
        println!("{}", out);

         */
        Ok(ExitCode::SUCCESS)
    }
}
