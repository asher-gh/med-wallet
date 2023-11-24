<p><a target="_blank" href="https://app.eraser.io/workspace/sSLZHdHweQLqykBez85u" id="edit-in-eraser-github-link"><img alt="Edit in Eraser" src="https://firebasestorage.googleapis.com/v0/b/second-petal-295822.appspot.com/o/images%2Fgithub%2FOpen%20in%20Eraser.svg?alt=media&amp;token=968381c8-a7e7-472a-8ed6-4a6626da5501"></a></p>

# MedWallet Server

## High Level Architecture

![HLD Architecture](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---uzSQ26XwRZWl6XBTdNnEt---figure---08fOkGLTLePHsOu7btjsPw.png "HLD Architecture")

### **Client(**[﻿github repo](https://github.com/MedWallet/MedWalletMobile) **)**

![Client](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---tZLKUFmYh2PckaM3mTHrG---figure---Unfdard2avAr7dxdj6h50w.png "Client")

- **UI**: Interface for user actions and interactions.
- **Key management**: Managing encryption keys (generation, storage, etc.) locally on the client device.
- **Document encryption and decryption**: Encrypting data before it's sent to the server. Decrypting data retrieved from the server.

### **Server**

![Backend](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---kuNmPGrSyfiPWfknXpzY3---figure---QDltSN54dWkqYNjSA2OQwA.png "Backend")

- **API Gateway**: Entry point for routing client requests to appropriate backend services.
- **User Management (Auth & Profile)**: Manages user authentication and profile data.
- **Main Service**:
  - **Manage User**: Handles operations related to user data.
  - **Access Control**: Manages access to documents, ensuring proper user permissions.
  - **Manage Doc**: Responsible for health document-related functionalities.
- **Health Document Management**: Deals with the encryption and decryption of health documents.
- **Storage (IPFS)**: Uses InterPlanetary File System for decentralized document storage.
- **FHE processes (FHE, ZKP)**: Implements Full Homomorphic Encryption compatible operations from user's documents.

### **Blockchain Integration**

![Solana Blockchain](/.eraser/sSLZHdHweQLqykBez85u___3oruMmg3JsO4hsgOZduK3ZmpKls2___---figure---AX4rTPdfmq_GbABamwOjL---figure---OW9wbWYX7OlKMjNh2XDQ3Q.png "Solana Blockchain")

- **RPC Node**: Enables interaction with the Solana blockchain through Remote Procedure Calls.
- **On-Chain Program (Smart Contract)**: The blockchain-resident program that handles the logic for transactions.
- **Transaction**: Represents the execution of a smart contract on the blockchain.

## Development

### Migrations

We are using [sqlx] for interacting with a PostgreSQL instance.

The accompanying [sqlx-cli] can be used to manage [schema migrations][migrations].

You can check the repo for more info, but basic usage is as follows.

_Create migration_

```shell
$ sqlx migrate add <name>
```

A new file (or two in case the first migration was added with `-r`) in the
`migrations/` directory. This is a standard `SQL` file that you can add table
creation queries.

In case of reversible migrations (enabled for this project), there will be two
files per migration `*.up.sql` and `*.down.sql`, corresponding to creation and reversal of a migration. When you revert a migration, the `*.down.sql` will be used, and applying the migrations will use `*.up.sql`.

_Run migration_

```shell
$ sqlx migrate run
```

_Revert last migration_

```shell
$ sqlx migrate revert
```

_Revert up to a migration_

```shell
$ sqlx migrate revert --target-version <TARGET_VERSION>
```

The `TARGET_VERSION` can be checked with, `sqlx migrate info`, eg.

```shell
$ sqlx migrate info
20231111154432/installed init
20231111154451/installed user table
20231114162015/pending documents
```

Here `20231111154432` is a target version for migration corresponding to `init`.

[sqlx]: https://github.com/launchbadge/sqlx/tree/main
[sqlx-cli]: https://github.com/launchbadge/sqlx/blob/main/sqlx-cli
[migrations]: https://en.wikipedia.org/wiki/Schema_migration

<!--- Eraser file: https://app.eraser.io/workspace/sSLZHdHweQLqykBez85u --->

# MedWallet Mobile

[![CircleCI](https://circleci.com/gh/infinitered/ignite.svg?style=svg)](https://circleci.com/gh/infinitered/ignite)

## Quick Start

The project's structure is this:

```
├── app
│   ├── components
│   ├── config
│   ├── i18n
│   ├── models
│   ├── navigators
│   ├── screens
│   ├── services
│   ├── theme
│   ├── utils
│   └── app.tsx
├── assets
│   ├── icons
│   └── images
├── test
│   ├── __snapshots__
│   ├── mockFile.ts
│   └── setup.ts
├── README.md
├── android
│   ├── app
│   ├── build.gradle
│   ├── gradle
│   ├── gradle.properties
│   ├── gradlew
│   ├── gradlew.bat
│   ├── keystores
│   └── settings.gradle
├── ignite
│   └── templates
|       |── app-icon
│       ├── component
│       ├── model
│       ├── navigator
│       └── screen
├── index.js
├── ios
│   ├── IgniteProject
│   ├── IgniteProject-tvOS
│   ├── IgniteProject-tvOSTests
│   ├── IgniteProject.xcodeproj
│   └── IgniteProjectTests
├── .env
└── package.json

```

### ./app directory

This is a directory you would normally have to create when using vanilla React Native.

The inside of the `app` directory looks similar to the following:

```
app
├── components
├── config
├── i18n
├── models
├── navigators
├── screens
├── services
├── theme
├── utils
└── app.tsx
```

**components**
This is where your reusable components live which help you build your screens.

**i18n**
This is where your translations will live if you are using `react-native-i18n`.

**models**
This is where your app's models will live. Each model has a directory which will contain the `mobx-state-tree` model file, test file, and any other supporting files like actions, types, etc.

**navigators**
This is where your `react-navigation` navigators will live.

**screens**
This is where your screen components will live. A screen is a React component which will take up the entire screen and be part of the navigation hierarchy. Each screen will have a directory containing the `.tsx` file, along with any assets or other helper files.

**services**
Any services that interface with the outside world will live here (think REST APIs, Push Notifications, etc.).

**theme**
Here lives the theme for your application, including spacing, colors, and typography.

**utils**
This is a great place to put miscellaneous helpers and utilities. Things like date helpers, formatters, etc. are often found here. However, it should only be used for things that are truly shared across your application. If a helper or utility is only used by a specific component or model, consider co-locating your helper with that component or model.

**app.tsx** This is the entry point to your app. This is where you will find the main App component which renders the rest of the application.

### ./assets directory

This directory is designed to organize and store various assets, making it easy for you to manage and use them in your application. The assets are further categorized into subdirectories, including `icons` and `images`:

```
assets
├── icons
└── images
```

**icons**
This is where your icon assets will live. These icons can be used for buttons, navigation elements, or any other UI components. The recommended format for icons is PNG, but other formats can be used as well.

Ignite comes with a built-in `Icon` component. You can find detailed usage instructions in the [docs](https://github.com/infinitered/ignite/blob/master/docs/Components-Icon.md).

**images**
This is where your images will live, such as background images, logos, or any other graphics. You can use various formats such as PNG, JPEG, or GIF for your images.

Another valuable built-in component within Ignite is the `AutoImage` component. You can find detailed usage instructions in the [docs](https://github.com/infinitered/ignite/blob/master/docs/Components-AutoImage.md).

How to use your `icon` or `image` assets:

```
import { Image } from 'react-native';

const MyComponent = () => {
  return (
    <Image source={require('../assets/images/my_image.png')} />
  );
};
```

### ./ignite directory

The `ignite` directory stores all things Ignite, including CLI and boilerplate items. Here you will find templates you can customize to help you get started with React Native.

### ./test directory

This directory will hold your Jest configs and mocks.

## Running Maestro end-to-end tests

Follow our [Maestro Setup](https://ignitecookbook.com/docs/recipes/MaestroSetup) recipe from the [Ignite Cookbook](https://ignitecookbook.com/)!
