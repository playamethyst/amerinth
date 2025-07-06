use crate::prelude::*;
use crate::projects::ProjectTypes;
use std::collections::HashMap;
use strum::EnumString;

/// ### Get a list of loaders
///
/// Gets an array of loaders, their icons, and supported project types.
///
/// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/loaderlist/) for more details.
pub async fn loaders<Auth: AuthState>(
    modrinth: &Modrinth<Auth>,
) -> Result<HashMap<Loader, LoaderInfo>, ModrinthError> {
    #[derive(Endpoint)]
    #[endpoint(
        method = "GET",
        path = "v2/tag/loader",
        response = "Vec<LoaderResponse>"
    )]
    struct GetLoaders;

    #[derive(Deserialize)]
    struct LoaderResponse {
        name: Loader,
        #[serde(flatten)]
        info: LoaderInfo,
    }

    let res: Vec<LoaderResponse> = exec!(GetLoaders, modrinth)?.parse()?;

    Ok(res
        .into_iter()
        .map(|loader| (loader.name, loader.info))
        .collect())
}

/// Represents the different types of mod loaders, plugin platforms, proxies,
/// and other mechanisms used to modify or extend Minecraft.
#[derive(Debug, Clone, EnumString, Hash, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum Loader {
    /// Babric brings [Fabric](Loader::Fabric) support to beta versions of Minecraft.
    Babric,
    /// Bukkit is a popular API and implementation for creating server-side plugins.
    Bukkit,
    /// Bungeecord is a proxy system that links multiple Minecraft servers together.
    Bungeecord,
    /// Canvas is a rendering engine mod that extends the Fabric rendering pipeline.
    Canvas,
    /// A data pack is a collection of data-driven content that modifies or adds to the game's mechanics.
    Datapack,
    /// Fabric is a lightweight, modular modding toolchain for Minecraft.
    Fabric,
    /// Folia is a high-performance fork of Paper with a regionized threading model.
    Folia,
    /// Forge is a widely-used Minecraft modding API that allows for deep modifications to the game.
    Forge,
    /// Iris is a mod that adds modern shader support and integrates with existing mods like Sodium.
    Iris,
    /// JavaAgent refers to mods or utilities that use Java instrumentation to modify the game at runtime.
    JavaAgent,
    /// Legacy Fabric brings Fabric-like modding support to older versions of Minecraft, including classic and alpha.
    LegacyFabric,
    /// LiteLoader is a lightweight modding platform for older versions of Minecraft.
    LiteLoader,
    /// Minecraft refers to the base game with no loaders or modifications.
    Minecraft,
    /// Modloader is one of the earliest Minecraft modding platforms, predating Forge.
    Modloader,
    /// Neoforge is a modern fork of Forge aiming for cleaner architecture and active development.
    Neoforge,
    /// Nilloader is a minimal mod loader designed for maximum compatibility and portability.
    Nilloader,
    /// Optifine is a standalone client-side mod that enhances performance and adds visual features.
    Optifine,
    /// Ornithe is a mod loader forked from Fabric, focused on expanding legacy support and experimentation.
    Ornithe,
    /// Paper is a high-performance Minecraft server built on top of Spigot with additional optimizations.
    Paper,
    /// Purpur is a fork of Paper that adds more configuration options and gameplay features.
    Purpur,
    /// Quilt is a community-driven fork of Fabric with additional modularity and loader features.
    Quilt,
    /// Rift is a lightweight modding API for Minecraft 1.13.
    Rift,
    /// Spigot is a performance-optimized fork of Bukkit used for running Minecraft servers with plugins.
    Spigot,
    /// Sponge is a modding and plugin platform designed to be compatible with Forge.
    Sponge,
    /// Vanilla refers to the unmodified Minecraft client or server.
    Vanilla,
    /// Velocity is a modern, high-performance Minecraft proxy for linking multiple servers.
    Velocity,
    /// Waterfall is a fork of Bungeecord that improves performance and bug fixes.
    Waterfall,
    /// A custom or unknown loader, represented by a string.
    #[strum(disabled)]
    Other(String),
}

deserialize_other!(Loader);

#[derive(Debug, Deserialize)]
pub struct LoaderInfo {
    /// The SVG icon of a loader
    icon: String,
    /// The project types that this loader is applicable to
    #[serde(rename = "supported_project_types")]
    project_types: ProjectTypes,
}
