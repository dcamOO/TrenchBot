import React from 'react';
import { ShieldAlert, ShieldCheck, Clock, ExternalLink } from 'lucide-react';

export const MonitoredTokensTable = ({ candidates }) => {
  const getAiBadge = (status) => {
    switch (status) {
      case 'safe':
        return (
          <span className="inline-flex items-center space-x-1 px-2 py-0.5 rounded text-xs font-medium bg-emerald-950 text-emerald-300 border border-emerald-800">
            <ShieldCheck className="w-3 h-3 text-emerald-400" />
            <span>Seguro (Gemini)</span>
          </span>
        );
      case 'risk':
        return (
          <span className="inline-flex items-center space-x-1 px-2 py-0.5 rounded text-xs font-medium bg-rose-950 text-rose-300 border border-rose-800">
            <ShieldAlert className="w-3 h-3 text-rose-400" />
            <span>Risco Scam</span>
          </span>
        );
      default:
        return (
          <span className="inline-flex items-center space-x-1 px-2 py-0.5 rounded text-xs font-medium bg-amber-950 text-amber-300 border border-amber-800">
            <Clock className="w-3 h-3 text-amber-400 animate-spin" />
            <span>Analisando IA</span>
          </span>
        );
    }
  };

  return (
    <div className="bg-slate-900/90 border border-slate-800 rounded-xl overflow-hidden shadow-sm mb-6">
      <div className="px-5 py-4 border-b border-slate-800 flex justify-between items-center">
        <div>
          <h2 className="text-base font-semibold text-slate-100 flex items-center space-x-2">
            <span>Oportunidades em Monitoramento & Análise de IA</span>
            <span className="text-xs bg-slate-800 text-slate-300 px-2 py-0.5 rounded-full">{candidates.length}</span>
          </h2>
          <p className="text-xs text-slate-400 mt-0.5">Triagem contínua via WebSocket pump.fun e auditoria com Google Gemini</p>
        </div>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-left text-xs">
          <thead className="bg-slate-950/60 text-slate-400 uppercase tracking-wider font-semibold border-b border-slate-800">
            <tr>
              <th className="px-5 py-3">Token / Mint</th>
              <th className="px-4 py-3">Criador & Lançamentos (X)</th>
              <th className="px-4 py-3">Histórico ATH (Y)</th>
              <th className="px-4 py-3">Preço SOL</th>
              <th className="px-4 py-3">Idade</th>
              <th className="px-4 py-3">Parecer IA (Gemini)</th>
              <th className="px-4 py-3 text-right">Ação</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800/60 text-slate-200">
            {candidates.length === 0 ? (
              <tr>
                <td colSpan="7" className="px-5 py-8 text-center text-slate-500">
                  Aguardando novos lançamentos no feed pump.fun...
                </td>
              </tr>
            ) : (
              candidates.map((c) => (
                <tr key={c.mint} className="hover:bg-slate-800/40 transition">
                  <td className="px-5 py-3 font-mono font-medium text-slate-100">
                    <div className="flex items-center space-x-1.5">
                      <span className="text-purple-400 font-bold">{c.symbol || 'MEME'}</span>
                      <span className="text-slate-400 text-[11px]">({c.mint.slice(0, 4)}...{c.mint.slice(-4)})</span>
                    </div>
                  </td>
                  <td className="px-4 py-3">
                    <span className="font-mono text-slate-300">{c.creator.slice(0, 4)}...{c.creator.slice(-4)}</span>
                    <span className="ml-2 px-1.5 py-0.5 rounded text-[10px] bg-slate-800 text-slate-300">
                      {c.launchCount} / {c.maxAllowed} lançados
                    </span>
                  </td>
                  <td className="px-4 py-3 font-mono text-emerald-400">{c.athProof || 'Nenhum'}</td>
                  <td className="px-4 py-3 font-mono">{c.priceSol.toFixed(6)} SOL</td>
                  <td className="px-4 py-3 text-slate-400">{c.ageSeconds}s atrás</td>
                  <td className="px-4 py-3">{getAiBadge(c.aiStatus)}</td>
                  <td className="px-4 py-3 text-right">
                    <a
                      href={`https://pump.fun/${c.mint}`}
                      target="_blank"
                      rel="noreferrer"
                      className="inline-flex items-center text-slate-400 hover:text-purple-300 transition"
                    >
                      <ExternalLink className="w-3.5 h-3.5" />
                    </a>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};
