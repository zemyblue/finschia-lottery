/* eslint-disable */
import {ChainInfo} from "@keplr-wallet/types";

export const connectDosiVault = async (chainInfo: ChainInfo) => {
  console.log(`Suggesting chain ${chainInfo.chainId}...`);
  try {
    if (window) {
      // @ts-ignore
      if (window["dosiVault"]) {
        // @ts-ignore
        if (window.dosiVault["experimentalSuggestChain"]) {
          // @ts-ignore
          await window.dosiVault.experimentalSuggestChain(chainInfo);
          // @ts-ignore
          await window.dosiVault.enable(chainInfo.chainId)
          // @ts-ignore
          const offlineSigner = window.dosiVault.getOfflineSigner(chainInfo.chainId);
          // @ts-ignore
          const accounts = await offlineSigner.getAccounts();
          return accounts[0].address;
        } else {
          console.debug("Error access experimental features, please update DOSI Vault");
        }
      } else {
        console.debug("Error accessing DOSI Vault");
      }
    } else {
      console.debug("Error parsing window object");
    }
  } catch (e) {
    console.error("Error suggestChain: ", e);
  }
};

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
  console.log(`writeWallet: ${connected.hdPath}`);
  localStorage.setItem(hdPath || DEFAULT_HDPATH, JSON.stringify(connected))
}

export function removeWallet(hdPath?: string) {
  console.log(`removeWallet: ${hdPath}`);
  localStorage.removeItem(hdPath || DEFAULT_HDPATH);
}
