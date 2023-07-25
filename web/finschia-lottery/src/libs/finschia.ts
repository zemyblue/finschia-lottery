export const finschia_chain_info = {
  "chainId": "simd-testing",
  "chainName": "localnet",
  "rest": "http://localhost:1317",
  "bip44": {
    "coinType": 438
  },
  "coinType": 438,
  "bech32Config": {
    "bech32PrefixAccAddr": "link",
    "bech32PrefixAccPub": "linkpub",
    "bech32PrefixValAddr": "linkvaloper",
    "bech32PrefixValPub": "linkvaloperpub",
    "bech32PrefixConsAddr": "linkvalcons",
    "bech32PrefixConsPub": "linkvalconspub"
  },
  "currencies": [
    {
      "coinDenom": "FNSA",
      "coinMinimalDenom": "cony",
      "coinDecimals": 6,
      "coinGeckoId": "unknown"
    }
  ],
  "feeCurrencies": [
    {
      "coinDenom": "FNSA",
      "coinMinimalDenom": "cony",
      "coinDecimals": 6,
      "coinGeckoId": "unknown",
      "gasPriceStep": {
        "low": 0.01,
        "average": 0.025,
        "high": 0.03
      }
    }
  ],
  "gasPriceStep": {
    "low": 0.01,
    "average": 0.025,
    "high": 0.03
  },
  "stakeCurrency": {
    "coinDenom": "FNSA",
    "coinMinimalDenom": "cony",
    "coinDecimals": 6,
    "coinGeckoId": "unknown"
  },
  "features": []
}
