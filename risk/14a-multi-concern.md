# WU-0A-14a Risk Gate: Multi-concern PR

**Reviewer:** claude-opus
**Phase:** Implementation pipeline Phase 8 — 4-gate review
**Severity:** LOW

## Summary

The diff is single-concern. `git diff main --stat` reports three modified files totaling 6 net additions, plus the new files inside the WU-0A-14a code/test boundary and the contract+fixtures+proposal docs. `Cargo.toml`, `tauri.conf.json`, and `bun.lock` are untouched. Two of the three modified files (`src-tauri/src/lib.rs`, `src-tauri/src/contracts/mod.rs`) are pure module-declaration wiring — a single `pub mod` line each — and qualify as "minimal lib.rs / mod.rs wiring" per the gate prompt. The third (`src-tauri/src/app_state.rs`) adds a 4-line `EventBusHandle::publish` method that the harness needs to replay recorded fixture events through the live broadcast bus; this is an incidental edit to a WU-0A-04-owned file that is small, additive, and necessary for the WU-0A-14a contract — flagged as a LOW finding.

## Verification

### `git diff main --stat`

```
 src-tauri/src/app_state.rs     | 4 ++++
 src-tauri/src/contracts/mod.rs | 1 +
 src-tauri/src/lib.rs           | 1 +
 3 files changed, 6 insertions(+)
```

Plus untracked additions (still in working tree, not committed yet):

```
product-strategy/contracts/fixtures/wu-0a-14a/
product-strategy/contracts/wu-0a-14a-temp-sqlite-harness-entrypoint.md
proposals/14a-wu-0a-14a.md
src-tauri/src/contracts/temp_harness.rs
src-tauri/src/test_harness/
src-tauri/tests/temp_harness_contract.rs
```

All untracked paths are inside the WU-0A-14a code/test boundary or the documentation track (`proposals/`, `product-strategy/contracts/`). No incidental new files outside the boundary.

### `lib.rs` and `contracts/mod.rs` are minimal wiring

`src-tauri/src/lib.rs` adds a single `pub mod test_harness;` line (line 7). The Tauri builder, the `tauri::generate_handler!` arguments, the `PHASE_0A_SCAFFOLD_COMMANDS` const, the runtime `build_runtime_harness_app_state` factory, and the existing test module are all unchanged.

`src-tauri/src/contracts/mod.rs` adds a single `pub mod temp_harness;` line. No reordering, no other edits.

These are both inside the prompt's "minimal lib.rs / mod.rs wiring" allowance.

### `app_state.rs` — incidental edit (the only real finding)

```diff
+    pub fn publish(&self, event: IpcEvent<Value>) -> usize {
+        self.sender.send(event).unwrap_or(0)
+    }
```

Inserted into `EventBusHandle` between `subscribe` and `receiver_count`. This is a 4-line, additive, public-API enlargement on a struct owned by WU-0A-04. The harness needs it because:

- `EventBusHandle.sender` (the `broadcast::Sender`) is private.
- `replay_recorded_runtime_events` needs to send fixture events to the same broadcast channel that the live `subscribe()` receivers consume.
- The pre-existing API only exposes `subscribe()`, `receiver_count()`, `register_subscription()`, `subscription_metadata()`, and `subscription_count()` — none of which can publish.

Without this addition, the harness would either need to duplicate the broadcast channel (defeating the purpose of replaying through the *live* bus) or expose `sender` as `pub` (a larger surface change). The chosen path is the smallest viable change. See the finding for details.

### `Cargo.toml`, `bun.lock`, `tauri.conf.json` unchanged

`git diff main -- src-tauri/Cargo.toml` returns no output. Same for `bun.lock` and `src-tauri/tauri.conf.json`. The implementer's note about needing `bun install --frozen-lockfile` did not produce a lockfile change, consistent with the prompt expectation.

### No edits to other WU-0A implementation files

The lib.rs commands list, the WU-0A-02 settings module, the WU-0A-03 storage module, the WU-0A-06 IPC envelope, the WU-0A-08 subscription command, the WU-0A-10 backend span event, and the WU-0A-13 workspace route shell are all consumed by import only. `git diff main -- src-tauri/src/contracts/` confirms only `mod.rs` was edited (one-line wiring) and `temp_harness.rs` is new. `git diff main -- src/` shows no frontend edits.

### No commits yet on the branch

`git log main..HEAD --oneline` returns empty — all WU-0A-14a work is staged-or-untracked in the working tree. When the implementer commits, the change set should remain bounded to the files enumerated above.

## Findings

### WU-0A-14a-MULTICONCERN-F01 — `EventBusHandle::publish` added to `app_state.rs` outside the WU-0A-14a code boundary (LOW)

**Where:** `src-tauri/src/app_state.rs:49-51`

**Detail:** The ticket's code boundary is exactly `src-tauri/src/test_harness/temp_harness.rs`, `src-tauri/src/contracts/temp_harness.rs`, `src-tauri/tests/temp_harness_contract.rs`. The implementer added a 4-line `pub fn publish(&self, event: IpcEvent<Value>) -> usize` method to `EventBusHandle` (a WU-0A-04 type) so `replay_recorded_runtime_events` can publish fixture events through the live broadcast channel. This is an incidental edit to a WU-0A-04 implementation file. The change is small, additive, non-breaking (no existing call site changes), and necessary for the runtime-event-fixture replay acceptance criterion. The alternative — exposing `sender` as `pub` — would enlarge the surface more, and routing replay through a duplicate channel would defeat the contract requirement that fixtures replay "through the live event bus".

**Impact:** Slight blurring of WU ownership boundaries. The `publish` method is not called by any production code today; only `replay_recorded_runtime_events` and (transitively) the contract test exercise it. The method is generally useful and could plausibly have shipped with WU-0A-04 in the first place, since the bus already exposes `subscribe()` and `receiver_count()`.

**Recommendation:** Accept as-is. Optional follow-up: document this addition in the WU-0A-14a proposal's "Adjacent public paths" line so the surface change is tracked. Not a blocker.

## Verdict

**LOW.** The PR is single-concern. The two pure-wiring edits (`lib.rs`, `contracts/mod.rs`) are inside the explicit prompt allowance. The one off-boundary edit (`EventBusHandle::publish`) is 4 additive lines, non-breaking, necessary for the replay contract, and the smallest viable design. `Cargo.toml`, `tauri.conf.json`, and `bun.lock` are untouched. No edits to other WU-0A implementation files.
