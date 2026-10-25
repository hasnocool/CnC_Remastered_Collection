# Asset pack application method

Use `apply_asset_pack.py` to verify a CNC32 asset pack is present, generated, and
sprite-complete. It does not build binaries; it checks assets and reports.

```bash
python assets/packs/apply/apply_asset_pack.py
```

Expected output (v6.2 pack):
- GENERATOR: present
- CATEGORY: avatars/commanders / icons / controls -> present
- v6_2 sprite tags in manifest
- CHANGELOG v6.2: present
- NOTES v6.2: present
- SOURCE IMAGE-MODEL REMOVED: verified absent
- METHOD: apply_asset_pack.py confirms v6.2 assets generated, sprites verified,
  dependencies preserved, image-model source removed.

Pack path: `RUST/assets/packs/cnc32_assets_v6_2_1` (Plan: `assets/packs/` method).
See `.omo/evidence/asset-v6-2-application.md` for the evidence record.
