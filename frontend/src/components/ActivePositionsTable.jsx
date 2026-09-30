import React from 'react';
import { DollarSign, ShieldAlert, ArrowUpRight, ArrowDownRight } from 'lucide-react';

export const ActivePositionsTable = ({ positions, onManualExit }) => {
  return (
    <div className="bg-slate-900/90 border border-slate-800 rounded-xl overflow-hidden shadow-sm mb-6">
      <div className="px-5 py-4 border-b border-slate-800 flex justify-between items-center">
        <div>
          <h2 className="text-base font-semibold text-slate-100 flex items-center space-x-2">
            <span>Posições Abertas em Execução</span>
            <span className="text-xs bg-slate-800 text-slate-300 px-2 py-0.5 rounded-full">{positions.length}</span>
          </h2>
          <p className="text-xs text-slate-400 mt-0.5">Gerenciamento automático de risco com Stop-Loss e Take-Profit em tempo real</p>
        </div>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-left text-xs">
          <thead className="bg-slate-950/60 text-slate-400 uppercase tracking-wider font-semibold border-b border-slate-800">
            <tr>
              <th className="px-5 py-3">Token</th>
              <th className="px-4 py-3">Quantidade</th>
              <th className="px-4 py-3">Custo Entrada</th>
              <th className="px-4 py-3">Preço Atual</th>
              <th className="px-4 py-3">PnL Flutuante</th>
              <th className="px-4 py-3">Alvos TP / SL</th>
              <th className="px-4 py-3 text-right">Ação</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800/60 text-slate-200">
            {positions.length === 0 ? (
              <tr>
                <td colSpan="7" className="px-5 py-8 text-center text-slate-500">
                  Nenhuma posição aberta no momento. O bot comprará automaticamente novos candidatos aprovados.
                </td>
              </tr>
            ) : (
              positions.map((p) => {
                const isProfit = p.unrealizedPnlSol >= 0;
                return (
                  <tr key={p.mint} className="hover:bg-slate-800/40 transition">
                    <td className="px-5 py-3 font-mono font-medium text-slate-100">
                      <div className="flex items-center space-x-1.5">
                        <span className="text-indigo-400 font-bold">{p.symbol || 'TOKEN'}</span>
                        <span className="text-slate-400 text-[11px]">({p.mint.slice(0, 4)}...{p.mint.slice(-4)})</span>
                      </div>
                    </td>
                    <td className="px-4 py-3 font-mono">{p.quantity.toLocaleString(undefined, { maximumFractionDigits: 2 })}</td>
                    <td className="px-4 py-3 font-mono">{(p.costLamports / 1e9).toFixed(3)} SOL</td>
                    <td className="px-4 py-3 font-mono">{p.currentPriceSol.toFixed(6)} SOL</td>
                    <td className="px-4 py-3 font-mono font-semibold">
                      <div className={`flex items-center space-x-1 ${isProfit ? 'text-emerald-400' : 'text-rose-400'}`}>
                        {isProfit ? <ArrowUpRight className="w-3.5 h-3.5" /> : <ArrowDownRight className="w-3.5 h-3.5" />}
                        <span>{isProfit ? '+' : ''}{p.unrealizedPnlSol.toFixed(4)} SOL</span>
                        <span className="text-[11px] font-normal">({isProfit ? '+' : ''}{p.unrealizedPnlPercent.toFixed(1)}%)</span>
                      </div>
                    </td>
                    <td className="px-4 py-3 text-[11px]">
                      <span className="text-emerald-400 font-mono">TP: +{p.tpPercent}%</span>
                      <span className="text-slate-500 mx-1">|</span>
                      <span className="text-rose-400 font-mono">SL: -{p.slPercent}%</span>
                    </td>
                    <td className="px-4 py-3 text-right">
                      <button
                        onClick={() => onManualExit && onManualExit(p.mint)}
                        className="text-[11px] bg-slate-800 hover:bg-rose-950/80 hover:text-rose-300 text-slate-300 px-2.5 py-1 rounded border border-slate-700 hover:border-rose-800/60 transition"
                      >
                        Fechar
                      </button>
                    </td>
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};
