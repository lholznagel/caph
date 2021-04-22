use caph_eve_data_wrapper::EveDataWrapper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = EveDataWrapper::new().await?;
    service.systems().await?;

    Ok(())
}
