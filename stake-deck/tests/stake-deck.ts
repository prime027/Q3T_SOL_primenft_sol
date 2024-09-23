import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { StakeDeck } from "../target/types/stake_deck";
import { Buffer } from "buffer";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("stake-deck", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace.StakeDeck as Program<StakeDeck>;

  let player1: anchor.web3.Keypair;
  let player2: anchor.web3.Keypair;
  let gamePda: PublicKey;
  let vaultPda: PublicKey;

  it("Initializes a game" ,async () => {
    player1 = anchor.web3.Keypair.generate();
  

    console.log("----requesting airdrop");
    // Airdrop SOL to the player account
    await connection.requestAirdrop(player1.publicKey, anchor.web3.LAMPORTS_PER_SOL);

    console.log("---- airdrop gotten");

    // Create PDA for the game account
    [gamePda] = await PublicKey.findProgramAddress(
      [Buffer.from("player"), player1.publicKey.toBuffer()],
      program.programId
    );
    [vaultPda] = await PublicKey.findProgramAddress(
      [Buffer.from("vault"), gamePda.toBuffer()],
      program.programId
    );

    // Initialize the game before placing a bet
    const minBet = new BN(1_000_000); // Minimum bet in lamports
    const maxPlayers = 4; // Maximum number of players
    const feePercentage = 5; // Fee percentage
    const payoutPercentage = 95; // Payout percentage

    console.log("----initializing game");
    await program.methods
      .initializeGame(minBet, maxPlayers, feePercentage, payoutPercentage) // Pass the parameters
      .accountsPartial({
        firstPlayer: player1.publicKey,
        gameAccount: gamePda, // Corrected to gameAccount
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([player1])
      .rpc();
    console.log("----game initialised");

    // Fetch the game account to verify its state
    let gameAccount = await program.account.gameAccount.fetch(gamePda);

    //console.log("game state", gameAccount.gameState);
    // Assertions to verify the game state
    assert.equal(gameAccount.minBet.toNumber(), minBet.toNumber(), "Minimum bet should match");
    assert.equal(gameAccount.maxPlayers, maxPlayers, "Maximum players should match");
    assert.equal(gameAccount.feePercentage, feePercentage, "Fee percentage should match");
    assert.equal(gameAccount.payoutPercentage, payoutPercentage, "Payout percentage should match");
    assert.isTrue(gameAccount.gameState.waitingForPlayers !== undefined, "Game state should be WaitingForPlayers");
    assert.equal(gameAccount.players.length, 1, "There should be one player in the game");
    assert.equal(gameAccount.players[0].pubkey.toString(), player1.publicKey.toString(), "Player1 should be added to the game");

  });

  it("Starts a game", async () => {
    player2 = anchor.web3.Keypair.generate();
    await connection.requestAirdrop(player2.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    console.log("----starting game");
    // Add player2 to the game
    await program.methods
      .startGame() // Assuming you have a method to start the game
      .accountsPartial({
        player: player2.publicKey,
        gameAccount: gamePda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([player2])
      .rpc();
      console.log("----game started");

    // Fetch the game account to verify its state
   let gameAccount = await program.account.gameAccount.fetch(gamePda);
    assert.equal(gameAccount.players.length, 2, "There should be two players in the game");
    assert.equal(gameAccount.players[1].pubkey.toString(), player2.publicKey.toString(), "Player2 should be added to the game");

  })

  it("Places a bet", async () => {
    const betAmount = new BN(500_000); // Amount to bet (in lamports)

    // Ensure the player has enough SOL to place the bet
    const playerLamports = await connection.getBalance(player1.publicKey);
    assert.isTrue(playerLamports >= betAmount.toNumber(), "Player does not have enough SOL to place the bet");

    // Call the placeBet method
    await program.methods
      .placeBet(betAmount)
      .accountsPartial({
        user: player1.publicKey,
        gameAccount: gamePda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([player1])
      .rpc();

    // Fetch the game account to verify the bet was placed
    const gameAccount = await program.account.gameAccount.fetch(gamePda);

    // Assertions to verify the bet was placed correctly
    const player = gameAccount.players.find(p => p.pubkey.equals(player1.publicKey));
    assert.isNotNull(player, "Player should be in the game");
    assert.equal(player.bet_amount.toNumber(), betAmount.toNumber(), "Player's bet amount should match the placed bet");

    // Optionally, check the vault balance if you have access to it
    const vaultBalance = await connection.getBalance(vaultPda.P);
    assert.equal(vaultBalance, betAmount.toNumber(), "Vault balance should reflect the bet amount");
  });
});
