import { useState, useEffect, useRef } from 'react';

const MOCK_STREAM = [
  { mint: '5K9aXbU5vK14q8sW93eRtYu23pLm', symbol: 'PEPE-PUMP', creator: '3aB8...9kLm', launchCount: 2, maxAllowed: 3, athProof: '$2.4M ATH', priceSol: 0.000040, targetTp: 0.000060, targetSl: 0.000032 },
  { mint: '8vL2vR8sP14m9tY82qWeYu34nJk', symbol: 'SOL-CHAD', creator: '9fD2...1xQz', launchCount: 1, maxAllowed: 3, athProof: '$1.8M ATH', priceSol: 0.000100, targetTp: 0.000150, targetSl: 0.000080 },
  { mint: '3xP3kL14m8sW92eRtYu34qLm21p', symbol: 'MOON-AI', creator: '4kM1...8zPx', launchCount: 1, maxAllowed: 3, athProof: '$3.1M ATH', priceSol: 0.000080, targetTp: 0.000120, targetSl: 0.000064 },
];

export function useSimulationFeed(initialData) {
  const [isSimulating, setIsSimulating] = useState(false);
  const [candidates, setCandidates] = useState(initialData.candidates);
  const [positions, setPositions] = useState(initialData.positions);
  const [trades, setTrades] = useState(initialData.trades);
  const stepRef = useRef(0);

  const toggleSimulation = () => setIsSimulating((prev) => !prev);

  useEffect(() => {
    if (!isSimulating) return;

    const interval = setInterval(() => {
      const streamItem = MOCK_STREAM[stepRef.current % MOCK_STREAM.length];
      const stepPhase = stepRef.current % 3;

      if (stepPhase === 0) {
        // Fase 1: Descoberta no WebSocket
        const newCandidate = {
          ...streamItem,
          ageSeconds: 5,
          aiStatus: 'pending',
        };
        setCandidates((prev) => [newCandidate, ...prev.slice(0, 3)]);
      } else if (stepPhase === 1) {
        // Fase 2: Gemini aprova e Motor compra
        setCandidates((prev) =>
          prev.map((c) => (c.mint === streamItem.mint ? { ...c, aiStatus: 'safe' } : c))
        );
        const newPos = {
          mint: streamItem.mint,
          symbol: streamItem.symbol,
          quantity: Math.round(100000000 / (streamItem.priceSol * 1e9)),
          costLamports: 100000000,
          currentPriceSol: streamItem.priceSol,
          unrealizedPnlSol: 0.0,
          unrealizedPnlPercent: 0.0,
          tpPercent: 50.0,
          slPercent: 20.0,
        };
        setPositions((prev) => [newPos, ...prev]);
      } else if (stepPhase === 2) {
        // Fase 3: Preço sobe e atinge Take-Profit (+50%)
        setPositions((prev) => {
          const match = prev.find((p) => p.mint === streamItem.mint);
          if (match) {
            const receivedLamports = 150000000;
            const newTrade = {
              mint: match.mint,
              symbol: match.symbol,
              openedAt: Math.floor(Date.now() / 1000) - 30,
              closedAt: Math.floor(Date.now() / 1000),
              costLamports: match.costLamports,
              receivedLamports,
              pnlLamports: 50000000,
              pnlSol: 0.05,
              pnlPercent: 50.0,
              reason: 'take_profit',
            };
            setTrades((t) => [newTrade, ...t]);
            return prev.filter((p) => p.mint !== streamItem.mint);
          }
          return prev;
        });
      }

      stepRef.current += 1;
    }, 3500);

    return () => clearInterval(interval);
  }, [isSimulating]);

  return {
    isSimulating,
    toggleSimulation,
    candidates,
    positions,
    trades,
    setPositions,
    setTrades,
  };
}
