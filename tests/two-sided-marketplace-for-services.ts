import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TwoSidedMarketplaceForServices } from "../target/types/two_sided_marketplace_for_services";
import { DataState, MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";

describe("two-sided-marketplace-for-services", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TwoSidedMarketplaceForServices as Program<TwoSidedMarketplaceForServices>;

  it("Can create an Asset", async () => {
    const asset = anchor.web3.Keypair.generate();
    // Add your test here.
    const tx = await program.methods.createService({
      name: "Hello Anchor!",
      uri: "www.example.com"
    })
      .accounts({
        asset: asset.publicKey,
        collection: null,
        payer: anchor.getProvider().publicKey,
        owner: null,
        updateAuthority: null,
        systemProgram: anchor.web3.SystemProgram.programId,
        logWrapper: null,
        mplCore: MPL_CORE_PROGRAM_ID,
      })
      .signers([asset])
      .rpc();
    console.log("Your transaction signature", tx);
  });

});
