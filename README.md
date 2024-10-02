```markdown
# GreenThumb
Gardening just got rewardning

GreenThumb is a decentralized application (dApp) that leverages blockchain technology to promote urban gardening and sustainability. The platform incentivizes users with tokens for their gardening activities and provides a community-focused approach to enhancing urban greenery.

## Table of Contents
- [Features](#features)
- [Technologies Used](#technologies-used)
- [Installation](#installation)
- [Usage](#usage)
- [Smart Contract](#smart-contract)
- [Deployment](#deployment)
- [License](#license)

## Features
- **Token Rewards**: Users earn GreenThumb tokens for engaging in gardening activities such as planting, watering, and harvesting.
- **Premium Subscription**: Users can subscribe to premium plans for enhanced rewards and features.
- **Community Engagement**: Users can share gardening tips, post pictures, and interact with each other.
- **Marketplace Integration**: A marketplace for users to trade or sell their gardening products.

## Technologies Used
- **Solana Blockchain**: For the smart contract and token management.
- **Rust**: For smart contract development.
- **SPL Token**: For creating and managing the GreenThumb tokens.
- **JavaScript/HTML/CSS**: For the frontend of the application.

## Installation

Clone the repository:

 ```bash
   git clone https://github.com/Hurotamo/greenthumb.git
   cd greenthumb
   ```

2. Install the Rust toolchain (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Install the Solana CLI:
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.11.0/install)"
   ```

4. Set up the Solana CLI to use devnet or testnet:
   ```bash
   solana config set --url https://api.devnet.solana.com
   ```

5. Build the smart contract:
   ```bash
   cargo build-bpf
   ```

## Usage

1. **Deploy the Smart Contract**:
   Use the following command to deploy the smart contract to the Solana blockchain:
   ```bash
   solana program deploy target/deploy/green_thumb.so
   ```

2. **Interact with the dApp**:
   Use the frontend interface to interact with the GreenThumb dApp. Users can log actions, subscribe to premium plans, and view their token balances.

## Smart Contract

The smart contract is located in the `program/src/` directory. It handles the following actions:
- Rewarding users for gardening activities.
- Processing premium subscriptions.
- Logging user actions.

### Important Constants:
- **Token rewards**:
  - Planting: 10 tokens
  - Watering: 15 tokens
  - Harvesting: 25 tokens

- **Premium Subscription Costs**:
  - 3 months: 3$ 
  - 7 months: 5$ 
  - 1 year: 10$ 

## Deployment

Follow the steps in the "Deployment" section above to deploy the GreenThumb project to the Solana devnet or testnet.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the Solana community for their support and resources.
- Inspired by the need for sustainable urban gardening practices.

```

