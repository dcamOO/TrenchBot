// Integração: App real (WalletContext + Header + useSimulationFeed + tabelas + export CSV).
// Apenas o adaptador Solana é substituído, para não depender da extensão Phantom.
import { act, fireEvent, render, screen, within } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import App from './App';

vi.mock('@solana/wallet-adapter-react', () => ({
  ConnectionProvider: ({ children }) => children,
  WalletProvider: ({ children }) => children,
  useWallet: () => ({ connected: false, publicKey: null, disconnect: vi.fn() }),
}));
vi.mock('@solana/wallet-adapter-phantom', () => ({ PhantomWalletAdapter: vi.fn() }));

const card = (title) => screen.getByText(title).closest('div.rounded-xl');
const metric = (label) => within(card(label));
const tick = () => act(() => vi.advanceTimersByTime(3500));
const click = (name) => fireEvent.click(screen.getByRole('button', { name }));

describe('App dashboard integration', () => {
  let exported;

  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2026-11-19T12:00:00Z'));
    exported = [];
    URL.createObjectURL = vi.fn((blob) => {
      exported.push(blob);
      return 'blob:pnl';
    });
    vi.spyOn(HTMLAnchorElement.prototype, 'click').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it('shows initial metrics derived from the seeded trades', () => {
    render(<App />);
    expect(metric('PnL Realizado').getByText('+0.0300 SOL')).toBeInTheDocument();
    expect(metric('PnL Realizado').getByText('(+15.0%)')).toBeInTheDocument();
    expect(metric('Taxa de Acerto').getByText('50.0%')).toBeInTheDocument();
    expect(metric('Posições Abertas').getByText('1')).toBeInTheDocument();
    expect(metric('Tokens em Triagem').getByText('2')).toBeInTheDocument();
  });

  it('connects the wallet, runs a simulated trade, exits manually and exports the report', async () => {
    render(<App />);

    click('Conectar Phantom');
    expect(screen.getByText('7xKX...gAsU')).toBeInTheDocument();
    expect(screen.getByText('10.000 SOL')).toBeInTheDocument();

    click(/Iniciar Simulação/);
    tick();
    expect(metric('Tokens em Triagem').getByText('3')).toBeInTheDocument();
    tick();
    expect(metric('Posições Abertas').getByText('2')).toBeInTheDocument();
    tick();
    expect(metric('Posições Abertas').getByText('1')).toBeInTheDocument();
    expect(metric('PnL Realizado').getByText('+0.0800 SOL')).toBeInTheDocument();
    expect(metric('Taxa de Acerto').getByText('66.7%')).toBeInTheDocument();
    click(/Pausar Demo/);

    const positions = within(card('Posições Abertas em Execução'));
    fireEvent.click(positions.getByRole('button', { name: 'Fechar' }));
    expect(positions.getByText(/Nenhuma posição aberta/)).toBeInTheDocument();
    expect(within(card('Histórico de Trades Fechados & PnL')).getByText('Manual')).toBeInTheDocument();
    expect(metric('PnL Realizado').getByText('+0.1100 SOL')).toBeInTheDocument();
    expect(metric('Taxa de Acerto').getByText('75.0%')).toBeInTheDocument();

    const exitAt = Math.floor(Date.now() / 1000);
    click(/Exportar Relatório CSV/);
    expect(exported).toHaveLength(1);
    const lines = (await exported[0].text()).trim().split('\n');
    expect(lines).toHaveLength(5);
    expect(lines[1]).toBe(`9zP3kL14m8sW92eRtYu34qLm21p,${exitAt - 60},${exitAt},100000000,130000000,30000000,0.030000,30.00,manual`);
    expect(lines[2]).toMatch(/^5K9aXbU5vK14q8sW93eRtYu23pLm,\d+,\d+,100000000,150000000,50000000,0\.050000,50\.00,take_profit$/);
  });

  it('header export uses the same report as the history table', async () => {
    render(<App />);
    click('Exportar PnL');
    click(/Exportar Relatório CSV/);
    const [fromHeader, fromTable] = await Promise.all(exported.map((b) => b.text()));
    expect(fromHeader).toBe(fromTable);
    expect(fromHeader.trim().split('\n')).toHaveLength(3);
  });
});
