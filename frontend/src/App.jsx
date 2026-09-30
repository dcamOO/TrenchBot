import React, { useState } from 'react';
import { SolanaWalletProvider } from './context/WalletContext';
import { Header } from './components/Header';
import { MetricsCards } from './components/MetricsCards';
import { MonitoredTokensTable } from './components/MonitoredTokensTable';
import { ActivePositionsTable } from './components/ActivePositionsTable';
import { PnlHistoryTable } from './components/PnlHistoryTable';

const INITIAL_CANDIDATES = [
  { mint: '7M9XbU5vK14q8sW93eRtYu23pLm', symbol: 'PEPE-PUMP', creator: '3aB8...9kLm', launchCount: 2, maxAllowed: 3, athProof: '$2.4M ATH', priceSol: 0.000045, ageSeconds: 14, aiStatus: 'safe' },
  { mint: '4kL2vR8sP14m9tY82qWeYu34nJk', symbol: 'MOON-DOG', creator: '9fD2...1xQz', launchCount: 1, maxAllowed: 3, athProof: '$1.1M ATH', priceSol: 0.000120, ageSeconds: 42, aiStatus: 'pending' },
];

const INITIAL_POSITIONS = [
  { mint: '9zP3kL14m8sW92eRtYu34qLm21p', symbol: 'SOL-CAT', quantity: 2500000, costLamports: 100000000, currentPriceSol: 0.000052, unrealizedPnlSol: 0.0300, unrealizedPnlPercent: 30.0, tpPercent: 50.0, slPercent: 20.0 },
];

const INITIAL_TRADES = [
  { mint: 'DEMO-A1b2C3d4E5f6G7h8J9k0L1m', symbol: 'CHAD-SOL', openedAt: 1727690000, closedAt: 1727690300, costLamports: 100000000, receivedLamports: 150000000, pnlLamports: 50000000, pnlSol: 0.050, pnlPercent: 50.0, reason: 'take_profit' },
  { mint: 'DEMO-B2c3D4e5F6g7H8j9K0l1M2n', symbol: 'WOJAK', openedAt: 1727690400, closedAt: 1727690550, costLamports: 100000000, receivedLamports: 80000000, pnlLamports: -20000000, pnlSol: -0.020, pnlPercent: -20.0, reason: 'stop_loss' },
];

function Dashboard() {
  const [candidates] = useState(INITIAL_CANDIDATES);
  const [positions, setPositions] = useState(INITIAL_POSITIONS);
  const [trades, setTrades] = useState(INITIAL_TRADES);

  const totalPnlSol = trades.reduce((acc, t) => acc + t.pnlSol, 0);
  const totalCost = trades.reduce((acc, t) => acc + t.costLamports, 0);
  const totalPnlPercent = totalCost > 0 ? (trades.reduce((acc, t) => acc + t.pnlLamports, 0) / totalCost) * 100 : 0;
  const wins = trades.filter((t) => t.pnlLamports > 0).length;
  const winRate = trades.length > 0 ? (wins / trades.length) * 100 : 0;

  const exportCsv = () => {
    let csv = 'mint,opened_at,closed_at,cost_lamports,received_lamports,pnl_lamports,pnl_sol,pnl_percent,reason\n';
    trades.forEach((t) => {
      csv += `${t.mint},${t.openedAt},${t.closedAt},${t.costLamports},${t.receivedLamports},${t.pnlLamports},${t.pnlSol.toFixed(6)},${t.pnlPercent.toFixed(2)},${t.reason}\n`;
    });
    const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.setAttribute('download', `trenchbot-pnl-${Date.now()}.csv`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  const handleManualExit = (mint) => {
    const pos = positions.find((p) => p.mint === mint);
    if (!pos) return;
    const receivedLamports = Math.round(pos.quantity * pos.currentPriceSol * 1e9);
    const pnlLamports = receivedLamports - pos.costLamports;
    const newTrade = {
      mint: pos.mint,
      symbol: pos.symbol,
      openedAt: Math.floor(Date.now() / 1000) - 60,
      closedAt: Math.floor(Date.now() / 1000),
      costLamports: pos.costLamports,
      receivedLamports,
      pnlLamports,
      pnlSol: pnlLamports / 1e9,
      pnlPercent: (pnlLamports / pos.costLamports) * 100,
      reason: 'manual',
    };
    setTrades([newTrade, ...trades]);
    setPositions(positions.filter((p) => p.mint !== mint));
  };

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col">
      <Header onExportPnl={exportCsv} hasTrades={trades.length > 0} />
      <main className="flex-1 max-w-7xl w-full mx-auto px-6 py-6">
        <MetricsCards metrics={{ totalPnlSol, totalPnlPercent, winRate, activeCount: positions.length, monitoredCount: candidates.length }} />
        <MonitoredTokensTable candidates={candidates} />
        <ActivePositionsTable positions={positions} onManualExit={handleManualExit} />
        <PnlHistoryTable trades={trades} onExportCsv={exportCsv} />
      </main>
    </div>
  );
}

export default function App() {
  return (
    <SolanaWalletProvider>
      <Dashboard />
    </SolanaWalletProvider>
  );
}
