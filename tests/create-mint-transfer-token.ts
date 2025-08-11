import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CreateMintTransferToken } from "../target/types/create_mint_transfer_token";
import { Keypair } from "@solana/web3.js";
import { getAccount, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { expect } from "chai";

describe("create-mint-transfer-token", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.createMintTransferToken as Program<CreateMintTransferToken>;

  const metadata = {
    name: 'Solana Gold',
    symbol: 'GOLDSOL',
    uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
  };

  const mintKeypair = new Keypair();

  const receiver = new Keypair();

  const senderTokenAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, payer.publicKey)
  const receiverTokenAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, receiver.publicKey)

  it("Create mint", async () => {
    const tx = await program.methods
      .createToken(metadata.name, metadata.symbol, metadata.uri)
      .accounts({mintAccount: mintKeypair.publicKey, payer: payer.publicKey})
      .signers([mintKeypair])
      .rpc();

    console.log(`Mint -> ${mintKeypair.publicKey} created successful !`)
    console.log("Your transaction signature", tx);
  });

  it("Mint tokens", async() => {
    const amount = new anchor.BN(69);

    const tx = await program.methods
      .mintToken(amount)
      .accounts({
        mintAccount: mintKeypair.publicKey,
        mintAuthority: payer.publicKey,
        receiver: payer.publicKey,
        //@ts-ignore
        associatedTokenAccount: senderTokenAddress
      })
      .rpc()

    const sendTokenAccount = await getAccount(provider.connection, senderTokenAddress);
  
    expect(amount.toNumber() * 1000_000_000).to.eq(Number(sendTokenAccount.amount));
    console.log(`Minted ${amount} tokens to ${senderTokenAddress}`);
    console.log("Your transaction signature", tx);
  })

  it("Transfer tokens", async() => {
    const amount = new anchor.BN(10 * 1000_000_000);

    const tx = await program.methods
      .transferTokens(amount)
      .accounts({
        sender: payer.publicKey,
        receiver: receiver.publicKey,
        mintAccount: mintKeypair.publicKey,
        // @ts-ignore
        receiverTokenAccount: receiverTokenAddress
      })
      .rpc();

      const receiverTokenAccount = await getAccount(provider.connection, receiverTokenAddress);

      expect(amount.toNumber()).to.eq(Number(receiverTokenAccount.amount));
      console.log(`Transfers ${amount} tokens from ${senderTokenAddress} to ${receiverTokenAddress}`);
      console.log("Your transaction signature", tx);
  })
});
