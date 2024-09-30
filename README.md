# Nizer LMT
LMT(Liquid Membership Token) is an advanced version of LST.  
Allowing integration with lending to apply leverage to LST and increase APY. It also provides the option to choose stablecoins instead of Solana as the base token to hedge against volatility. Plays a central role in the value redistribution provided by Nizer.

## Overview
Circulate value within the community centered around LMT.
<img width="713" alt="image" src="https://github.com/user-attachments/assets/369faa5e-4cd4-4358-87e2-2873d856bd6e">


## Features
Core features of Nizer LMT
- [x] Token Wrap / Unwrap
- [ ] Manage APY reward pool
- [ ] APY reward multiplier
- [ ] Manage membership (Mint and transfer NFTs based on token holdings)

## Specifications
### Wrap / Unwrap
Wrap/unwrap are processes conducted to ensure the liquidity of LMT issued by individual communities, and they are based on the native LMT.
This is part of the functionality provided by Nizer LMT.

LMT is wrapped into an SPL token (WLMT) to interact with the Solana blockchain's SPL token ecosystem.
To explain this further, the wrap and unwrap processes can be implemented in the following way:

1. Wrap (Convert LMT to WLMT):
When you wrap LMT, you are effectively locking the native LMT and minting an equivalent amount of WLMT (an SPL token). This allows LMT to be used in the SPL ecosystem like other tokens.

2. Unwrap (Convert WLMT to SOL):
When you unwrap WLMT, you are effectively burning the WLMT and releasing the equivalent amount of LMT. This allows WLMT to be converted back to native LMT.

## Usage

### Building

```bash
cargo build-bpf
```
