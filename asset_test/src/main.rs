use caph_connector::{ConnectCharacterService, EveAuthClient, CharacterAssetEntry, ItemId};
use uuid::Uuid;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("EVE_CLIENT_ID", "4dd388ab3fc74a19b4fbe4346b409381");
    std::env::set_var("EVE_SECRET_KEY", "DAO1QfR8fUKFIbQkfcQoWrDgjnDJCXYRLMVYAHKS");

    let token = "npe+nazH9UuUND93TblmCw==";
    let cid = 2117848811;

    let mut last: HashMap<ItemId, CharacterAssetEntry> = HashMap::new();

    loop {
        std::env::set_var("EVE_USER_AGENT", Uuid::new_v4().to_string());

        let client = EveAuthClient::new(token.into()).unwrap();
        let assets = ConnectCharacterService::new(&client, cid.into())
            .assets()
            .await
            .unwrap()
            .into_iter()
            .map(|x| (x.item_id, x))
            .collect::<HashMap<_, _>>();

        for (iid, x) in last.iter() {
            let a = assets.get(&iid).unwrap();

            if a.location_id != x.location_id {
                dbg!(a);
            }
        }
        last = assets;

        println!("Done");
        tokio::time::sleep(std::time::Duration::from_secs(60 * 10)).await;
    }
}
