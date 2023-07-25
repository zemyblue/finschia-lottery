import type { EncodeObject } from '@cosmjs/proto-signing';
import type { SignerData, StdFee } from '@cosmjs/stargate';

export interface Transaction {
  chainId: string;
  signerAddress: string;
  messages: readonly EncodeObject[];
  fee: StdFee;
  memo: string;
  signerData: SignerData
}

