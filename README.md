This is a deterministic, non-custodial, revenue-generating merchant vault protocol on Solana with automated fee splitting.

A program that allows customers to pay merchants in SOL or SPL tokens, automatically deducts 1.5% platform fee, and lets merchants withdraw securely — without admin control.

The Flow diagram 

<img width="2880" height="1524" alt="image" src="https://github.com/user-attachments/assets/37c0af89-af69-49d0-848f-a3e83ec28bea" />

### How it works

Merchant: Alice

Customer: Bob

Payment: 10 SOL

## Step 1 — Merchant Registers

Alice calls:  initialize_merchant

Program creates:

Merchant PDA
Merchant Vault PDA

These are deterministic accounts based on her public key.

## Step 2 — Customer Pays

Bob calls: pay

He sends 10 SOL.

Program calculates:

1.5% fee = 0.15 SOL
Merchant gets = 9.85 SOL

Program transfers:

9.85 SOL → Merchant Vault
0.15 SOL → Treasury

## Step 3 — Merchant Withdraws

Alice calls: withdraw

Program verifies:

Signer == merchant.authority

Then transfers SOL from:

Merchant Vault → Alice Wallet


Deploying cluster: https://api.devnet.solana.com

Upgrade authority: ./keys/sponsor-keypair.json

Deploying program "merchant_vault"...

Program Id: 57iN23Yr2fMV8bLcmtCXXWsnayE7LidWsHDYGRsn7bPZ

Signature: 4dv7qrwUeb8WKnFuZjFcGiCAeBAXCjKEr7ERfpPgvTHMAG6RVGBtj2hpVSZP1qrWMhw7HJ2RiCK9uJy7GPPMQ7Gy

<img width="2880" height="1706" alt="image" src="https://github.com/user-attachments/assets/6c003e56-b671-4a4c-baca-326b9e112818" />



