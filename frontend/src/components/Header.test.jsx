import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { Header } from './Header';
import { useTrenchWallet } from '../context/WalletContext';

vi.mock('../context/WalletContext', () => ({ useTrenchWallet: vi.fn() }));

const wallet = (overrides = {}) => ({
  isConnected: false,
  address: null,
  balanceSol: 10,
  connectPaperWallet: vi.fn(),
  disconnectWallet: vi.fn(),
  ...overrides,
});

describe('Header', () => {
  beforeEach(() => vi.mocked(useTrenchWallet).mockReset());

  it('offers phantom connection when disconnected', async () => {
    const w = wallet();
    vi.mocked(useTrenchWallet).mockReturnValue(w);
    render(<Header />);
    await userEvent.click(screen.getByRole('button', { name: /conectar phantom/i }));
    expect(w.connectPaperWallet).toHaveBeenCalledOnce();
  });

  it('shows shortened address and balance when connected', () => {
    vi.mocked(useTrenchWallet).mockReturnValue(
      wallet({ isConnected: true, address: '7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU', balanceSol: 9.87654 })
    );
    render(<Header />);
    expect(screen.getByText('7xKX...gAsU')).toBeInTheDocument();
    expect(screen.getByText('9.877 SOL')).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /conectar phantom/i })).not.toBeInTheDocument();
  });

  it('disconnects the wallet', async () => {
    const w = wallet({ isConnected: true, address: 'ABCDEFGHIJ' });
    vi.mocked(useTrenchWallet).mockReturnValue(w);
    render(<Header />);
    await userEvent.click(screen.getByTitle('Desconectar Carteira'));
    expect(w.disconnectWallet).toHaveBeenCalledOnce();
  });

  it('disables pnl export without trades', () => {
    vi.mocked(useTrenchWallet).mockReturnValue(wallet());
    render(<Header onExportPnl={vi.fn()} hasTrades={false} />);
    expect(screen.getByRole('button', { name: /exportar pnl/i })).toBeDisabled();
  });

  it('exports pnl when trades exist', async () => {
    vi.mocked(useTrenchWallet).mockReturnValue(wallet());
    const onExport = vi.fn();
    render(<Header onExportPnl={onExport} hasTrades />);
    await userEvent.click(screen.getByRole('button', { name: /exportar pnl/i }));
    expect(onExport).toHaveBeenCalledOnce();
  });

  it('toggles the simulation label', async () => {
    vi.mocked(useTrenchWallet).mockReturnValue(wallet());
    const onToggle = vi.fn();
    const { rerender } = render(<Header isSimulating={false} onToggleSimulation={onToggle} />);
    await userEvent.click(screen.getByRole('button', { name: /iniciar simulação/i }));
    expect(onToggle).toHaveBeenCalledOnce();
    rerender(<Header isSimulating onToggleSimulation={onToggle} />);
    expect(screen.getByRole('button', { name: /pausar demo/i })).toBeInTheDocument();
  });
});
