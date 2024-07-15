import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TwoSidedMarketplaceForServices } from "../target/types/two_sided_marketplace_for_services";
import { DataState, MPL_CORE_PROGRAM_ID, fetchAssetV1 } from "@metaplex-foundation/mpl-core";
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { getOrCreateAssociatedTokenAccount, ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID} from '@solana/spl-token';

describe("two-sided-marketplace-for-services", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TwoSidedMarketplaceForServices as Program<TwoSidedMarketplaceForServices>;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const connection = anchor.getProvider().connection;


  // USDC
  // 4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU
  // EURC
  // HzwqbKZw8HxMN6bF2yFZNrht3c2iXXzpKcFu7uBEDKtr
  let priceMint = new anchor.web3.PublicKey("HzwqbKZw8HxMN6bF2yFZNrht3c2iXXzpKcFu7uBEDKtr");

  let secretKeyArray = new Uint8Array([79,141,93,196,162,170,88,132,242,83,26,21,24,114,41,207,138,153,163,26,19,41,63,145,100,253,242,228,49,171,129,164,58,0,210,152,87,185,2,201,28,192,62,177,99,33,74,79,93,209,103,211,85,19,194,103,242,250,136,30,250,103,28,149]);
  
  let buyer = anchor.web3.Keypair.fromSecretKey(secretKeyArray);
  console.log("Buyer public key: ", buyer.publicKey.toString());
  

  const asset = anchor.web3.Keypair.generate();
  const newAsset = anchor.web3.Keypair.generate();


  const seedNumber = 49
  const listingSeed: any = new anchor.BN(seedNumber);
  const newListingSeed: any = new anchor.BN(seedNumber*1000);


  const listingPDA = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("listing"),
      program.provider.publicKey.toBuffer(),
      listingSeed.toBuffer("le", 8),
    ],
    program.programId,
  )[0];

  const newListingPDA = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("listing"),
      program.provider.publicKey.toBuffer(),
      newListingSeed.toBuffer("le", 8),
    ],
    program.programId,
  )[0];

  console.log("listing pda public key: ", listingPDA.toString());


  it("Can create a Service NFT", async () => {

    const tx = await program.methods.createService({
      name: "1:1 Rust & Anchor lesson (1 hour videocall)",
      uri: "https://turbin3.com/",
      royalty: 100,
      freezable: false,
    })
      .accounts({
        asset: asset.publicKey,
        collection: null,
        payer: anchor.getProvider().publicKey,
        owner: null,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([asset])
      .rpc();
    console.log("Your transaction signature: ", tx);
  });


  it("Can update a Service NFT", async () => {

    const tx = await program.methods.updateService({
      newName: "1:1 Rust & Anchor lesson (2 hours videocall)",
      newUri: null,
      newUpdateAuthority: null,
    })
      .accounts({
        asset: asset.publicKey,
        collection: null,
        authority: null,
        payer: anchor.getProvider().publicKey,
        owner: null,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([asset])
      .rpc();
    console.log("Your transaction signature: ", tx);
  });


  it("Can delete a Service NFT", async () => {

    const tx = await program.methods.deleteService(
      {compressionProof: null}
    )
      .accounts({
        asset: asset.publicKey,
        collection: null,
        authority: null,
        payer: anchor.getProvider().publicKey,
        owner: null,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([asset])
      .rpc();
    console.log("Your transaction signature", tx);
  });


  it("Can create a *new* Service NFT", async () => {

    const tx = await program.methods.createService({
      name: "1-1 Rust & Anchor lesson (1 hour videocall)",
      uri: "https://turbin3.com/",
      royalty: 100,
      freezable: true,
    })
      .accounts({
        asset: newAsset.publicKey,
        collection: null,
        payer: anchor.getProvider().publicKey,
        owner: null,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([newAsset])
      .rpc();
    console.log("Your transaction signature: ", tx);
  });


  it("Can create a Listing", async () => {

    const tx = await program.methods.createListing(
      {
        price: new anchor.BN(100),
        seed: listingSeed,
      }
    )
      .accounts({
        payer: anchor.getProvider().publicKey,
        priceMint: priceMint,
        listing: listingPDA,
        asset: newAsset.publicKey,
        collection: null,
        authority: null,
        logWrapper: null,
      })
      .rpc();
    console.log("Your transaction signature: ", tx);

    const listingLog = await program.account.listing.fetch(listingPDA);
    console.log("Listing account: ", listingLog);

  });


  it("Can update a Listing", async () => {

    const tx = await program.methods.updateListing(
      {
        price: new anchor.BN(90),
        seed: listingSeed,
      }
    )
      .accounts({
        payer: anchor.getProvider().publicKey,
        priceMint: priceMint,
        listing: listingPDA,
        asset: newAsset.publicKey,
        collection: null,
        authority: null,
        logWrapper: null,
      })
      .rpc();
    console.log("Your transaction signature: ", tx);

    const listingLog = await program.account.listing.fetch(listingPDA);
    console.log("Listing account: ", listingLog);

  });


  it("Can delete a Listing", async () => {

    const tx = await program.methods.deleteListing(
      {
        price: null,
        seed: listingSeed,
      }
    )
      .accounts({
        payer: anchor.getProvider().publicKey,
        priceMint: priceMint,
        listing: listingPDA,
        asset: newAsset.publicKey,
        collection: null,
        authority: null,
        logWrapper: null,
      })
      .rpc();
    console.log("Your transaction signature: ", tx);

  });


  it("Can create a *new* Listing", async () => {

    const tx = await program.methods.createListing(
      {
        price: new anchor.BN(36),
        seed: newListingSeed,
      }
    )
      .accounts({
        payer: anchor.getProvider().publicKey,
        priceMint: priceMint,
        listing: newListingPDA,
        asset: newAsset.publicKey,
        collection: null,
        authority: null,
        logWrapper: null,
      })
      .rpc();
    console.log("Your transaction signature: ", tx);

    const newListingLog = await program.account.listing.fetch(newListingPDA);
    console.log("Listing account: ", newListingLog);

  });


  it("Can buy a Service", async () => {

    let buyerAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      priceMint,
      buyer.publicKey,
      false
    );

    console.log(buyerAta.address);

    let listerAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      priceMint,
      anchor.getProvider().publicKey,
      false
    );


    const tx = await program.methods.buyNow(
      {
        price: new anchor.BN(1),
        seed: newListingSeed,
      }
    )
      .accounts({
        buyer: buyer.publicKey,
        buyerAta: buyerAta.address,
        lister: anchor.getProvider().publicKey,
        listerAta: listerAta.address,
        priceMint: priceMint,
        listing: newListingPDA,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        asset: newAsset.publicKey,
        collection: null,
        authority: newListingPDA,
        logWrapper: null,
        mplCore: MPL_CORE_PROGRAM_ID,
      })
      .signers([buyer])
      .rpc();
    console.log("Your transaction signature: ", tx);

  });


});
