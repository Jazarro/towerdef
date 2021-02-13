# Contributing
[![GitHub issues by-label](https://img.shields.io/github/issues/amethyst/dwarf_seeks_fortune/good%20first%20issue?color=7057FF&label=good%20first%20issues)](https://github.com/amethyst/dwarf_seeks_fortune/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
[![GitHub issues by-label](https://img.shields.io/github/issues/amethyst/dwarf_seeks_fortune/help%20wanted?color=008672&label=help%20wanted)](https://github.com/amethyst/dwarf_seeks_fortune/issues?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22)

## New Levels
One simple way anyone can contribute to the game is making new levels. Each level is a separate, independent unit, so your level is guaranteed not to conflict with anyone else's contribution! There is literally no way to go wrong here. Check out the [level design guide](/docs/LevelDesign.md) for more information.

## Code
### New tiles
You can always add new puzzle elements. There are some ideas in in [the brainstorm doc](docs/brainstorm.md) but you can also invent your own. The cool thing about adding a new type of tile is that it doesn't affect any levels that don't include that tile. This allows adding pretty crazy stuff without breaking any puzzles or affecting the overall balancing of the game.

### Code Style
Please try to resolve all `clippy` and compiler warnings and run `cargo fmt` before sending in your pull request, the CI checks will fail otherwise.

## Art
### Graphics
There is already an artist working on pixel art, so there is not much to do on this front. In the interest of maintaining a uniform, cohesive art style, I'd prefer it if all the foreground-stuff (characters, items, blocks) is made by the same artist.

If you still want to contribute, there is a lot of polish needed in particle and dust-effects, and backgrounds and background clutter. For the latter two, try to match the general aesthetic of the game. This is obviously not possible until some of the final sprites are merged in. (Hopefully soon)

### Music
If you're a composer or if you have any ideas about the music, please raise them in an issue. 

### Sound effects
If you want to design sound effects for the game, raise an issue and lay out your ideas for discussion. For now, the only concrete thoughts I have on the final sounds are that they should be fairly comical and cartoony.