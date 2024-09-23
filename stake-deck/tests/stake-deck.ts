import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { StakeDeck } from "../target/types/stake_deck";
import { Buffer } from "buffer";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from 'chai';

describe("stake-deck", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace.StakeDeck as Program<StakeDeck>;

  it("Initializes a player account", async () => {
    const player1 = anchor.web3.Keypair.generate();
    const player2 = anchor.web3.Keypair.generate();

    // Airdrop SOL to the player account
    const airdropSignature1 = await connection.requestAirdrop(player1.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature1);
    const airdropSignature2 = await connection.requestAirdrop(player2.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature2);

    const [playerPda1] = await PublicKey.findProgramAddress(
      [Buffer.from("player"), player1.publicKey.toBuffer()],
      program.programId
    );
    await program.methods
      .initializePlayer(0) // Pass the bump as a number
      .accountsPartial({
        player: player1.publicKey,
        playerAccount: playerPda1, // Corrected to playerAccount
        systemProgram: SystemProgram.programId,
      })
      .signers([player1])
      .rpc();

    // const betAmount1 = new BN(10);
    // await program.methods
    //   .placeBet(betAmount1)
    //   .accountsPartial({
    //     user: player1.publicKey,
    //     playersAccount: playerPda1, // Corrected to playersAccount
    //     systemProgram: SystemProgram.programId,
    //   })
    //   .signers([player1])
    //   .rpc();
      
    const [playerPda2] = await PublicKey.findProgramAddress(
      [Buffer.from("player"), player2.publicKey.toBuffer()],
      program.programId
    );
    await program.methods
      .initializePlayer(0) // Pass the bump as a number
      .accountsPartial({
        player: player2.publicKey,
        playerAccount: playerPda2, // Corrected to playerAccount
        systemProgram: SystemProgram.programId,
      })
      .signers([player2])
      .rpc();

    // const betAmount2 = new BN(20);
    // await program.methods
    //   .placeBet(betAmount2)
    //   .accountsPartial({
    //     user: player2.publicKey,
    //     playersAccount: playerPda2, // Corrected to playersAccount
    //     systemProgram: SystemProgram.programId,
    //   })
    //   .signers([player2])
    //   .rpc();

    // const account1 = await program.account.playersAccount.fetch(playerPda1);
    // expect(account1.player1.equals(player1.publicKey)).to.be.true;
    // expect(account1.betAmount.toNumber()).to.equal(betAmount1.toNumber());

    // const account2 = await program.account.playersAccount.fetch(playerPda2);
    // expect(account2.player2.equals(player2.publicKey)).to.be.true;
    // expect(account2.betAmount.toNumber()).to.equal(betAmount2.toNumber());
  });
});
