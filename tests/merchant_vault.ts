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
  const payer = provider.wallet as anchor.Wallet;

  // ── Shared state ──────────────────────────────────────────────────────────
  let configPda: anchor.web3.PublicKey;
  let treasuryPda: anchor.web3.PublicKey;
  let merchantPda: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;
  let mint: anchor.web3.PublicKey;
  let payerAta: Awaited<ReturnType<typeof getOrCreateAssociatedTokenAccount>>;
  let vaultAta: Awaited<ReturnType<typeof getOrCreateAssociatedTokenAccount>>;
  let treasuryAta: Awaited<
    ReturnType<typeof getOrCreateAssociatedTokenAccount>
  >;

  before(() => {
    [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId,
    );
    [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId,
    );
    [merchantPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("merchant"), payer.publicKey.toBuffer()],
      program.programId,
    );
    [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), merchantPda.toBuffer()],
      program.programId,
    );
  });

  // ── 1 ─────────────────────────────────────────────────────────────────────
  it("1. Initialize config with 150 bps fee", async () => {
    await program.methods
      .initializeConfig(150)
      .accounts({
        config: configPda,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const config = await program.account.config.fetch(configPda);

    // ✅ Your Rust struct uses platform_fee_bps → JS sees platformFeeBps
    assert.equal(config.platformFeeBps, 150, "Platform fee should be 150 bps");
  });

  // ── 2 ─────────────────────────────────────────────────────────────────────
  it("2. Initialize treasury account", async () => {
    await program.methods
      .initializeTreasury()
      .accounts({
        treasury: treasuryPda,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const treasury = await program.account.treasury.fetch(treasuryPda);
    assert.ok(treasury.bump, "Treasury should have a bump");
  });

  // ── 3 ─────────────────────────────────────────────────────────────────────
  it("3. Initialize merchant and vault PDAs", async () => {
    await program.methods
      .initializeMerchant()
      .accounts({
        merchant: merchantPda,
        vault: vaultPda,
        authority: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const merchant = await program.account.merchant.fetch(merchantPda);
    const vault = await program.account.vault.fetch(vaultPda);

    assert.equal(
      merchant.authority.toBase58(),
      payer.publicKey.toBase58(),
      "Merchant authority should match payer",
    );
    assert.equal(
      vault.merchant.toBase58(),
      merchantPda.toBase58(),
      "Vault should reference merchant PDA",
    );
  });

  // ── 4 ─────────────────────────────────────────────────────────────────────
  it("4. Deposit SOL into vault via pay_with_sol", async () => {
    const depositLamports = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
    const vaultBefore = await provider.connection.getBalance(vaultPda);

    await program.methods
      .payWithSol(depositLamports)
      .accounts({
        payer: payer.publicKey,
        config: configPda,
        merchant: merchantPda,
        vault: vaultPda,
        treasury: treasuryPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const vaultAfter = await provider.connection.getBalance(vaultPda);
    assert.isAbove(
      vaultAfter,
      vaultBefore,
      "Vault SOL balance should increase",
    );
  });

  // ── 5 ─────────────────────────────────────────────────────────────────────
  it("5. Deposit SPL token into vault via pay_with_token", async () => {
    mint = await createMint(
      provider.connection,
      payer.payer,
      payer.publicKey,
      null,
      6,
    );

    payerAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint,
      payer.publicKey,
    );
    vaultAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint,
      vaultPda,
      true,
    );
    treasuryAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint,
      treasuryPda,
      true,
    );

    await mintTo(
      provider.connection,
      payer.payer,
      mint,
      payerAta.address,
      payer.publicKey,
      1_000_000_000,
    );

    // ✅ Fetch current payment_count from merchant to derive correct payment PDA
    // Your Rust seeds are: ["payment", merchant_key, payment_count.to_le_bytes()]
    const merchantAccount = await program.account.merchant.fetch(merchantPda);
    const paymentCount =
      merchantAccount.paymentCount ??
      (merchantAccount as any).payment_count ??
      0;

    const countBytes = Buffer.alloc(8);
    countBytes.writeBigUInt64LE(BigInt(paymentCount));

    const [paymentPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("payment"), merchantPda.toBuffer(), countBytes],
      program.programId,
    );

    await program.methods
      .payWithToken(new anchor.BN(1_000_000))
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

    const expectedFee = (1_000_000 * 150) / 10_000;
    const expectedMerchant = 1_000_000 - expectedFee;

    const vaultToken = await getAccount(provider.connection, vaultAta.address);
    const treasuryToken = await getAccount(
      provider.connection,
      treasuryAta.address,
    );

    assert.equal(
      Number(vaultToken.amount),
      expectedMerchant,
      "Vault receives correct amount after fee",
    );
    assert.equal(
      Number(treasuryToken.amount),
      expectedFee,
      "Treasury receives correct fee",
    );
  });

  // ── 6 ─────────────────────────────────────────────────────────────────────
  it("6. Withdraw SOL from vault via withdraw_sol", async () => {
    const withdrawLamports = new anchor.BN(0.01 * anchor.web3.LAMPORTS_PER_SOL);
    const vaultBefore = await provider.connection.getBalance(vaultPda);
    const payerBefore = await provider.connection.getBalance(payer.publicKey);

    await program.methods
      .withdrawSol(withdrawLamports)
      .accounts({
        authority: payer.publicKey,
        merchant: merchantPda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const vaultAfter = await provider.connection.getBalance(vaultPda);
    const payerAfter = await provider.connection.getBalance(payer.publicKey);

    assert.isBelow(vaultAfter, vaultBefore, "Vault SOL should decrease");
    assert.isAbove(
      payerAfter,
      payerBefore - anchor.web3.LAMPORTS_PER_SOL,
      "Payer should receive SOL minus tx fees",
    );
  });

  // ── 7 ─────────────────────────────────────────────────────────────────────
  it("7. Withdraw SPL token from vault via withdraw_token", async () => {
    const withdrawAmount = new anchor.BN(500_000); // 0.5 tokens

    const vaultAtaBefore = await getAccount(
      provider.connection,
      vaultAta.address,
    );
    const payerAtaBefore = await getAccount(
      provider.connection,
      payerAta.address,
    );

    // ✅ Real on-chain call — requires withdraw_token in lib.rs
    await program.methods
      .withdrawToken(withdrawAmount)
      .accounts({
        authority: payer.publicKey,
        merchant: merchantPda,
        vault: vaultPda,
        vaultTokenAccount: vaultAta.address,
        authorityTokenAccount: payerAta.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const vaultAtaAfter = await getAccount(
      provider.connection,
      vaultAta.address,
    );
    const payerAtaAfter = await getAccount(
      provider.connection,
      payerAta.address,
    );

    assert.equal(
      Number(vaultAtaAfter.amount),
      Number(vaultAtaBefore.amount) - 500_000,
      "Vault should decrease by exactly 0.5 tokens",
    );
    assert.equal(
      Number(payerAtaAfter.amount),
      Number(payerAtaBefore.amount) + 500_000,
      "Payer should receive exactly 0.5 tokens",
    );
  });
});
