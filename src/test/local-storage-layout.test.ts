import { describe, expect, it } from "vitest";

import canonicalLayout from "../../product-strategy/contracts/fixtures/wu-0a-03/canonical-layout.json";
import deriveErrors from "../../product-strategy/contracts/fixtures/wu-0a-03/derive-errors.json";
import deriveSuccess from "../../product-strategy/contracts/fixtures/wu-0a-03/derive-success.json";
import invalidLayoutShapes from "../../product-strategy/contracts/fixtures/wu-0a-03/invalid-layout-shapes.json";
import invalidStorageLayoutError from "../../product-strategy/contracts/fixtures/wu-0a-03/invalid-storage-layout-error.json";
import storageLayoutErrors from "../../product-strategy/contracts/fixtures/wu-0a-03/storage-layout-errors.json";
import {
  STORAGE_LAYOUT_ERRORS,
  parseLocalStorageLayout,
  parseStorageLayoutError,
  type LocalStorageLayout,
  type StorageLayoutError,
} from "../contracts/local-storage-layout";

describe("LocalStorageLayout contract", () => {
  it("parses canonical and derived fixtures with the exact Rust-owned DTO field set", () => {
    // Risk: TypeScript accepts shapes Rust does not own. Level: unit/typecheck.
    // Source: proposal test-intent "TypeScript DTO and parser shape".
    const canonical = parseLocalStorageLayout(canonicalLayout);
    const derived = parseLocalStorageLayout(deriveSuccess.expected);

    const typedCanonical: LocalStorageLayout = canonical;
    const typedDerived: LocalStorageLayout = derived;

    expect(typedCanonical).toEqual(canonicalLayout);
    expect(typedDerived).toEqual(deriveSuccess.expected);
    expect(Object.keys(canonical).sort()).toEqual([
      "database_path",
      "evidence_root",
      "fixture_root",
      "log_root",
      "storage_root",
      "temp_root",
    ]);
  });

  it("rejects invalid LocalStorageLayout fixture shapes", () => {
    // Risk: TypeScript accepts values Rust rejects. Level: unit. Source:
    // proposal test-intent "TypeScript DTO and parser shape".
    for (const invalidLayout of invalidLayoutShapes) {
      expect(() => parseLocalStorageLayout(invalidLayout.value), invalidLayout.name).toThrow(
        "Invalid LocalStorageLayout",
      );
    }
  });

  it("round-trips documented StorageLayoutError strings and rejects unknown variants", () => {
    // Risk: TypeScript error taxonomy drifts from Rust. Level: unit/typecheck.
    // Source: proposal test-intent "TypeScript error union".
    const expected = [
      "StorageRootEscapesWorkspace",
      "DatabasePathOutsideStorageRoot",
      "EvidenceRootOutsideStorageRoot",
      "FixtureRootOutsideStorageRoot",
      "LogRootOutsideStorageRoot",
      "TempRootOutsideStorageRoot",
    ] satisfies StorageLayoutError[];
    const selected: StorageLayoutError = "DatabasePathOutsideStorageRoot";
    // @ts-expect-error "UnknownStorageLayoutError" is outside the documented error union.
    const rejected: StorageLayoutError = "UnknownStorageLayoutError";

    expect(STORAGE_LAYOUT_ERRORS).toEqual(expected);
    expect(STORAGE_LAYOUT_ERRORS).toEqual(storageLayoutErrors);
    expect(selected).toBe("DatabasePathOutsideStorageRoot");
    expect(rejected).toBe("UnknownStorageLayoutError");

    expect(storageLayoutErrors.map(parseStorageLayoutError)).toEqual(expected);
    for (const errorCase of deriveErrors) {
      expect(parseStorageLayoutError(errorCase.expected_error)).toBe(errorCase.expected_error);
    }
    expect(() => parseStorageLayoutError(invalidStorageLayoutError)).toThrow(
      "Invalid StorageLayoutError",
    );
  });
});
