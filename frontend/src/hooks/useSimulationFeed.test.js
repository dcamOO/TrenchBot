import { act, renderHook } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { useSimulationFeed } from './useSimulationFeed';

const EMPTY = { candidates: [], positions: [], trades: [] };
const TICK = 3500;

function startFeed(initial = EMPTY) {
  const hook = renderHook(() => useSimulationFeed(initial));
  act(() => hook.result.current.toggleSimulation());
  return hook;
}

// One act per interval, so React renders between ticks as it does in the browser.
const tick = (n = 1) => {
  for (let i = 0; i < n; i += 1) act(() => vi.advanceTimersByTime(TICK));
};

describe('useSimulationFeed', () => {
  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());

  it('starts paused with the initial data', () => {
    const initial = { candidates: [{ mint: 'X' }], positions: [], trades: [] };
    const { result } = renderHook(() => useSimulationFeed(initial));
    expect(result.current.isSimulating).toBe(false);
    expect(result.current.candidates).toEqual(initial.candidates);
  });

  it('discovers a pending candidate on the first tick', () => {
    const { result } = startFeed();
    tick();
    expect(result.current.candidates).toHaveLength(1);
    expect(result.current.candidates[0]).toMatchObject({ symbol: 'PEPE-PUMP', aiStatus: 'pending', ageSeconds: 5 });
  });

  it('approves the candidate and opens a 0.1 SOL position on the second tick', () => {
    const { result } = startFeed();
    tick(2);
    expect(result.current.candidates[0].aiStatus).toBe('safe');
    expect(result.current.positions).toHaveLength(1);
    expect(result.current.positions[0]).toMatchObject({ costLamports: 100000000, quantity: 2500, tpPercent: 50, slPercent: 20 });
  });

  it('closes the position by take profit on the third tick', () => {
    const { result } = startFeed();
    tick(3);
    expect(result.current.positions).toHaveLength(0);
    expect(result.current.trades).toHaveLength(1);
    expect(result.current.trades[0]).toMatchObject({ reason: 'take_profit', pnlLamports: 50000000, pnlPercent: 50 });
  });

  it('records a single trade per take profit under strict mode', () => {
    const { result } = renderHook(() => useSimulationFeed(EMPTY), { reactStrictMode: true });
    act(() => result.current.toggleSimulation());
    tick(3);
    expect(result.current.trades).toHaveLength(1);
  });

  it('moves to the next token only after a full cycle', () => {
    const { result } = startFeed();
    tick(4);
    expect(result.current.candidates.map((c) => c.symbol)).toEqual(['SOL-CHAD', 'PEPE-PUMP']);
    expect(result.current.trades).toHaveLength(1);
  });

  it('keeps at most four candidates', () => {
    const { result } = startFeed();
    tick(15);
    expect(result.current.candidates).toHaveLength(4);
  });

  it('stops producing events when paused', () => {
    const { result } = startFeed();
    tick();
    act(() => result.current.toggleSimulation());
    tick(5);
    expect(result.current.isSimulating).toBe(false);
    expect(result.current.candidates).toHaveLength(1);
    expect(result.current.positions).toHaveLength(0);
  });
});
