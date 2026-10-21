# Build Status

> **Ledger caveat (added 2026-10-20, documentation reconciliation):** the raw `cargo test` transcripts below aggregate to a figure that is **not** reproducible with a single workspace run. `cargo test --workspace --no-fail-fast` at commit `5d1fd8b` measures **1318 passed / 4 failed** (pre-existing baseline debt: 3 in `cnc-map-editor::app::tests` — `layout_controls_stay_inside_the_window`, `trigger_filter_matches_group_and_name`, `trigger_inspector_controls_and_frame_render` — and 1 in `cnc-tiberian-dawn::scenario_loader::tests::records_become_executable_triggers`), with `cargo fmt --check` exit 0 and `cargo clippy --workspace --all-targets` at 0 errors. The workspace has **17 members** since `cnc-events`/`cnc-logs` landed; `cnc-console` still has no workspace entry. Cite the measured workspace run, not the per-crate transcript sum. *re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §A, §B, §C, §M.*

_Last verified: 2026-10-19 22:17:04 UTC (cargo cargo 1.98.1 (797e8a9bc 2026-08-05) / rustc rustc 1.98.1 (48a229cea 2026-09-01))._

## Verification


running 23 tests
test agent::tests::validation_reports_script_errors_and_unknown_classes ... ok
test intent::tests::prompt_contains_vision_and_observation ... ok
test intent::tests::response_is_validated_and_converted ... ok
test intent::tests::vision_ascii_roundtrip ... ok
test intent::tests::vision_image_is_optional_and_prompted ... ok
test intent::tests::slots_are_extracted_in_order_and_deduplicated ... ok
test ladder::tests::fingerprint_changes_with_the_script ... ok
test ladder::tests::registration_pins_the_script_and_doctrine ... ok
test runtime::tests::division_by_zero_yields_zero ... ok
test agent::tests::every_starter_compiles ... ok
test runtime::tests::every_handler_runs_on_its_schedule ... ok
test runtime::tests::nearest_queries_use_distance_and_threat ... ok
test runtime::tests::intents_use_cache_or_fallback ... ok
test script::tests::class_references_are_collected ... ok
test runtime::tests::variables_persist_across_ticks ... ok
test runtime::tests::while_loop_is_bounded ... ok
test script::tests::rejects_effectless_expressions ... ok
test script::tests::comments_and_escapes ... ok
test script::tests::reports_line_and_column ... ok
test script::tests::rejects_unknown_functions_and_bad_arity ... ok
test script::tests::parses_handlers_and_precedence ... ok
test runtime::tests::deterministic_random_is_seeded ... ok
test script::tests::stray_semicolons_are_ignored ... ok

test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 56 tests
test anim::tests::once_finishes_exactly_at_total_elapsed ... ok
test anim::tests::single_frame_loop_never_finishes_and_wraps ... ok
test anim::tests::spec_clamps_degenerate_durations ... ok
test anim::tests::extreme_dt_does_not_explode ... ok
test animation::dbg::probe_parse ... ok
test animation::tests::durations_never_cancel_progress ... ok
test animation::tests::loop_mode_cycles_and_returns_to_start ... ok
test animation::tests::from_durations_validates_lengths ... ok
test animation::tests::mid_frame_advance_keeps_tick_counter ... ok
test animation::tests::one_shot_stops_on_last_frame_and_finishes ... ok
test animation::tests::ping_pong_reverses_at_edges ... ok
test animation::tests::palette_swap_decodes_through_player_remap ... ok
test animation::tests::single_frame_ping_pong_never_moves ... ok
test animation::tests::trigger_fires_each_loop_of_long_advance ... ok
test animation::tests::trigger_fires_once_per_step_entry ... ok
test archive::tests::rejects_bad_signature ... ok
test archive::tests::extracts_uncompressed_entries ... ok
test audio::tests::test_voc_basic ... ok
test convert::audio_tests::audio_format_parses_names_and_extensions ... ok
test convert::audio_tests::audio_loop_rejects_inverted_ranges ... ok
test convert::audio_tests::voc_loop_points_are_preserved_by_default ... ok
test convert::audio_tests::wav_without_loop_has_no_smpl_chunk ... ok
test archive::tests::extracts_compressed_entries ... ok
test convert::audio_tests::wav_smpl_chunk_encodes_loop_and_updates_riff_size ... ok
test convert::tests::audio_conversion_handles_voc_and_aud ... ok
test atlas::tests::generation_populates_all_frames ... ok
test convert::tests::batch_with_sprite_sheet_mode_and_bad_game ... ok
test convert::tests::frames_are_transparent_at_index_zero ... ok
test convert::tests::frames_preserve_palette_colours ... ok
test convert::tests::sprite_sheet_packs_frames_horizontally ... ok
test convert::tests::mix_extraction_writes_entries ... ok
test font::tests::encode_parse_roundtrip ... ok
test font::tests::too_small_header_fails ... ok
test font::tests::truncated_glyph_fails_loudly ... ok
test palette::tests::test_pal_roundtrip ... ok
test convert::tests::batch_records_failures_without_aborting ... ok
test font::tests::zero_max_height_fails ... ok
test registry::tests::register_replaces_bytes_and_preserves_index ... ok
test shape::tests::test_rle_decode ... ok
test convert::tests::writes_frame_pngs_and_sheet_manifest ... ok
test font::tests::blank_chars_resolve_no_glyph ... ok
test font::tests::rasterize_places_glyph_pixels ... ok
test video::tests::compressed_variants_report_the_missing_codec ... ok
test video::tests::container_walk_parses_header_and_chunks ... ok
test video::tests::frame_decode_is_column_major_through_the_codebook ... ok
test video::tests::malformed_containers_are_rejected_cleanly ... ok
test video::tests::codebook_length_is_validated ... ok
test convert::tests::batch_conversion_mirrors_tree_and_counts ... ok
test convert::tests::palette_exports_are_consistent ... ok
test convert::audio_tests::ogg_without_loop_points_has_no_loop_comments ... ok
test convert::audio_tests::ogg_stream_is_valid_and_carries_loop_comments ... ok
test convert::audio_tests::write_audio_converts_aud_to_ogg_with_override ... ok
test tileset_pack::tests::synthetic_pack_rejects_unknown_palette ... ok
test tileset_pack::tests::synthetic_pack_loads_and_addresses_tiles ... ok
test atlas::tests::save_and_load_roundtrip ... ok
test tileset_pack::tests::real_pack_composites_all_eight_categories ... ok

test result: ok. 56 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
test event_fires_exactly_once_per_arrival ... ok
test empty_spec_is_always_finished ... ok
test once_mode_finishes_and_stays_finished ... ok
test loop_mode_wraps_around ... ok
test palette_id_is_passthrough_independent_of_frames ... ok
test pingpong_reverses_at_both_ends ... ok
test reset_restores_initial_state_keeping_palette ... ok
test non_positive_dt_is_a_noop ... ok
test once_mode_events_fire_before_finish ... ok
test event_at_initial_position_fires_on_first_advance ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test qa_anim_over_registered_mix_ships_one_line ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test batch_returns_error_when_a_file_fails ... ok
test mix_list_and_extract ... ok
test shp_frames_and_sprite_sheet ... ok
test voc_and_aud_to_wav ... ok
test batch_converts_tree_and_reports_counts ... ok
test pal_export_formats ... ok
test aud_to_ogg_with_loop_points_and_batch_ogg ... ok
test generate_atlas_writes_manifest ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 10 tests
test first_registration_generation_is_zero ... ok
test equal_handles_compare_equal_for_same_slot ... ok
test legacy_register_get_api_still_works ... ok
test handle_missing_at_creation_is_stale ... ok
test handle_reports_name_and_derefs_to_loaded_bytes ... ok
test get_async_resolves_registered_bytes_without_executor ... ok
test registry_keeps_index_ordered_names ... ok
test stale_handle_detects_replace_but_keeps_snapshot_bytes ... ok
test replacing_same_name_bumps_generation ... ok
test preload_reports_missing_names_deterministically ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 35 tests
test mixer::tests::audio_error_is_send_sync_std_error ... ok
test mixer::tests::attenuate_endpoints_and_linear_middle ... ok
test mixer::tests::channel_volume_and_pitch_setters_clamp ... ok
test mixer::tests::doppler_factor_is_sane ... ok
test mixer::tests::default_volume_knobs_keep_headless_recordings_identical ... ok
test mixer::tests::crossfade_music_records_in_headless ... ok
test mixer::tests::channel_busy_equal_or_lower_priority_is_dropped ... ok
test mixer::tests::fade_in_envelope_starts_at_zero_and_ramps_to_one ... ok
test mixer::tests::effects_and_music_volumes_scale_headless_recordings ... ok
test mixer::tests::music_and_effects_volume_knobs_default_full_and_clamp ... ok
test mixer::tests::headless_play_records_request ... ok
test mixer::tests::pan_gains_keep_constant_power ... ok
test mixer::tests::pause_and_resume_track_channel_state ... ok
test mixer::tests::master_channel_voice_volume_product ... ok
test mixer::tests::headless_zero_crossfade_is_recorded_as_zero ... ok
test mixer::tests::shared_fade_ramp_sample_count_math ... ok
test mixer::tests::invalid_sfx_channel_is_rejected ... ok
test mixer::tests::runtime_fade_out_from_any_elapsed ... ok
test mixer::tests::static_fade_out_envelope_starts_at_one_and_ends_silent ... ok
test mixer::tests::volume_buses_compose_with_master_and_channel ... ok
test music::tests::default_player_is_neutral ... ok
test mixer::tests::stop_channel_cuts_current_voice ... ok
test mixer::tests::stop_by_handle_frees_channel ... ok
test sfx::tests::from_file_missing_reports_io_error ... ok
test sfx::tests::play_on_records_request_with_defaults ... ok
test music::tests::crossfade_time_is_recorded_and_sanitized ... ok
test sfx::tests::invalid_bytes_fail_with_decoder_error ... ok
test sfx::tests::play_positional_uses_attenuation ... ok
test music::tests::looping_metadata_roundtrips ... ok
test music::tests::pause_resume_and_stop_track_the_mixer ... ok
test sfx::tests::play_uses_documented_ambient_defaults ... ok
test sfx::tests::same_effect_can_play_on_two_channels ... ok
test music::tests::play_next_walks_queue_in_order ... ok
test music::tests::set_intensity_switches_layers ... ok
test mixer::tests::stop_all_clears_every_voice ... ok

test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
test tests::frame_delta_is_sixtieth_of_a_second ... ok
test tests::bench_scenario_fixture_parses ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 120 tests
test class_id::tests::test_class_id ... ok
test class_id::tests::test_class_id_unknown ... ok
test color::tests::test_rgb555 ... ok
test color::tests::test_rgb888_conversion ... ok
test color::tests::test_player_remap ... ok
test color::tests::test_argb8888 ... ok
test class_id::tests::test_registry ... ok
test entity::tests::object_dead_objects_take_no_damage_and_cannot_heal ... ok
test entity::tests::has_object_trait_delegates_health_ops ... ok
test entity::tests::object_heal_never_exceeds_strength ... ok
test entity::tests::object_new_starts_at_full_health ... ok
test entity::tests::object_serde_round_trip ... ok
test entity::tests::object_take_damage_clamps_at_zero_and_marks_destroyed ... ok
test entity::tests::team_add_remove_members_is_deduplicating ... ok
test entity::tests::taskforce_unit_count_sums_members ... ok
test entity::tests::team_balance_progress_clamps_at_hundred ... ok
test entity::tests::team_from_type_copies_objective_and_start_state ... ok
test entity::tests::team_is_complete_at_hundred_and_progress_is_clearable ... ok
test entity::tests::team_set_objective_changes_instance_copy_only ... ok
test entity::tests::team_serde_round_trip ... ok
test entity::tests::team_type_serde_round_trip ... ok
test entity::tests::trigger_disabled_never_matches_nor_fires ... ok
test entity::tests::trigger_matches_only_its_event ... ok
test entity::tests::trigger_repeatable_fires_every_time ... ok
test entity::tests::team_type_from_ini_parses_members_owner_and_mission ... ok
test entity::tests::trigger_serde_round_trip ... ok
test entity::tests::trigger_with_payload_event_matches_exact_payload_only ... ok
test entity::tests::trigger_from_ini_parses_record_events_and_actions ... ok
test entity::tests::debug_entity_foundation_dump ... ok
test fixed::tests::property_tests::div_matches_i64_formula_for_nonoverflowing_inputs ... ok
test fixed::tests::property_tests::saturating_div_by_zero_saturates ... ok
test fixed::tests::property_tests::mul_matches_i64_shift_for_nonoverflowing_inputs ... ok
test fixed::tests::property_tests::abs_is_nonnegative_and_idempotent ... ok
test fixed::tests::test_atan2 ... ok
test fixed::tests::property_tests::neg_is_involutive ... ok
test fixed::tests::test_basic_arithmetic ... ok
test fixed::tests::test_from_int ... ok
test fixed::tests::test_sin_cos ... ok
test fixed::tests::test_overflow_behavior ... ok
test fixed::tests::test_sqrt ... ok
test entity::tests::property_tests::object_health_stays_clamped ... ok
test fixed::tests::property_tests::from_int_round_trips_for_small_integers ... ok
test float_bias::tests::test_arithmetic ... ok
test float_bias::tests::test_bias_comparison ... ok
test float_bias::tests::test_sqrt ... ok
test float_bias::tests::test_trig ... ok
test ini::tests::case_insensitive_lookup_and_strings ... ok
test ini::tests::numeric_values_survive_boolean_like_literals ... ok
test ini::tests::parses_hex_and_negative_literals ... ok
test fixed::tests::property_tests::sqrt_squares_back_within_tolerance ... ok
test fixed::tests::property_tests::wrapping_add_sub_act_on_i32_bits ... ok
test ini::tests::rejects_missing_parent_and_cycles ... ok
test fixed::tests::property_tests::ordering_matches_i32_ordering ... ok
test ini::tests::resolves_multi_level_inheritance ... ok
test ini::tests::round_trips_sections_and_globals ... ok
test math::tests::test_fixed_sqrt_large_value ... ok
test math::tests::test_mat2x2_rotation ... ok
test math::tests::test_rect ... ok
test math::tests::test_vec2_fixed ... ok
test math::tests::test_vec2_long_length ... ok
test math::tests::test_vec2i ... ok
test object_pool::tests::test_deallocate_drop_runs_destructor_and_reuses_ptr ... ok
test object_pool::tests::test_pool_allocates_multiple_blocks_and_values_survive ... ok
test object_pool::tests::test_pool_error_display ... ok
test object_pool::tests::test_pool_basic ... ok
test object_pool::tests::test_pool_handle ... ok
test object_pool::tests::test_pool_handle_drop_runs_destructor_and_releases_slot ... ok
test object_pool::tests::test_pool_reuse ... ok
test fixed::tests::property_tests::wrapping_div_by_zero_is_zero ... ok
test object_pool::tests::test_thread_local_pool_mirrors_object_pool ... ok
test entity::tests::property_tests::team_membership_tracks_set_oracle ... ok
test float_bias::tests::property_tests::approx_eq_is_reflexive_and_symmetric_for_finite_values ... ok
test palette::tests::test_find_closest ... ok
test palette::tests::test_grayscale ... ok
test palette::tests::test_palette_creation ... ok
test palette::tests::test_pal_roundtrip ... ok
test pointers::tests::test_code_decode_pointers ... ok
test pointers::tests::test_encode_box_some_and_none ... ok
test pointers::tests::test_null_pointer ... ok
test pointers::tests::test_register_ignores_null_and_invalid_ids ... ok
test pointers::tests::test_same_object_same_id ... ok
test pointers::tests::test_try_decode_returns_none_for_null_and_unregistered ... ok
test pointers::tests::test_verify_complete_errors_on_mismatched_counts ... ok
test pointers::tests::test_hundred_distinct_objects_get_ids_in_encode_order ... ok
test rules::classification_tests::section_classification_prefers_types_with_prerequisites ... ok
test rules::classification_tests::parses_unlisted_weapon_and_warhead_sections ... ok
test rules::tests::defaults_match_rules_cpp ... ok
test rules::classification_tests::structures_are_types_not_weapons ... ok
test rules::tests::difficulty_sections_are_typed ... ok
test rules::tests::difficulty_accepts_red_alert_difficult_alias ... ok
test float_bias::tests::property_tests::approx_eq_matches_the_bias_band ... ok
test rules::tests::level_parser_is_case_insensitive ... ok
test rules::tests::parses_houses ... ok
test rules::tests::parses_inheritance_inside_rules ... ok
test rules::tests::general_section_overrides_defaults ... ok
test rules::tests::parses_type_sections_and_costs ... ok
test rules::tests::parses_weapon_and_warhead_tables ... ok
test serialization::tests::test_basic_serialization ... ok
test serialization::tests::test_option_serialization ... ok
test serialization::tests::test_vec_serialization ... ok
test smart_ptr::tests::test_ccptr_basic ... ok
test smart_ptr::tests::test_ccptr_clone_counts_and_shared_value ... ok
test smart_ptr::tests::test_ccptr_deleter_runs_independent_of_drop_order ... ok
test smart_ptr::tests::test_ccptr_drop ... ok
test smart_ptr::tests::test_ccweak_survives_strong_drop_then_frees_block ... ok
test smart_ptr::tests::test_ccweak_upgrade_after_partial_strong_drop ... ok
test smart_ptr::tests::test_ccweak_upgrade_while_strong_alive ... ok
test smart_ptr::tests::test_cuptr_new_drops_value_on_drop ... ok
test smart_ptr::tests::test_cuptr_with_deleter_runs_deleter_on_drop ... ok
test smart_ptr::tests::test_noinit ... ok
test smart_ptr::tests::test_noinit_double_init_err_carries_value ... ok
test smart_ptr::tests::test_noinit_get_mut ... ok
test smart_ptr::tests::test_ccweak_lifecycle_stress ... ok
test float_bias::tests::property_tests::approx_le_and_ge_extend_lt_gt_and_eq ... ok
test float_bias::tests::property_tests::approx_lt_and_approx_gt_are_mutually_exclusive ... ok
test float_bias::tests::property_tests::comparisons_agree_with_f32_ordering_beyond_twice_bias ... ok
test serialization::tests::property_tests::write_then_read_round_trips_mixed_values ... ok
test ini::tests::property_tests::comma_arrays_split_and_round_trip ... ok
test ini::tests::property_tests::sections_keys_and_values_survive_parse_format_parse ... ok
test entity::tests::property_tests::trigger_matches_exactly_when_events_equal ... ok

test result: ok. 120 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 31 tests
test lobby::tests::empty_lobby_stats ... ok
test lobby::tests::unknown_peer_returns_error ... ok
test lobby::tests::chat_cap_prevents_unbounded_growth ... ok
test lobby::tests::ready_toggle_idempotent ... ok
test lobby::tests::history_preserves_append_order ... ok
test menu::tests::options_modal_click_zones_adjust_and_activate ... ok
test menu::tests::options_modal_controls_list_scrolls_and_clamps ... ok
test menu::tests::options_modal_enter_saves_and_escape_cancels ... ok
test menu::tests::options_modal_focus_wraps_over_every_row ... ok
test menu::tests::options_modal_groups_controls_by_game ... ok
test menu::tests::options_modal_renderer_toggles_with_arrows_and_enter ... ok
test menu::tests::options_modal_seeds_from_the_persisted_values ... ok
test menu::tests::options_modal_volume_keys_step_and_clamp_at_both_ends ... ok
test ready::tests::all_ready_false_while_anyone_unreadies ... ok
test ready::tests::all_ready_requires_positive_expected_count ... ok
test ready::tests::all_ready_saturates_when_extra_players_ready_up ... ok
test ready::tests::clear_resets_roster_and_gate ... ok
test ready::tests::format_emits_wire_line ... ok
test ready::tests::format_round_trips_through_parse ... ok
test ready::tests::marking_same_name_twice_keeps_one_roster_row ... ok
test ready::tests::parse_reads_ready_flag ... ok
test ready::tests::mark_ready_toggles_and_counts ... ok
test ready::tests::parse_rejects_bad_flag_values ... ok
test ready::tests::parse_rejects_extra_tokens ... ok
test ready::tests::parse_rejects_truncated_lines ... ok
test ready::tests::parse_rejects_non_utf8 ... ok
test ready::tests::parse_rejects_wrong_prefix ... ok
test ready::tests::parse_tolerates_trailing_newline_like_chat ... ok
test ready::tests::roster_output_is_sorted_not_arrival_order ... ok
test ready::tests::unregister_drops_the_player ... ok
test menu::tests::options_modal_draws_the_whole_dialog_without_a_window ... ok

test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 125 tests
test lobby::tests::empty_lobby_stats ... ok
test lobby::tests::ready_toggle_idempotent ... ok
test lobby::tests::chat_cap_prevents_unbounded_growth ... ok
test lobby::tests::history_preserves_append_order ... ok
test menu::tests::options_modal_controls_list_scrolls_and_clamps ... ok
test menu::tests::options_modal_enter_saves_and_escape_cancels ... ok
test menu::tests::options_modal_click_zones_adjust_and_activate ... ok
test lobby::tests::unknown_peer_returns_error ... ok
test menu::tests::options_modal_focus_wraps_over_every_row ... ok
test menu::tests::options_modal_groups_controls_by_game ... ok
test menu::tests::options_modal_renderer_toggles_with_arrows_and_enter ... ok
test menu::tests::options_modal_seeds_from_the_persisted_values ... ok
test menu::tests::options_modal_volume_keys_step_and_clamp_at_both_ends ... ok
test options::tests::apply_to_mixer_pushes_every_volume_bus ... ok
test options::tests::controls_list_covers_the_hardcoded_hotkeys ... ok
test options::tests::defaults_pick_wgpu_full_volume_and_a_controls_list ... ok
test options::tests::path_precedence_cli_over_env_over_save_dir ... ok
test options::tests::missing_file_falls_back_to_defaults ... ok
test ready::tests::all_ready_false_while_anyone_unreadies ... ok
test ready::tests::all_ready_requires_positive_expected_count ... ok
test ready::tests::all_ready_saturates_when_extra_players_ready_up ... ok
test options::tests::env_override_selects_the_options_file ... ok
test options::tests::corrupt_file_falls_back_to_defaults ... ok
test ready::tests::format_emits_wire_line ... ok
test options::tests::loaded_volumes_clamp_into_range ... ok
test ready::tests::clear_resets_roster_and_gate ... ok
test ready::tests::mark_ready_toggles_and_counts ... ok
test ready::tests::format_round_trips_through_parse ... ok
test ready::tests::marking_same_name_twice_keeps_one_roster_row ... ok
test options::tests::renderer_resolution_matches_rendererkind_parse ... ok
test options::tests::unknown_keys_are_ignored ... ok
test ready::tests::parse_reads_ready_flag ... ok
test options::tests::save_and_load_round_trip_preserves_settings ... ok
test ready::tests::parse_rejects_bad_flag_values ... ok
test ready::tests::parse_rejects_extra_tokens ... ok
test ready::tests::parse_rejects_non_utf8 ... ok
test ready::tests::parse_rejects_truncated_lines ... ok
test ready::tests::parse_tolerates_trailing_newline_like_chat ... ok
test ready::tests::parse_rejects_wrong_prefix ... ok
test ready::tests::roster_output_is_sorted_not_arrival_order ... ok
test ready::tests::unregister_drops_the_player ... ok
test score::tests::extreme_values_saturate_without_panicking ... ok
test score::tests::defeat_gets_consolation_and_no_time_bonus ... ok
test score::tests::multiplier_scales_the_total ... ok
test score::tests::negative_harvest_clamps_to_zero ... ok
test score::tests::rows_are_in_draw_order ... ok
test score::tests::reveal_animates_in_stages ... ok
test score::tests::victory_bonuses_compose_deterministically ... ok
test tests::campaign_terrain_tile_shores_water_against_land ... ok
test tests::campaign_without_scenarios_reports_how_to_fix_it ... ok
test tests::game_kind_accepts_td_and_ra_aliases ... ok
test tests::health_bars_report_fraction_and_color ... ok
test tests::land_mask_at_treats_out_of_map_as_unmatched ... ok
test tests::main_menu_campaign_page_opens_in_game_scenario_selectors ... ok
test tests::lan_beacons_parse_and_deduplicate ... ok
test tests::main_menu_navigates_and_launches_local_game ... ok
test tests::main_menu_options_entry_opens_the_options_modal ... ok
test tests::menu_escape_backs_out_and_quits_from_main ... ok
test tests::menu_mouse_click_selects_rows ... ok
test menu::tests::options_modal_draws_the_whole_dialog_without_a_window ... ok
test tests::minimap_layout_scales_to_the_panel ... ok
test tests::net_lobby_roster_rows_show_ready_state ... ok
test tests::network_build_command_places_remote_structure ... ok
test tests::network_deploy_command_replaces_remote_mcv ... ok
test tests::main_menu_draws_every_page_without_a_window ... ok
test tests::deploy_selected_mcv_queues_locally ... ok
test tests::chat_lines_parse_and_log ... ok
test tests::enemy_target_picks_enemy_units_only ... ok
test tests::network_move_command_orders_remote_units ... ok
test tests::hud_world_maps_zoomed_clicks_to_the_same_screen_point ... ok
test tests::network_power_command_toggles_remote_structure ... ok
test tests::network_produce_command_queues_remote_unit ... ok
test tests::network_repair_command_heals_remote_structure ... ok
test tests::network_lobby_edits_and_starts_both_roles ... ok
test tests::network_sell_command_removes_remote_structure ... ok
test tests::next_incomplete_skips_and_wraps ... ok
test tests::online_page_offers_red_alert_only ... ok
test tests::options_modal_cancel_discards_edits_and_restores_the_mixer ... ok
test tests::options_modal_opens_seeded_from_the_persisted_options ... ok
test tests::options_modal_renderer_choice_is_persisted_on_save ... ok
test tests::network_lost_peer_rejoins_from_a_new_port ... ok
test tests::options_modal_writes_a_screenshot_for_visual_qa ... ignored, visual QA capture; writes RUST/target/scratch-launcher/options-modal.png
test tests::options_modal_volume_edits_apply_live_and_save_persists_them ... ok
test tests::control_groups_store_and_recall ... ok
test tests::production_percent_handles_progress ... ok
test tests::game_speed_adjusts_and_clamps ... ok
test tests::campaign_select_loads_and_advances_missions ... ok
test tests::network_peer_timeout_reports_loss_once ... ok
test tests::edge_pan_detects_edges_and_moves_camera ... ok
test tests::minimap_clicks_center_the_camera ... ok
test tests::network_replay_saves_command_log ... ok
test tests::ra_setup_preview_maps_are_playable ... ok
test tests::ra_terrain_tile_beach_column_is_water_edge_mask ... ok
test tests::ra_terrain_tile_shores_water_and_sand_shores_beach ... ok
test tests::camera_bookmarks_save_and_jump ... ok
test tests::network_join_claims_host_after_timeout ... ok
test tests::pause_menu_restarts_and_aborts ... ok
test tests::pause_menu_saves_and_loads_the_game ... ok
test tests::ra_finished_match_aborts_to_the_right_screen ... ok
test tests::save_slot_label_reports_age_and_empty ... ok
test tests::queued_game_commands_are_routed_into_the_game ... ok
test tests::network_peers_exchange_move_commands ... ok
test tests::scenario_count_ignores_non_ini_files ... ok
test tests::ra_host_start_gate_blocks_until_all_ready ... ok
test tests::ra_launcher_bootstraps_a_skirmish_without_a_window ... ok
test tests::ra_produce_unit_queues_locally ... ok
test tests::ra_untouched_host_lobby_starts_and_arms_expected ... ok
test tests::ready_receive_guard_marks_tracker_in_poll ... ok
test tests::ready_send_marks_local_player_like_chat ... ok
test tests::ra_setup_cycles_sizes_and_difficulty ... ok
test tests::scatter_orders_selected_units ... ok
test tests::tiberium_fx_family_buckets_by_amount ... ok
test tests::water_shore_tile_matches_pack_art_convention ... ok
test tests::selected_structure_sells_locally ... ok
test tests::replay_playback_applies_frames_in_order ... ok
test tests::td_host_start_gate_blocks_unready ... ok
test tests::td_mission_finish_returns_to_selector_and_advances ... ok
test tests::td_pause_menu_resumes_and_returns_to_selector ... ok
test tests::td_repair_selected_building_costs_and_notices ... ok
test tests::td_campaign_completion_shows_final_screen ... ok
test tests::td_sell_selected_building_refunds_and_notices ... ok
test tests::save_slots_cycle_and_use_distinct_paths ... ok
test tests::return_to_menu_flag_is_consumed_once ... ok
test tests::td_mission_select_lists_and_loads ... ok
test tests::zoom_defaults_clamp_and_pin_the_world_under_the_cursor ... ok

test result: ok. 124 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 1.34s


running 2 tests
test tests::lobby_messages_roundtrip ... ok
test tests::relayed_packets_route_moves_into_the_ra_game ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test list_campaign_reports_every_ra_mission ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 91 tests
test app::tests::delete_selected_objects_is_one_undo_step ... ok
test app::tests::agent_studio_creates_checks_commits_and_undoes ... ok
test app::tests::copy_paste_object_is_undoable_and_rejects_occupied_cells ... ok
test app::tests::duplicate_selection_copies_offset_and_undoes ... ok
test app::tests::agent_studio_editor_accepts_typing_and_navigation ... ok
test app::tests::dragging_cannot_stack_objects_on_an_occupied_cell ... ok
test app::tests::check_all_agents_reports_each_script ... ok
test app::tests::agent_studio_reports_compile_errors_and_dry_run ... ok
test app::tests::custom_dimensions_drive_new_and_generated_maps ... ok
test app::tests::generate_button_replaces_the_map_and_resets_history ... ok
test app::tests::fit_shows_the_whole_map_and_cell_conversion_roundtrips ... ok
test app::tests::layout_controls_stay_inside_the_window ... ok
test app::tests::marquee_selects_every_object_inside_the_rectangle ... ok
test app::tests::multi_object_copy_paste_preserves_offsets_and_is_one_undo_step ... ok
test app::tests::native_dialog_defaults_follow_the_path_field ... ok
test app::tests::native_dialog_success_cancel_and_fallback_paths ... ok
test app::tests::object_categories_filter_the_palette ... ok
test app::tests::object_owner_cycle_and_nudge_are_undoable ... ok
test app::tests::object_search_filters_the_palette_and_controls ... ok
test app::tests::painting_is_one_undo_step_and_restores ... ok
test app::tests::placed_object_owner_and_class_filter_cycles ... ok
test app::tests::replace_selected_class_changes_all_and_undoes ... ok
test app::tests::scenario_preview_reads_terrain ... ok
test app::tests::placed_search_filters_the_object_list ... ok
test app::tests::oracle_export_writes_request_prompt_and_vision_image ... ok
test app::tests::scenario_browser_lists_ini_files ... ok
test app::tests::select_all_and_clear_cover_the_map ... ok
test app::tests::selecting_a_trigger_loads_its_fields ... ok
test app::tests::save_and_open_roundtrip_through_the_path_field ... ok
test app::tests::select_tool_drags_objects_as_one_undo_step ... ok
test app::tests::map_info_modal_commits_and_undoes ... ok
test app::tests::text_input_respects_focus_and_seed_digits ... ok
test app::tests::trigger_copy_paste_duplicates_fields_and_is_undoable ... ok
test app::tests::trigger_condition_and_action_fields_are_editable_and_undoable ... ok
test app::tests::trigger_filter_matches_group_and_name ... ok
test app::tests::help_overlay_toggles_closes_and_renders ... ok
test app::tests::minimap_maps_clicks_to_cells_and_centers_the_view ... ok
test app::tests::renderer_draws_a_non_empty_frame ... ok
test generator::tests::noise_is_deterministic_and_bounded ... ok
test app::tests::inspector_tabs_show_object_and_trigger_controls ... ok
test app::tests::trigger_templates_follow_the_active_plugin ... ok
test app::tests::trigger_tool_placement_selects_the_new_trigger_for_editing ... ok
test history::tests::agent_commands_undo_and_redo_exactly ... ok
test history::tests::grouped_stroke_is_one_undo_step ... ok
test history::tests::labels_report_undo_and_redo_stacks ... ok
test history::tests::object_owner_change_is_undoable ... ok
test history::tests::paint_undo_redo_restores_exact_values ... ok
test history::tests::place_and_remove_restore_object_ids ... ok
test history::tests::redo_stack_clears_on_new_edit_and_limit_trims ... ok
test history::tests::trigger_add_and_remove_roundtrip ... ok
test integration_tests::authoring_session_roundtrips_through_ini ... ok
test map::tests::agents_roundtrip_with_scripts_containing_ini_metacharacters ... ok
test map::tests::extension_sections_roundtrip_terrain_overlay_and_elevation ... ok
test map::tests::ini_roundtrip_preserves_cells_objects_and_triggers ... ok
test map::tests::inserts_objects_with_unique_ids_and_finds_them ... ok
test generator::tests::generated_map_has_water_land_and_resources ... ok
test map::tests::scenarios_without_optional_sections_load_flat ... ok
test map::tests::validation_flags_missing_starts_and_out_of_bounds_objects ... ok
test map::tests::validation_reports_agent_problems ... ok
test map::tests::world_and_cell_coordinates_roundtrip_through_a_cell_centre ... ok
test plugin::tests::object_validation_checks_bounds_and_house_owner ... ok
test map::tests::ra_campaign_metadata_survives_editor_roundtrip ... ok
test app::tests::history_tab_exposes_undo_redo_and_labels ... ok
test plugin::tests::plugins_expose_side_locked_objects ... ok
test plugin::tests::plugins_expose_distinct_catalogs ... ok
test plugin::tests::theaters_and_templates_are_derived ... ok
test presenter::tests::softbuffer_pixels_drop_the_alpha_byte ... ok
test tiles::tests::elevation_draws_cliff_faces_ramps_and_plateaus ... ok
test tiles::tests::resource_underlays_and_sprites_layer_over_terrain ... ok
test tiles::tests::road_and_wall_overlay_auto_connect ... ok
test tiles::tests::shore_bitmask_picks_shore_family ... ok
test tiles::tests::transition_family_pick_for_a_two_terrain_adjacency ... ok
test tiles::tests::transition_tie_breaks_on_lowest_row ... ok
test tiles::tests::variant_hash_is_deterministic ... ok
test tiles::tests::unpainted_elevation_renders_plain_and_water_adjacent_work ... ok
test tools::tests::drag_paints_a_line ... ok
test tools::tests::object_tool_rejects_out_of_bounds_and_occupied_cells ... ok
test tools::tests::terrain_click_builds_a_command_without_mutating ... ok
test tools::tests::trigger_tool_is_bounds_checked ... ok
test ui::tests::clicks_are_undoable_and_selection_tracks_objects ... ok
test ui::tests::delete_at_is_undoable ... ok
test ui::tests::render_draws_terrain_objects_grid_and_status ... ok
test generator::tests::starts_are_on_land_distinct_and_spawn_mcvs ... ok
test generator::tests::elevation_field_is_persisted_and_deterministic ... ok
test app::tests::agent_studio_renders_overlay_and_still_saves ... ok
test generator::tests::same_seed_produces_identical_maps ... ok
test generator::tests::different_seeds_produce_different_terrain ... ok
test generator::tests::theater_controls_snow_and_desert_classification ... ok
test generator::tests::generated_map_roundtrips_through_ini ... ok
test app::tests::trigger_inspector_controls_and_frame_render ... ok
test app::tests::validation_tab_reports_map_issues ... ok

test result: ok. 91 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
tileset pack '/home/hyperion/Projects/Games/CnC_Remastered_Collection/RUST/cnc-map-editor/../assets/packs/rts_16x16_tileset' (palette 'ra_inspired') loaded: 8 categories, 1024x512 atlas
generated: 48x48 Temperate - 2 objects, 0 triggers, 0 agents, 2 starts (seed 42, players 2)
terrain histogram: [("Grass", 34), ("Water", 159), ("Sand", 164), ("Dirt", 928), ("Rock", 567), ("Rough", 452)]
overlay cells: 796
player starts: [Vec2i { x: 12, y: 12 }, Vec2i { x: 43, y: 43 }]
saved: /tmp/cnc-editor-headless-3003902/map.ini (18898 bytes)
screenshot: /tmp/cnc-editor-headless-3003902/editor.png
headless self-test OK
test headless_binary_generates_map_ini_and_screenshot ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.97s


running 1 test
test validate_accepts_a_valid_map_and_reports_issues ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 14 tests
test midi::tests::errors_on_garbage ... ok
test midi::tests::note_on_velocity_zero_is_note_off ... ok
test midi::tests::parses_fixture ... ok
test midi::tests::running_status_reuses_note_on ... ok
test midi::tests::vlq_decodes_multi_byte ... ok
test synth::tests::midi_freq_matches_equal_temperament ... ok
test synth::tests::tick_to_secs_honors_mid_file_tempo_change ... ok
test synth::tests::tick_to_secs_uses_tempo ... ok
test synth::tests::overlapping_notes_mix_without_clipping ... ok
test synth::tests::notes_pair_on_off_and_render_is_deterministic ... ok
test encode::tests::empty_input_still_produces_a_stream ... ok
test encode::tests::encodes_ogg_with_header ... ok
test encode::tests::encoding_is_deterministic_in_size ... ok
test encode::tests::write_ogg_round_trips_to_disk ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 58 tests
test bencode::tests::decodes_canonical_examples ... ok
test bencode::tests::deep_input_is_rejected ... ok
test bencode::tests::rejects_malformed_input ... ok
test bencode::tests::roundtrips_nested_values ... ok
test bencode::tests::torrent_parses_and_hashes_info_dict ... ok
test bencode::tests::magnet_roundtrips_hash_and_web_seed ... ok
test discovery::tests::empty_seed_is_rejected ... ok
test discovery::tests::bootstrap_dedupes_sources ... ok
test matchmaking::tests::expected_score_is_symmetric ... ok
test matchmaking::tests::ai_and_human_queues_never_pair ... ok
test matchmaking::tests::distant_ratings_wait_for_a_wider_window ... ok
test discovery::tests::literal_addresses_resolve_unchanged ... ok
test matchmaking::tests::human_registration_rejects_agent_metadata ... ok
test matchmaking::tests::ai_only_matches_are_rated_on_the_ai_ladder ... ok
test matchmaking::tests::rating_starts_in_gold_and_provisional ... ok
test matchmaking::tests::queue_and_status_reflect_state ... ok
test matchmaking::tests::only_a_participant_can_report ... ok
test model::tests::game_kind_and_state_roundtrip ... ok
test model::tests::filter_matches_case_insensitively ... ok
test matchmaking::tests::tick_pairs_similar_players_and_publishes_a_lobby ... ok
test matchmaking::tests::report_moves_ratings_and_updates_records ... ok
test model::tests::order_key_breaks_ties_deterministically ... ok
test model::tests::validation_normalises_tags_and_defaults_ttl ... ok
test model::tests::validation_rejects_bad_capacity ... ok
test model::tests::validation_rejects_unspecified_address ... ok
test peer::tests::advertised_addresses_skip_self ... ok
test peer::tests::failures_escalate_and_observe_revives ... ok
test peer::tests::pinned_peers_survive_purge ... ok
test peer::tests::table_is_bounded ... ok
test peer::tests::gossip_targets_exclude_dead_peers ... ok
test matchmaking::tests::persistence_roundtrips_ratings ... ok
test protocol::tests::foreign_datagram_is_rejected ... ok
test protocol::tests::empty_game_batch_roundtrips ... ok
test discovery::tests::localhost_without_port_gets_default ... ok
test protocol::tests::every_message_roundtrips ... ok
test protocol::tests::truncated_datagram_is_rejected ... ok
test protocol::tests::unknown_tag_is_rejected ... ok
test protocol::tests::wrong_version_is_rejected ... ok
test registry::tests::delta_sync_returns_newer_records_only ... ok
test registry::tests::directory_announce_heartbeat_withdraw ... ok
test registry::tests::malformed_remote_record_is_rejected ... ok
test registry::tests::own_address_is_never_a_peer ... ok
test registry::tests::tombstone_beats_older_live_record ... ok
test protocol::tests::chunking_respects_budget_and_keeps_every_record ... ok
test registry::tests::expired_records_disappear ... ok
test registry::tests::merge_converges_regardless_of_order ... ok
test torrent::tests::node_list_roundtrips ... ok
test torrent::tests::source_parsing_prefers_magnet_and_urls ... ok
test torrent::tests::web_seed_candidates_include_the_filename ... ok
test torrent::tests::bundle_writes_both_files ... ok
test torrent::tests::bundle_is_self_consistent ... ok
test api::tests::health_reports_the_node ... ok
test api::tests::publish_browse_and_withdraw_over_http ... ok
test client::tests::publish_and_browse_through_the_client ... ok
test api::tests::invalid_announcement_is_rejected ... ok
test client::tests::failover_skips_unreachable_nodes ... ok
test node::tests::two_nodes_converge_on_a_game ... ok
test discovery::lan::tests::two_sockets_exchange_an_announcement ... ok

test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.50s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test a_player_cannot_join_twice_while_a_match_is_pending ... ok
test two_players_are_paired_and_the_result_moves_the_ladder ... ok
test ai_agents_use_a_separate_ladder_and_announce_ai_lobbies ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.06s


running 3 tests
test restarted_node_catches_up_from_a_peer ... ok
test surviving_nodes_keep_serving_after_one_dies ... ok
test a_chain_of_nodes_converges_on_one_lobby ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s


running 2 tests
test qbittorrent_path_recovers_the_node_list ... ok
test isolated_node_finds_the_mesh_through_a_web_seed ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 35 tests
test briefing::tests::faction_constructors_map_player_indices ... ok
test math::tests::clamp_bounds ... ok
test math::tests::mix_endpoints ... ok
test math::tests::remap_clamps_input_range ... ok
test math::tests::smoothstep_endpoints_and_zero_derivative ... ok
test prng::tests::different_seed_diverges ... ok
test prng::tests::hash_coord_is_stable_and_seed_sensitive ... ok
test prng::tests::next_f64_is_unit_interval ... ok
test prng::tests::next_range_stays_in_bounds ... ok
test prng::tests::same_seed_same_sequence ... ok
test prng::tests::splitmix64_known_vector ... ok
test sprites::tests::accent_uses_remapped_palette_entry ... ok
test sprites::tests::family_mapping_follows_class_ranges ... ok
test sprites::tests::seed_is_stable_for_bitword_helpers ... ok
test sprites::tests::sprite_has_size_transparent_bg_and_content ... ok
test terrain::tests::different_seed_differs ... ok
test terrain::tests::heightmap_is_normalized_and_finite ... ok
test sprites::tests::pose_changes_the_sprite ... ok
test sprites::tests::sprite_is_deterministic ... ok
test terrain::tests::tiles_wrap_seamlessly_edges_match_lattice ... ok
test terrain::tests::same_seed_is_bit_identical ... ok
test sprites::tests::every_family_generates_content ... ok
test terrain::tests::texture_matches_heightmap_dimensions_and_is_opaque ... ok
test texture::tests::noise_texture_seed_sensitive_and_sized ... ok
test terrain::tests::water_band_exists_for_high_sea_level ... ok
test texture::tests::noise_texture_is_bit_deterministic ... ok
test texture::tests::tiberium_texture_is_seamless_at_edges ... ok
test briefing::tests::portraits_carry_player_accent_collar ... ok
test texture::tests::tiberium_texture_has_patch_variance_and_determinism ... ok
test briefing::tests::briefing_art_is_sized_with_transparent_background ... ok
test briefing::tests::briefing_art_is_bit_deterministic ... ok
test briefing::tests::every_faction_emblem_has_glyph_contrast ... ok
test briefing::tests::seed_changes_emblem_dither_and_portrait_shading ... ok
test briefing::tests::factions_and_mission_index_change_the_art ... ok
test briefing::tests::briefing_art_is_palette_bounded ... ok

test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s


running 7 tests
test heightmap_matches_texture_dimensions ... ok
test terrain_uses_a_bounded_palette ... ok
test generated_sprites_use_only_remapped_palette_entries ... ok
test terrain_pipeline_is_deterministic_and_sized ... ok
test sprite_pose_affects_output_but_seed_free_determinism_holds ... ok
test textures_and_sprites_generate_for_all_public_entry_points ... ok
test briefing_art_is_deterministic_and_palette_bounded ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 73 tests
test aircraft::tests::airfield_enforces_capacity ... ok
test aircraft::tests::dead_aircraft_cannot_operate ... ok
test aircraft::tests::helipad_holds_one_aircraft ... ok
test aircraft::tests::aircraft_burns_ammo_then_rearms_only_while_docked ... ok
test game::tests::beach_masks_water_edges_then_rock_edges ... ok
test game::tests::cancelling_production_refunds_the_front_item ... ok
test game::tests::box_selection_only_takes_owned_units_inside ... ok
test game::tests::chronosphere_and_iron_curtain_use_the_shared_superweapon_state ... ok
test game::tests::grass_transitions_only_on_ra_expressible_pairs ... ok
test game::tests::mcv_deploys_into_a_construction_yard ... ok
test game::tests::battlefield_conditions_and_spawn_actions_fire ... ok
test game::tests::network_commands_place_structures_once ... ok
test game::tests::ore_cells_swap_base_tile_to_ore_ground ... ok
test game::tests::network_commands_queue_units ... ok
test game::tests::harvesters_mine_ore_into_credits ... ok
test game::tests::ore_amount_maps_to_deterministic_sprite_columns ... ok
test game::tests::player_side_controls_ai_and_selection ... ok
test game::tests::naval_yard_production_spawns_a_ship_on_water ... ok
test game::tests::repairing_a_structure_costs_and_heals ... ok
test game::tests::scatter_targets_are_deterministic_nearby_cells ... ok
test game::tests::move_order_pathfinds_and_arrives_on_land ... ok
test game::tests::skirmish_starts_with_bases_units_and_superweapons ... ok
test game::tests::scenario_builds_the_game_and_fires_timed_triggers ... ok
test game::tests::network_commands_move_only_the_listed_units ... ok
test game::tests::selling_a_structure_refunds_half ... ok
test game::tests::queueing_and_production_spawns_a_unit_for_the_owner ... ok
test game::tests::variant_columns_follow_the_shared_convention ... ok
test game::tests::water_selection_treats_map_edges_as_open_water ... ok
test game::tests::water_cells_blend_into_shore_families_by_land_neighbors ... ok
test game::tests::stop_commands_clear_paths_and_targets ... ok
test gapgen::tests::cannot_activate_without_power_and_dies_cleanly ... ok
test game::tests::toggling_structure_power_changes_house_power ... ok
test game::tests::structure_placement_enforces_economy_and_space ... ok
test game::tests::tesla_coil_fires_at_enemies_in_range ... ok
test gapgen::tests::gap_generator_shrouds_only_while_active_and_powered ... ok
test house::tests::country_gate_allows_only_the_unique_structure ... ok
test house::tests::low_power_halves_production_speed ... ok
test house::tests::spending_and_refunds_are_exact ... ok
test integration_tests::allied_country_can_charge_its_superweapon_and_defend_a_base ... ok
test integration_tests::soviet_mcv_deploys_into_a_yard_and_iron_curtain_protects_it ... ok
test map::tests::ore_can_be_mined_until_empty ... ok
test map::tests::new_map_has_a_river_and_ore_fields ... ok
test mcv::tests::deployment_takes_time_and_can_be_cancelled ... ok
test mcv::tests::footprint_and_clearance ... ok
test naval::tests::dead_or_landlocked_ships_do_not_move ... ok
test naval::tests::ship_moves_toward_its_order_and_stops ... ok
test naval::tests::submerged_submarine_cannot_be_targeted_or_fire ... ok
test pathfind::tests::air_units_ignore_water_and_blocked_cells ... ok
test house::tests::countries_map_to_factions_and_unique_structures ... ok
test pathfind::tests::ground_path_stays_on_land_and_avoids_the_river ... ok
test mcv::tests::dead_mcv_cannot_deploy ... ok
test scenario::tests::class_names_and_numeric_ids_both_parse ... ok
test pathfind::tests::blocked_cells_force_a_detour_or_failure ... ok
test game::tests::mid_mission_save_roundtrips_state ... ok
test spy::tests::spy_effects_depend_on_the_target ... ok
test scenario::tests::rejects_scenarios_without_map_dimensions ... ok
test pathfind::tests::naval_paths_follow_water_and_ground_cannot_enter_it ... ok
test spy::tests::disguise_requires_a_living_spy ... ok
test scenario::tests::briefing_image_present_round_trips ... ok
test spy::tests::thief_caps_total_carryings ... ok
test scenario::tests::briefing_image_absent_is_none ... ok
test superweapon::tests::charging_needs_the_full_duration ... ok
test superweapon::tests::chronosphere_moves_units_then_consumes_charge ... ok
test superweapon::tests::gps_reveals_every_enemy_and_resets ... ok
test superweapon::tests::iron_curtain_blocks_damage_for_its_window ... ok
test scenario::tests::parses_map_objects_ore_and_triggers ... ok
test tesla::tests::coil_charges_then_fires_once_and_reloads ... ok
test tesla::tests::destroying_the_coil_disables_it ... ok
test tesla::tests::out_of_range_and_unpowered_coils_do_not_fire ... ok
test game::tests::network_commands_attack_only_listed_units ... ok
test game::tests::ai_commander_builds_units_over_time ... ok
test game::tests::combat_damages_and_removes_an_enemy ... ok
test game::tests::pack_resolves_expected_rows_and_ore_fx_rects ... ok

test result: ok. 73 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s


running 1 test
test authored_ra_campaign_scenarios_load ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 18 tests
test camera::tests::add_shake_takes_max_and_rejects_negative ... ok
test camera::tests::shake_jitter_is_deterministic_and_bounded ... ok
test camera::tests::shake_decays_to_rest ... ok
test camera::tests::screen_to_world_ignores_shake ... ok
test camera::tests::viewport_clipping_covers_the_visible_world_rect ... ok
test palette_shader::tests::cpu_contract_normal_shadow_transparent ... ok
test palette_shader::tests::mode_flags_bits ... ok
test renderer::tests::software_renderer_draws_text_pixels ... ok
test palette_shader::tests::cpu_contract_with_player_remap ... ok
test renderer::tests::software_renderer_samples_two_atlases ... ok
test sprite_batch::tests::instance_layout_matches_struct_size ... ok
test sprite_batch::tests::z_order_sort_is_stable_and_ascending ... ok
test ui::tests::button_draws_border_and_centered_label ... ok
test ui::tests::console_grows_one_row_per_line ... ok
test ui::tests::hit_test_finds_first_containing_widget ... ok
test ui::tests::panel_tints_by_title_presence ... ok
test palette_shader::tests::shader_is_valid_wgsl ... ok
test sprite_batch::tests::shader_is_valid_wgsl ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
test sprite_atlas_uv_selection_is_sampled_on_gpu ... ok
test sprite_batch_renders_a_red_quad_on_gpu ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s


running 17 tests
test effects::tests::wav_bytes_have_riff_header_and_match_len ... ok
test effects::tests::construction_loop_periods_are_identical ... ok
test effects::tests::engine_loop_is_seamless ... ok
test waves::tests::adsr_attacks_then_decays ... ok
test waves::tests::decay_env_monotone_and_bounded ... ok
test waves::tests::one_pole_moves_toward_input ... ok
test effects::tests::iron_curtain_hum_is_seamless ... ok
test waves::tests::square_wave_values_are_binary ... ok
test waves::tests::waves_stay_in_range ... ok
test effects::tests::effects_are_deterministic_per_seed_and_seed_sensitive ... ok
test effects::tests::ui_bleeps_are_short_distinct_and_bounded ... ok
test effects::tests::percussive_effects_decay ... ok
test effects::tests::explosion_variants_decay_with_debris_tails ... ok
test effects::tests::superweapon_cues_charge_then_shift ... ok
test effects::tests::weapon_variants_are_deterministic_per_seed ... ok
test effects::tests::weapon_variants_are_distinct_and_bounded ... ok
test effects::tests::every_effect_is_bounded_and_nonzero ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 195 tests
test ai::tests::harvester_is_available_to_both_factions ... ok
test building::tests::capture_transfers_ownership_and_marks_the_building ... ok
test ai::tests::weighted_choice_is_deterministic_and_respects_weights ... ok
test ai::tests::expert_ai_rebuilds_power_when_missing ... ok
test ai::tests::rules_cost_and_prerequisites_change_the_table ... ok
test ai::tests::production_weights_honour_tech_level_and_faction ... ok
test building::tests::plant_c4_sets_going_to_blow_and_the_building_h_countdown ... ok
test ai::tests::expert_ai_staffs_refineries_with_harvesters ... ok
test ai::tests::expert_ai_builds_a_refinery_when_income_is_missing ... ok
test building::tests::detonate_applies_warhead_fire_damage_of_5000 ... ok
test carryover::tests::bytes_round_trip_preserves_the_carryover ... ok
test ai::tests::expert_ai_launches_an_attack_wave_when_the_force_is_ready ... ok
test building::tests::new_buildings_carry_no_sabotage_state ... ok
test carryover::tests::corrupted_buffers_are_rejected_cleanly ... ok
test ai::tests::expert_ai_triggers_fire_sale_on_a_collapsing_base ... ok
test carryover::tests::apply_grants_capped_credits_and_redeploys_units ... ok
test carryover::tests::file_round_trip_through_a_temp_path ... ok
test carryover::tests::from_game_captures_only_surviving_owner_units ... ok
test ai::tests::expert_ai_is_deterministic ... ok
test game::prerequisite_tests::ai_production_uses_rules_cost_and_tech_level ... ok
test game::prerequisite_tests::building_prerequisite_is_enforced ... ok
test game::prerequisite_tests::faction_ownership_uses_owner_tokens_and_td_fallback ... ok
test game::prerequisite_tests::owner_targeted_orders_only_touch_listed_units ... ok
test game::prerequisite_tests::gdi_and_nod_barracks_aliases_are_equivalent ... ok
test game::prerequisite_tests::prerequisite_only_sections_are_registered_as_types ... ok
test game::prerequisite_tests::cancelling_td_production_refunds_the_front_item ... ok
test game::prerequisite_tests::rules_tech_level_override_replaces_the_fallback ... ok
test game::prerequisite_tests::sidebar_keeps_rows_visible_when_unaffordable ... ok
test game::prerequisite_tests::prerequisite_lists_require_every_token ... ok
test game::prerequisite_tests::tech_level_gates_unit_and_structure_production ... ok
test game::prerequisite_tests::none_and_faction_tokens_are_supported ... ok
test game::prerequisite_tests::repairing_a_building_costs_and_heals ... ok
test game::prerequisite_tests::sidebar_hides_tech_and_prerequisite_locked_rows ... ok
test game::prerequisite_tests::selling_a_building_refunds_half ... ok
test game::rules_integration_tests::ini_names_match_original_data_files ... ok
test game::rules_integration_tests::empty_rules_keep_original_costs ... ok
test game::prerequisite_tests::unit_prerequisite_requires_owned_structure ... ok
test game::prerequisite_tests::sidebar_filters_owner_locked_rows ... ok
test game::prerequisite_tests::unknown_prerequisite_fails_closed ... ok
test game::rules_integration_tests::rules_override_building_cost_and_charge ... ok
test game::rules_integration_tests::load_rules_from_disk_reports_type_count ... ok
test game::rules_integration_tests::set_rules_replaces_database ... ok
test game::prerequisite_tests::types_without_rules_have_no_prerequisites ... ok
test game::rules_integration_tests::rules_override_strength_when_building_is_placed ... ok
test game::rules_integration_tests::rules_override_unit_cost_and_charge ... ok
test game::rules_integration_tests::unknown_type_falls_back_to_static_stats ... ok
test game::tests::aircraft_rearms_at_helipad ... ok
test game::tests::capture_respects_the_bdata_captureable_flag ... ok
test game::tests::aircraft_fire_consumes_ammo ... ok
test game::tests::aircraft_rotor_and_vehicle_tracks_animate_while_moving ... ok
test game::tests::click_on_empty_ground_clears_selection ... ok
test game::tests::aircraft_flies_over_impassable_terrain ... ok
test game::tests::enemy_at_finds_nod_unit ... ok
test game::rules_integration_tests::rules_override_strength_when_unit_spawns ... ok
test game::tests::box_selection_selects_multiple_units ... ok
test game::tests::harvester_harvests_tiberium ... ok
test game::tests::atlas_covers_demo_classes ... ok
test game::tests::engineer_captures_an_enemy_building_and_is_consumed ... ok
test game::tests::harvester_unloads_at_refinery ... ok
test game::tests::homing_projectile_retargets_on_even_frames ... ok
test game::tests::house_spends_tiberium_before_credits ... ok
test game::tests::house_harvested_caps_at_capacity ... ok
test game::tests::ion_cannon_requires_building_and_charge ... ok
test game::tests::click_selects_nearest_unit ... ok
test game::tests::idle_units_auto_acquire_nearby_enemy ... ok
test game::tests::mammoth_uses_primary_inside_range_and_tusk_outside ... ok
test game::tests::move_order_avoids_building_cells ... ok
test game::tests::explosion_damage_hits_nearby_enemies ... ok
test game::tests::group_move_uses_nearby_cells ... ok
test game::tests::commando_sabotage_arms_the_building_then_blows_it_up ... ok
test game::tests::move_order_routes_around_impassable_terrain ... ok
test game::tests::nuke_hits_wide_area_and_buildings ... ok
test game::tests::group_moves_reserve_distinct_cells_until_arrival ... ok
test game::tests::queue_building_requires_construction_yard ... ok
test game::tests::queue_requires_matching_factory ... ok
test game::tests::sabotage_orders_are_restricted_to_commandos_and_enemy_buildings ... ok
test game::tests::sabotage_command_applies_through_the_network_handler ... ok
test game::tests::setup_demo_computes_house_power ... ok
test game::tests::shift_click_adds_to_selection ... ok
test game::tests::queue_unit_spends_credits_and_spawns_at_factory ... ok
test game::tests::silo_increases_storage_capacity ... ok
test game::tests::status_reports_mission_and_credits ... ok
test game::tests::tiberium_grows_and_spreads ... ok
test game::tests::superweapons_charge_over_time ... ok
test game::tests::world_center_is_map_middle ... ok
test game::tests::turret_rotates_before_firing ... ok
test game::weapon_rules_tests::rules_spread_attenuates_damage_with_distance ... ok
test game::tests::queue_building_completes_to_ready ... ok
test game::tests::place_building_rejects_occupied_cell ... ok
test game::weapon_rules_tests::set_rules_refreshes_existing_unit_weapons ... ok
test game::weapon_rules_tests::warhead_verses_override_without_weapon_section ... ok
test game::weapon_rules_tests::weapon_ini_names_match_rules_tables ... ok
test game::weapon_rules_tests::weapon_damage_rof_and_range_come_from_rules ... ok
test game::tests::place_ready_building_adds_structure ... ok
test game::weapon_rules_tests::production_spawns_units_with_rules_weapons ... ok
test map::tests::land_mask_counts_cardinal_land_only ... ok
test map::tests::tiberium_stage_maps_to_fx_family ... ok
test game::weapon_rules_tests::weapon_warhead_override_selects_rules_warhead ... ok
test map::tests::water_shore_family_follows_land_neighbor_mask ... ok
test map::tests::water_terrain_encoding_has_no_collision ... ok
test map::tests::water_terrain_survives_serialization ... ok
test mission::tests::briefing_image_present_round_trips ... ok
test mission::tests::condition_combinators_and_negation ... ok
test mission::tests::briefing_image_absent_is_none ... ok
test mission::tests::disable_action_stops_trigger ... ok
test mission::tests::parses_mission_ini ... ok
test mission::tests::destroying_nod_structures_runs_script_and_wins ... ok
test mission::tests::repeatable_trigger_fires_every_tick ... ok
test mission::tests::spawn_and_build_actions_create_entities ... ok
test mission::tests::zone_condition_detects_units ... ok
test network::migration_tests::host_loss_elects_lowest_alive_and_claim_is_accepted ... ok
test network::migration_tests::migration_message_roundtrips ... ok
test mission::tests::elapsed_trigger_fires_once_and_grants_credits ... ok
test mission::tests::shipped_sample_mission_parses ... ok
test network::migration_tests::non_host_departure_keeps_the_host_stable ... ok
test network::migration_tests::peer_that_missed_the_loss_resynchronises_before_claiming ... ok
test network::migration_tests::relay_cache_drops_duplicates_and_evicts ... ok
test network::migration_tests::relay_clear_accepts_replayed_packets ... ok
test network::migration_tests::stale_or_illegitimate_claims_are_rejected ... ok
test network::netcode_tests::all_command_variants_round_trip ... ok
test network::netcode_tests::codec_detects_corrupted_payload ... ok
test network::netcode_tests::codec_rejects_bad_magic_version_and_truncation ... ok
test network::netcode_tests::command_order_is_preserved_on_the_wire ... ok
test network::netcode_tests::datagram_dispatch_routes_command_and_handshake_frames ... ok
test network::netcode_tests::handshake_host_validates_slots_and_duplicates ... ok
test network::netcode_tests::handshake_messages_round_trip ... ok
test network::netcode_tests::handshake_rejects_closed_rooms ... ok
test network::netcode_tests::replay_load_reports_missing_files ... ok
test network::netcode_tests::replay_rejects_corruption_and_bad_headers ... ok
test network::netcode_tests::removed_peers_stop_receiving ... ok
test network::netcode_tests::replay_round_trips_through_disk ... ok
test game::tests::sabotage_ticks_deterministically_across_identical_games ... ok
test network::netcode_tests::codec_rejects_oversized_command_lists ... ok
test network::netcode_tests::udp_broadcast_without_peers_is_a_noop ... ok
test network::tests::command_hashes_are_stable_and_distinct ... ok
test network::tests::frame_checksums_are_order_independent_and_detect_desync ... ok
test network::netcode_tests::handshake_join_and_leave_over_loopback ... ok
test network::tests::packets_checksum_and_verify ... ok
test network::tests::queue_inserts_out_of_order_commands_deterministically ... ok
test network::netcode_tests::udp_broadcast_reaches_every_peer ... ok
test network::tests::queue_keeps_future_commands ... ok
test network::tests::replay_can_be_replayed_deterministically ... ok
test network::tests::replay_records_executed_commands_in_order ... ok
test network::tests::session_gates_commands_by_frame ... ok
test network::tests::session_rejects_invalid_player_and_stale_frames ... ok
test network::tests::sabotage_command_decodes_and_reaches_the_simulation ... ok
test network::netcode_tests::udp_rejects_invalid_datagrams ... ok
test pathfind::tests::goal_must_be_free ... ok
test pathfind::tests::blocked_cells_are_avoided ... ok
test pathfind::tests::straight_path_uses_diagonals ... ok
test pathfind::tests::no_path_when_wall_has_no_gap ... ok
test powerup::tests::crates_expire_after_their_lifetime ... ok
test pathfind::tests::path_detours_around_a_wall ... ok
test powerup::tests::dead_units_never_collect ... ok
test powerup::tests::drive_over_collects_and_removes_the_crate ... ok
test powerup::tests::splitmix_matches_pinned_vectors ... ok
test powerup::tests::spawns_are_deterministic_and_cadenced ... ok
test savegame::tests::rejects_corrupt_or_truncated_saves ... ok
test td_rules::tests::aircraft_stats_match_aadata ... ok
test td_rules::tests::bullet_flight_flags_match_bbdata ... ok
test td_rules::tests::commando_row_matches_idata_cpp ... ok
test td_rules::tests::conversions_match_display_constants ... ok
test td_rules::tests::engineer_row_matches_idata_cpp ... ok
test td_rules::tests::harvester_stats_match_udata ... ok
test td_rules::tests::infantry_speeds_are_slower_than_tanks ... ok
test game::prerequisite_tests::nod_mission_controls_the_nod_house ... ok
test td_rules::tests::mammoth_tank_stats_match_udata ... ok
test td_rules::tests::modify_damage_matches_combat_cpp ... ok
test td_rules::tests::production_time_scales_with_power_fraction ... ok
test td_rules::tests::sabotage_constants_match_infantry_cpp ... ok
test td_rules::tests::structure_rows_carry_bdata_is_captureable ... ok
test td_rules::tests::structure_stats_match_bdata ... ok
test td_rules::tests::superweapon_structure_stats_match_bdata ... ok
test td_rules::tests::weapon_rows_match_const_cpp ... ok
test unit::tests::single_shooter_always_uses_normal_rearm ... ok
test unit::tests::special_infantry_carry_role_flags ... ok
test unit::tests::turret_turns_toward_heading ... ok
test unit::tests::two_shooter_alternates_short_and_long_rearm ... ok
test weapon::tests::ap_damage_is_reduced_against_infantry_armor ... ok
test weapon::tests::cannon_uses_ap_warhead_and_is_not_instant ... ok
test weapon::tests::fueled_projectile_expires_when_fuse_runs_out ... ok
test weapon::tests::m16_weapon_uses_td_stats ... ok
test weapon::tests::arcing_projectile_gains_and_loses_altitude ... ok
test weapon::tests::projectile_reaches_target ... ok
test weapon::tests::rifle_is_instant ... ok
test savegame::tests::roundtrip_preserves_mission_progress ... ok
test savegame::tests::file_roundtrip_creates_directories ... ok
test game::tests::attack_order_fires_projectiles_and_damages_enemy ... ok
test savegame::tests::roundtrip_preserves_live_state ... ok
test network::tests::sabotage_apply_is_deterministic_across_peers ... ok
test game::tests::move_order_drives_unit_to_target ... ok
test game::tests::render_with_sprites_emits_unit_and_building_sprites ... ok
test network::migration_tests::relay_forwards_to_others_but_not_the_source ... ok
test game::tests::demo_game_updates_and_renders ... ok
test game::tests::move_order_draws_target_marker ... ok

test result: ok. 195 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s


running 1 test
test authored_td_campaign_missions_parse ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test host_migration_updates_the_elected_host_address ... ok
test network_config_validates_nat_friendly_settings ... ok
test live_session_relays_a_command_once_and_builds_packets ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test cnc-ai/src/ladder.rs - ladder (line 8) - compile ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test cnc-audio/src/lib.rs - (line 51) - compile ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test cnc-net-directory/src/lib.rs - (line 29) - compile ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s


running 1 test
test cnc-procgen/src/briefing.rs - briefing (line 23) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Per-target tests:

| Target | Tests |
|---|---|
| cnc-ai | 23 passed, 0 failed |
| cnc-asset | 56 passed, 0 failed |
| cnc-audio | 35 passed, 0 failed |
| cnc-bench | 2 passed, 0 failed |
| cnc-core | 120 passed, 0 failed |
| cnc-launcher | 31 passed, 0 failed |
| cnc-map-editor | 91 passed, 0 failed |
| cnc-music | 14 passed, 0 failed |
| cnc-net-directory | 58 passed, 0 failed |
| cnc-procgen | 35 passed, 0 failed |
| cnc-red-alert | 73 passed, 0 failed |
| cnc-renderer | 18 passed, 0 failed |
| cnc-sfx | 17 passed, 0 failed |
| cnc-tiberian-dawn | 195 passed, 0 failed |

---

## Crate Health Matrix

| Crate | Version | README | CHANGELOG | codemap | Tests |
|---|---|---|---|---|---|
| cnc-ai | true  | ✅ | ✅ | ✅ | 23✅/0❌ |
| cnc-asset | true  | ✅ | ✅ | ✅ | 56✅/0❌ |
| cnc-audio | true  | ✅ | ✅ | ✅ | 35✅/0❌ |
| cnc-bench | 0.1.0 | ✅ | ✅ | ✅ | 2✅/0❌ |
| cnc-core | true  | ✅ | ✅ | ✅ | 120✅/0❌ |
| cnc-launcher | true  | ✅ | ✅ | ✅ | 31✅/0❌ |
| cnc-map-editor | true  | ✅ | ✅ | ✅ | 91✅/0❌ |
| cnc-music | true  | ✅ | ✅ | ✅ | 14✅/0❌ |
| cnc-net-directory | true  | ✅ | ✅ | ✅ | 58✅/0❌ |
| cnc-procgen | true  | ✅ | ✅ | ✅ | 35✅/0❌ |
| cnc-red-alert | true  | ✅ | ✅ | ✅ | 73✅/0❌ |
| cnc-renderer | true  | ✅ | ✅ | ✅ | 18✅/0❌ |
| cnc-sfx | true  | ✅ | ✅ | ✅ | 17✅/0❌ |
| cnc-tiberian-dawn | true  | ✅ | ✅ | ✅ | 195✅/0❌ |

---

## Cargo Check



---

## Clippy



---

## Formatting

Diff in /home/hyperion/Projects/Games/CnC_Remastered_Collection/RUST/cnc-tiberian-dawn/src/powerup.rs:209:
 #[cfg(test)]
 mod tests {
     use super::*;
-    use cnc_core::Fixed16_16;
     use crate::map::{CellGrid, Map};
     use crate::td_rules::UnitType;
+    use cnc_core::Fixed16_16;
 
     fn sample_grid() -> CellGrid {
         Map::new("test".to_string(), 20, 20).grid

---

*Auto-generated by sync-docs.sh on 2026-10-19 22:17:04 UTC*

---

## `cnc32_assets_v6_2_1` — pack engine tests (2026-10-20 15:26 UTC, cargo 1.98.1 / rustc 1.98.1)

Engine tests authored inside the pack at `assets/packs/cnc32_assets_v6_2_1/engine_tests/rust/`
and bridged through `cnc-asset/tests/cnc32_engine_tests.rs`
(`classic_pack` loader added to `cnc-asset/src/classic_pack.rs`).

```
running 10 tests
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s
```

### Full verification (`CNC_CNC32_VERIFY=1`)

```
running 10 tests
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s
```

Structural pass covered all 4,577 manifest sprites (unique ids, safe relative paths, on-disk
files, 64-hex digests) and re-verified all 940 terrain-transition masks from `mask_bits`.
All 3,909 `atlas_rect` entries bounds-checked against the 14 decode-once atlases. Gated pass
additionally verified PNG dimensions for 287 stride-sampled sprites and pixel-exact
atlas crop-vs-standalone equivalence for 66 sampled tiles. Whole crate `cargo test -p cnc-asset`:
102 passed. Missing-pack discovery degrades to printed skips (verified with
`CNC_CNC32_PACK_DIR=/nonexistent`).

---

## `cnc-map-editor` — hard-switch to `cnc32_assets_v6_2_1` (2026-10-20 16:0x UTC, cargo 1.98.1 / rustc 1.98.1)

Editor loads the v6.2.1 pack by default through `cnc-asset` `PackTileset` (8192×160 composed
atlas, 32px tiles) and ports the generator's tile-matching logic: 8-bit blob masks
(47 valid + idempotent repair), road/shore/cliff/wall/resource/decoration sprite resolution.
Legacy 16px `tileset_pack` path retired from the editor's default; `--tileset-palette` removed.
New module `cnc-map-editor/src/pack_tileset.rs`; `app.rs`/`ui.rs` draw glue switched;
`--tileset` repurposed as a cnc32 pack-dir override.

```
cargo test -p cnc-map-editor           → 95 passed; 1 failed (pre-existing app
                                         layout_controls_stay_inside_the_window,
                                         trigger-tab layout — orthogonal to this
                                         change, our diff does not touch that code)
cargo test -p cnc-map-editor --test headless --test validate → 2 passed
cargo test -p cnc-map-editor --lib pack_tileset → 5 passed
cargo clippy -p cnc-map-editor --all-targets → 0 warnings in lane files
Headless smoke: --headless --seed 42 --screenshot → exit 0,
  PNG 1400×900 (822 KB), "tileset pack loaded: 8192x160 composed atlas (tile size 32)"
```

Flag matrix verified: `--no-tileset` flat path OK; valid `--tileset` override OK;
nonexistent `--tileset` warns + flat fallback; `--tileset-palette` fails as unknown arg (documented removal).

---

## Visual QA — map editor cnc32 switch (2026-10-20, NVIDIA VL pinned subagent, 5 passes)

Two-pass loop (review → ground → fix → regenerate → re-review by same session):
- **Pass 1** (legacy-pipeline `tileset_render` harness + headless smoke): 3 text items flagged → all grounded as intentional non-defects; 1 clipping finding → grounded as by-construction (rock ridge to map edge).
- **Code-level grounded defect** from pass-1 grounding: `"Players {}"` caption double-drawn in `app.rs draw_panel` → duplicate removed.
- **Pass 2** (cnc32 harness, ported example): concrete patch rendered **black** → grounded: harness skipped `cell_draw == None` fallback → fixed by painting `app::terrain_color` fallback.
- **Pass 3** (closure, both artifacts): **PASS, 0 major / 0 minor** — grey fallback confirmed; single caption confirmed; all semantic checks hold (blob scallops, shoreline orientation, decor-on-base layering, grid alignment, no seams/halos).
- **Passes 4/4b/4c** (LIVE window captures `/tmp/editor_live{,2,3,4}.png`; first two honestly ABSENT/BLOCKED due to focus races with the user's own desktop windows, then a fresh instance captured with focus verified pre-grab): live **wgpu** render — **PASS, 0 defects**. UI chrome clean (single Players caption holds), grid intact, no washout/blur/alpha artifacts in the GPU path.

Harness port: `examples/tileset_render.rs` now uses `pack_tileset::cell_draw`/`sprite_rect` (32px cnc32 pipeline; SCALE=1 == legacy 16px/×2 canvas 2616×1312) with `CNC_TILESET_QA_DUMP` ported. Fixture known-good artifacts: `/tmp/opencode/tileset_qa.png`, `/tmp/cnc32_editor_smoke.png`, `/tmp/editor_live4.png`.

