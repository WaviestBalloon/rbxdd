pub static PLAYER_EXTRACT_BINDINGS: [(&str, &str); 22] = [
	("RobloxApp.zip", ""),
	("redist.zip", ""),
	("shaders.zip", "shaders/"),
	("ssl.zip", "ssl/"),
	("WebView2.zip", ""),
	("WebView2RuntimeInstaller.zip", "WebView2RuntimeInstaller/"), // (Not usual required)

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
pub static STUDIO_EXTRACT_BINDINGS: [(&str, &str); 34] = [
	("RobloxStudio.zip", ""),
	("RibbonConfig.zip", "RibbonConfig/"),
	("redist.zip", ""),
	("Libraries.zip", ""),
	("LibrariesQt5.zip", ""),
	("WebView2.zip", ""),
	("WebView2RuntimeInstaller.zip", "WebView2RuntimeInstaller/"), // (Not usual required)

	("shaders.zip", "shaders/"),
	("ssl.zip", "ssl/"),
	("Qml.zip", "Qml/"),
	("Plugins.zip", "Plugins/"),
	("StudioFonts.zip", "StudioFonts/"),
	("BuiltInPlugins.zip", "BuiltInPlugins/"),
	("ApplicationConfig.zip", "ApplicationConfig/"),
	("BuiltInStandalonePlugins.zip", "BuiltInStandalonePlugins/"),

	("content-qt_translations.zip", "content/qt_translations/"),
	("content-sky.zip", "content/sky/"),
	("content-fonts.zip", "content/fonts/"),
	("content-avatar.zip", "content/avatar/"),
	("content-models.zip", "content/models/"),
	("content-sounds.zip", "content/sounds/"),
	("content-configs.zip", "content/configs/"),
	("content-api-docs.zip", "content/api_docs/"),
	("content-studio_svg_textures.zip", "content/studio_svg_textures/"),
	("content-terrain.zip", "PlatformContent/pc/terrain/"),
	("content-textures2.zip", "content/textures/"),
	("content-textures3.zip", "PlatformContent/pc/textures/"),

	("content-platform-fonts.zip", "PlatformContent/pc/fonts/"),
	("content-platform-dictionaries.zip", "PlatformContent/pc/shared_compression_dictionaries/"),

	("extracontent-translations.zip", "ExtraContent/translations/"),
	("extracontent-luapackages.zip", "ExtraContent/LuaPackages/"),
	("extracontent-textures.zip", "ExtraContent/textures/"),
	("extracontent-scripts.zip", "ExtraContent/scripts/"),
	("extracontent-models.zip", "ExtraContent/models/")
];

pub static LATEST_VERSION_PLAYER: &str = "https://clientsettings.roblox.com/v2/client-version/WindowsPlayer";
pub static LATEST_VERSION_STUDIO: &str = "https://clientsettings.roblox.com/v2/client-version/WindowsStudio64";
pub static DEPLOYMENT_CDN: &str = "https://setup.rbxcdn.com";
pub static DEPLOYMENT_CDN_CACHEFLY: &str = "https://roblox-setup.cachefly.net";
pub static CHANNEL_DEPLOYMENT_CDN: &str = "https://roblox-setup.cachefly.net/channel";
