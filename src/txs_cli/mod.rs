use crate::txs::util::format_signed_transaction;
use anyhow::Result;
use aptos::common::types::ProfileOptions;
use clap::Parser;
use colored::Colorize;
use indoc::indoc;
use std::path::PathBuf;

mod create_account;
mod demo;
mod generate_local_account;
mod generate_transaction;
mod get_account_balance;
mod get_account_resource;
mod init_config;
mod submit_transaction;
mod transfer_coin;
mod view;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), author, version, about, long_about = None, arg_required_else_help = true)]
pub struct TxsCli {
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Demo transfer coin example for local testnet
    Demo,

    /// Generate yaml files that store the 0L configs
    InitConfig {
        #[clap(flatten)]
        profile_options: ProfileOptions,

        /// Whether to skip the faucet for a non-faucet endpoint
        #[clap(long)]
        skip_faucet: bool,

        /// Mutually exclusive with --private-key
        #[clap(long, group = "private_key_input", parse(from_os_str))]
        private_key_file: Option<PathBuf>,

        /// Mutually exclusive with --private-key-file
        #[clap(long, group = "private_key_input")]
        private_key: Option<String>,
    },

    /// Generate keys and account address locally
    GenerateLocalAccount {
        /// Generate account from the given private key
        #[clap(short, long)]
        private_key: Option<String>,

        /// Path of the directory to store yaml files
        #[clap(short, long)]
        output_dir: Option<String>,
    },

    /// Create onchain account by using Aptos faucet
    CreateAccount {
        /// Create onchain account with the given address
        #[clap(short, long)]
        account_address: String,

        /// The amount of coins to fund the new account
        #[clap(short, long)]
        coins: Option<u64>,
    },

    /// Get account balance
    GetAccountBalance {
        /// Address of the onchain account to get balance from
        #[clap(short, long)]
        account_address: String,
    },

    /// Get account resource
    GetAccountResource {
        /// Address of the onchain account to get resource from
        #[clap(short, long)]
        account_address: String,

        /// Type of the resource to get from account
        #[clap(short, long)]
        resource_type: Option<String>,
    },

    /// Transfer coins between accounts
    TransferCoins {
        /// Address of the recipient
        #[clap(short, long)]
        to_account: String,

        /// The amount of coins to transfer
        #[clap(short, long)]
        amount: u64,

        /// Private key of the account to withdraw money from
        #[clap(short, long)]
        private_key: String,

        /// Maximum number of gas units to be used to send this transaction
        #[clap(short, long)]
        max_gas: Option<u64>,

        /// The amount of coins to pay for 1 gas unit. The higher the price is, the higher priority your transaction will be executed with
        #[clap(short, long)]
        gas_unit_price: Option<u64>,
    },

    /// Generate a transaction that executes an Entry function on-chain
    GenerateTransaction {
        #[clap(
            short,
            long,
            help = indoc!{r#"
                Function identifier has the form <ADDRESS>::<MODULE_ID>::<FUNCTION_NAME>

                Example:
                0x1::coin::transfer
            "#}
        )]
        function_id: String,

        #[clap(
            short,
            long,
            help = indoc!{ r#"
                Type arguments separated by commas

                Example: 
                'u8, u16, u32, u64, u128, u256, bool, address, vector<u8>, signer'
                '0x1::aptos_coin::AptosCoin'
            "#}
        )]
        type_args: Option<String>,

        #[clap(
            short,
            long,
            help = indoc!{ r#"
                Function arguments separated by commas

                Example:
                '0x1, true, 12, 24_u8, x"123456"'
            "#}
        )]
        args: Option<String>,

        /// Maximum amount of gas units to be used to send this transaction
        #[clap(short, long)]
        max_gas: Option<u64>,

        /// The amount of coins to pay for 1 gas unit. The higher the price is, the higher priority your transaction will be executed with
        #[clap(short, long)]
        gas_unit_price: Option<u64>,

        /// Private key to sign the transaction
        #[clap(short, long)]
        private_key: String,

        /// Submit the generated transaction to the blockchain
        #[clap(short, long)]
        submit: bool,
    },

    /// Execute a View function on-chain
    View {
        #[clap(
            short,
            long,
            help = indoc!{r#"
                Function identifier has the form <ADDRESS>::<MODULE_ID>::<FUNCTION_NAME>

                Example:
                0x1::coin::balance
            "#}
        )]
        function_id: String,

        #[clap(
            short,
            long,
            help = indoc!{ r#"
                Type arguments separated by commas

                Example: 
                'u8, u16, u32, u64, u128, u256, bool, address, vector<u8>, signer'
                '0x1::aptos_coin::AptosCoin'
            "#}
        )]
        type_args: Option<String>,

        #[clap(
            short,
            long,
            help = indoc!{ r#"
                Function arguments separated by commas

                Example:
                '0x1, true, 12, 24_u8, x"123456"'
            "#}
        )]
        args: Option<String>,
    },
}

impl TxsCli {
    pub async fn run(&self) -> Result<()> {
        match &self.subcommand {
            Some(Subcommand::Demo) => demo::run().await,
            Some(Subcommand::InitConfig {
                profile_options,
                skip_faucet,
                private_key_file,
                private_key,
            }) => {
                init_config::run(
                    profile_options,
                    *skip_faucet,
                    private_key_file.to_owned(),
                    private_key.to_owned(),
                )
                .await
            }
            Some(Subcommand::GenerateLocalAccount {
                private_key,
                output_dir,
            }) => {
                println!(
                    "{}",
                    generate_local_account::run(
                        &private_key.clone().unwrap_or_default(),
                        output_dir.as_ref().map(PathBuf::from)
                    )
                    .await?
                );
                Ok(())
            }
            Some(Subcommand::CreateAccount {
                account_address,
                coins,
            }) => create_account::run(account_address, coins.unwrap_or_default()).await,
            Some(Subcommand::GetAccountBalance { account_address }) => {
                println!("{}", get_account_balance::run(account_address).await?);
                Ok(())
            }
            Some(Subcommand::GetAccountResource {
                account_address,
                resource_type,
            }) => {
                println!(
                    "{}",
                    get_account_resource::run(account_address, resource_type.to_owned()).await?
                );
                Ok(())
            }
            Some(Subcommand::TransferCoins {
                to_account,
                amount,
                private_key,
                max_gas,
                gas_unit_price,
            }) => {
                transfer_coin::run(
                    to_account,
                    amount.to_owned(),
                    private_key,
                    max_gas.to_owned(),
                    gas_unit_price.to_owned(),
                )
                .await
            }
            Some(Subcommand::GenerateTransaction {
                function_id,
                type_args,
                args,
                max_gas,
                gas_unit_price,
                private_key,
                submit,
            }) => {
                println!("====================");
                let signed_trans = generate_transaction::run(
                    function_id,
                    private_key,
                    type_args.to_owned(),
                    args.to_owned(),
                    max_gas.to_owned(),
                    gas_unit_price.to_owned(),
                )
                .await?;

                println!("{}", format_signed_transaction(&signed_trans));

                if *submit {
                    println!("{}", "Submitting transaction...".green().bold());
                    submit_transaction::run(&signed_trans).await?;
                    println!("Success!");
                }
                Ok(())
            }
            Some(Subcommand::View {
                function_id,
                type_args,
                args,
            }) => {
                println!("====================");
                println!(
                    "{}",
                    view::run(function_id, type_args.to_owned(), args.to_owned()).await?
                );
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
