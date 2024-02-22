use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    open::that("https://www.youtube.com/watch?v=dQw4w9WgXcQ")?;

    Ok(())
}
