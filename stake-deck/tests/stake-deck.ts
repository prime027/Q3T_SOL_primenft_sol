import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { StakeDeck } from "../target/types/stake_deck";
import { Buffer } from "buffer";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("stake-deck", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace.StakeDeck as Program<StakeDeck>;

  let player1: anchor.web3.Keypair;
  let player2: anchor.web3.Keypair;
  let gamePda: PublicKey;
  let vaultPda: PublicKey;

  it("Initializes a game", async () => {
    player1 = anchor.web3.Keypair.generate();

    // Request airdrop for player1
    const airdropSignature = await connection.requestAirdrop(player1.publicKey, 3 * anchor.web3.LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature);

    // Ensure player1 has enough balance
    let player1Balance = await connection.getBalance(player1.publicKey);
    assert.isTrue(player1Balance >= 2 * anchor.web3.LAMPORTS_PER_SOL, "Player 1 should have enough balance");

    // Rent exemption
    const rentExemption = await connection.getMinimumBalanceForRentExemption(
      program.account.gameAccount.size
    );

    // Derive PDAs
    [gamePda] = await PublicKey.findProgramAddress(
      [Buffer.from("player"), player1.publicKey.toBuffer()],
      program.programId
    );
    [vaultPda] = await PublicKey.findProgramAddress(
      [Buffer.from("vault"), gamePda.toBuffer()],
      program.programId
    );

    const minBet = new BN(1_000_000); // 1 SOL in lamports
    const maxPlayers = 4;
    const feePercentage = 5;
    const payoutPercentage = 95;

    // Initialize the game
    await program.methods
      .initializeGame(minBet, maxPlayers, feePercentage, payoutPercentage)
      .accounts({
        firstPlayer: player1.publicKey,
        gameAccount: gamePda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([player1])
      .rpc();

    // Verify initialization
    let gameAccount = await program.account.gameAccount.fetch(gamePda);
    assert.equal(gameAccount.minBet.toNumber(), minBet.toNumber());
    assert.equal(gameAccount.maxPlayers, maxPlayers);
  });

  it("Starts a game", async () => {
    player2 = anchor.web3.Keypair.generate();

    // Airdrop for player2
    const airdropSignature = await connection.requestAirdrop(player2.publicKey, 3 * anchor.web3.LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature);

    await program.methods
      .startGame()
      .accounts({
        player: player2.publicKey,
        gameAccount: gamePda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([player2])
      .rpc();

    const gameAccount = await program.account.gameAccount.fetch(gamePda);
    assert.equal(gameAccount.players.length, 2, "There should be two players in the game");
  });

  it("Places a bet", async () => {
    const betAmount = new BN(1_000_000);

    // Verify player1 has enough balance
    const playerLamports = await connection.getBalance(player1.publicKey);
    assert.isTrue(playerLamports >= betAmount.toNumber(), "Player does not have enough SOL to place the bet");

    await program.methods
      .placeBet(betAmount)
      .accounts({
        user: player1.publicKey,
        gameAccount: gamePda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([player1])
      .rpc();

    const gameAccount = await program.account.gameAccount.fetch(gamePda);
    const player = gameAccount.players.find(p => p.pubkey.equals(player1.publicKey));
    assert.equal(player.betAmount.toNumber(), betAmount.toNumber());
  });
});
