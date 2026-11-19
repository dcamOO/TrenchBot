import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { PnlHistoryTable } from './PnlHistoryTable';
import { MetricsCards } from './MetricsCards';

const win = { mint: 'DEMO-A1b2C3d4E5f6G7h8J9k0L1m', symbol: 'CHAD-SOL', openedAt: 1727690000, closedAt: 1727690300, costLamports: 100000000, receivedLamports: 150000000, pnlLamports: 50000000, pnlSol: 0.05, pnlPercent: 50, reason: 'take_profit' };
const loss = { mint: 'DEMO-B2c3D4e5F6g7H8j9K0l1M2n', symbol: 'WOJAK', openedAt: 1727690400, closedAt: 1727690550, costLamports: 100000000, receivedLamports: 80000000, pnlLamports: -20000000, pnlSol: -0.02, pnlPercent: -20, reason: 'stop_loss' };
const exportButton = () => screen.getByRole('button', { name: /exportar relatório csv/i });

describe('PnlHistoryTable', () => {
  it('disables export and shows empty state without trades', () => {
    render(<PnlHistoryTable trades={[]} onExportCsv={vi.fn()} />);
    expect(exportButton()).toBeDisabled();
    expect(screen.getByText(/nenhum trade encerrado/i)).toBeInTheDocument();
  });

  it('exports when trades exist', async () => {
    const onExportCsv = vi.fn();
    render(<PnlHistoryTable trades={[win]} onExportCsv={onExportCsv} />);
    await userEvent.click(exportButton());
    expect(onExportCsv).toHaveBeenCalledOnce();
  });

  it('renders invested return and signed pnl for each trade', () => {
    render(<PnlHistoryTable trades={[win, loss]} onExportCsv={vi.fn()} />);
    const winRow = screen.getByText('CHAD-SOL').closest('tr');
    expect(within(winRow).getByText('0.100 SOL')).toBeInTheDocument();
    expect(within(winRow).getByText('0.150 SOL')).toBeInTheDocument();
    expect(within(winRow).getByText('+0.0500 SOL', { exact: false })).toBeInTheDocument();
    expect(within(winRow).getByText('Take-Profit')).toBeInTheDocument();
    const lossRow = screen.getByText('WOJAK').closest('tr');
    expect(within(lossRow).getByText('-0.0200 SOL', { exact: false })).toBeInTheDocument();
    expect(within(lossRow).getByText('Stop-Loss')).toBeInTheDocument();
  });

  it('labels manual exits as manual instead of stop loss', () => {
    render(<PnlHistoryTable trades={[{ ...loss, reason: 'manual' }]} onExportCsv={vi.fn()} />);
    expect(screen.getByText('Manual')).toBeInTheDocument();
    expect(screen.queryByText('Stop-Loss')).not.toBeInTheDocument();
  });

  it('shows unknown reasons verbatim', () => {
    render(<PnlHistoryTable trades={[{ ...loss, reason: 'expired' }]} onExportCsv={vi.fn()} />);
    expect(screen.getByText('expired')).toBeInTheDocument();
  });

  it('shortens the mint address', () => {
    render(<PnlHistoryTable trades={[win]} onExportCsv={vi.fn()} />);
    expect(screen.getByText('DEMO...0L1m')).toBeInTheDocument();
  });
});

describe('MetricsCards', () => {
  const metrics = { totalPnlSol: 0.03, totalPnlPercent: 15, winRate: 50, activeCount: 1, monitoredCount: 2 };

  it('renders positive pnl win rate and counters', () => {
    render(<MetricsCards metrics={metrics} />);
    expect(screen.getByText('+0.0300 SOL')).toBeInTheDocument();
    expect(screen.getByText('(+15.0%)')).toBeInTheDocument();
    expect(screen.getByText('50.0%')).toBeInTheDocument();
    expect(screen.getByText('1')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
  });

  it('renders negative pnl without plus sign and in red', () => {
    render(<MetricsCards metrics={{ ...metrics, totalPnlSol: -0.05, totalPnlPercent: -25 }} />);
    const value = screen.getByText('-0.0500 SOL');
    expect(value).toHaveClass('text-rose-400');
    expect(screen.getByText('(-25.0%)')).toBeInTheDocument();
  });
});
