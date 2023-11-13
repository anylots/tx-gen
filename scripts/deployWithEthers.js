const { BigNumber } = require("ethers")

const overrides = {
  gasLimit: 15000000,
  gasPrice: 40 * 10 ** 9,
};

// This is a script for deploying your contracts. You can adapt it to deploy
// yours, or create new ones.
async function main() {

  ///prepare deployer
  const [deployer] = await ethers.getSigners();
  console.log(
    "Deploying contracts with the account:",
    await deployer.getAddress()
  );
  console.log("Account balance:", (await deployer.getBalance()).toString());


  ///deploy AleoToken
  const Token = await ethers.getContractFactory("Token");
  const token = await Token.deploy(BigNumber.from(10 ** 12).mul(BigNumber.from(10 ** 6)));
  await token.deployed();
  console.log("token address:", token.address);
}




main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
