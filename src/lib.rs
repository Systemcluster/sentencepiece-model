//! SentencePiece model parser generated from the SentencePiece protobuf definition.
//!
//! See [`SentencePieceModel`] for the entry point for parsing and accessing sentencepiece models.
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use sentencepiece_model::SentencePieceModel;
//!
//! let model = SentencePieceModel::from_file("tests/t5-spiece.model")?;
//! assert_eq!(model.pieces.len(), 32000);
//! assert_eq!(model.trainer().unwrap().unk_id(), 2);
//! # Ok(())
//! # }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

use core::ops::Deref;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/sentencepiece.rs"));
}

pub use proto::model_proto::sentence_piece::Type;
pub use proto::model_proto::SentencePiece;
pub use proto::self_test_data::Sample;
pub use proto::trainer_spec::ModelType;
pub use proto::{ModelProto, NormalizerSpec, SelfTestData, TrainerSpec};

use prost::bytes::Buf;
use prost::Message;

/// SentencePiece model.
/// Provides access to the underlying `sentencepiece` model.
#[derive(Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct SentencePieceModel {
    model: ModelProto,
}
impl SentencePieceModel {
    /// Parses a `SentencePieceModel` from a byte slice.
    #[inline]
    pub fn from_slice(bytes: impl AsRef<[u8]>) -> Result<Self, prost::DecodeError> {
        let model = ModelProto::decode(bytes.as_ref())?;
        Ok(Self { model })
    }

    /// Parses a `SentencePieceModel` from a reader.
    #[inline]
    pub fn from_reader<R: Buf>(reader: &mut R) -> Result<Self, prost::DecodeError> {
        let model = ModelProto::decode(reader)?;
        Ok(Self { model })
    }

    /// Parses a `SentencePieceModel` from a file.
    #[cfg(feature = "std")]
    #[inline]
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
        let bytes = std::fs::read(path)?;
        Self::from_slice(bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    /// Returns the underlying `ModelProto`.
    #[inline]
    pub fn model(&self) -> &ModelProto {
        &self.model
    }

    /// Returns the `SentencePiece` list of the model.
    #[inline]
    pub fn pieces(&self) -> &[SentencePiece] {
        &self.model.pieces
    }

    /// Returns the `TrainerSpec` of the model if it exists.
    #[inline]
    pub fn trainer(&self) -> Option<&TrainerSpec> {
        self.model.trainer_spec.as_ref()
    }

    /// Returns the `NormalizerSpec` of the model if it exists.
    #[inline]
    pub fn normalizer(&self) -> Option<&NormalizerSpec> {
        self.model.normalizer_spec.as_ref()
    }

    /// Returns the `DenormalizerSpec` of the model if it exists.
    #[inline]
    pub fn denormalizer(&self) -> Option<&NormalizerSpec> {
        self.model.denormalizer_spec.as_ref()
    }

    /// Returns the `SelfTestData` of the model if it exists.
    #[inline]
    pub fn self_test_data(&self) -> Option<&SelfTestData> {
        self.model.self_test_data.as_ref()
    }
}
impl Deref for SentencePieceModel {
    type Target = ModelProto;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.model
    }
}
