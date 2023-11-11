# MedWallet Server

<!--- Eraser file: https://app.eraser.io/workspace/sSLZHdHweQLqykBez85u --->
<p><a target="_blank" href="https://app.eraser.io/workspace/sSLZHdHweQLqykBez85u" id="edit-in-eraser-github-link"><img alt="Edit in Eraser" src="https://firebasestorage.googleapis.com/v0/b/second-petal-295822.appspot.com/o/images%2Fgithub%2FOpen%20in%20Eraser.svg?alt=media&amp;token=968381c8-a7e7-472a-8ed6-4a6626da5501"></a></p>

## High Level Architecture

![diagram-export-11-11-2023-12_47_46](https://github.com/MedWallet/MedWalletServer/assets/74317567/6525351b-2dc3-40ed-9fe2-8e022ef9e94a)

- **Client([github repo][client github repo])**: Interface for user actions and interactions.
- **Backend:**
  - **API Gateway**: Entry point for routing client requests to appropriate backend services.
  - **User Management (Auth & Profile)**: Manages user authentication and profile data.
  - **Main Service**:
    - **Manage User**: Handles operations related to user data.
    - **Access Control**: Manages access to documents, ensuring proper user permissions.
    - **Manage Doc**: Responsible for health document-related functionalities.
  - **Health Document Management**: Deals with the encryption and decryption of health documents.
  - **Storage (IPFS)**: Uses InterPlanetary File System for decentralized document storage.
  - **Encryption (FHE, ZKP)**: Implements Full Homomorphic Encryption and Zero-Knowledge Proofs for data security.
- **Blockchain Integration:**
  - **Solana Blockchain**
    - **RPC Node**: Enables interaction with the Solana blockchain through Remote Procedure Calls.
    - **On-Chain Program (Smart Contract)**: The blockchain-resident program that handles the logic for transactions.
    - **Transaction**: Represents the execution of a smart contract on the blockchain.

[client github repo]: https://github.com/MedWallet/MedWalletMobile
