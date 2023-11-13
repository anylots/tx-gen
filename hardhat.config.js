require("@nomicfoundation/hardhat-toolbox");


/** @type import('hardhat/config').HardhatUserConfig */

module.exports = {
  solidity: "0.8.18",
  networks: {
    local: {
      url: `http://localhost:6688`,
      accounts: ['1212121212121212121212121212121212121212121212121212121212121212']
    }
  }
};
