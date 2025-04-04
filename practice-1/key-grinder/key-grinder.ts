import {Keypair} from "@solana/web3.js"
import { performance } from 'perf_hooks'
import bs58 from 'bs58';


const myName = "anton"
const start = performance.now();

let match = false
let iteration = 0

while(!match) {
    iteration++
    console.log("Iteration #", iteration)
    const keypair = Keypair.generate()
    const pubKey = keypair.publicKey.toBase58()
    const pubKeyLowerCase = pubKey.toLowerCase()

    if (pubKeyLowerCase.startsWith(myName.toLowerCase())) {
        const end = performance.now();
    
        console.log('Found!');
        console.log('Public key:', pubKey);
        console.log('Number of attempts:',  iteration)
        console.log('Search time:', (end - start).toFixed(2), 'ms');
    
        match = true;
      }
    
}

