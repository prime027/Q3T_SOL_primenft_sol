import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "./target/types/vault";

describe("vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const user = provider.wallet;

  let vaultStateAccount = null;

  const program = anchor.workspace.AnchorVaultQ32024 as Program<Vault>;

  it("Is initialized!", async () => {
    // Add your test here.
    vaultStateAccount = anchor.web3.Keypair.generate();
    const tx = await program.methods.initialize().accounts({
      user: user.publicKey,
      vaultState: vaultStateAccount.publicKey,
      vault: anchor.web3.SystemProgram.programId,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([vaultStateAccount]).rpc();

    console.log("Your transaction signature", tx);
  });
});
