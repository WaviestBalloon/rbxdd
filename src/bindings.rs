pub static PLAYER_EXTRACT_BINDINGS: [(&str, &str); 22] = [
	("RobloxApp.zip", ""),
	("redist.zip", ""),
	("shaders.zip", "shaders/"),
	("ssl.zip", "ssl/"),
	("WebView2.zip", ""),
	("WebView2RuntimeInstaller.zip", "WebView2RuntimeInstaller/"),

	("content-avatar.zip", "content/avatar/"),
	("content-configs.zip", "content/configs/"),
	("content-fonts.zip", "content/fonts/"),
	("content-sky.zip", "content/sky/"),
	("content-sounds.zip", "content/sounds/"),
	("content-models.zip", "content/models/"),
	("content-terrain.zip", "PlatformContent/pc/terrain/"),
	("content-textures2.zip", "content/textures/"),
	("content-textures3.zip", "PlatformContent/pc/textures/"),

	("content-platform-fonts.zip", "PlatformContent/pc/fonts/"),
	("content-platform-dictionaries.zip", "PlatformContent/pc/shared_compression_dictionaries/"),

	("extracontent-luapackages.zip", "ExtraContent/LuaPackages/"),
	("extracontent-translations.zip", "ExtraContent/translations/"),
	("extracontent-models.zip", "ExtraContent/models/"),
	("extracontent-textures.zip", "ExtraContent/textures/"),
	("extracontent-places.zip", "ExtraContent/places/")
];
pub static STUDIO_EXTRACT_BINDINGS: [(&str, &str); 33] = [
	("ApplicationConfig.zip", "ApplicationConfig/"),
	("redist.zip", ""),
	("RobloxStudio.zip", ""),
	("Libraries.zip", ""),
	("content-avatar.zip", "content/avatar/"),
	("content-configs.zip", "content/configs/"),
	("content-fonts.zip", "content/fonts/"),
	("content-sky.zip", "content/sky/"),
	("content-sounds.zip", "content/sounds/"),
	("content-textures2.zip", "content/textures/"),
	("content-studio_svg_textures.zip", "content/studio_svg_textures/"),
	("content-models.zip", "content/models/"),
	("content-textures3.zip", "PlatformContent/pc/textures/"),
	("content-terrain.zip", "PlatformContent/pc/terrain/"),
	("content-platform-fonts.zip", "PlatformContent/pc/fonts/"),
	("content-platform-dictionaries.zip", "PlatformContent/pc/shared_compression_dictionaries/"),
	("content-qt_translations.zip", "content/qt_translations/"),
	("content-api-docs.zip", "content/api_docs/"),
	("extracontent-scripts.zip", "ExtraContent/scripts/"),
	("extracontent-luapackages.zip", "ExtraContent/LuaPackages/"),
	("extracontent-translations.zip", "ExtraContent/translations/"),
	("extracontent-models.zip", "ExtraContent/models/"),
	("extracontent-textures.zip", "ExtraContent/textures/"),
	("shaders.zip", "shaders/"),
	("BuiltInPlugins.zip", "BuiltInPlugins/"),
	("BuiltInStandalonePlugins.zip", "BuiltInStandalonePlugins/"),
	("LibrariesQt5.zip", ""),
	("Plugins.zip", "Plugins/"),
	("RibbonConfig.zip", "RibbonConfig/"),
	("StudioFonts.zip", "StudioFonts/"),
	("ssl.zip", "ssl/"),
	("WebView2.zip", ""),
	("WebView2RuntimeInstaller.zip", "WebView2RuntimeInstaller/")
];

pub static LATEST_VERSION_PLAYER: &str = "https://clientsettings.roblox.com/v2/client-version/WindowsPlayer";
pub static LATEST_VERSION_STUDIO: &str = "https://clientsettings.roblox.com/v2/client-version/WindowsStudio64";
pub static DEPLOYMENT_CDN: &str = "https://setup.rbxcdn.com";
pub static DEPLOYMENT_CDN_CACHEFLY: &str = "https://roblox-setup.cachefly.net";
pub static CHANNEL_DEPLOYMENT_CDN: &str = "https://roblox-setup.cachefly.net/channel";
