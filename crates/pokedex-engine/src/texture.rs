use enum_map::{enum_map, EnumMap};
use std::collections::HashMap;

use pokedex::{item::ItemId, pokemon::PokemonId};

use engine::{
    error::ImageError,
    graphics::{Texture, TextureManager},
    Context,
};

pub type NpcGroupTextures = TextureManager<crate::NpcGroupId>;
pub type ItemTextures = TextureManager<ItemId>;

pub use firecore_pokedex_engine_builder::pokemon::PokemonTexture;

pub struct PokemonTextures(HashMap<PokemonId, EnumMap<PokemonTexture, Texture>>);

impl PokemonTextures {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn insert(
        &mut self,
        ctx: &mut Context,
        id: PokemonId,
        textures: EnumMap<PokemonTexture, Vec<u8>>,
    ) -> Result<(), ImageError> {
        self.0.insert(
            id,
            enum_map! {
                PokemonTexture::Front => Texture::new(ctx, &textures[PokemonTexture::Front])?,
                PokemonTexture::Back => Texture::new(ctx, &textures[PokemonTexture::Back])?,
                PokemonTexture::Icon => Texture::new(ctx, &textures[PokemonTexture::Icon])?,
            },
        );
        Ok(())
    }

    pub fn get(&self, id: &PokemonId, side: PokemonTexture) -> &Texture {
        self.0
            .get(id)
            .map(|m| &m[side])
            .unwrap_or_else(|| panic!("Could not get texture for pokemon with ID {}", id))
    }
}
