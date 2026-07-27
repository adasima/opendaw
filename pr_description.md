🎯 **What:** Added comprehensive unit tests for `MidiMappingRegistry` in `opendaw-wasm/src/midi/mapping.rs` to address a critical testing gap.

📊 **Coverage:** Covered all core functionalities including initialization (`new`/`default`), MIDI learn and unlearn capabilities (`learn_mapping`/`unlearn_mapping`), handling CC input mapped and unmapped cases correctly normalizing values 0-127 to 0.0-1.0 (`handle_cc_input`), and retrieving values (`get_parameter_value`).

✨ **Result:** Improved test coverage and provided a safety net for future refactoring by validating both happy paths and edge cases of the registry's state transitions.
