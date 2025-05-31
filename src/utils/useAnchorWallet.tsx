// /utils/useAnchorWallet.tsx
import {
    PublicKey,
    Transaction,
    VersionedTransaction,
  } from "@solana/web3.js";
  import { useMemo } from "react";
  import { useMobileWallet } from "./useMobileWallet";
  import { useAuthorization } from "./useAuthorization";
  
  export interface AnchorWallet {
    publicKey: PublicKey;
    signTransaction<T extends Transaction | VersionedTransaction>(
      transaction: T
    ): Promise<T>;
    signAllTransactions<T extends Transaction | VersionedTransaction>(
      transactions: T[]
    ): Promise<T[]>;
  }
  
  export function useAnchorWallet(): AnchorWallet | undefined {
    const { selectedAccount } = useAuthorization();
    const mobileWallet = useMobileWallet();
  
    return useMemo(() => {
      if (!selectedAccount) {
        return undefined;
      }
  
      return {
        signTransaction: async <T extends Transaction | VersionedTransaction>(
          transaction: T
        ) => {
          // Use signAndSendTransaction but only return the signed transaction
          const minContextSlot = 0; // Default value since we're just signing
          await mobileWallet.signAndSendTransaction(transaction, minContextSlot);
          return transaction;
        },
        signAllTransactions: async <T extends Transaction | VersionedTransaction>(
          transactions: T[]
        ) => {
          // Sign each transaction individually since mobile wallet doesn't support batch signing
          const signedTransactions = await Promise.all(
            transactions.map(async (tx) => {
              const minContextSlot = 0;
              await mobileWallet.signAndSendTransaction(tx, minContextSlot);
              return tx;
            })
          );
          return signedTransactions;
        },
        publicKey: selectedAccount.publicKey,
      };
    }, [mobileWallet, selectedAccount]);
  }