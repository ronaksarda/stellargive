import { describe, it, expect, vi } from "vitest";

// Mock rpc.Server so the module-level `new rpc.Server(RPC_URL)` doesn't throw in tests
vi.mock("@stellar/stellar-sdk", async (importActual) => {
  const actual = await importActual<typeof import("@stellar/stellar-sdk")>();
  return {
    ...actual,
    rpc: { ...actual.rpc, Server: vi.fn(() => ({})) },
  };
});

import { fromStroops, toStroops } from "@/lib/soroban";

describe("fromStroops", () => {
  it("returns '0' for 0 stroops", () => {
    expect(fromStroops(0n)).toBe("0");
    expect(fromStroops(0)).toBe("0");
    expect(fromStroops("0")).toBe("0");
  });

  it("returns '0.0000001' for 1 stroop (minimum unit)", () => {
    expect(fromStroops(1n)).toBe("0.0000001");
  });

  it("displays 500_000 stroops as '0.05', not '.05'", () => {
    const result = fromStroops(500_000n);
    expect(result).toBe("0.05");
    expect(result.startsWith("0.")).toBe(true);
  });

  it("trims trailing zeros from decimal part", () => {
    expect(fromStroops(5_000_000n)).toBe("0.5");
    expect(fromStroops(10_100_000n)).toBe("1.01");
    expect(fromStroops(15_000_000n)).toBe("1.5");
  });

  it("returns whole number string when no fractional part", () => {
    expect(fromStroops(10_000_000n)).toBe("1");
    expect(fromStroops(100_000_000n)).toBe("10");
  });

  it("handles large value: 1_000_000_000 stroops = 100 XLM", () => {
    expect(fromStroops(1_000_000_000n)).toBe("100");
  });

  it("preserves all 7 significant decimal places when needed", () => {
    expect(fromStroops(1_234_567n)).toBe("0.1234567");
  });

  it("handles fractional values at various sub-decimal positions", () => {
    expect(fromStroops(50_000n)).toBe("0.005");
    expect(fromStroops(5_000n)).toBe("0.0005");
    expect(fromStroops(500n)).toBe("0.00005");
    expect(fromStroops(50n)).toBe("0.000005");
    expect(fromStroops(5n)).toBe("0.0000005");
  });

  it("accepts string and number inputs without floating-point artifacts", () => {
    expect(fromStroops("10000000")).toBe("1");
    expect(fromStroops(10_000_000)).toBe("1");
    expect(fromStroops("500000")).toBe("0.05");
  });
});

describe("toStroops", () => {
  it("converts whole XLM amounts", () => {
    expect(toStroops("1")).toBe(10_000_000n);
    expect(toStroops("100")).toBe(1_000_000_000n);
  });

  it("converts fractional XLM amounts", () => {
    expect(toStroops("0.5")).toBe(5_000_000n);
    expect(toStroops("0.05")).toBe(500_000n);
    expect(toStroops("0.0000001")).toBe(1n);
  });

  it("accepts number input", () => {
    expect(toStroops(1)).toBe(10_000_000n);
  });

  it("truncates decimals beyond 7 places", () => {
    expect(toStroops("1.12345678")).toBe(toStroops("1.1234567"));
  });

  it("round-trips with fromStroops", () => {
    const amounts = ["0.0000001", "0.05", "0.5", "1", "100", "1234.5678901"];
    for (const amount of amounts) {
      expect(fromStroops(toStroops(amount))).toBe(amount);
    }
  });
});
