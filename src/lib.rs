//! SentencePiece model parser generated from the SentencePiece protobuf definition.
//!
//! See [`SentencePieceModel`] for the entry point for parsing and accessing sentencepiece models.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

use core::ops::Deref;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/sentencepiece.rs"));
}

pub use proto::model_proto::sentence_piece::Type;
pub use proto::model_proto::SentencePiece;
pub use proto::trainer_spec::ModelType;
pub use proto::{ModelProto, NormalizerSpec, TrainerSpec};

use prost::bytes::Buf;
use prost::Message;

/// SentencePiece model.
/// Provides access to the underlying `sentencepiece` model.
#[derive(Clone, PartialEq, Debug)]
pub struct SentencePieceModel {
    model: ModelProto,
}
impl SentencePieceModel {
    pub fn from_slice(bytes: impl AsRef<[u8]>) -> Result<Self, prost::DecodeError> {
        let model = ModelProto::decode(bytes.as_ref())?;
        Ok(Self { model })
    }

    pub fn from_reader<R: Buf>(reader: &mut R) -> Result<Self, prost::DecodeError> {
        let model = ModelProto::decode(reader)?;
        Ok(Self { model })
    }

    #[cfg(feature = "std")]
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
        let bytes = std::fs::read(path)?;
        Self::from_slice(bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn model(&self) -> &ModelProto {
        &self.model
    }

    pub fn trainer(&self) -> Option<&TrainerSpec> {
        self.model.trainer_spec.as_ref()
    }

    pub fn normalizer(&self) -> Option<&NormalizerSpec> {
        self.model.normalizer_spec.as_ref()
    }

    pub fn denormalizer(&self) -> Option<&NormalizerSpec> {
        self.model.denormalizer_spec.as_ref()
    }
}
impl Deref for SentencePieceModel {
    type Target = ModelProto;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}
