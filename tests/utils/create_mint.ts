/** @format */

import { AnchorProvider, web3 } from "@coral-xyz/anchor";
import {
  createInitializeMintInstruction,
  MintLayout,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

// create new Token (Mint account)

export const createMint = async (
  provider: AnchorProvider,
  decimals = 0 // no decimals
): Promise<web3.PublicKey> => {
  
  //token key pair
  const tokenMint = new web3.Keypair();

  // calculate rent
  const lamportsForMint =
    await provider.connection.getMinimumBalanceForRentExemption(
      MintLayout.span // Mint layout
    );

  // allocate mint and wallet account
  await provider.sendAndConfirm(
    new web3.Transaction()
      .add(
        //create mint account
        web3.SystemProgram.createAccount({
          programId: TOKEN_PROGRAM_ID, // program_id
          space: MintLayout.span, //space
          fromPubkey: provider.wallet.publicKey, //payer
          newAccountPubkey: tokenMint.publicKey, // token address
          lamports: lamportsForMint, // rent
        })
      )
      .add(

        // initialize mint account
        createInitializeMintInstruction(
          tokenMint.publicKey, // token address
          decimals, // decimals
          provider.wallet.publicKey, //mint authority
          provider.wallet.publicKey, // freeze authority

        )
      ),
      [tokenMint] //signer
  );
  //return new token address
  return tokenMint.publicKey
};
