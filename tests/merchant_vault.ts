import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MerchantVault } from "../target/types/merchant_vault";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import { assert } from "chai";

describe("merchant-vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MerchantVault as Program<MerchantVault>;
  const payer = provider.wallet;

  let mint: anchor.web3.PublicKey;

  it("Full token payment flow", async () => {
    /*
      Initialize Config
    */

    const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId,
    );

    await program.methods
      .initializeConfig(150)
      .accounts({
        config: configPda,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const configAccount = await program.account.config.fetch(configPda);
    console.log("\n=== CONFIG ACCOUNT ===");
    console.log("Config PDA:          ", configPda.toBase58());

    /*
      Initialize Treasury
    */

    const [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId,
    );

    await program.methods
      .initializeTreasury()
      .accounts({
        treasury: treasuryPda,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const treasuryOnChain = await program.account.treasury.fetch(treasuryPda);
    console.log("\n=== TREASURY ACCOUNT ===");
    console.log("Treasury PDA:        ", treasuryPda.toBase58());
    console.log("Bump:                ", treasuryOnChain.bump);
    const treasuryLamports = await provider.connection.getBalance(treasuryPda);
    console.log(
      "Treasury SOL:        ",
      treasuryLamports / anchor.web3.LAMPORTS_PER_SOL,
      "SOL",
    );

    /*
      Create Merchant + Vault PDA
    */

    const [merchantPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("merchant"), payer.publicKey.toBuffer()],
      program.programId,
    );

    const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), merchantPda.toBuffer()],
      program.programId,
    );

    await program.methods
      .initializeMerchant()
      .accounts({
        merchant: merchantPda,
        vault: vaultPda,
        authority: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const merchantAccount = await program.account.merchant.fetch(merchantPda);
    const vaultOnChain = await program.account.vault.fetch(vaultPda);
    const vaultLamports = await provider.connection.getBalance(vaultPda);

    console.log("\n=== MERCHANT ACCOUNT ===");
    console.log("Merchant PDA:        ", merchantPda.toBase58());
    console.log("Authority:           ", merchantAccount.authority.toBase58());

    console.log("\n=== VAULT ACCOUNT ===");
    console.log("Vault PDA:           ", vaultPda.toBase58());
    console.log("Merchant (ref):      ", vaultOnChain.merchant.toBase58());
    console.log("Bump:                ", vaultOnChain.bump);
    console.log(
      "Vault SOL Balance:   ",
      vaultLamports / anchor.web3.LAMPORTS_PER_SOL,
      "SOL",
    );

    /*
      Create Mock Token Mint
    */

    mint = await createMint(
      provider.connection,
      payer.payer,
      payer.publicKey,
      null,
      6,
    );

    console.log("\n=== MINT ===");
    console.log("Mint Address:        ", mint.toBase58());
    console.log("Decimals:            ", 6);
    console.log("Mint Authority:      ", payer.publicKey.toBase58());

    /*
      Create Token Accounts
    */

    const payerAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint,
      payer.publicKey,
    );

    const vaultAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint,
      vaultPda,
      true,
    );

    const treasuryAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint,
      treasuryPda,
      true,
    );

    console.log("\n=== TOKEN ACCOUNTS ===");
    console.log("Payer ATA:           ", payerAta.address.toBase58());
    console.log("Payer ATA owner:     ", payerAta.owner.toBase58());
    console.log("Vault ATA:           ", vaultAta.address.toBase58());
    console.log("Vault ATA owner:     ", vaultAta.owner.toBase58());
    console.log("Treasury ATA:        ", treasuryAta.address.toBase58());
    console.log("Treasury ATA owner:  ", treasuryAta.owner.toBase58());

    /*
      Mint Tokens To Payer
    */

    await mintTo(
      provider.connection,
      payer.payer,
      mint,
      payerAta.address,
      payer.publicKey,
      1_000_000_000,
    );

    const payerAtaBefore = await getAccount(
      provider.connection,
      payerAta.address,
    );
    console.log("\n=== BALANCES BEFORE PAYMENT ===");
    console.log(
      "Payer token balance: ",
      Number(payerAtaBefore.amount) / 1_000_000,
      "tokens",
    );
    console.log(
      "Vault token balance: ",
      Number(vaultAta.amount) / 1_000_000,
      "tokens",
    );
    console.log(
      "Treasury balance:    ",
      Number(treasuryAta.amount) / 1_000_000,
      "tokens",
    );

    /*
      Pay With Token
    */

    const amount = new anchor.BN(1_000_000); // 1 token

    const [paymentPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("payment"),
        merchantPda.toBuffer(),
        payer.publicKey.toBuffer(),
      ],
      program.programId,
    );

    await program.methods
      .payWithToken(amount)
      .accounts({
        payer: payer.publicKey,
        config: configPda,
        merchant: merchantPda,
        vault: vaultPda,
        treasury: treasuryPda,
        payerTokenAccount: payerAta.address,
        vaultTokenAccount: vaultAta.address,
        treasuryTokenAccount: treasuryAta.address,
        payment: paymentPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const paymentAccount = await program.account.payment.fetch(paymentPda);
    console.log("\n=== 🧾 PAYMENT RECORD ===");
    console.log("Payment PDA:         ", paymentPda.toBase58());
    console.log("Payer:               ", paymentAccount.payer.toBase58());
    console.log("Mint:                ", paymentAccount.mint.toBase58());
    console.log("Amount:              ", paymentAccount.amount.toString());
    console.log("Fee Amount:          ", paymentAccount.feeAmount.toString());
    console.log(
      "Timestamp:           ",
      new Date(paymentAccount.timestamp.toNumber() * 1000).toISOString(),
    );
    console.log("Bump:                ", paymentAccount.bump);

    /*
      Assert Balances
    */

    const vaultTokenAccount = await getAccount(
      provider.connection,
      vaultAta.address,
    );
    const treasuryTokenAccount = await getAccount(
      provider.connection,
      treasuryAta.address,
    );
    const payerAtaAfter = await getAccount(
      provider.connection,
      payerAta.address,
    );

    const expectedFee = (1_000_000 * 150) / 10000;
    const expectedMerchant = 1_000_000 - expectedFee;

    console.log("\n=== BALANCES AFTER PAYMENT ===");
    console.log(
      "Payer token balance: ",
      Number(payerAtaAfter.amount) / 1_000_000,
      "tokens",
    );
    console.log(
      "Vault token balance: ",
      Number(vaultTokenAccount.amount) / 1_000_000,
      "tokens",
    );
    console.log(
      "Treasury balance:    ",
      Number(treasuryTokenAccount.amount) / 1_000_000,
      "tokens",
    );
    console.log(
      "Expected merchant:   ",
      expectedMerchant / 1_000_000,
      "tokens",
    );
    console.log("Expected fee:        ", expectedFee / 1_000_000, "tokens");

    assert.equal(Number(vaultTokenAccount.amount), expectedMerchant);
    assert.equal(Number(treasuryTokenAccount.amount), expectedFee);

    console.log("\n=== ASSERTIONS PASSED ===");
    console.log(
      "Vault received:   ",
      Number(vaultTokenAccount.amount) / 1_000_000,
      "tokens",
    );
    console.log(
      "Treasury received:",
      Number(treasuryTokenAccount.amount) / 1_000_000,
      "tokens",
    );
  });
});
