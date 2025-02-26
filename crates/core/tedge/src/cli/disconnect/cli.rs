use crate::cli::disconnect::disconnect_bridge::*;
use crate::command::*;
use crate::system_services::service_manager;
use structopt::StructOpt;

const C8Y_CONFIG_FILENAME: &str = "c8y-bridge.conf";
const AZURE_CONFIG_FILENAME: &str = "az-bridge.conf";

#[derive(StructOpt, Debug)]
pub enum TEdgeDisconnectBridgeCli {
    /// Remove bridge connection to Cumulocity.
    C8y,
    /// Remove bridge connection to Azure.
    Az,
}

impl BuildCommand for TEdgeDisconnectBridgeCli {
    fn build_command(self, context: BuildContext) -> Result<Box<dyn Command>, crate::ConfigError> {
        let cmd = match self {
            TEdgeDisconnectBridgeCli::C8y => DisconnectBridgeCommand {
                config_location: context.config_location.clone(),
                config_file: C8Y_CONFIG_FILENAME.into(),
                cloud: Cloud::C8y,
                use_mapper: true,
                use_agent: true,
                service_manager: service_manager(
                    context.user_manager.clone(),
                    context.config_location.tedge_config_root_path,
                )?,
            },
            TEdgeDisconnectBridgeCli::Az => DisconnectBridgeCommand {
                config_location: context.config_location.clone(),
                config_file: AZURE_CONFIG_FILENAME.into(),
                cloud: Cloud::Azure,
                use_mapper: true,
                use_agent: false,
                service_manager: service_manager(
                    context.user_manager.clone(),
                    context.config_location.tedge_config_root_path,
                )?,
            },
        };
        Ok(cmd.into_boxed())
    }
}
