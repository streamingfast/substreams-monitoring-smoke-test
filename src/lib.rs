mod pb;

use chrono::NaiveDateTime;
use pb::sf::substreams::monitoring::v1::{BlockMetadata, Stats};
use substreams::errors::Error;
use substreams::prelude::*;
use substreams::{store, Hex};
use substreams_ethereum::pb::eth;
use substreams_solana::pb::sol;

const KEY_TOTAL_BLOCK_COUNT: &str = "total_block_count";
const KEY_TOTAL_TRX_COUNT: &str = "total_trx_count";

#[substreams::handlers::map]
fn map_eth_block(block: eth::v2::Block) -> Result<BlockMetadata, Error> {
    let header = block.header.as_ref().unwrap();

    Ok(BlockMetadata {
        hash: Hex(&block.hash).to_string(),
        number: block.number,
        parent_hash: Hex(&header.parent_hash).to_string(),
        parent_number: match block.number {
            0 => 0,
            x => x - 1,
        },
        timestamp: header.timestamp.as_ref().unwrap().to_string(),
        transaction_count: block.transaction_traces.len() as u64,
    })
}

#[substreams::handlers::store]
fn store_eth_stats(metadata: BlockMetadata, stats: store::StoreAddInt64) {
    store_stats(metadata, stats)
}

#[substreams::handlers::map]
fn map_eth_stats(
    metadata: BlockMetadata,
    stats: store::Deltas<DeltaInt64>,
) -> Result<Stats, Error> {
    map_stats(metadata, stats)
}

#[substreams::handlers::map]
fn map_sol_block(block: sol::v1::Block) -> Result<BlockMetadata, Error> {
    Ok(BlockMetadata {
        hash: block.blockhash,
        number: block.slot,
        parent_hash: block.previous_blockhash,
        parent_number: block.parent_slot,
        timestamp: block
            .block_time
            .as_ref()
            .map(|x| {
                let seconds = x.timestamp / 1_000;
                let nano_seconds = (x.timestamp % 1_000) * 1_000_000;

                NaiveDateTime::from_timestamp_opt(seconds, nano_seconds as u32)
                    .unwrap_or_else(|| panic!("invalid date for timestamp {}", x.timestamp))
                    .to_string()
            })
            .unwrap(),
        transaction_count: block.transactions.len() as u64,
    })
}

#[substreams::handlers::store]
fn store_sol_stats(metadata: BlockMetadata, stats: store::StoreAddInt64) {
    store_stats(metadata, stats)
}

#[substreams::handlers::map]
fn map_sol_stats(
    metadata: BlockMetadata,
    stats: store::Deltas<DeltaInt64>,
) -> Result<Stats, Error> {
    map_stats(metadata, stats)
}

fn store_stats(metadata: BlockMetadata, stats: store::StoreAddInt64) {
    stats.add(metadata.number, KEY_TOTAL_BLOCK_COUNT, 1);
    stats.add(
        metadata.number,
        KEY_TOTAL_TRX_COUNT,
        metadata.transaction_count as i64,
    );
}

fn map_stats(metadata: BlockMetadata, stats: store::Deltas<DeltaInt64>) -> Result<Stats, Error> {
    let mut total_block_count = None;
    let mut total_trx_count = None;

    stats
        .deltas
        .iter()
        .for_each(|delta| match delta.key.as_str() {
            KEY_TOTAL_BLOCK_COUNT => total_block_count = Some(delta.new_value),
            KEY_TOTAL_TRX_COUNT => total_trx_count = Some(delta.new_value),
            x => panic!("unhandled key {}", x),
        });

    match (total_block_count, total_trx_count) {
        (Some(total_block_count), Some(total_trx_count)) => Ok(Stats {
            head_block: Some(metadata),
            total_block_count: total_block_count as u64,
            total_transaction_count: total_trx_count as u64,
        }),
        _ => panic!(
            "both key {} and {} should have been updated",
            KEY_TOTAL_BLOCK_COUNT, KEY_TOTAL_TRX_COUNT
        ),
    }
}
