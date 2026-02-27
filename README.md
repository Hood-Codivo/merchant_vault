This is a deterministic, non-custodial, revenue-generating merchant vault protocol on Solana with automated fee splitting.

A smart contract that allows customers to pay merchants in SOL or SPL tokens, automatically deducts 1.5% platform fee, and lets merchants withdraw securely — without admin control.

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

Program path: /mnt/c/Users/godwi/merchant_vault/target/deploy/merchant_vault.so...

Program Id: 2vm8xz2TFAQH2N6ijqLNRt6J7bh8h7hy92hYM19Vy4SD                                                                                     

Signature: 3xfhaz1crdrFFRY2MUiYQf3NmbmmYBXxoPvH3bk5akPp1XyNpkvAxJYpL3TNT72FdHnqmjQwNbspzfec3SewmKJC

<img width="2880" height="1710" alt="image" src="https://github.com/user-attachments/assets/6067e015-02dd-4e77-910c-0729fc104b97" />

Did a test with token to (mockup USDC)

<img width="2880" height="1700" alt="image" src="https://github.com/user-attachments/assets/e0f9cdbb-ce48-4752-a991-e1812398675a" />

