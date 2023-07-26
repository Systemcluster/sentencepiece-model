use std::path::PathBuf;

use sentencepiece_model::{ModelType, SentencePieceModel};

#[test]
pub fn test_load_model() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/t5-spiece.model");
    let data = std::fs::read(path).unwrap();
    let model = SentencePieceModel::from_slice(data).unwrap();
    assert_eq!(model.pieces.len(), 32000);
    assert_eq!(model.trainer().unwrap().unk_id(), 2);
    assert_eq!(model.trainer().unwrap().model_type(), ModelType::Unigram);
}
