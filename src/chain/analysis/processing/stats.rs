use super::super::super::info::{JupiterInfo, MintInfo, Stats, TokenInfo};

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct ProcessedStats {
    pub buy_sell_imbalance: Option<f64>,
    pub buy_volume_share: Option<f64>,
    pub total_volume: Option<f64>,
    pub total_trades: Option<u64>,
}

pub(crate) fn calculate(info: &TokenInfo) -> DerivedStats {
    let mint = &info.mint_info;
    let jupiter = &info.jupiter;

    DerivedStats {
        supply: mint.supply as f64
            / 10_f64.powi(mint.decimals as i32),

        top_1_share: concentration(mint, 1),
        top_5_share: concentration(mint, 5),
        top_10_share: concentration(mint, 10),

        liquidity_to_market_cap:
            liquidity_ratio(jupiter),

        stats_5m: process_stats(jupiter.stats_5m.as_ref()),
        stats_1h: process_stats(jupiter.stats_1h.as_ref()),
        stats_6h: process_stats(jupiter.stats_6h.as_ref()),
        stats_24h: process_stats(jupiter.stats_24h.as_ref()),
    }
}

fn concentration(
    mint: &MintInfo,
    n: usize,
) -> Option<f64> {
    if mint.supply == 0 {
        return None;
    }

    let amount = mint
        .largest_accounts
        .iter()
        .take(n)
        .map(|account| account.amount)
        .sum::<u64>();

    Some(amount as f64 / mint.supply as f64 * 100.0)
}

fn liquidity_ratio(
    jupiter: &JupiterInfo,
) -> Option<f64> {
    let liquidity = jupiter.liquidity_usd?;
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
