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


#[cfg(test)]
mod tests {
    use super::*;

    fn make_jupiter_info(
        liquidity: Option<f64>,
        market_cap: Option<f64>,
    ) -> JupiterInfo {
        JupiterInfo {
            price_usd: None,
            liquidity,
            market_cap_usd: market_cap,
            holder_count: None,
            verified: None,
            audit: None,
            first_pool: None,
            stats_5m: None,
            stats_1h: None,
            stats_6h: None,
            stats_24h: None,
        }
    }

    fn make_stats(
        buy: Option<f64>,
        sell: Option<f64>,
        num_buys: Option<u64>,
        num_sells: Option<u64>,
    ) -> Stats {
        Stats {
            price_change: None,
            buy_volume: buy,
            sell_volume: sell,
            num_buys,
            num_sells,
            num_traders: None,
        }
    }

    #[test]
    fn valid_liquidity_ratio() {
        let jupiter = make_jupiter_info(
            Some(25_000.0),
            Some(100_000.0),
        );

        assert_eq!(liquidity_ratio(&jupiter), Some(25.0));
    }

    #[test]
    fn missing_liquidity() {
        let jupiter = make_jupiter_info(None, Some(100_000.0));

        assert_eq!(liquidity_ratio(&jupiter), None);
    }

    #[test]
    fn missing_market_cap() {
        let jupiter = make_jupiter_info(Some(25_000.0), None);

        assert_eq!(liquidity_ratio(&jupiter), None);
    }

    #[test]
    fn zero_market_cap() {
        let jupiter = make_jupiter_info(Some(25_000.0), Some(0.0));

        assert_eq!(liquidity_ratio(&jupiter), None);
    }

    #[test]
    fn negative_market_cap() {
        let jupiter = make_jupiter_info(
            Some(25_000.0),
            Some(-100_000.0),
        );

        assert_eq!(liquidity_ratio(&jupiter), None);
    }

    #[test]
    fn valid_stats() {
        let stats = make_stats(
            Some(75.0),
            Some(25.0),
            Some(10),
            Some(5),
        );

        let result = process_stats(Some(&stats)).unwrap();

        assert_eq!(result.buy_sell_imbalance, Some(50.0));
        assert_eq!(result.buy_volume_share, Some(75.0));
        assert_eq!(result.total_volume, Some(100.0));
        assert_eq!(result.total_trades, Some(15));
    }

    #[test]
    fn missing_stats() {
        assert!(process_stats(None).is_none());
    }

    #[test]
    fn missing_buy_volume() {
        let stats = make_stats(
            None,
            Some(25.0),
            Some(10),
            Some(5),
        );

        assert!(process_stats(Some(&stats)).is_none());
    }

    #[test]
    fn missing_sell_volume() {
        let stats = make_stats(
            Some(75.0),
            None,
            Some(10),
            Some(5),
        );

        assert!(process_stats(Some(&stats)).is_none());
    }

    #[test]
    fn zero_volume() {
        let stats = make_stats(
            Some(0.0),
            Some(0.0),
            Some(10),
            Some(5),
        );

        let result = process_stats(Some(&stats)).unwrap();

        assert_eq!(result.buy_sell_imbalance, None);
        assert_eq!(result.buy_volume_share, None);
        assert_eq!(result.total_volume, Some(0.0));
        assert_eq!(result.total_trades, None);
    }

    #[test]
    fn negative_total_volume() {
        let stats = make_stats(
            Some(-10.0),
            Some(5.0),
            Some(10),
            Some(5),
        );

        let result = process_stats(Some(&stats)).unwrap();

        assert_eq!(result.buy_sell_imbalance, None);
        assert_eq!(result.buy_volume_share, None);
        assert_eq!(result.total_volume, Some(0.0));
        assert_eq!(result.total_trades, None);
    }

    #[test]
    fn missing_trade_counts() {
        let stats = make_stats(
            Some(75.0),
            Some(25.0),
            Some(10),
            None,
        );

        let result = process_stats(Some(&stats)).unwrap();

        assert_eq!(result.total_trades, None);
    }

    #[test]
    fn valid_trade_counts() {
        let stats = make_stats(
            Some(75.0),
            Some(25.0),
            Some(10),
            Some(5),
        );

        let result = process_stats(Some(&stats)).unwrap();

        assert_eq!(result.total_trades, Some(15));
    }
}
