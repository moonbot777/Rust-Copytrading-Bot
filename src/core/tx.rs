use anyhow::Result;
use colored::Colorize;
use solana_sdk::{
    instruction::Instruction, signature::Keypair, signer::Signer, system_instruction,
    transaction::Transaction,
};
use spl_token::ui_amount_to_amount;
use std::sync::Arc;
use tokio::time::Instant;

use crate::{
    common::logger::Logger,
    services::{
        jito::{self, get_tip_account, get_tip_value, wait_for_bundle_confirmation, JitoClient},
        nozomi::{self, TemporalClient},
    },
};

pub async fn new_signed_and_send(
    recent_blockhash: solana_sdk::hash::Hash,
    keypair: &Keypair,
    mut instructions: Vec<Instruction>,
    start_time: Instant,
    logger: &Logger,
) -> Result<Vec<String>> {

    Ok(txs)
}

pub async fn new_signed_and_send_nozomi(
    recent_blockhash: solana_sdk::hash::Hash,
    keypair: &Keypair,
    mut instructions: Vec<Instruction>,
    logger: &Logger,
) -> Result<Vec<String>> {

    Ok(txs)
}
