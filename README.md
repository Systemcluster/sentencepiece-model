# sentencepiece-model

[![Crates.io](https://img.shields.io/crates/v/sentencepiece-model)](https://crates.io/crates/sentencepiece-model)
[![Docs.rs](https://img.shields.io/docsrs/sentencepiece-model)](https://docs.rs/sentencepiece-model)

**SentencePiece model parser** generated from the SentencePiece protobuf definition.

```rust
use sentencepiece_model::SentencePieceModel;

let model = SentencePieceModel::from_file("tests/t5-spiece.model")?;
assert_eq!(model.pieces.len(), 32000);
assert_eq!(model.trainer()?.unk_id(), 2);
```
