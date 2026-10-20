# cnc-audio test fixtures

- `synthetic.csf` — hand-authored CSF string table used by `tests/csf.rs`.
  It is **synthetic**: the labels and values were written for this
  repository specifically (`TXT_EVA_*`, `TXT_UNIT_MAMMOTH`, ...) and no
  retail Command & Conquer data is included. Layout: CSF version 3, language
  0 (US), 14 label blocks (one duplicate name), 15 string pairs (one
  `WRTS` extra value block), one non-ASCII value (`Gebäude fertig.`) and
  several `%LABEL%` references, including a cycle and an unknown label.
