# Simplicity


An aptly named "simple open" source Discord bot.

Command files are found in `src/handler/commands`, with the exception of `mod.rs`, `prelude.rs`, and `utils.rs`, which are project files.  `sample.rs` is a file that describes what a command file looks like.  Since you can't dynamically compile files at runtime, all commands have to be added beforehand to `src/handler/commands/mod.rs`.

**Note: To use this bot, you must have a file called 'SECRET' in the same directory as the executable that contains your token.**