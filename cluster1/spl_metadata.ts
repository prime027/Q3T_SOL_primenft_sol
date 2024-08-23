import "dotenv/config";
import { readFileSync } from "fs";
import {
  Keypair,
  Connection,
  clusterApiUrl,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { 
  createUpdateMetadataAccountV2Instruction, 
  Metadata 
} from "@metaplex-foundation/mpl-token-metadata";
import { getExplorerLink } from "@solana-developers/helpers";

// Import wallet from JSON file
const wallet = JSON.parse(readFileSync("./wba-wallet.json", "utf8"));

// Create keypair from the imported wallet
const user = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection(clusterApiUrl("devnet"));

console.log(
  `🔑 We've loaded our keypair securely from the JSON file! Our public key is: ${user.publicKey.toBase58()}`
);

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

// Substitute in your token mint account
const tokenMintAccount = new PublicKey("2r8J4attqFRKHzwATCHgHMn5hvfFh2EA3JXCJNn5jBbo");

const metadataData = {
  name: "Skelly",
  symbol: "SKELLY",
  // Arweave / IPFS / Pinata etc link using metaplex standard for off-chain data
  uri: "https://arweave.net/TQgy5ot5on437YHXbE-boNSVNrCH-61FL8UCvu792wM",
  sellerFeeBasisPoints: 0,
  creators: null,
  collection: null,
  uses: null,
};

const metadataPDAAndBump = PublicKey.findProgramAddressSync(
  [
    Buffer.from("metadata"),
    TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    tokenMintAccount.toBuffer(),
  ],
  TOKEN_METADATA_PROGRAM_ID
);

const metadataPDA = metadataPDAAndBump[0];

(async () => {
  try {
    const accountInfo = await connection.getAccountInfo(metadataPDA);
    if (accountInfo === null) {
      throw new Error("Metadata account does not exist");
    }
    const metadata = Metadata.deserialize(accountInfo.data)[0];
    console.log("Fetched existing metadata account:", metadata);

    // Update the metadata
    const updateTransaction = new Transaction();

    const updateMetadataAccountInstruction =
      createUpdateMetadataAccountV2Instruction(
        {
          metadata: metadataPDA,
          updateAuthority: user.publicKey,
        },
        {
          updateMetadataAccountArgsV2: {
            data: metadataData,
            updateAuthority: user.publicKey,
            primarySaleHappened: metadata.primarySaleHappened,
            isMutable: metadata.isMutable,
          },
        }
      );

    updateTransaction.add(updateMetadataAccountInstruction);

    const updateTransactionSignature = await sendAndConfirmTransaction(
      connection,
      updateTransaction,
      [user]
    );

    const updateTransactionLink = getExplorerLink(
      "transaction",
      updateTransactionSignature,
      "devnet"
    );

    console.log(`✅ Metadata updated, explorer link is: ${updateTransactionLink}!`);
  } catch (e) {
    console.error("An error occurred:", e);
  }
})();
