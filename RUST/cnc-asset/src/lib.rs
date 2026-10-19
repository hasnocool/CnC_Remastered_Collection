//! C&C Asset - asset loading and parsing

pub mod anim;
pub mod animation;
pub mod archive;
pub mod atlas;
pub mod audio;
pub mod convert;
pub mod font;
pub mod palette;
pub mod registry;
pub mod shape;
pub mod string_table;
pub mod tileset_pack;

pub use anim::{
    AnimationEvent, AnimationInstance, AnimationSpec, FrameEvent, LoopMode, PaletteId,
    MIN_FRAME_DURATION,
};
pub use animation::{frame_to_rgba_remapped, AnimFrame, Animation, AnimationPlayer, PlaybackMode};
pub use archive::{MixArchive, MixEntry};
pub use atlas::{AtlasStyle, SpriteAtlas, SpriteFrame};
pub use audio::{AudFile, VocFile};
pub use convert::{BatchOptions, BatchReport, SpriteSheet};
pub use font::{FontFile, FontGlyph, NO_DATA_OFFSET};
pub use palette::PalFile;
pub use registry::{AssetHandle, AssetRegistry, PreloadResult, PreloadTask, PreloadedAssets};
pub use shape::{ShpFile, ShpFrame};
pub use string_table::CsfFile;
pub use tileset_pack::TilesetPack;
