use curl::easy::Easy;
use serde_json::Value;
use crate::{bindings, error::{NoAccessError, NotFoundError}};

#[derive(Debug)]
pub struct Manifest {
	pub version: String,
	pub contents: Vec<ManifestPackage>
}
#[derive(Debug)]
pub struct ManifestPackage {
	pub package_name: String,
	pub hash: String,
}

pub enum Binary {
	Studio,
	Player
}

pub fn get_latest_version(binary: Binary, channel: Option<&str>) -> Result<String, String> {
	let base_clientsettings_url = match binary {
		Binary::Player => bindings::LATEST_VERSION_PLAYER,
		Binary::Studio => bindings::LATEST_VERSION_STUDIO
	};
	let mut clientsettings_url = base_clientsettings_url.to_string();
	if let Some(channel) = channel { // If a channel was passed, append it to the URL
		clientsettings_url.push_str(&format!("/channel/{}", channel));
	}
	
	let mut curl = Easy::new();
	curl.url(&clientsettings_url).unwrap();
	let mut response = Vec::new();
	{
		let mut transfer = curl.transfer();
		transfer.write_function(|data| {
			response.extend_from_slice(data);
			Ok(data.len())
		}).unwrap();
		transfer.perform().unwrap();
	}

	let http_code = curl.response_code().unwrap();
	match http_code {
		200 => {},
		401 => return Err(NoAccessError { // The channel is restricted to Roblox employees only
			http_code,
			http_body: String::from_utf8(response).unwrap(),
			message: format!("Channel {} is a restricted channel", channel.unwrap())
		}.to_string()),
		404 => return Err(NotFoundError { // The channel doesn't exist
			http_code,
			http_body: String::from_utf8(response).unwrap(),
			message: format!("Channel {} is a invalid channel", channel.unwrap())
		}.to_string()),
		_ => return Err(format!("Failed to get latest version, response code: {}", http_code))
	}

	let string_readable = String::from_utf8(response).unwrap();
	let json: Value = serde_json::from_str(&string_readable).unwrap();

	Ok(json["clientVersionUpload"].as_str().unwrap().to_string())
}

pub fn get_manifest(version_hash: String) -> Result<Manifest, String> {
	let pkg_manifest_url = format!("{}/{}-rbxPkgManifest.txt", bindings::DEPLOYMENT_CDN, version_hash); // E.g. roblox-setup.cachefly.net/version-2355c01e37774010-rbxPkgManifest.txt
	let mut curl = Easy::new();
	curl.url(&pkg_manifest_url).unwrap();
	let mut response = Vec::new();
	{
		let mut transfer = curl.transfer();
		transfer.write_function(|data| {
			response.extend_from_slice(data);
			Ok(data.len())
		}).unwrap();
		transfer.perform().unwrap();
	}

	let http_code = curl.response_code().unwrap();
	match http_code {
		200 => {},
		403 => return Err(NoAccessError {
			http_code,
			http_body: String::from_utf8(response).unwrap(),
			message: "ClientSettings endpoint returned 403 Forbidden".to_string()
		}.to_string()),
		_ => return Err(format!("Failed to get latest version, response code: {}", http_code))
	}

	let string_readable = String::from_utf8(response).unwrap();
	let manifest_vec = string_readable.split('\n')
		.collect::<Vec<&str>>().iter()
		.map(|s| s.trim())
		.collect::<Vec<&str>>(); // Parse the manifest into a vector of strings while also trimming out `\r` and the new line at the end of the response
	let mut manifest = Manifest { // TODO: messy :(
		version: manifest_vec[0].to_string(), // First line of the manifest is the version
		contents: Vec::new()
	};

	println!("{:?}", manifest_vec);

	for (i, package_string) in manifest_vec.iter().enumerate().skip(1) { // Skip version line
		if (i + 1) % 2 == 0 || package_string.parse::<usize>().is_ok() { // TODO: make this better, it hurts to look at - skip over number entries as well
			continue;
		}

		let package_name = manifest_vec[i - 1];
		let package_hash = manifest_vec[i];
		manifest.contents.push(ManifestPackage {
			package_name: package_name.to_string(),
			hash: package_hash.to_string()
		});
	}

	Ok(manifest)
}
