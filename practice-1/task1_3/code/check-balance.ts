import "dotenv/config"

import {
    Connection, 
    LAMPORTS_PER_SOL, 
    PublicKey,
    clusterApiUrl,
} from "@solana/web3.js"


import {
    airdropIfRequired
} from "@solana-developers/helpers"


const connection = new Connection(clusterApiUrl("devnet"))
console.log("Connected to devnet!")

const myPublicKey = new PublicKey("BKk8vDCwiBhFG98MDkGUyxPAU81kfZjz9VMEfAJAGae3")
console.log("My public key:", myPublicKey.toBase58())

const myBalanceInLamports = await connection.getBalance(myPublicKey)
const myBalanceInSOL = myBalanceInLamports/LAMPORTS_PER_SOL

console.log(`The balance for the wallet at address ${myPublicKey} is: ${myBalanceInSOL}`)

await airdropIfRequired(
    connection,
    myPublicKey,
    1 * LAMPORTS_PER_SOL,
    0.5 * LAMPORTS_PER_SOL
  );
  