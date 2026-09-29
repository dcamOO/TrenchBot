//! Uses the immutable `user` account of create/create_v2, not mutable fee attribution.
use super::{known::NON_CREATION, model::Deployment};
use anyhow::{Context, Result, ensure};
use serde_json::Value;

pub const PUMP: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

pub fn deployments(tx: &Value, signature: &str) -> Result<Vec<Deployment>> {
    ensure!(
        !tx.is_null(),
        "transação indisponível; histórico incompleto"
    );
    let signatures = tx["transaction"]["signatures"]
        .as_array()
        .context("assinaturas ausentes")?;
    ensure!(
        signatures.first().and_then(Value::as_str) == Some(signature),
        "assinatura divergente"
    );
    let meta = tx
        .get("meta")
        .filter(|m| m.is_object())
        .context("meta ausente")?;
    if !meta.get("err").context("meta sem err")?.is_null() {
        return Ok(Vec::new());
    }
    let outer = tx["transaction"]["message"]["instructions"]
        .as_array()
        .context("instruções ausentes")?;
    let mut instructions: Vec<&Value> = outer.iter().collect();
    let groups = meta["innerInstructions"]
        .as_array()
        .context("histórico sem instruções internas")?;
    for group in groups {
        instructions.extend(group["instructions"].as_array().context("CPI inválida")?);
    }
    let mut found = Vec::new();
    for ix in instructions {
        let program = ix["programId"].as_str().context("programId ausente")?;
        if program != PUMP {
            continue;
        }
        let data = bs58::decode(ix["data"].as_str().context("pump sem data")?).into_vec()?;
        let discriminator: [u8; 8] = data.get(..8).context("pump truncada")?.try_into()?;
        let user_index = match discriminator {
            [24, 30, 200, 40, 5, 28, 7, 119] => 7,
            [214, 144, 76, 236, 95, 139, 49, 180] => 5,
            other => {
                ensure!(
                    NON_CREATION.contains(&other),
                    "instrução pump desconhecida; revisar IDL"
                );
                continue;
            }
        };
        let accounts = ix["accounts"].as_array().context("contas ausentes")?;
        let account = |index: usize| -> Result<String> {
            let value = accounts
                .get(index)
                .and_then(Value::as_str)
                .context("conta inválida")?;
            value.parse::<solana_pubkey::Pubkey>()?;
            Ok(value.into())
        };
        found.push(Deployment {
            mint: account(0)?,
            creator: account(user_index)?,
            launched_at: tx["blockTime"]
                .as_u64()
                .context("deployment sem blockTime")?,
        });
    }
    Ok(found)
}
