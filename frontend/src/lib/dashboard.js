export const PNL_CSV_HEADER =
  'mint,opened_at,closed_at,cost_lamports,received_lamports,pnl_lamports,pnl_sol,pnl_percent,reason';

export function computeMetrics(trades) {
  const totalPnlSol = trades.reduce((acc, t) => acc + t.pnlSol, 0);
  const totalCost = trades.reduce((acc, t) => acc + t.costLamports, 0);
  const totalPnlPercent = totalCost > 0 ? (trades.reduce((acc, t) => acc + t.pnlLamports, 0) / totalCost) * 100 : 0;
  const wins = trades.filter((t) => t.pnlLamports > 0).length;
  const winRate = trades.length > 0 ? (wins / trades.length) * 100 : 0;
  return { totalPnlSol, totalPnlPercent, winRate };
}

export function buildPnlCsv(trades) {
  let csv = `${PNL_CSV_HEADER}\n`;
  trades.forEach((t) => {
    csv += `${t.mint},${t.openedAt},${t.closedAt},${t.costLamports},${t.receivedLamports},${t.pnlLamports},${t.pnlSol.toFixed(6)},${t.pnlPercent.toFixed(2)},${t.reason}\n`;
  });
  return csv;
}

export function closePosition(pos, nowSeconds) {
  const receivedLamports = Math.round(pos.quantity * pos.currentPriceSol * 1e9);
  const pnlLamports = receivedLamports - pos.costLamports;
  return {
    mint: pos.mint,
    symbol: pos.symbol,
    openedAt: nowSeconds - 60,
    closedAt: nowSeconds,
    costLamports: pos.costLamports,
    receivedLamports,
    pnlLamports,
    pnlSol: pnlLamports / 1e9,
    pnlPercent: (pnlLamports / pos.costLamports) * 100,
    reason: 'manual',
  };
}
