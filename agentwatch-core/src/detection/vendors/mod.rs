// ── Vendor-specific detector modules ─────────────────────────────────────────
// Each module exports fingerprints for one agent ecosystem.
// Phase 6 of the pair-review plan: stop growing one giant fingerprint table.

pub mod anthropic;
pub mod openai;
pub mod google;
pub mod microsoft;
pub mod cursor;
pub mod codeium;
pub mod others;
