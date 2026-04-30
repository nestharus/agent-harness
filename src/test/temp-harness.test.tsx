import { cleanup, screen, waitFor } from "@testing-library/react";
import { useQueryClient } from "@tanstack/react-query";
import { useRouter } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import handleRoundTrip from "../../product-strategy/contracts/fixtures/wu-0a-14a/handle-round-trip.json";
import tempHarnessErrors from "../../product-strategy/contracts/fixtures/wu-0a-14a/temp-harness-errors.json";
import canonicalSeedRenders from "../../product-strategy/contracts/fixtures/wu-0a-14b/canonical-seed-renders.json";
import ipcSeeding from "../../product-strategy/contracts/fixtures/wu-0a-14b/ipc-seeding.json";
import providerMounting from "../../product-strategy/contracts/fixtures/wu-0a-14b/provider-mounting.json";
import renderResultShape from "../../product-strategy/contracts/fixtures/wu-0a-14b/render-result-shape.json";
import seedFailures from "../../product-strategy/contracts/fixtures/wu-0a-14b/seed-failure.json";
import structuralAbsence from "../../product-strategy/contracts/fixtures/wu-0a-14b/structural-absence.json";
import {
  TEMP_HARNESS_ERRORS,
  TempHarnessErrorFailure,
  parseTempHarnessError,
  parseTempHarnessHandle,
  type TempHarnessError,
  type TempHarnessHandle,
} from "../contracts/temp-harness";
import { invokeCommand } from "../ipc/invoke-command";
import { renderWithHarness } from "./render-with-harness";

const sideEffectSpies = vi.hoisted(() => ({
  invoke: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: sideEffectSpies.invoke,
}));

function ProviderProbe() {
  const queryClient = useQueryClient();
  const router = useRouter();

  return (
    <>
      <span data-testid={providerMounting.query_client_probe_test_id}>
        {queryClient ? providerMounting.combined_ready_text : "missing-query-client"}
      </span>
      <span data-testid={providerMounting.router_probe_test_id}>
        {router ? providerMounting.combined_ready_text : "missing-router"}
      </span>
    </>
  );
}

function PingRuntimeProbe() {
  const [responseText, setResponseText] = useState("pending");

  useEffect(() => {
    let active = true;

    void invokeCommand("ping_runtime", {}).then((response) => {
      if (active) {
        setResponseText(`${response.command}:${response.phase}:${response.runtime}`);
      }
    });

    return () => {
      active = false;
    };
  }, []);

  return <span data-testid="ping-runtime-response">{responseText}</span>;
}

function HarnessSettingsProbe() {
  const [agentRunnerBin, setAgentRunnerBin] = useState("pending");

  useEffect(() => {
    let active = true;

    void invokeCommand("get_harness_settings", {}).then((response) => {
      if (active) {
        setAgentRunnerBin(response.agent_runner_bin);
      }
    });

    return () => {
      active = false;
    };
  }, []);

  return <span data-testid="agent-runner-bin">{agentRunnerBin}</span>;
}

describe("TempHarness DTO and renderWithHarness", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    cleanup();
  });

  it("round-trips WU-0A-14a TempHarnessHandle and error fixtures through the TypeScript DTO", () => {
    // Risk: TypeScript accepts temp harness handles or errors that Rust rejects.
    // Level: unit/component fixture. Source: proposal test-intent "DTO
    // fixture parity".
    const parsed = parseTempHarnessHandle(handleRoundTrip);
    const typed: TempHarnessHandle = parsed;
    const expectedErrors = [
      "UnknownSeed",
      "DatabaseCreateFailed",
      "AppStateInitFailed",
      "RealAgentsInvocationAttempted",
      "FixtureManifestMissing",
    ] satisfies TempHarnessError[];
    // @ts-expect-error "OtherError" is outside the documented TempHarnessError union.
    const rejected: TempHarnessError = "OtherError";

    expect(typed).toEqual(handleRoundTrip);
    expect(TEMP_HARNESS_ERRORS).toEqual(expectedErrors);
    expect(TEMP_HARNESS_ERRORS).toEqual(tempHarnessErrors);
    expect(rejected).toBe("OtherError");

    for (const errorName of tempHarnessErrors) {
      expect(parseTempHarnessError(errorName)).toBe(errorName);
    }

    expect(() => parseTempHarnessError("OtherError")).toThrow("Invalid TempHarnessError");
    expect(() => parseTempHarnessHandle({ ...handleRoundTrip, extra: true })).toThrow(
      "Invalid TempHarnessHandle",
    );
  });

  it("returns the documented React Testing Library RenderResult surface", () => {
    // Risk: the harness wrapper hides or mutates the standard RTL RenderResult.
    // Level: component. Source: proposal test-intent "RenderResult shape".
    const result = renderWithHarness(<span data-testid="rendered-child">mounted</span>, "empty");

    for (const key of renderResultShape.required_keys) {
      expect(result).toHaveProperty(key);
    }

    expect(result.container).toBeInstanceOf(HTMLElement);
    expect(result.baseElement).toBeInstanceOf(HTMLElement);
    expect(result.debug).toEqual(expect.any(Function));
    expect(result.rerender).toEqual(expect.any(Function));
    expect(result.unmount).toEqual(expect.any(Function));
    expect(result.asFragment).toEqual(expect.any(Function));
    expect(result.getByTestId("rendered-child")).toHaveTextContent("mounted");
    expect(result.queryByText("mounted")).toBeInTheDocument();
    expect(renderResultShape.query_examples).toEqual(["getByTestId", "queryByText", "findByRole"]);
  });

  it("mounts the supplied UI under QueryClientProvider and RouterProvider", () => {
    // Risk: the renderer mounts UI outside required app providers. Level:
    // component. Source: proposal test-intent "Provider mounting".
    renderWithHarness(<ProviderProbe />, "minimal-runtime");

    expect(screen.getByTestId(providerMounting.query_client_probe_test_id)).toHaveTextContent(
      providerMounting.combined_ready_text,
    );
    expect(screen.getByTestId(providerMounting.router_probe_test_id)).toHaveTextContent(
      providerMounting.combined_ready_text,
    );
  });

  it("seeds invokeCommand responses from the selected WU-0A-14a seed without real Tauri invoke", async () => {
    // Risk: IPC fixture seeding falls through to the real Tauri invoke path or
    // returns data outside the documented seed. Level: component. Source:
    // proposal test-intent "IPC seeding".
    renderWithHarness(<PingRuntimeProbe />, "minimal-runtime");
    const minimalRuntimeSeed = ipcSeeding.find((seed) => seed.seed_name === "minimal-runtime");

    expect(minimalRuntimeSeed).toBeDefined();
    expect(await screen.findByTestId("ping-runtime-response")).toHaveTextContent(
      `${minimalRuntimeSeed!.responses.ping_runtime.command}:${minimalRuntimeSeed!.responses.ping_runtime.phase}:${minimalRuntimeSeed!.responses.ping_runtime.runtime}`,
    );
    expect(sideEffectSpies.invoke).not.toHaveBeenCalled();
  });

  it("intercepts harness settings IPC so the real agents binary and migration commands are never invoked", async () => {
    // Risk: renderer escapes the fixture seam into real subprocess or migration
    // behavior. Level: component/static boundary. Source: proposal test-intent
    // "Runtime side-effect absence".
    renderWithHarness(<HarnessSettingsProbe />, "minimal-runtime");

    expect(await screen.findByTestId("agent-runner-bin")).toHaveTextContent(
      "product-strategy/contracts/fixtures/wu-0a-02/bin/fake-agents",
    );
    expect(structuralAbsence.phase0a_registered_commands_remain).toEqual([
      "subscribe_workspace_events",
    ]);
    expect(structuralAbsence.renderer_invokes_tauri).toBe(false);
    expect(structuralAbsence.renderer_applies_migrations).toBe(false);
    expect(sideEffectSpies.invoke).not.toHaveBeenCalled();

    for (const forbiddenCommand of structuralAbsence.forbidden_tauri_commands) {
      expect(sideEffectSpies.invoke).not.toHaveBeenCalledWith(
        forbiddenCommand,
        expect.anything(),
      );
    }
    expect(sideEffectSpies.invoke).not.toHaveBeenCalledWith(
      expect.stringContaining(structuralAbsence.forbidden_agents_binary),
      expect.anything(),
    );
  });

  it("throws documented seed failures before returning a mounted RenderResult", async () => {
    // Risk: failed seeds produce a partial mounted tree or hide the WU-0A-14a
    // error taxonomy. Level: component. Source: proposal test-intent "Seed
    // failure".
    for (const failure of seedFailures) {
      expect(() =>
        renderWithHarness(<span data-testid={`failed-${failure.seed_name}`}>mounted</span>, failure.seed_name),
      ).toThrow(TempHarnessErrorFailure);

      try {
        renderWithHarness(<span data-testid={`failed-${failure.seed_name}`}>mounted</span>, failure.seed_name);
      } catch (error) {
        expect(error).toMatchObject({
          kind: failure.expected_error,
          seedName: failure.seed_name,
        });
      }

      await waitFor(() => {
        expect(screen.queryByTestId(`failed-${failure.seed_name}`)).not.toBeInTheDocument();
      });
    }
  });

  it("keeps every WU-0A-14a happy seed renderable through a parsed handle", () => {
    // Risk: WU-0A-14b fixture coverage drifts from the WU-0A-14a happy seed
    // registry. Level: component fixture. Source: proposal test-intent "DTO
    // fixture parity" and "RenderResult shape".
    for (const seed of canonicalSeedRenders) {
      expect(parseTempHarnessHandle(seed.handle).fixture_manifest_id).toContain(seed.seed_name);
      expect(() => renderWithHarness(<span data-testid={seed.seed_name}>mounted</span>, seed.seed_name)).not.toThrow();
      expect(screen.getByTestId(seed.seed_name)).toHaveTextContent("mounted");
      cleanup();
    }
  });
});
