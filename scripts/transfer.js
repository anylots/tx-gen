const { BigNumber } = require("ethers")
const { ethers } = require('ethers');
const Token_Artifact = require("../src/abi/Token.json");
require("dotenv").config({ path: ".env" });

// This is a script for deploying your contracts. You can adapt it to deploy
// yours, or create new ones.
async function main() {

    let privateKey = requireEnv("PRIVATE_KEY");
    let customHttpProvider = new ethers.providers.JsonRpcProvider(
        "http://localhost:8545"
    );

    const signer = new ethers.Wallet(privateKey, customHttpProvider);
    console.log("signer.address: " + signer.address);


    ///deploy ERC20 Token
    let token = new ethers.Contract(requireEnv("TOKEN_ADDRESS"), Token_Artifact.abi, signer);

    let nonce = await signer.getTransactionCount();
    // const gasPrice = await customHttpProvider.getGasPrice();
    const gasLimit = await token.estimateGas.transfer("0x70997970C51812dc3A010C7d01b50e0d17dc79C8", 1);
    console.log("gasLimit: " + gasLimit);

    console.log("current blockNumber: " + await customHttpProvider.getBlockNumber());
    for (let i = 0; i < 1000; i++) {
        try {

            const randomWallet = ethers.Wallet.createRandom();
            const address = randomWallet.address;

            const tx = await token.transfer(address, 1, {
                nonce: nonce + i,
                gasLimit: gasLimit,
                maxPriorityFeePerGas: ethers.utils.parseUnits("1", "gwei"),
                maxFeePerGas: ethers.utils.parseUnits("10", "gwei"),
                type: 2
            });
            if (i % 99 == 0) {
                let receipt = await tx.wait();
                console.log("receipt.status: " + receipt.status);

            }
            console.log(`Transaction ${i + 1} sent. Hash: ${tx.hash}`);
        } catch (error) {
            console.error(`Error in transaction ${i + 1}:`, error);
        }

        await new Promise(resolve => setTimeout(resolve, 20));
    }
    console.log("current blockNumber: " + await customHttpProvider.getBlockNumber());
}

/**
 * Load environment variables 
 * 
 * @param {*} entry 
 * @returns 
 */
function requireEnv(entry) {
    if (process.env[entry]) {
        return process.env[entry]
    } else {
        throw new Error(`${entry} not defined in .env`)
    }
}


main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
