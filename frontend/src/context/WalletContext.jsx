import React, { createContext, useContext, useState, useMemo } from 'react';
import { ConnectionProvider, WalletProvider, useWallet } from '@solana/wallet-adapter-react';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import { clusterApiUrl } from '@solana/web3.js';

const TrenchWalletContext = createContext(null);

export const SolanaWalletProvider = ({ children }) => {
  const endpoint = useMemo(() => clusterApiUrl('mainnet-beta'), []);
  const wallets = useMemo(() => [new PhantomWalletAdapter()], []);

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <TrenchWalletInnerProvider>{children}</TrenchWalletInnerProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};

const TrenchWalletInnerProvider = ({ children }) => {
  const baseWallet = useWallet();
  const [paperConnected, setPaperConnected] = useState(false);
  const [paperAddress, setPaperAddress] = useState('7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU');
  const [balanceSol, setBalanceSol] = useState(10.0);

  const isConnected = baseWallet.connected || paperConnected;
  const address = baseWallet.publicKey
    ? baseWallet.publicKey.toBase58()
    : paperConnected
      ? paperAddress
      : null;

  const connectPaperWallet = () => setPaperConnected(true);
  const disconnectWallet = () => {
    if (baseWallet.connected) {
      baseWallet.disconnect();
    }
    setPaperConnected(false);
  };

  return (
    <TrenchWalletContext.Provider
      value={{
        ...baseWallet,
        isConnected,
        address,
        balanceSol,
        setBalanceSol,
        connectPaperWallet,
        disconnectWallet,
      }}
    >
      {children}
    </TrenchWalletContext.Provider>
  );
};

export const useTrenchWallet = () => {
  const context = useContext(TrenchWalletContext);
  if (!context) {
    throw new Error('useTrenchWallet must be used within SolanaWalletProvider');
  }
  return context;
};
