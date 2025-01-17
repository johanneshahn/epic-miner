// Copyright 2018 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Configuration file management

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use core::config::MinerConfig;
use toml;
use crate::types::{ConfigError, ConfigMembers, GlobalConfig};
use util::LoggingConfig;

extern crate dirs;

/// The default file name to use when trying to derive
/// the config file location

const CONFIG_FILE_NAME: &'static str = "epic-miner.toml";
const EPIC_HOME: &'static str = ".epic";

/// Returns the defaults, as strewn throughout the code
impl Default for ConfigMembers {
	fn default() -> ConfigMembers {
		ConfigMembers {
			mining: MinerConfig::default(),
			logging: Some(LoggingConfig::default()),
		}
	}
}

impl Default for GlobalConfig {
	fn default() -> GlobalConfig {
		GlobalConfig {
			config_file_path: None,
			using_config_file: false,
			members: Some(ConfigMembers::default()),
		}
	}
}

impl GlobalConfig {
	/// Copy the epic-miner.toml from the default locations to the current folder
	pub fn copy_config_file(&mut self) -> Result<(), ConfigError> {
		let mut config_path_new = env::current_dir().unwrap();
		config_path_new.push(CONFIG_FILE_NAME);
		if config_path_new.exists() {
			return Err(ConfigError::FileAlreadyExistsError());
		}
		self.derive_config_location()?;
		let config_path_original = self
			.config_file_path
			.clone()
			.unwrap_or(PathBuf::from("".to_owned()));
		std::fs::copy(&config_path_original, &config_path_new).map_err(|e| {
			ConfigError::FileIOError(
				format!(
					"Unable to copy the file {} to {}! :",
					config_path_original.display(),
					config_path_new.display()
				),
				format!("{:?}", e),
			)
		})?;
		println!(
			"Successfully copied the file {} to {}",
			config_path_original.display(),
			config_path_new.display()
		);
		Ok(())
	}

	fn derive_config_location(&mut self) -> Result<(), ConfigError> {
		// First, check working directory
		let mut config_path = env::current_dir().unwrap();
		config_path.push(CONFIG_FILE_NAME);
		if config_path.exists() {
			self.config_file_path = Some(config_path);
			return Ok(());
		}
		println!(
			"The file {} was not found! Moving to the next location!",
			config_path.display()
		);
		// Next, look in directory of executable
		let mut config_path = env::current_exe().unwrap();
		config_path.pop();
		config_path.push(CONFIG_FILE_NAME);
		if config_path.exists() {
			self.config_file_path = Some(config_path);
			return Ok(());
		}
		println!(
			"The file {} was not found! Moving to the next location!",
			config_path.display()
		);
		// Then look in {user_home}/.epic
		let config_path = dirs::home_dir();
		if let Some(mut p) = config_path {
			p.push(EPIC_HOME);
			p.push(CONFIG_FILE_NAME);
			if p.exists() {
				self.config_file_path = Some(p);
				return Ok(());
			}
			println!(
				"The file {} was not found! Moving to the next location!",
				p.display()
			);
		}
		// Then look in /etc/epic-miner.toml
		let config_path = PathBuf::from(r"/etc/epic-miner.toml");
		if config_path.exists() {
			self.config_file_path = Some(config_path);
			return Ok(());
		}
		println!("The file {} was not found!", config_path.display());
		// Give up
		Err(ConfigError::FileNotFoundError())
	}

	/// Takes the path to a config file, or if NONE, tries
	/// to determine a config file based on rules in
	/// derive_config_location

	pub fn new(file_path: Option<&str>) -> Result<GlobalConfig, ConfigError> {
		let mut return_value = GlobalConfig::default();
		if let Some(fp) = file_path {
			return_value.config_file_path = Some(PathBuf::from(&fp));
		} else {
			let _result = return_value.derive_config_location();
		}

		// No attempt at a config file, just return defaults
		if let None = return_value.config_file_path {
			return Ok(return_value);
		}

		// Config file path is given but not valid
		if !return_value.config_file_path.as_mut().unwrap().exists() {
			println!(
				"Checking the file {}",
				return_value.config_file_path.unwrap().display()
			);
			return Err(ConfigError::FileNotFoundError());
		}

		// Try to parse the config file if it exists
		// explode if it does exist but something's wrong
		// with it
		return_value.read_config()
	}

	/// Read config
	pub fn read_config(mut self) -> Result<GlobalConfig, ConfigError> {
		let mut file = File::open(self.config_file_path.as_mut().unwrap())?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;
		let decoded: Result<ConfigMembers, toml::de::Error> = toml::from_str(&contents);
		match decoded {
			Ok(gc) => {
				// Put the struct back together, because the config
				// file was flattened a bit
				self.using_config_file = true;
				self.members = Some(gc);
				return Ok(self);
			}
			Err(e) => {
				return Err(ConfigError::ParseError(
					String::from(
						self.config_file_path
							.as_mut()
							.unwrap()
							.to_str()
							.unwrap()
							
					),
					String::from(format!("{}", e)),
				));
			}
		}
	}

	/// Serialize config
	pub fn ser_config(&mut self) -> Result<String, ConfigError> {
		let encoded: Result<String, toml::ser::Error> =
			toml::to_string(self.members.as_mut().unwrap());
		match encoded {
			Ok(enc) => return Ok(enc),
			Err(e) => {
				return Err(ConfigError::SerializationError(String::from(format!(
					"{}",
					e
				))));
			}
		}
	}
}
