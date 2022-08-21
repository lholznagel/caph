mod asset;

use appraisal::{Appraisal, Janice};
use caph_connector::{EveAuthClient, CorporationService};
use num_format::{Locale, ToFormattedString};
use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_subscriber::EnvFilter;

/// ENV variable for the database URL
const PG_ADDR: &str = "DATABASE_URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pg_addr = std::env::var(PG_ADDR)
        .expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;

    let _journal = journal(&pool).await;

    let asset_worth = assets(&pool).await;
    let wallet = wallets(&pool).await;

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        // TODO: insert BOT token
        HeaderValue::from_static("")
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    #[derive(Debug, Serialize)]
    struct Message {
        content:          String,
        allowed_mentions: AllowedMentions
    }

    #[derive(Debug, Serialize)]
    struct AllowedMentions {
        parse: Vec<String>,
    }

    impl Default for AllowedMentions {
        fn default() -> Self {
            Self { parse: vec!["users".into(), "roles".into()] }
        }
    }

    client.post(
            "https://discord.com/api/v10/channels/980171856768270387/messages"
        )
        .json(&Message {
            content:          format!(r#"
<@318403897972621312>

Ungefährerer derzeitiger Wert der Corp wallets + Assets.

```
Master Wallet   {} ISK
Moon Taxes      {} ISK
Alliance Taxes  {} ISK
Assets:         {} ISK

Total           {} ISK
```
"#,
                (wallet.master as u64).to_formatted_string(&Locale::de),
                (wallet.moon as u64).to_formatted_string(&Locale::de),
                (wallet.alliance as u64).to_formatted_string(&Locale::de),
                (asset_worth as u64).to_formatted_string(&Locale::de),
                ((wallet.master + wallet.moon + wallet.moon + asset_worth) as u64).to_formatted_string(&Locale::de)
            ),
            allowed_mentions: AllowedMentions::default()
        })
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(())
}

async fn assets(pool: &PgPool) -> f32 {
    let info = sqlx::query!(r#"
                SELECT
                    c.corporation_id AS "corporation_id!",
                    l.refresh_token  AS "refresh_token!"
                FROM characters c
                JOIN logins l
                ON l.character_id = c.character_id
                WHERE 'esi-assets.read_corporation_assets.v1' = ANY(esi_tokens)
                  AND c.corporation_name = 'Rip0ff Industries'
                  AND l.refresh_token IS NOT NULL
                  AND c.corporation_id IS NOT NULL
            "#
        )
        .fetch_one(pool)
        .await
        .unwrap();

    let client = EveAuthClient::new(info.refresh_token).unwrap();
    let corporation_service = CorporationService::new(info.corporation_id.into());
    let assets = corporation_service
        .assets(&client)
        .await
        .unwrap()
        .into_iter()
        .filter(|x| !x.is_blueprint_copy)
        .collect::<Vec<_>>();

    let mut entries = Vec::new();
    for asset in assets {
        let name = sqlx::query!("
                    SELECT name
                    FROM items
                    WHERE type_id = $1
                ",
                *asset.type_id
            )
            .fetch_one(pool)
            .await
            .unwrap()
            .name;
        entries.push(format!("{} {}", name, asset.quantity));
    }

    Janice::validate().unwrap();
    let janice = Janice::init().unwrap();

    janice
        .create(false, entries)
        .await
        .unwrap()
        .sell_price
}

async fn journal(pool: &PgPool) {
    let info = sqlx::query!(r#"
                SELECT
                    c.corporation_id AS "corporation_id!",
                    l.refresh_token  AS "refresh_token!"
                FROM characters c
                JOIN logins l
                ON l.character_id = c.character_id
                WHERE 'esi-assets.read_corporation_assets.v1' = ANY(esi_tokens)
                  AND c.corporation_name = 'Rip0ff Industries'
                  AND l.refresh_token IS NOT NULL
                  AND c.corporation_id IS NOT NULL
            "#
        )
        .fetch_one(pool)
        .await
        .unwrap();

    let client = EveAuthClient::new(info.refresh_token).unwrap();
    let corporation_service = CorporationService::new(info.corporation_id.into());
    let journal = corporation_service
        .wallet_journal(&client)
        .await
        .unwrap();
    dbg!(journal);
}

async fn wallets(pool: &PgPool) -> Wallets {
    let info = sqlx::query!(r#"
                SELECT
                    c.corporation_id AS "corporation_id!",
                    l.refresh_token  AS "refresh_token!"
                FROM characters c
                JOIN logins l
                ON l.character_id = c.character_id
                WHERE 'esi-assets.read_corporation_assets.v1' = ANY(esi_tokens)
                  AND c.corporation_name = 'Rip0ff Industries'
                  AND l.refresh_token IS NOT NULL
                  AND c.corporation_id IS NOT NULL
            "#
        )
        .fetch_one(pool)
        .await
        .unwrap();

    let client = EveAuthClient::new(info.refresh_token).unwrap();
    let corporation_service = CorporationService::new(info.corporation_id.into());
    let wallets = corporation_service
        .wallets(&client)
        .await
        .unwrap();

    Wallets {
        master:   wallets[0].balance,
        moon:     wallets[1].balance,
        alliance: wallets[2].balance,
    }
}

struct Wallets {
    master:   f32,
    moon:     f32,
    alliance: f32,
}
