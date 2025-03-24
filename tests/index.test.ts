import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

import { sha256 } from "js-sha256";
import secp256k1 from "secp256k1";
import type { ThrustApp } from "../target/types/thrust_app";

const MAIN_STATE_SEED = "main_4";
const signer = anchor.Wallet.local().payer;
const TOKEN_PROGRAM = TOKEN_PROGRAM_ID;
const ASSOCIATED_TOKEN_PROGRAM = ASSOCIATED_TOKEN_PROGRAM_ID;
const METADATA_PROGRAM = new web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
let mint = web3.Keypair.generate();

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.ThrustApp as anchor.Program<ThrustApp>;

describe("Test Initialize", () => {
  it("init", async () => {
    const mainStatePDA = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(MAIN_STATE_SEED)],
      program.programId
    );
    try {
      const deserializedAccountData = await program.account.mainState.fetch(
        mainStatePDA[0].toBase58()
      );
      if (deserializedAccountData.initialized == true) {
        console.log("already initialized");
        return;
      }
    } catch { }
    const tx = await program.methods
      .initMainState()
      .accounts({
        owner: signer.publicKey,
        verifySignerPubkey: signer.publicKey,
        mainState: mainStatePDA[0],
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
    console.log(`Use 'solana confirm -v ${tx}' to see the logs`);
    const deserializedAccountData = await program.account.mainState.fetch(
      mainStatePDA[0].toBase58()
    );
    assert.equal(
      deserializedAccountData.initialized,
      true,
      "initialize failed"
    );
  });

  it("update mainstate", async () => {
    const mainStatePDA = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(MAIN_STATE_SEED)],
      program.programId
    );
    console.log("updating...");
    const tx = await program.methods
      .updateMainState({
        owner: signer.publicKey,
        feeRecipient: signer.publicKey,
        tradingFee: new BN(1000),
        referralRewardFee: new BN(10000),
        referralTradeLimit: new BN(100),
        totalTokenSupply: new BN(1_000_000_000 * 1000_000),
        initRealBaseReserves: new BN(800_000_000 * 1000_000),
        initVirtBaseReserves: new BN(200_000_000 * 1000_000),
        initVirtQuoteReserves: new BN(24 * 1000_000_000),
        solPrice: new BN(130_000_000_000),
      })
      .accounts({
        owner: signer.publicKey,
        verifySignerPubkey: signer.publicKey,
        mainState: mainStatePDA[0],
      })
      .rpc();
    await program.provider.connection.confirmTransaction(tx);
    console.log(`Use 'solana confirm -v ${tx}' to see the logs`);

    const deserializedAccountData = await program.account.mainState.fetch(
      mainStatePDA[0].toBase58()
    );
    assert.equal(
      deserializedAccountData.initialized,
      true,
      "Initialize Failed"
    );
    assert.equal(
      deserializedAccountData.owner.toBase58(),
      signer.publicKey.toBase58(),
      "owner was not updated"
    );
    assert.equal(
      deserializedAccountData.feeRecipient.toBase58(),
      signer.publicKey.toBase58(),
      "feeRecipient was not updated"
    );
    assert.equal(
      deserializedAccountData.tradingFee,
      1000,
      "tradingFee was not updated"
    );
    assert.equal(
      deserializedAccountData.referralRewardFee,
      10000,
      "referralRewardFee was not updated"
    );
    assert.equal(
      deserializedAccountData.referralTradeLimit,
      100,
      "referralTradeLimit was not updated"
    );
    assert.equal(
      deserializedAccountData.totalTokenSupply,
      1_000_000_000_000_000,
      "totalTokenSupply was not updated"
    );
    assert.equal(
      deserializedAccountData.initRealBaseReserves,
      800_000_000_000_000,
      "initRealBaseReserves was not updated"
    );
    assert.equal(
      deserializedAccountData.initVirtBaseReserves,
      200_000_000_000_000,
      "initVirtBaseReserves was not updated"
    );
    assert.equal(
      deserializedAccountData.initVirtQuoteReserves,
      24_000_000_000,
      "initVirtQuoteReserves was not updated"
    );
  });
  it("update sol price", async () => {
    const mainStatePDA = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(MAIN_STATE_SEED)],
      program.programId
    );
    const tx = await program.methods
      .updateSolPrice(new BN(160_000_000_000))
      .accounts({
        owner: signer.publicKey,
        verifySignerPubkey: signer.publicKey,
        mainState: mainStatePDA[0],
      })
      .rpc();
    await program.provider.connection.confirmTransaction(tx);
    console.log(`Use 'solana confirm -v ${tx}' to see the logs`);

    const deserializedAccountData = await program.account.mainState.fetch(
      mainStatePDA[0].toBase58()
    );
    assert.equal(
      deserializedAccountData.solPrice,
      160_000_000_000,
      "sol price set failed"
    );
  });
});
describe("Test Create Pool", () => {
  it("create pool", async () => {
    console.log("creating pool for", mint.publicKey.toBase58());
    const mainStatePDA = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(MAIN_STATE_SEED)],
      program.programId
    );
    try {
      const deserializedAccountData = await program.account.mainState.fetch(
        mainStatePDA[0].toBase58()
      );
      if (deserializedAccountData.initialized == false) {
        console.log("not initialized");
        return;
      }
    } catch {
      assert(false, "create pool failed");
    }

    const [metadataAccount] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM.toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      METADATA_PROGRAM
    );
    const [poolState] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), mint.publicKey.toBuffer()],
      program.programId
    );
    const [userState] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), signer.publicKey.toBuffer()],
      program.programId
    );
    const [reserveAta] = web3.PublicKey.findProgramAddressSync(
      [
        poolState.toBuffer(),
        TOKEN_PROGRAM.toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM
    );
    console.log("metadata account ->", metadataAccount.toBase58());
    const builder = program.methods
      .createPool({
        mintName: "bimple the boring bird",
        mintSymbol: "BBB",
        mintUri: "https://cryptologos.cc/logos/solana-sol-logo.svg",
        tradeStartTime: new BN(0),
        taxType: {
          higherSellTax: {
            thresholdPercentage: new BN(3000),
            higherTaxRate: new BN(20000),
            standardTaxRate: new BN(5000),
            duration: { lifetime: {} },
          },
        },
        waitingRoomConfig: {
          minTrades: 0,
          maxParticipants: 500,
          walletLimitPercent: 2,
          closureCondition: { 
            participantCount: {
              maxParticipants: 500
            } 
          },
        }
      })
      .accounts({
        mint: mint.publicKey,
        creator: signer.publicKey,
        metadataAccount,
        mainState: mainStatePDA[0],
        poolState,
        userState,
        referrer: web3.PublicKey.default,
        reserverBaseAta: reserveAta,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
        tokenProgram: TOKEN_PROGRAM,
        metadataProgram: METADATA_PROGRAM,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([mint]);
    const txHash = await builder.rpc({ commitment: "confirmed", skipPreflight: true });
    console.log("Created pool ->", poolState.toBase58());
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  });
});
describe("Test Buy and Sell", () => {
  const mintPublickey = mint.publicKey;
  it("buy", async () => {
    const mainStatePDA = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(MAIN_STATE_SEED)],
      program.programId
    );
    try {
      const deserializedAccountData = await program.account.mainState.fetch(
        mainStatePDA[0].toBase58()
      );
      if (deserializedAccountData.initialized == false) {
        console.log("not initialized");
        return;
      }
    } catch { }
    const [poolState] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), mintPublickey.toBuffer()],
      program.programId
    );
    try {
      const poolStateData = await program.account.poolState.fetch(
        poolState.toBase58()
      );
      console.log("buying for", poolStateData.mint.toBase58());
      if (poolStateData.mint.toBase58() != mintPublickey.toBase58()) {
        console.log("pool wasn't initialized");
      }
    } catch {
      return;
    }
    const [reservePda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("reserve"), mintPublickey.toBuffer()],
      program.programId
    );
    const [userState] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), signer.publicKey.toBuffer()],
      program.programId
    );
    const [reserveAta] = web3.PublicKey.findProgramAddressSync(
      [
        poolState.toBuffer(),
        TOKEN_PROGRAM.toBuffer(),
        mintPublickey.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM
    );
    const [buyerBaseAta] = web3.PublicKey.findProgramAddressSync(
      [
        signer.publicKey.toBuffer(),
        TOKEN_PROGRAM.toBuffer(),
        mintPublickey.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM
    );
    const deserializedAccountData = await program.account.mainState.fetch(
      mainStatePDA[0].toBase58()
    );

    const message = new Uint8Array([]);
    const messageHash = new Uint8Array(sha256.array(message));
    const signature = secp256k1.ecdsaSign(messageHash, signer.secretKey.slice(0, 32));
    const recoveryId = signature.recid;
    const signatureBytes = signature.signature;
    const serializedSignature = new Uint8Array([...signatureBytes, recoveryId]);

    const builder = program.methods.buy({
      amount: new BN(100000000),
      signature: Array.from(serializedSignature),
    })
      .accounts({
        buyer: signer.publicKey,
        mainState: mainStatePDA[0],
        feeRecipient: deserializedAccountData.feeRecipient,
        userState,
        referrer: web3.PublicKey.default,
        poolState,
        mint: mintPublickey,
        buyerBaseAta,
        reservePda,
        reserverBaseAta: reserveAta,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
        tokenProgram: TOKEN_PROGRAM,
        systemProgram: web3.SystemProgram.programId,
      });
    try {
      const txHash = await builder.rpc({ commitment: "confirmed" });
      console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    } catch {
      const tx = await builder.transaction();
      const { blockhash: recentBlockhash } =
        await program.provider.connection.getLatestBlockhash("confirmed");
      const message = new web3.TransactionMessage({
        payerKey: signer.publicKey,
        recentBlockhash,
        instructions: tx.instructions,
      }).compileToV0Message();
      const txMain = new web3.VersionedTransaction(message);
      txMain.sign([signer]);
      const simRes = await program.provider.connection.simulateTransaction(txMain);
      console.log(simRes.value);
    }
  });

  it("sell", async () => {
    const mainStatePDA = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(MAIN_STATE_SEED)],
      program.programId
    );
    try {
      const deserializedAccountData = await program.account.mainState.fetch(
        mainStatePDA[0].toBase58()
      );
      if (deserializedAccountData.initialized == false) {
        console.log("not initialized");
        return;
      }
    } catch { }
    const [poolState] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), mintPublickey.toBuffer()],
      program.programId
    );
    try {
      const poolStateData = await program.account.poolState.fetch(
        poolState.toBase58()
      );
      console.log("selling for", poolStateData.mint.toBase58());
      if (poolStateData.mint.toBase58() != mintPublickey.toBase58()) {
        console.log("pool wasn't initialized");
      }
    } catch {
      return;
    }
    const [reservePda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("reserve"), mintPublickey.toBuffer()],
      program.programId
    );
    const [userState] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), signer.publicKey.toBuffer()],
      program.programId
    );
    const [reserveAta] = web3.PublicKey.findProgramAddressSync(
      [
        poolState.toBuffer(),
        TOKEN_PROGRAM.toBuffer(),
        mintPublickey.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM
    );
    const [sellerBaseAta] = web3.PublicKey.findProgramAddressSync(
      [
        signer.publicKey.toBuffer(),
        TOKEN_PROGRAM.toBuffer(),
        mintPublickey.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM
    );
    const deserializedAccountData = await program.account.mainState.fetch(
      mainStatePDA[0].toBase58()
    );

    const message = new Uint8Array([]);
    const messageHash = new Uint8Array(sha256.array(message));
    const signature = secp256k1.ecdsaSign(messageHash, signer.secretKey.slice(0, 32));
    const recoveryId = signature.recid;
    const signatureBytes = signature.signature;
    const serializedSignature = new Uint8Array([...signatureBytes, recoveryId]);

    const builder = program.methods
      .sell({
        amount: new BN(3_000_000_000_000),
        signature: Array.from(serializedSignature),
        lastReceivedTime: new BN(Date.now() / 1000 - 86400)
      })
      .accounts({
        seller: signer.publicKey,
        mainState: mainStatePDA[0],
        feeRecipient: deserializedAccountData.feeRecipient,
        userState,
        referrer: web3.PublicKey.default,
        poolState,
        mint: mintPublickey,
        sellerBaseAta,
        reservePda,
        reserverBaseAta: reserveAta,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
        tokenProgram: TOKEN_PROGRAM,
        systemProgram: web3.SystemProgram.programId,
      });
    try {
      const txHash = await builder.rpc({ commitment: "confirmed" });
      console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    } catch {
      const tx = await builder.transaction();
      const { blockhash: recentBlockhash } =
        await program.provider.connection.getLatestBlockhash("confirmed");
      const message = new web3.TransactionMessage({
        payerKey: signer.publicKey,
        recentBlockhash,
        instructions: tx.instructions,
      }).compileToV0Message();
      const txMain = new web3.VersionedTransaction(message);
      txMain.sign([signer]);
      const simRes = await program.provider.connection.simulateTransaction(txMain);
      console.log(simRes.value);
    }
  });
});

// can't test withdraw because bonding curve is not completed, pool has buy limit for each wallet, so can't buy all amount
// it's impossible to withdraw before bonding curve is completed
// describe("Withdraw", () => {
//   const mintPublickey = mint.publicKey;
//   it("withdraw", async () => {
//     const mainStatePDA = web3.PublicKey.findProgramAddressSync(
//       [Buffer.from(MAIN_STATE_SEED)],
//       program.programId
//     );
//     try {
//       const deserializedAccountData = await program.account.mainState.fetch(
//         mainStatePDA[0].toBase58()
//       );
//       if (deserializedAccountData.initialized == false) {
//         console.log("not initialized");
//         return;
//       }
//     } catch {}
//     const [poolState] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("pool"), mintPublickey.toBuffer()],
//       program.programId
//     );
//     try {
//       const poolStateData = await program.account.poolState.fetch(
//         poolState.toBase58()
//       );
//       if (poolStateData.mint.toBase58() != mintPublickey.toBase58()) {
//         console.log("pool wasn't initialized");
//       }
//     } catch {
//       return;
//     }
//     const [reservePda] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("reserve"), mintPublickey.toBuffer()],
//       program.programId
//     );
//     const [userState] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("user"), signer.publicKey.toBuffer()],
//       program.programId
//     );
//     const [reserveAta] = web3.PublicKey.findProgramAddressSync(
//       [
//         poolState.toBuffer(),
//         TOKEN_PROGRAM.toBuffer(),
//         mintPublickey.toBuffer(),
//       ],
//       ASSOCIATED_TOKEN_PROGRAM
//     );
//     const [ownerBaseAta] = web3.PublicKey.findProgramAddressSync(
//       [
//         signer.publicKey.toBuffer(),
//         TOKEN_PROGRAM.toBuffer(),
//         mintPublickey.toBuffer(),
//       ],
//       ASSOCIATED_TOKEN_PROGRAM
//     );
//     const deserializedAccountData = await program.account.mainState.fetch(
//       mainStatePDA[0].toBase58()
//     );
//     const builder = program.methods.withdraw().accounts({
//       owner: signer.publicKey,
//       mainState: mainStatePDA[0],
//       poolState,
//       mint: mintPublickey,
//       ownerBaseAta,
//       reservePda,
//       reserverBaseAta: reserveAta,
//       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
//       tokenProgram: TOKEN_PROGRAM,
//       systemProgram: web3.SystemProgram.programId,
//     });
//     try {
//       const txHash = await builder.rpc({ commitment: "confirmed" });
//       console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
//     } catch {
//       const tx = await builder.transaction();
//       const { blockhash: recentBlockhash } =
//         await program.provider.connection.getLatestBlockhash("confirmed");
//       const message = new web3.TransactionMessage({
//         payerKey: signer.publicKey,
//         recentBlockhash,
//         instructions: tx.instructions,
//       }).compileToV0Message();
//       const txMain = new web3.VersionedTransaction(message);
//       txMain.sign([signer]);
//       const simRes = await program.provider.connection.simulateTransaction(txMain);
//       console.log(simRes.value);
//     }
//   });
// });
