import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { MonitoredTokensTable } from './MonitoredTokensTable';
import { ActivePositionsTable } from './ActivePositionsTable';

const candidate = { mint: '7M9XbU5vK14q8sW93eRtYu23pLm', symbol: 'PEPE-PUMP', creator: '3aB8...9kLm', launchCount: 2, maxAllowed: 3, athProof: '$2.4M ATH', priceSol: 0.000045, ageSeconds: 14, aiStatus: 'safe' };
const position = { mint: '9zP3kL14m8sW92eRtYu34qLm21p', symbol: 'SOL-CAT', quantity: 2500, costLamports: 100000000, currentPriceSol: 0.000052, unrealizedPnlSol: 0.03, unrealizedPnlPercent: 30, tpPercent: 50, slPercent: 20 };

describe('MonitoredTokensTable', () => {
  it('shows a waiting message without candidates', () => {
    render(<MonitoredTokensTable candidates={[]} />);
    expect(screen.getByText(/aguardando novos lançamentos/i)).toBeInTheDocument();
  });

  it('renders creator limits ath proof price and pump.fun link', () => {
    render(<MonitoredTokensTable candidates={[candidate]} />);
    const row = screen.getByText('PEPE-PUMP').closest('tr');
    expect(within(row).getByText('2 / 3 lançados')).toBeInTheDocument();
    expect(within(row).getByText('$2.4M ATH')).toBeInTheDocument();
    expect(within(row).getByText('0.000045 SOL')).toBeInTheDocument();
    expect(within(row).getByText('14s atrás')).toBeInTheDocument();
    expect(within(row).getByRole('link')).toHaveAttribute('href', `https://pump.fun/${candidate.mint}`);
  });

  it.each([
    ['safe', 'Seguro (Gemini)'],
    ['risk', 'Risco Scam'],
    ['pending', 'Analisando IA'],
  ])('maps ai status %s to its badge', (aiStatus, label) => {
    render(<MonitoredTokensTable candidates={[{ ...candidate, aiStatus }]} />);
    expect(screen.getByText(label)).toBeInTheDocument();
  });
});

describe('ActivePositionsTable', () => {
  it('shows an empty state without positions', () => {
    render(<ActivePositionsTable positions={[]} />);
    expect(screen.getByText(/nenhuma posição aberta/i)).toBeInTheDocument();
  });

  it('renders cost unrealized pnl and exit targets', () => {
    render(<ActivePositionsTable positions={[position]} />);
    const row = screen.getByText('SOL-CAT').closest('tr');
    expect(within(row).getByText('0.100 SOL')).toBeInTheDocument();
    expect(within(row).getByText('+0.0300 SOL')).toBeInTheDocument();
    expect(within(row).getByText('TP: +50%')).toBeInTheDocument();
    expect(within(row).getByText('SL: -20%')).toBeInTheDocument();
  });

  it('shows losses without a plus sign', () => {
    render(<ActivePositionsTable positions={[{ ...position, unrealizedPnlSol: -0.01, unrealizedPnlPercent: -10 }]} />);
    expect(screen.getByText('-0.0100 SOL')).toBeInTheDocument();
    expect(screen.getByText('(-10.0%)')).toBeInTheDocument();
  });

  it('requests a manual exit for the clicked position', async () => {
    const onManualExit = vi.fn();
    render(<ActivePositionsTable positions={[position]} onManualExit={onManualExit} />);
    await userEvent.click(screen.getByRole('button', { name: 'Fechar' }));
    expect(onManualExit).toHaveBeenCalledWith(position.mint);
  });
});
