#[cfg(feature = "coord_2d")]
pub use coord_2d;

#[cfg(feature = "grid_2d")]
pub use grid_2d;

#[cfg(feature = "line_2d")]
pub use line_2d;

#[cfg(feature = "direction")]
pub use direction;

#[cfg(feature = "rgb_int")]
pub use rgb_int;

#[cfg(feature = "entity_table")]
pub use entity_table;

#[cfg(feature = "entity_table_realtime")]
pub use entity_table_realtime;

#[cfg(feature = "spatial_table")]
pub use spatial_table;

#[cfg(feature = "shadowcast")]
pub use shadowcast;

#[cfg(feature = "visible_area_detection")]
pub use visible_area_detection;

#[cfg(feature = "grid_search_cardinal")]
pub use grid_search_cardinal;

#[cfg(feature = "chargrid")]
pub use chargrid;

#[cfg(feature = "chargrid_wgpu")]
pub use chargrid_wgpu;

#[cfg(feature = "chargrid_ggez")]
pub use chargrid_ggez;

#[cfg(feature = "chargrid_ansi_terminal")]
pub use chargrid_ansi_terminal;

#[cfg(feature = "chargrid_web")]
pub use chargrid_web;

#[cfg(feature = "general_storage_static")]
pub mod storage {
    #[cfg(feature = "storage_backend_web")]
    pub use general_storage_static::backend::LocalStorage;
    #[cfg(feature = "storage_backend_file")]
    pub use general_storage_static::backend::{FileStorage, IfDirectoryMissing};
    pub use general_storage_static::{
        format, LoadError, LoadRawError, RemoveError, StaticStorage as Storage, StoreError,
        StoreRawError, StringError,
    };
}

#[cfg(feature = "general_audio_static")]
pub mod audio {
    #[cfg(feature = "audio_backend_web")]
    pub use general_audio_static::backend::WebAudioPlayer;
    #[cfg(feature = "audio_backend_native")]
    pub use general_audio_static::backend::{Error as NativeAudioError, NativeAudioPlayer};
    pub use general_audio_static::{
        StaticAudioPlayer as AudioPlayer, StaticHandle as AudioHandle, StaticSound as Audio,
    };
}
