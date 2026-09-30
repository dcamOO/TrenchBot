import React from 'react';
import { Download, FileText, CheckCircle2, XCircle } from 'lucide-react';

export const PnlHistoryTable = ({ trades, onExportCsv }) => {
  return (
    <div className="bg-slate-900/90 border border-slate-800 rounded-xl overflow-hidden shadow-sm">
      <div className="px-5 py-4 border-b border-slate-800 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
        <div>
          <h2 className="text-base font-semibold text-slate-100 flex items-center space-x-2">
            <FileText className="w-4 h-4 text-indigo-400" />
            <span>Histórico de Trades Fechados & PnL</span>
            <span className="text-xs bg-slate-800 text-slate-300 px-2 py-0.5 rounded-full">{trades.length}</span>
          </h2>
          <p className="text-xs text-slate-400 mt-0.5">Registro auditável para análise de rendimentos e declaração de impostos</p>
        </div>
        <button
          onClick={onExportCsv}
          disabled={trades.length === 0}
          className="flex items-center space-x-2 text-xs font-medium bg-indigo-600/90 hover:bg-indigo-600 disabled:opacity-40 disabled:hover:bg-indigo-600/90 text-white px-3.5 py-1.5 rounded-lg border border-indigo-500/30 transition shadow-sm"
        >
          <Download className="w-3.5 h-3.5" />
          <span>Exportar Relatório CSV (US8)</span>
        </button>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-left text-xs">
          <thead className="bg-slate-950/60 text-slate-400 uppercase tracking-wider font-semibold border-b border-slate-800">
            <tr>
              <th className="px-5 py-3">Token</th>
              <th className="px-4 py-3">Abertura / Fechamento</th>
              <th className="px-4 py-3">Investido</th>
              <th className="px-4 py-3">Retorno</th>
              <th className="px-4 py-3">PnL Líquido</th>
              <th className="px-4 py-3 text-right">Motivo Encerramento</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800/60 text-slate-200">
            {trades.length === 0 ? (
              <tr>
                <td colSpan="6" className="px-5 py-8 text-center text-slate-500">
                  Nenhum trade encerrado ainda. O histórico aparecerá aqui conforme ordens forem finalizadas.
                </td>
              </tr>
            ) : (
              trades.map((t, idx) => {
                const isProfit = t.pnlLamports >= 0;
                return (
                  <tr key={`${t.mint}-${idx}`} className="hover:bg-slate-800/40 transition">
                    <td className="px-5 py-3 font-mono font-medium text-slate-100">
                      <span className="text-purple-300 font-bold">{t.symbol || 'TOKEN'}</span>
                      <span className="text-slate-400 text-[11px] block">{t.mint.slice(0, 4)}...{t.mint.slice(-4)}</span>
                    </td>
                    <td className="px-4 py-3 text-slate-400 font-mono text-[11px]">
                      <div>Entrada: {new Date(t.openedAt * 1000).toLocaleTimeString()}</div>
                      <div>Saída: {new Date(t.closedAt * 1000).toLocaleTimeString()}</div>
                    </td>
                    <td className="px-4 py-3 font-mono">{(t.costLamports / 1e9).toFixed(3)} SOL</td>
                    <td className="px-4 py-3 font-mono">{(t.receivedLamports / 1e9).toFixed(3)} SOL</td>
                    <td className="px-4 py-3 font-mono font-semibold">
                      <span className={isProfit ? 'text-emerald-400' : 'text-rose-400'}>
                        {isProfit ? '+' : ''}{t.pnlSol.toFixed(4)} SOL
                        <span className="ml-1 text-[11px] font-normal">({isProfit ? '+' : ''}{t.pnlPercent.toFixed(1)}%)</span>
                      </span>
                    </td>
                    <td className="px-4 py-3 text-right">
                      <span className={`inline-flex items-center space-x-1 px-2 py-0.5 rounded text-[11px] font-medium ${
                        t.reason === 'take_profit'
                          ? 'bg-emerald-950 text-emerald-300 border border-emerald-800'
                          : 'bg-rose-950 text-rose-300 border border-rose-800'
                      }`}>
                        {t.reason === 'take_profit' ? (
                          <>
                            <CheckCircle2 className="w-3 h-3 text-emerald-400" />
                            <span>Take-Profit</span>
                          </>
                        ) : (
                          <>
                            <XCircle className="w-3 h-3 text-rose-400" />
                            <span>Stop-Loss</span>
                          </>
                        )}
                      </span>
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
