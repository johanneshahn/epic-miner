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

//! Public types for config modules


use std::path::PathBuf;
use std::{fmt, io};

use core::config::MinerConfig;

use util;

/// Error type wrapping config errors.
#[derive(Debug)]
pub enum ConfigError {
	/// Error with parsing of config file
	ParseError(String, String),

	/// Error with fileIO while reading config file
	FileIOError(String, String),

	/// No file found
	FileNotFoundError(),

	/// Error serializing config values
	SerializationError(String),

	/// Error when trying to create another epic-miner.toml
	/// and the file already exists in the current directory
	FileAlreadyExistsError(),
}

impl fmt::Display for ConfigError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ConfigError::ParseError(ref file_name, ref message) => write!(
				f,
				"Error parsing configuration file at {} - {}",
				file_name, message
			),
			ConfigError::FileIOError(ref file_name, ref message) => {
				write!(f, "{} {}", message, file_name)
			}
			ConfigError::FileNotFoundError() => {
				write!(f, "Could not find a valid epic-miner.toml!")
			}
			ConfigError::SerializationError(ref message) => {
				write!(f, "Error serializing configuration: {}", message)
			}
			ConfigError::FileAlreadyExistsError() => {
				write!(f, "It's not possible to create a new epic-miner.toml, a file with the same name already exists in this folder!")
			}
		}
	}
}

impl From<io::Error> for ConfigError {
	fn from(error: io::Error) -> ConfigError {
		ConfigError::FileIOError(
			String::from(""),
			String::from(format!("Error loading config file: {}", error)),
		)
	}
}

/// separately for now, then put them together as a single
/// ServerConfig object afterwards. This is to flatten
/// out the configuration file into logical sections,
/// as they tend to be quite nested in the code
/// Most structs optional, as they may or may not
/// be needed depending on what's being run
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
	/// Keep track of the file we've read
	pub config_file_path: Option<PathBuf>,
	/// keep track of whether we're using
	/// a config file or just the defaults
	/// for each member
	pub using_config_file: bool,
	/// Global member config
	pub members: Option<ConfigMembers>,
}

/// Keeping an 'inner' structure here, as the top
/// level GlobalConfigContainer options might want to keep
/// internal state that we don't necessarily
/// want serialised or deserialised
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigMembers {
	/// Server config
	/// Mining config
	pub mining: MinerConfig,
	/// Logging config
	pub logging: Option<util::types::LoggingConfig>,
}
