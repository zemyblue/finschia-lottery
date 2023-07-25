/* eslint-disable */
import type {Transaction} from "./type";
import {Registry} from "@cosmjs/proto-signing";
import {finschiaRegistryTypes} from "@finschia/finschia"
import {DosiVaultWallet} from "@/libs/DosiVaultWallet";


export interface Account {
  address: string,
  algo: string,
  pubkey: Uint8Array,
}

export interface WalletArgument {
  chainId?: string,
  hdPath?: string,
  address?: string,
  name?: string,
  transport?: string
  prefix?: string,
}

export interface AbstractWallet {
  /**
   * The accounts from the wallet (addresses)
   */
  getAccounts(): Promise<Account[]>
  supportCoinType(coinType?: string): Promise<boolean>
  sign(transaction: Transaction): Promise<any>
}

export interface ConnectedWallet {
  cosmosAddress: string
  hdPath?: string
}


// finschia HDPath
export const DEFAULT_HDPATH = "m/44'/438/0'/0/0";

export function readWallet(hdPath?: string) {
  return JSON.parse(
    localStorage.getItem(hdPath || DEFAULT_HDPATH) || '{}'
  ) as ConnectedWallet
}

export function writeWallet(connected: ConnectedWallet, hdPath?: string) {
  localStorage.setItem(hdPath || DEFAULT_HDPATH, JSON.stringify(connected))
}

export function removeWallet(hdPath?: string) {
  localStorage.removeItem(hdPath || DEFAULT_HDPATH);
}

export function createWallet(arg: WalletArgument, registry?: Registry): AbstractWallet {
  // @ts-ignore
  const reg = registry || new Registry(finschiaRegistryTypes)
  return new DosiVaultWallet(arg, reg)
}
