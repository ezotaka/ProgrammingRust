use thiserror::Error;

fn main() -> Result<(), TideError> {
    let tides = calculate_tides()?;
    println!("{:?}", tides);
    Ok(())
}

#[derive(Debug, Error)]
#[error("{message:}")]
pub struct TideError {
    message: String,
}

fn calculate_tides() -> Result<(), TideError> {
    Err(TideError {
        message: "moon not found".to_string(),
    })
}
