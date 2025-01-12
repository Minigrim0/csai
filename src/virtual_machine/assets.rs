use bevy::{asset::{io::Reader, AssetLoader, LoadContext}, prelude::*};
use thiserror::Error;

use super::{Instruction, parser::parse};


#[derive(Asset, TypePath, Debug)]
pub struct Program {
    instructions: Vec<Instruction>
}

#[non_exhaustive]
#[derive(Debug, Error)]
enum ProgramLoaderError {
    #[error("Could not load asset: {0}")]
    FileNotFound(#[from] std::io::Error),
    #[error("Invalid instruction")]
    InvalidInstruction,
}

#[derive(Default)]
struct ProgramLoader;

impl AssetLoader for ProgramLoader {
    type Asset = Program;
    type Settings = ();
    type Error = ProgramLoaderError;

    async fn load(
        &self, reader: &mut dyn Reader,
        _setting: &(),
        _load_context: &mut LoadContext<'_>
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text: String = bytes.iter().map(|b| char::from(*b)).collect();
        let instructions = parse(text);
        Ok(Program { instructions })
    }
}