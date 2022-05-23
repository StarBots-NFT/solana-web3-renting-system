import * as anchor from '@project-serum/anchor';
import fs from 'fs';
import * as bs58 from 'bs58';
import {web3, utils, BN, Accounts} from '@project-serum/anchor'
import {IdlAccountItem} from "@project-serum/anchor/dist/cjs/idl";
import {ASSOCIATED_PROGRAM_ID} from "@project-serum/anchor/src/utils/token";
import {PublicKey} from "@solana/web3.js";

const provider = anchor.AnchorProvider.env();

anchor.setProvider(provider);

export const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
);

async function main() {
    // Read the generated IDL.
    // Address of the deployed program.
    const programId = new anchor.web3.PublicKey('2QofFxHF1vPYKp6hUhi83iZpwLfXSKZsb1QKZwZoDSR8');

    const keypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array([
        224, 34, 123, 251, 222, 121, 159, 205, 198, 219, 97,
        146, 169, 24, 121, 140, 212, 142, 48, 10, 14, 98,
        238, 27, 166, 100, 78, 216, 75, 148, 248, 40, 29,
        122, 163, 221, 110, 183, 140, 40, 33, 251, 150, 160,
        88, 6, 228, 137, 59, 211, 106, 141, 186, 236, 69,
        56, 215, 190, 227, 33, 108, 44, 209, 120
    ]))
    // Generate the program client from IDL.

    const idl = await anchor.Program.fetchIdl(programId.toString());
    const program = new anchor.Program(idl, programId);
    const mint = new anchor.web3.PublicKey("AFFr2Bg6r5ZbKnXK2ny3agBPAesJZEy8H2jHGWTrpUMe")

    const item = await web3.PublicKey.findProgramAddress(
        [Buffer.from('ballot'), mint.toBuffer(), provider.wallet.publicKey.toBuffer()],
        program.programId,
    )

    const metaDataAddress = await anchor.web3.PublicKey.findProgramAddress(
        [
            Buffer.from('metadata'),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID,
    )

    const nftAta = new anchor.web3.PublicKey("GqquvykDyo9diYN8pgbtb11bJQqQ1U8vDGMbm8gsYPTt");

    const [treasurerPublicKey] = await web3.PublicKey.findProgramAddress(
        [Buffer.from('treasurer'), item[0].toBuffer()],
        program.programId,
    )
    console.log("treasurerPublicKey: ", treasurerPublicKey.toString())

    let nftHolder = await utils.token.associatedAddress({
        mint: mint,
        owner: treasurerPublicKey,
    })

    let treasurer = treasurerPublicKey

    console.log('idl: ', idl.instructions[0].accounts)
    console.log('idl: ', idl.instructions[0].args)
    console.log('ata: ', utils.token.ASSOCIATED_PROGRAM_ID.toString())
    console.log('rent: ', web3.SYSVAR_RENT_PUBKEY.toString())
    console.log('mint: ', mint.toString())
    console.log('metadata: ', metaDataAddress[0].toString())

    // price tính theo lampart là bội số của 10
    const price = new BN(110);
    // priods tính theo s là bội số của ngày
    const period = new BN(0 * 86400);
    // isListing : 1: tiếp tục tự động list, 0: ko tiếp tục tự động list
    const isListing = new BN(1)
    const tx = await program.rpc.listItem(price, period, isListing,{
        accounts: {
            authority: provider.wallet.publicKey,
            item: item[0],
            treasurer: treasurer,
            mint: mint,
            nftHolder: nftHolder,
            nftAta: nftAta,
            nftMetadataAddress: metaDataAddress[0],
            systemProgram: web3.SystemProgram.programId,
            tokenProgram: utils.token.TOKEN_PROGRAM_ID,
            associatedTokenProgram: utils.token.ASSOCIATED_PROGRAM_ID,
            rent: web3.SYSVAR_RENT_PUBKEY,
        },
    })
    console.log('tx: ', tx)
}

main().then(() => console.log('Success'));
