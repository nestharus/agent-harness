import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { AppShell } from "../App";

describe("Phase 0A shell scaffold", () => {
  it("renders the inert application root without external runtime dependencies", async () => {
    // Risk: shell route accidentally depends on GraphStore, provider credentials,
    // or agents. Level: component. Source: proposal test-intent "React shell smoke".
    render(<AppShell />);

    expect(await screen.findByTestId("app-root")).toBeInTheDocument();
    expect(await screen.findByRole("heading", { name: "Agent Harness" })).toBeVisible();
  });
});
