import React from 'react';
import { useTrenchWallet } from '../context/WalletContext';
import { Wallet, ShieldCheck, Download, Activity, LogOut } from 'lucide-react';

export const Header = ({ onExportPnl, hasTrades }) => {
  const { isConnected, address, balanceSol, connectPaperWallet, disconnectWallet } = useTrenchWallet();

  const shortenAddress = (addr) => {
    if (!addr) return '';
    return `${addr.slice(0, 4)}...${addr.slice(-4)}`;
  };

  return (
    <header className="border-b border-slate-800 bg-slate-900/80 backdrop-blur px-6 py-4 sticky top-0 z-50">
      <div className="max-w-7xl mx-auto flex items-center justify-between">
        {/* Branding & Status */}
        <div className="flex items-center space-x-4">
          <div className="flex items-center space-x-2">
            <span className="text-2xl">🚀</span>
            <span className="text-xl font-bold bg-gradient-to-r from-purple-400 to-indigo-300 bg-clip-text text-transparent">
              TrenchBot
            </span>
          </div>
          <div className="flex items-center space-x-2 text-xs bg-emerald-950/60 border border-emerald-500/30 text-emerald-400 px-3 py-1 rounded-full">
            <Activity className="w-3.5 h-3.5 animate-pulse" />
            <span className="font-medium">PumpPortal WS Ativo</span>
          </div>
          <div className="hidden sm:flex items-center space-x-1 text-xs bg-purple-950/50 border border-purple-500/30 text-purple-300 px-2.5 py-1 rounded-full">
            <ShieldCheck className="w-3.5 h-3.5 text-purple-400" />
            <span>Gemini AI Guard</span>
          </div>
        </div>

        {/* Actions & Wallet */}
        <div className="flex items-center space-x-3">
          {onExportPnl && (
            <button
              onClick={onExportPnl}
              disabled={!hasTrades}
              className="flex items-center space-x-1.5 text-xs bg-slate-800 hover:bg-slate-700 disabled:opacity-40 disabled:hover:bg-slate-800 text-slate-200 px-3 py-2 rounded-lg border border-slate-700 transition"
              title="Exportar Relatório PnL (CSV)"
            >
              <Download className="w-3.5 h-3.5" />
              <span>Exportar PnL</span>
            </button>
          )}

          {isConnected ? (
            <div className="flex items-center space-x-3 bg-slate-800/90 border border-slate-700 px-3 py-1.5 rounded-lg">
              <div className="flex flex-col text-right">
                <span className="text-xs text-slate-400 font-mono">{shortenAddress(address)}</span>
                <span className="text-xs font-semibold text-emerald-400">{balanceSol.toFixed(3)} SOL</span>
              </div>
              <button
                onClick={disconnectWallet}
                className="text-slate-400 hover:text-red-400 p-1 transition"
                title="Desconectar Carteira"
              >
                <LogOut className="w-4 h-4" />
              </button>
            </div>
          ) : (
            <button
              onClick={connectPaperWallet}
              className="flex items-center space-x-2 text-xs font-semibold bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-2 rounded-lg shadow-lg shadow-indigo-600/20 transition"
            >
              <Wallet className="w-4 h-4" />
              <span>Conectar Phantom</span>
            </button>
          )}
        </div>
      </div>
    </header>
  );
};
