## rbxdd

🚰 Cross-platform library for processing Roblox deployments and assisting with install construction, work-in-progress.

### Example - Get info of a version's rbxPkgManifest

```rust
use rbxdd::rbxcdn::{get_latest_version, get_manifest, Binary, Manifest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let version_hash: String = get_latest_version(Binary::Player, None)?;
	let manifest: Manifest = get_manifest(&version_hash)?;

	println!("{}'s manifest contains {} packages!", version_hash, manifest.contents.len());

	for package in manifest.contents {
		println!("Name: {} - Hash: {}", package.package_name, package.hash);
	}

	Ok(())
}
```
