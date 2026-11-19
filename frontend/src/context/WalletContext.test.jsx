import { act, renderHook } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useWallet } from '@solana/wallet-adapter-react';
import { SolanaWalletProvider, useTrenchWallet } from './WalletContext';

vi.mock('@solana/wallet-adapter-react', () => ({
  ConnectionProvider: ({ children }) => children,
  WalletProvider: ({ children }) => children,
  useWallet: vi.fn(),
}));
vi.mock('@solana/wallet-adapter-phantom', () => ({ PhantomWalletAdapter: vi.fn() }));

const renderWallet = () => renderHook(() => useTrenchWallet(), { wrapper: SolanaWalletProvider });

describe('WalletContext', () => {
  beforeEach(() => {
    vi.mocked(useWallet).mockReturnValue({ connected: false, publicKey: null, disconnect: vi.fn() });
  });

  it('requires the provider', () => {
    vi.spyOn(console, 'error').mockImplementation(() => {});
    expect(() => renderHook(() => useTrenchWallet())).toThrow(/within SolanaWalletProvider/);
  });

  it('starts disconnected with a simulated 10 SOL balance', () => {
    const { result } = renderWallet();
    expect(result.current.isConnected).toBe(false);
    expect(result.current.address).toBeNull();
    expect(result.current.balanceSol).toBe(10);
  });

  it('connects and disconnects the paper wallet', () => {
    const { result } = renderWallet();
    act(() => result.current.connectPaperWallet());
    expect(result.current.isConnected).toBe(true);
    expect(result.current.address).toBe('7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU');
    act(() => result.current.disconnectWallet());
    expect(result.current.isConnected).toBe(false);
  });

  it('prefers the phantom public key when phantom is connected', () => {
    const disconnect = vi.fn();
    vi.mocked(useWallet).mockReturnValue({ connected: true, publicKey: { toBase58: () => 'PHANTOM-KEY' }, disconnect });
    const { result } = renderWallet();
    expect(result.current.isConnected).toBe(true);
    expect(result.current.address).toBe('PHANTOM-KEY');
    act(() => result.current.disconnectWallet());
    expect(disconnect).toHaveBeenCalledOnce();
  });

  it('does not call phantom disconnect for the paper wallet', () => {
    const disconnect = vi.fn();
    vi.mocked(useWallet).mockReturnValue({ connected: false, publicKey: null, disconnect });
    const { result } = renderWallet();
    act(() => result.current.connectPaperWallet());
    act(() => result.current.disconnectWallet());
    expect(disconnect).not.toHaveBeenCalled();
  });
});
