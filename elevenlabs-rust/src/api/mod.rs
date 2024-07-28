// The `mod.rs` file declares the submodules in the `api` module.
// This allows the rest of your crate to use `api::submodule` to access the contents of each submodule.

// Declare each submodule here. Each submodule corresponds to a file with the same name.
pub mod sts;
pub mod tts;
pub mod user;
pub mod voice_generation;
pub mod voices;
pub mod utils;

// Optionally, you can re-export important structs or functions to be accessed directly through `api::`
// For example, if you want to re-export the `TextToSpeechClient` struct for direct access:
// pub use tts::TextToSpeechClient;

// Continue with re-exports for other key structs or enums as needed
// ...

// Remember to adjust the visibility (`pub`) of the items inside your submodules according to how you
// want them to be accessed from outside the `api` module.
