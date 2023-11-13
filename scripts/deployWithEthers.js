const { BigNumber } = require("ethers")
const { ethers } = require('ethers');
const Token_Artifact = require("../artifacts/contracts/Token.sol/Token.json");

const overrides = {
    gasLimit: 15000000,
    gasPrice: 40 * 10 ** 9,
};

// This is a script for deploying your contracts. You can adapt it to deploy
// yours, or create new ones.
async function main() {

    ///prepare deployer
    let privateKey = "0x1212121212121212121212121212121212121212121212121212121212121212";
    let customHttpProvider = new ethers.providers.JsonRpcProvider(
        "http://localhost:6688"
    );
    const signer = new ethers.Wallet(privateKey, customHttpProvider);
    console.log("signer.address: " + signer.address);


    ///deploy AleoToken
    let TokenFactory = new ethers.ContractFactory(Token_Artifact.abi, Token_Artifact.bytecode, signer);
    const token = await TokenFactory.deploy(BigNumber.from(10 ** 12).mul(BigNumber.from(10 ** 6)));
    console.log("contract deploying...");

    await token.deployed();
    console.log("token address:", token.address);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
