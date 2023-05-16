# Fighting Game
A simple co-op fighting game designed to be stupid and fun.
It's going to be completely over the top with weird lore.
Made with **Rust** and the [Bevy Game Engine](https://bevyengine.org).

Not going to have a story mode, or massive multiplayer. Mainly just designed for playing with your friends.
It's also going to have absurd amounts of screenshake & juice.

## Development
We use the [just](https://just.systems/) command runner for organizing build and run commands.
Recipes are placed in the `Justfile`, and can be listed with `just --list`.

`src/` contains all the main code, and different sets of functionality are placed in their own folder, loaded in with a Bevy `Plugin`.
`main.rs` should just load these plugins, and not really do much else.
Additionally, specific functionality is placed in separate files, loaded by their parent plugin as another plugin.

`assets/` contains all non-code assets, such as images, spritesheets, audio, and fonts.
Additionally, textures are loaded from `RON` configuration files using [bevy_titan](https://lib.rs/crates/bevy_titan).

Characters are specified in the path `assets/characters/<name>/`.
There will be a `<state>.png` file as their spritesheets, along with `<state>.titan` for configuration.
A `<name>.prototype.ron` file specifies the actual character information.

This allows not only for easier development iteration, but also simple modding.

### Assets
Characters are designed in [Blender](https://blender.org), rigged and animated, run through compositor nodes to turn it into pixel art, then exported out into spritesheets.
More information on this process will be put here once I figure it out, along with the actual source files.
