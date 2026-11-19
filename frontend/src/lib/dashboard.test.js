import { describe, expect, it } from 'vitest';
import { PNL_CSV_HEADER, buildPnlCsv, closePosition, computeMetrics } from './dashboard';

const win = { mint: 'WIN', openedAt: 100, closedAt: 200, costLamports: 100000000, receivedLamports: 150000000, pnlLamports: 50000000, pnlSol: 0.05, pnlPercent: 50, reason: 'take_profit' };
const loss = { mint: 'LOSS', openedAt: 300, closedAt: 400, costLamports: 100000000, receivedLamports: 80000000, pnlLamports: -20000000, pnlSol: -0.02, pnlPercent: -20, reason: 'stop_loss' };

describe('computeMetrics', () => {
  it('returns zeros without trades', () => {
    expect(computeMetrics([])).toEqual({ totalPnlSol: 0, totalPnlPercent: 0, winRate: 0 });
  });

  it('aggregates pnl over total cost and counts wins', () => {
    const m = computeMetrics([win, loss]);
    expect(m.totalPnlSol).toBeCloseTo(0.03);
    expect(m.totalPnlPercent).toBeCloseTo(15);
    expect(m.winRate).toBe(50);
  });

  it('does not count breakeven trades as wins', () => {
    const even = { ...win, pnlLamports: 0, pnlSol: 0 };
    expect(computeMetrics([even]).winRate).toBe(0);
  });
});

describe('buildPnlCsv', () => {
  it('uses the same header as the backend export', () => {
    expect(PNL_CSV_HEADER).toBe(
      'mint,opened_at,closed_at,cost_lamports,received_lamports,pnl_lamports,pnl_sol,pnl_percent,reason'
    );
    expect(buildPnlCsv([])).toBe(`${PNL_CSV_HEADER}\n`);
  });

  it('formats one row per trade with fixed decimals', () => {
    const lines = buildPnlCsv([win, loss]).trim().split('\n');
    expect(lines).toHaveLength(3);
    expect(lines[1]).toBe('WIN,100,200,100000000,150000000,50000000,0.050000,50.00,take_profit');
    expect(lines[2]).toBe('LOSS,300,400,100000000,80000000,-20000000,-0.020000,-20.00,stop_loss');
  });
});

describe('closePosition', () => {
  const pos = { mint: 'POS', symbol: 'CAT', quantity: 2500, costLamports: 100000000, currentPriceSol: 0.000052 };

  it('realizes pnl at the current price', () => {
    const trade = closePosition(pos, 5000);
    expect(trade.receivedLamports).toBe(130000000);
    expect(trade.pnlLamports).toBe(30000000);
    expect(trade.pnlSol).toBeCloseTo(0.03);
    expect(trade.pnlPercent).toBeCloseTo(30);
    expect(trade.reason).toBe('manual');
  });

  it('stamps open and close times from the given clock', () => {
    const trade = closePosition(pos, 5000);
    expect(trade.closedAt).toBe(5000);
    expect(trade.openedAt).toBe(4940);
  });
});
