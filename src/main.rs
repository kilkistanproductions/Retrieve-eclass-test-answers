use thirtyfour::prelude::*;


#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4442", caps).await?;

    // Navigate to https://wikipedia.org.
    driver.goto("https://wikipedia.org").await?;
    
    let username = "ddimitr";
    let password = "RPeQxx9kYPQE3d$@";

    // Login url
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Get input field
    let username_field =driver.find(By::XPath("/html/body/div[2]/div[1]/div[3]/div/div/div[4]/div/div/div/div[2]/form/div[1]/div/input")).await.expect("Can't find username field");
    let login_field = driver.find(By::XPath("/html/body/div[2]/div[1]/div[3]/div/div/div[4]/div/div/div/div[2]/form/div[2]/div/input")).await?;

    //Send keys
    username_field.send_keys(username).await?;
    login_field.send_keys(password).await?;
    login_field.send_keys(Key::Enter).await?;

    // Sleep to observe the result (optional)
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Close the browser window
    driver.quit().await?;

    Ok(())
}
