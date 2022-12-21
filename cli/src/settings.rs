use std::io::{self, Write};
use std::path::PathBuf;

use printnanny_settings::error::PrintNannySettingsError;
use printnanny_settings::printnanny::PrintNannySettings;
use printnanny_settings::vcs::VersionControlledSettings;
use printnanny_settings::SettingsFormat;

pub struct SettingsCommand;

impl SettingsCommand {
    pub async fn handle(sub_m: &clap::ArgMatches) -> Result<(), PrintNannySettingsError> {
        let config: PrintNannySettings = PrintNannySettings::new()?;
        match sub_m.subcommand() {
            Some(("clone", args)) => {
                let dir = args.value_of("dir").map(PathBuf::from).unwrap();
                let settings = PrintNannySettings::new()?;
                settings.init_git_repo(&dir, &settings.git)?;
            }
            Some(("get", args)) => {
                let key = args.value_of("key");
                let f: SettingsFormat = args.value_of_t("format").unwrap();
                let v = match f {
                    SettingsFormat::Json => match key {
                        Some(k) => {
                            let data = PrintNannySettings::find_value(k)?;
                            serde_json::to_vec_pretty(&data)?
                        }
                        None => {
                            let data = PrintNannySettings::new()?;
                            serde_json::to_vec_pretty(&data)?
                        }
                    },
                    SettingsFormat::Toml => match key {
                        Some(k) => {
                            let data = PrintNannySettings::find_value(k)?;
                            toml::ser::to_vec(&data)?
                        }
                        None => {
                            let data = PrintNannySettings::new()?;
                            toml::ser::to_vec(&data)?
                        }
                    },
                    SettingsFormat::Ini | SettingsFormat::Yaml => todo!(),
                };
                io::stdout().write_all(&v)?;
            }
            Some(("set", args)) => {
                let key = args.value_of("key").unwrap();
                let value = args.value_of("value").unwrap();
                let figment = PrintNannySettings::figment()?;
                let data = figment::providers::Serialized::global(key, &value);
                let figment = figment.merge(data);
                let config: PrintNannySettings = figment.extract()?;
                let content = config.to_toml_string()?;
                let now = std::time::SystemTime::now();
                config
                    .save_and_commit(
                        &content,
                        Some(format!("PrintNannySettings.{} updated at {:?}", key, now)),
                    )
                    .await?;
            }
            Some(("show", args)) => {
                let f: SettingsFormat = args.value_of_t("format").unwrap();
                let v = match f {
                    SettingsFormat::Json => serde_json::to_vec_pretty(&config)?,
                    SettingsFormat::Toml => toml::ser::to_vec(&config)?,
                    _ => unimplemented!("show command is not implemented for format: {}", f),
                };
                io::stdout().write_all(&v)?;
            }
            _ => panic!("Expected get|set|show subcommand"),
        };
        Ok(())
    }
}
