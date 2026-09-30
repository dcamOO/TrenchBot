import React from 'react';
import { TrendingUp, TrendingDown, Target, Eye, DollarSign } from 'lucide-react';

export const MetricsCards = ({ metrics }) => {
  const { totalPnlSol, totalPnlPercent, winRate, activeCount, monitoredCount } = metrics;
  const isProfitable = totalPnlSol >= 0;

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      {/* Total Realized PnL */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-xl p-4 shadow-sm">
        <div className="flex items-center justify-between text-slate-400 mb-2">
          <span className="text-xs uppercase font-semibold tracking-wider">PnL Realizado</span>
          {isProfitable ? (
            <TrendingUp className="w-4 h-4 text-emerald-400" />
          ) : (
            <TrendingDown className="w-4 h-4 text-rose-400" />
          )}
        </div>
        <div className="flex items-baseline space-x-2">
          <span className={`text-2xl font-bold font-mono ${isProfitable ? 'text-emerald-400' : 'text-rose-400'}`}>
            {totalPnlSol >= 0 ? '+' : ''}{totalPnlSol.toFixed(4)} SOL
          </span>
          <span className={`text-xs font-semibold ${isProfitable ? 'text-emerald-500' : 'text-rose-500'}`}>
            ({totalPnlPercent >= 0 ? '+' : ''}{totalPnlPercent.toFixed(1)}%)
          </span>
        </div>
      </div>

      {/* Win Rate */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-xl p-4 shadow-sm">
        <div className="flex items-center justify-between text-slate-400 mb-2">
          <span className="text-xs uppercase font-semibold tracking-wider">Taxa de Acerto</span>
          <Target className="w-4 h-4 text-indigo-400" />
        </div>
        <div className="flex items-baseline space-x-2">
          <span className="text-2xl font-bold font-mono text-slate-100">{winRate.toFixed(1)}%</span>
          <span className="text-xs text-slate-400">Target TP: 50%</span>
        </div>
      </div>

      {/* Active Positions */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-xl p-4 shadow-sm">
        <div className="flex items-center justify-between text-slate-400 mb-2">
          <span className="text-xs uppercase font-semibold tracking-wider">Posições Abertas</span>
          <DollarSign className="w-4 h-4 text-amber-400" />
        </div>
        <div className="flex items-baseline space-x-2">
          <span className="text-2xl font-bold font-mono text-slate-100">{activeCount}</span>
          <span className="text-xs text-slate-400">Em monitoramento contínuo</span>
        </div>
      </div>

      {/* Monitored Tokens */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-xl p-4 shadow-sm">
        <div className="flex items-center justify-between text-slate-400 mb-2">
          <span className="text-xs uppercase font-semibold tracking-wider">Tokens em Triagem</span>
          <Eye className="w-4 h-4 text-purple-400" />
        </div>
        <div className="flex items-baseline space-x-2">
          <span className="text-2xl font-bold font-mono text-purple-300">{monitoredCount}</span>
          <span className="text-xs text-purple-400/80">Filtro X/Y & IA</span>
        </div>
      </div>
    </div>
  );
};
