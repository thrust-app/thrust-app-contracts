<div align="left" style="position: relative;">
<img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" align="right" width="30%" style="margin: -20px 0 0 20px;">
<h1>THRUST_ANCHOR</h1>
<p align="left">
	<em><code>❯ Thrust App</code></em>
</p>
<p align="left">
	<img src="https://img.shields.io/github/license/lxmnsingh/solana-fun-contracts?style=default&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/last-commit/lxmnsingh/solana-fun-contracts?style=default&logo=git&logoColor=white&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/github/languages/top/lxmnsingh/solana-fun-contracts?style=default&color=0080ff" alt="repo-top-language">
	<img src="https://img.shields.io/github/languages/count/lxmnsingh/solana-fun-contracts?style=default&color=0080ff" alt="repo-language-count">
</p>
<p align="left"><!-- default option, no dependency badges. -->
</p>
<p align="left">
	<!-- default option, no dependency badges. -->
</p>
</div>
<br clear="right">

##  Table of Contents

- [ Overview](#-overview)
- [ Features](#-features)
- [ Project Structure](#-project-structure)
  - [ Project Index](#-project-index)
- [ Getting Started](#-getting-started)
  - [ Prerequisites](#-prerequisites)
  - [ Installation](#-installation)
  - [ Usage](#-usage)
  - [ Testing](#-testing)
- [ Project Roadmap](#-project-roadmap)
- [ Contributing](#-contributing)
- [ License](#-license)
- [ Acknowledgments](#-acknowledgments)

---

##  Overview

<code>This is a Solana program built with Rust Anchor for a launchpad project on Solana.</code>

---

##  Features

<code>Create Meme Coin launchpad and let others buy or sell it. After reaching marketcap, it will be listed to Raydium.</code>

---

##  Project Structure

```sh
└── thrust_anchor/
    ├── Anchor.toml
    ├── Cargo.toml
    ├── README.md
    ├── migrations
    │   └── deploy.ts
    ├── package.json
    ├── programs
    │   └── thrust_app
    │       ├── Cargo.toml
    │       ├── Xargo.toml
    │       └── src
    │           ├── constants.rs
    │           ├── error.rs
    │           ├── lib.rs
    │           ├── main_state
    │           │   ├── ixs
    │           │   ├── mod.rs
    │           │   └── state.rs
    │           ├── pool
    │           │   ├── event.rs
    │           │   ├── ixs
    │           │   ├── mod.rs
    │           │   └── state.rs
    │           ├── user
    │           │   ├── mod.rs
    │           │   └── state.rs
    │           └── utils.rs
    ├── tests
    │   └── index.ts
    └── tsconfig.json
```


###  Project Index
<details open>
	<summary><b><code>THRUST_ANCHOR/</code></b></summary>
	<details> <!-- __root__ Submodule -->
		<summary><b>__root__</b></summary>
		<blockquote>
			<table>
			<tr>
				<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/tsconfig.json'>tsconfig.json</a></b></td>
				<td><code>Typescript Configuration File</code></td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/Cargo.toml'>Cargo.toml</a></b></td>
				<td><code>Anchor Configuration File</code></td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/Anchor.toml'>Anchor.toml</a></b></td>
				<td><code>Anchor Project Configuration File</code></td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/package.json'>package.json</a></b></td>
				<td><code>Node Js Project Configuration File</code></td>
			</tr>
			</table>
		</blockquote>
	</details>
	<details> <!-- programs Submodule -->
		<summary><b>programs</b></summary>
		<blockquote>
			<details>
				<summary><b>thrust_app</b></summary>
				<blockquote>
					<table>
					<tr>
						<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/Cargo.toml'>Cargo.toml</a></b></td>
						<td><code>Anchor Configuration File</code></td>
					</tr>
					<tr>
						<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/Xargo.toml'>Xargo.toml</a></b></td>
						<td><code>Anchor Configuration File</code></td>
					</tr>
					</table>
					<details>
						<summary><b>src</b></summary>
						<blockquote>
							<table>
							<tr>
								<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/lib.rs'>lib.rs</a></b></td>
								<td><code>Main entry point for the Thrust App program</code></td>
							</tr>
							<tr>
								<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/utils.rs'>utils.rs</a></b></td>
								<td><code>Main utility functions for the Thrust App program</code></td>
							</tr>
							<tr>
								<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/error.rs'>error.rs</a></b></td>
								<td><code>Declaration of error code</code></td>
							</tr>
							<tr>
								<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/constants.rs'>constants.rs</a></b></td>
								<td><code>Main constants for the Thrust App program</code></td>
							</tr>
							</table>
							<details>
								<summary><b>pool</b></summary>
								<blockquote>
									<table>
									<tr>
										<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/pool/mod.rs'>mod.rs</a></b></td>
										<td><code>Entry point for the pool state</code></td>
									</tr>
									<tr>
										<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/pool/event.rs'>event.rs</a></b></td>
										<td><code>Declaration of event for pool state instructions</code></td>
									</tr>
									<tr>
										<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/pool/state.rs'>state.rs</a></b></td>
										<td><code>Declaration of pool state structure</code></td>
									</tr>
									</table>
									<details>
										<summary><b>ixs</b></summary>
										<blockquote>
											<table>
											<tr>
												<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/pool/ixs/mod.rs'>mod.rs</a></b></td>
												<td><code>Entry point of pool state instructions</code></td>
											</tr>
											<tr>
												<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/pool/ixs/buy.rs'>buy.rs</a></b></td>
												<td><code>Declaration of buy instruction</code></td>
											</tr>
											<tr>
												<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/pool/ixs/withdraw.rs'>withdraw.rs</a></b></td>
												<td><code>Delcaration of withdraw instruction</code></td>
											</tr>
											<tr>
												<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/pool/ixs/create_pool.rs'>create_pool.rs</a></b></td>
												<td><code>Delcaration of create pool instruction</code></td>
											</tr>
											<tr>
												<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/pool/ixs/sell.rs'>sell.rs</a></b></td>
												<td><code>Delcaration of sell instruction</code></td>
											</tr>
											</table>
										</blockquote>
									</details>
								</blockquote>
							</details>
							<details>
								<summary><b>main_state</b></summary>
								<blockquote>
									<table>
									<tr>
										<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/main_state/mod.rs'>mod.rs</a></b></td>
										<td><code>Entry point of main state instructions</code></td>
									</tr>
									<tr>
										<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/main_state/state.rs'>state.rs</a></b></td>
										<td><code>Declaration of main state structure</code></td>
									</tr>
									</table>
									<details>
										<summary><b>ixs</b></summary>
										<blockquote>
											<table>
											<tr>
												<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/main_state/ixs/mod.rs'>mod.rs</a></b></td>
												<td><code>Entry point of pool state instructions</code></td>
											</tr>
											<tr>
												<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/main_state/ixs/init_main_state.rs'>init_main_state.rs</a></b></td>
												<td><code>Initialize Main State</code></td>
											</tr>
											<tr>
												<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/main_state/ixs/update_main_state_owner.rs'>update_main_state_owner.rs</a></b></td>
												<td><code>Update Main State</code></td>
											</tr>
											</table>
										</blockquote>
									</details>
								</blockquote>
							</details>
							<details>
								<summary><b>user</b></summary>
								<blockquote>
									<table>
									<tr>
										<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/user/mod.rs'>mod.rs</a></b></td>
										<td><code>Entry point of user state instructions</code></td>
									</tr>
									<tr>
										<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/programs/thrust_app/src/user/state.rs'>state.rs</a></b></td>
										<td><code>Declaration of user state structure</code></td>
									</tr>
									</table>
								</blockquote>
							</details>
						</blockquote>
					</details>
				</blockquote>
			</details>
		</blockquote>
	</details>
	<details> <!-- migrations Submodule -->
		<summary><b>migrations</b></summary>
		<blockquote>
			<table>
			<tr>
				<td><b><a href='https://github.com/lxmnsingh/solana-fun-contracts/blob/master/migrations/deploy.ts'>deploy.ts</a></b></td>
				<td><code>deploy script</code></td>
			</tr>
			</table>
		</blockquote>
	</details>
</details>

---
##  Getting Started

###  Prerequisites

Before getting started with thrust_anchor, ensure your runtime environment meets the following requirements:

- **Programming Language:** Rust
- **Package Manager:** Cargo, Npm


###  Installation

Install thrust_anchor using one of the following methods:

**Build from source:**

1. Clone the thrust_anchor repository:
```sh
❯ git clone https://github.com/lxmnsingh/solana-fun-contracts
```

2. Navigate to the project directory:
```sh
❯ cd thrust_anchor
```

3. Install the project dependencies:

**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo build
```

**Using `npm`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Node.js-000000.svg?style={badge_style}&logo=node.js&logoColor=white" />](https://nodejs.org/)

```sh
❯ npm install
```

**Using `anchor`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Anchor-000000.svg?style={badge_style}&logo=anchor&logoColor=white" />](https://project-serum.github.io/anchor/)

```sh
❯ anchor build
```

###  Deployment
**Using `solana`** &nbsp; 
```sh
❯ solana program deploy ./target/deploy/thrust_app.so
```

###  Testing
Run the test suite using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo test
```

---
##  Project Roadmap

- [X] **`Task 1`**: <strike>Smart Contract Development</strike>
- [ ] **`Task 2`**: Backend Development
- [ ] **`Task 3`**: Frontend Development

---

##  Contributing

- **💬 [Join the Discussions](https://github.com/lxmnsingh/solana-fun-contracts/discussions)**: Share your insights, provide feedback, or ask questions.
- **🐛 [Report Issues](https://github.com/lxmnsingh/solana-fun-contracts/issues)**: Submit bugs found or log feature requests for the `thrust_anchor` project.
- **💡 [Submit Pull Requests](https://github.com/lxmnsingh/solana-fun-contracts/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.

<details closed>
<summary>Contributing Guidelines</summary>

1. **Fork the Repository**: Start by forking the project repository to your github account.
2. **Clone Locally**: Clone the forked repository to your local machine using a git client.
   ```sh
   git clone https://github.com/lxmnsingh/solana-fun-contracts
   ```
3. **Create a New Branch**: Always work on a new branch, giving it a descriptive name.
   ```sh
   git checkout -b new-feature-x
   ```
4. **Make Your Changes**: Develop and test your changes locally.
5. **Commit Your Changes**: Commit with a clear message describing your updates.
   ```sh
   git commit -m 'Implemented new feature x.'
   ```
6. **Push to github**: Push the changes to your forked repository.
   ```sh
   git push origin new-feature-x
   ```
7. **Submit a Pull Request**: Create a PR against the original project repository. Clearly describe the changes and their motivations.
8. **Review**: Once your PR is reviewed and approved, it will be merged into the main branch. Congratulations on your contribution!
</details>

<details closed>
<summary>Contributor Graph</summary>
<br>
<p align="left">
   <a href="https://github.com{/lxmnsingh/solana-fun-contracts/}graphs/contributors">
      <img src="https://contrib.rocks/image?repo=lxmnsingh/solana-fun-contracts">
   </a>
</p>
</details>

---

##  License

This project is protected under the [The Unlicense](https://choosealicense.com/licenses) License. For more details, refer to the [LICENSE](https://choosealicense.com/licenses/) file.

---

##  Acknowledgments

- List any resources, contributors, inspiration, etc. here.

---
