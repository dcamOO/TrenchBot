use super::super::super::info::{JupiterInfo, MintInfo, Stats, TokenInfo};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct DerivedStats {
    pub supply: f64,

    pub top_1_share: Option<f64>,
    pub top_5_share: Option<f64>,
    pub top_10_share: Option<f64>,

    pub liquidity_to_market_cap: Option<f64>,

    pub stats_5m: Option<ProcessedStats>,
    pub stats_1h: Option<ProcessedStats>,
    pub stats_6h: Option<ProcessedStats>,
    pub stats_24h: Option<ProcessedStats>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ProcessedStats {
    pub buy_sell_imbalance: Option<f64>,
    pub buy_volume_share: Option<f64>,
    pub total_volume: Option<f64>,
    pub total_trades: Option<u64>,
}

fn liquidity_ratio(
    jupiter: &JupiterInfo,
) -> Option<f64> {
    let liquidity = jupiter.liquidity?;
    let market_cap = jupiter.market_cap_usd?;

    if market_cap <= 0.0 {
        return None;
    }

    Some(liquidity / market_cap * 100.0)
}

fn process_stats(
    stats: Option<&Stats>,
) -> Option<ProcessedStats> {
    let stats = stats?;

    let buy = stats.buy_volume?;
    let sell = stats.sell_volume?;

    let total = buy + sell;

    if total <= 0.0 {
        return Some(ProcessedStats {
            buy_sell_imbalance: None,
            buy_volume_share: None,
            total_volume: Some(0.0),
            total_trades: None,
        });
    }

    Some(ProcessedStats {
        buy_sell_imbalance:
            Some((buy - sell) / total * 100.0),

        buy_volume_share:
            Some(buy / total * 100.0),

        total_volume: Some(total),

        total_trades: stats
            .num_buys
            .zip(stats.num_sells)
            .map(|(buys, sells)| buys + sells),
    })
}
