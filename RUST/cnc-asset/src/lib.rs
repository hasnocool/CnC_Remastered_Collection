#![allow(clippy::manual_is_multiple_of)]
//! C&C Asset - asset loading and parsing

pub mod adpcm;
pub mod anim;
pub mod animation;
pub mod archive;
pub mod atlas;
pub mod audio;
pub mod base64;
pub mod classic_pack;
pub mod convert;
pub mod font;
pub mod format80;
pub mod megafile;
pub mod original_assets;
pub mod original_templates;
pub mod palette;
pub mod ra_pack;
pub mod registry;
pub mod shape;
pub mod string_table;
pub mod tile_join;
pub mod tileset_pack;
pub mod unit_atlas;
pub mod video;
pub mod vxl;

pub use adpcm::{AdpcmError, Decoder as AdpcmDecoder};

pub use anim::{
    AnimationEvent, AnimationInstance, AnimationSpec, FrameEvent, LoopMode, PaletteId,
    MIN_FRAME_DURATION,
};
pub use animation::{frame_to_rgba_remapped, AnimFrame, Animation, AnimationPlayer, PlaybackMode};
pub use archive::{MixArchive, MixEntry};
pub use atlas::{AtlasStyle, SpriteAtlas, SpriteFrame};
pub use audio::{AudFile, VocFile};
pub use base64::{decode as base64_decode, encode as base64_encode};
pub use convert::{BatchOptions, BatchReport, SpriteSheet};
pub use font::{FontFile, FontGlyph, NO_DATA_OFFSET};
pub use megafile::{Megafile, SubFileData};
pub use palette::PalFile;
pub use registry::{AssetHandle, AssetRegistry, PreloadResult, PreloadTask, PreloadedAssets};
pub use shape::{ShpFile, ShpFrame};
pub use string_table::CsfFile;
pub use tileset_pack::TilesetPack;
pub use video::{
    decode_frame, decode_frames, demux, demux_strict, Vqa, VqaChunk, VqaError, VqaHeader,
};
pub use vxl::{VxlError, VxlFile, VxlHeader, VxlLayer};
