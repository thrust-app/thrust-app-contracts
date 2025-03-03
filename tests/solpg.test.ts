import {
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    MINT_SIZE,
    AuthorityType,
    getMinimumBalanceForRentExemptMint,
    createInitializeMintInstruction,
    getAssociatedTokenAddress,
    createAssociatedTokenAccountInstruction,
    createMintToInstruction,
    createSetAuthorityInstruction,
  } from "@solana/spl-token";
  import {
    createCreateMetadataAccountV3Instruction,
    PROGRAM_ID,
  } from "@metaplex-foundation/mpl-token-metadata";
  
  const MAIN_STATE_SEED = "main_4";
  const signer = pg.wallet.keypair;
  const TOKEN_PROGRAM = TOKEN_PROGRAM_ID;
  const ASSOCIATED_TOKEN_PROGRAM = ASSOCIATED_TOKEN_PROGRAM_ID;
  const METADATA_PROGRAM = PROGRAM_ID;
  let mint = web3.Keypair.generate();
  describe("Test Initialize", () => {
    it("init", async () => {
      const mainStatePDA = web3.PublicKey.findProgramAddressSync(
        [Buffer.from(MAIN_STATE_SEED)],
        pg.PROGRAM_ID
      );
      try {
        const deserializedAccountData = await pg.program.account.mainState.fetch(
          mainStatePDA[0].toBase58()
        );
        if (deserializedAccountData.initialized == true) {
          console.log("already initialized");
          return;
        }
      } catch {}
      const tx = await pg.program.methods
        .initMainState()
        .accounts({
          owner: pg.wallet.publicKey,
          mainState: mainStatePDA[0],
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();
      console.log(`Use 'solana confirm -v ${tx}' to see the logs`);
      const deserializedAccountData = await pg.program.account.mainState.fetch(
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
        pg.PROGRAM_ID
      );
      console.log("updating...");
      const tx = await pg.program.methods
        .updateMainState({
          owner: pg.wallet.publicKey,
          feeRecipient: pg.wallet.publicKey,
          tradingFee: new BN(1000),
          referralRewardFee: new BN(10000),
          referralTradeLimit: new BN(100),
          totalTokenSupply: new BN(1_000_000_000 * 1000_000),
          initRealBaseReserves: new BN(800_000_000 * 1000_000),
          initVirtBaseReserves: new BN(200_000_000 * 1000_000),
          initVirtQuoteReserves: new BN(24 * 1000_000_000),
        })
        .accounts({
          owner: pg.wallet.publicKey,
          mainState: mainStatePDA[0],
        })
        .rpc();
      await pg.connection.confirmTransaction(tx);
      console.log(`Use 'solana confirm -v ${tx}' to see the logs`);
  
      const deserializedAccountData = await pg.program.account.mainState.fetch(
        mainStatePDA[0].toBase58()
      );
      assert.equal(
        deserializedAccountData.initialized,
        true,
        "Initialize Failed"
      );
      assert.equal(
        deserializedAccountData.owner.toBase58(),
        pg.wallet.publicKey.toBase58(),
        "owner was not updated"
      );
      assert.equal(
        deserializedAccountData.feeRecipient.toBase58(),
        pg.wallet.publicKey.toBase58(),
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
        pg.PROGRAM_ID
      );
      const tx = await pg.program.methods
        .updateSolPrice(new BN(160_000_000_000))
        .accounts({
          owner: pg.wallet.publicKey,
          mainState: mainStatePDA[0],
        })
        .rpc();
      await pg.connection.confirmTransaction(tx);
      console.log(`Use 'solana confirm -v ${tx}' to see the logs`);
  
      const deserializedAccountData = await pg.program.account.mainState.fetch(
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
        pg.PROGRAM_ID
      );
      try {
        const deserializedAccountData = await pg.program.account.mainState.fetch(
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
        pg.PROGRAM_ID
      );
      const [userState] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user"), pg.wallet.publicKey.toBuffer()],
        pg.PROGRAM_ID
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
      const builder = pg.program.methods
        .createPool({
          mintName: "bimple the boring bird",
          mintSymbol: "BBB",
          mintUri: "https://cryptologos.cc/logos/solana-sol-logo.svg",
          tradeStartTime: new BN(0),
        })
        .accounts({
          mint: mint.publicKey,
          creator: pg.wallet.publicKey,
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
      // try {
      const txHash = await builder.rpc({ commitment: "confirmed" });
      console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
      // } catch {
      //   const tx = await builder.transaction();
      //   const { blockhash: recentBlockhash } =
      //     await pg.connection.getLatestBlockhash("confirmed");
      //   const message = new web3.TransactionMessage({
      //     payerKey: pg.wallet.publicKey,
      //     recentBlockhash,
      //     instructions: tx.instructions,
      //   }).compileToV0Message();
      //   const txMain = new web3.VersionedTransaction(message);
      //   txMain.sign([pg.wallet.keypair, mint]);
      //   const simRes = await pg.connection.simulateTransaction(txMain);
      //   console.log(simRes.value);
      // }
    });
  });
  describe("Test Buy and Sell", () => {
    it("buy", async () => {
      const mainStatePDA = web3.PublicKey.findProgramAddressSync(
        [Buffer.from(MAIN_STATE_SEED)],
        pg.PROGRAM_ID
      );
      try {
        const deserializedAccountData = await pg.program.account.mainState.fetch(
          mainStatePDA[0].toBase58()
        );
        if (deserializedAccountData.initialized == false) {
          console.log("not initialized");
          return;
        }
      } catch {}
      const [poolState] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("pool"), mint.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      try {
        const poolStateData = await pg.program.account.poolState.fetch(
          poolState.toBase58()
        );
        console.log("buying for", poolStateData.mint.toBase58());
        if (poolStateData.mint.toBase58() != mint.publicKey.toBase58()) {
          console.log("pool wasn't initialized");
        }
      } catch {
        return;
      }
      const [reservePda] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("reserve"), mint.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      const [userState] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user"), pg.wallet.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      const [reserveAta] = web3.PublicKey.findProgramAddressSync(
        [
          poolState.toBuffer(),
          TOKEN_PROGRAM.toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM
      );
      const [buyerBaseAta] = web3.PublicKey.findProgramAddressSync(
        [
          pg.wallet.publicKey.toBuffer(),
          TOKEN_PROGRAM.toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM
      );
      const deserializedAccountData = await pg.program.account.mainState.fetch(
        mainStatePDA[0].toBase58()
      );
      const builder = pg.program.methods.buy(new BN(100000000)).accounts({
        buyer: pg.wallet.publicKey,
        mainState: mainStatePDA[0],
        feeRecipient: deserializedAccountData.feeRecipient,
        userState,
        referrer: web3.PublicKey.default,
        poolState,
        mint: mint.publicKey,
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
          await pg.connection.getLatestBlockhash("confirmed");
        const message = new web3.TransactionMessage({
          payerKey: pg.wallet.publicKey,
          recentBlockhash,
          instructions: tx.instructions,
        }).compileToV0Message();
        const txMain = new web3.VersionedTransaction(message);
        txMain.sign([pg.wallet.keypair]);
        const simRes = await pg.connection.simulateTransaction(txMain);
        console.log(simRes.value);
      }
    });
  
    it("sell", async () => {
      const mainStatePDA = web3.PublicKey.findProgramAddressSync(
        [Buffer.from(MAIN_STATE_SEED)],
        pg.PROGRAM_ID
      );
      try {
        const deserializedAccountData = await pg.program.account.mainState.fetch(
          mainStatePDA[0].toBase58()
        );
        if (deserializedAccountData.initialized == false) {
          console.log("not initialized");
          return;
        }
      } catch {}
      const [poolState] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("pool"), mint.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      try {
        const poolStateData = await pg.program.account.poolState.fetch(
          poolState.toBase58()
        );
        console.log("selling for", poolStateData.mint.toBase58());
        if (poolStateData.mint.toBase58() != mint.publicKey.toBase58()) {
          console.log("pool wasn't initialized");
        }
      } catch {
        return;
      }
      const [reservePda] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("reserve"), mint.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      const [userState] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user"), pg.wallet.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      const [reserveAta] = web3.PublicKey.findProgramAddressSync(
        [
          poolState.toBuffer(),
          TOKEN_PROGRAM.toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM
      );
      const [sellerBaseAta] = web3.PublicKey.findProgramAddressSync(
        [
          pg.wallet.publicKey.toBuffer(),
          TOKEN_PROGRAM.toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM
      );
      const deserializedAccountData = await pg.program.account.mainState.fetch(
        mainStatePDA[0].toBase58()
      );
      const builder = pg.program.methods
        .sell(new BN(3_000_000_000_000))
        .accounts({
          seller: pg.wallet.publicKey,
          mainState: mainStatePDA[0],
          feeRecipient: deserializedAccountData.feeRecipient,
          userState,
          referrer: web3.PublicKey.default,
          poolState,
          mint: mint.publicKey,
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
          await pg.connection.getLatestBlockhash("confirmed");
        const message = new web3.TransactionMessage({
          payerKey: pg.wallet.publicKey,
          recentBlockhash,
          instructions: tx.instructions,
        }).compileToV0Message();
        const txMain = new web3.VersionedTransaction(message);
        txMain.sign([pg.wallet.keypair]);
        const simRes = await pg.connection.simulateTransaction(txMain);
        console.log(simRes.value);
      }
    });
  });
  describe("Withdraw", () => {
    it("ending", async () => {
      const mainStatePDA = web3.PublicKey.findProgramAddressSync(
        [Buffer.from(MAIN_STATE_SEED)],
        pg.PROGRAM_ID
      );
      try {
        const deserializedAccountData = await pg.program.account.mainState.fetch(
          mainStatePDA[0].toBase58()
        );
        if (deserializedAccountData.initialized == false) {
          console.log("not initialized");
          return;
        }
      } catch {}
      const [poolState] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("pool"), mint.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      try {
        const poolStateData = await pg.program.account.poolState.fetch(
          poolState.toBase58()
        );
        console.log("buying for", poolStateData.mint.toBase58());
        if (poolStateData.mint.toBase58() != mint.publicKey.toBase58()) {
          console.log("pool wasn't initialized");
        }
      } catch {
        return;
      }
      const [reservePda] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("reserve"), mint.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      const [userState] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user"), pg.wallet.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      const [reserveAta] = web3.PublicKey.findProgramAddressSync(
        [
          poolState.toBuffer(),
          TOKEN_PROGRAM.toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM
      );
      const [buyerBaseAta] = web3.PublicKey.findProgramAddressSync(
        [
          pg.wallet.publicKey.toBuffer(),
          TOKEN_PROGRAM.toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM
      );
      const deserializedAccountData = await pg.program.account.mainState.fetch(
        mainStatePDA[0].toBase58()
      );
      const builder = pg.program.methods.buy(new BN(2000000000)).accounts({
        buyer: pg.wallet.publicKey,
        mainState: mainStatePDA[0],
        feeRecipient: deserializedAccountData.feeRecipient,
        userState,
        referrer: web3.PublicKey.default,
        poolState,
        mint: mint.publicKey,
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
          await pg.connection.getLatestBlockhash("confirmed");
        const message = new web3.TransactionMessage({
          payerKey: pg.wallet.publicKey,
          recentBlockhash,
          instructions: tx.instructions,
        }).compileToV0Message();
        const txMain = new web3.VersionedTransaction(message);
        txMain.sign([pg.wallet.keypair]);
        const simRes = await pg.connection.simulateTransaction(txMain);
        console.log(simRes.value);
      }
    });
    it("withdraw", async () => {
      const mainStatePDA = web3.PublicKey.findProgramAddressSync(
        [Buffer.from(MAIN_STATE_SEED)],
        pg.PROGRAM_ID
      );
      try {
        const deserializedAccountData = await pg.program.account.mainState.fetch(
          mainStatePDA[0].toBase58()
        );
        if (deserializedAccountData.initialized == false) {
          console.log("not initialized");
          return;
        }
      } catch {}
      const [poolState] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("pool"), mint.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      try {
        const poolStateData = await pg.program.account.poolState.fetch(
          poolState.toBase58()
        );
        if (poolStateData.mint.toBase58() != mint.publicKey.toBase58()) {
          console.log("pool wasn't initialized");
        }
      } catch {
        return;
      }
      const [reservePda] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("reserve"), mint.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      const [userState] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user"), pg.wallet.publicKey.toBuffer()],
        pg.PROGRAM_ID
      );
      const [reserveAta] = web3.PublicKey.findProgramAddressSync(
        [
          poolState.toBuffer(),
          TOKEN_PROGRAM.toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM
      );
      const [ownerBaseAta] = web3.PublicKey.findProgramAddressSync(
        [
          pg.wallet.publicKey.toBuffer(),
          TOKEN_PROGRAM.toBuffer(),
          mint.publicKey.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM
      );
      const deserializedAccountData = await pg.program.account.mainState.fetch(
        mainStatePDA[0].toBase58()
      );
      const builder = pg.program.methods.withdraw().accounts({
        owner: pg.wallet.publicKey,
        mainState: mainStatePDA[0],
        poolState,
        mint: mint.publicKey,
        ownerBaseAta,
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
          await pg.connection.getLatestBlockhash("confirmed");
        const message = new web3.TransactionMessage({
          payerKey: pg.wallet.publicKey,
          recentBlockhash,
          instructions: tx.instructions,
        }).compileToV0Message();
        const txMain = new web3.VersionedTransaction(message);
        txMain.sign([pg.wallet.keypair]);
        const simRes = await pg.connection.simulateTransaction(txMain);
        console.log(simRes.value);
      }
    });
  });
  