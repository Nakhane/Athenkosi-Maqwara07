# Linear Regression Model

## SetUp
- Installed Rust: Downloaded and install Rust from Rust's official website.
- Verified installation: rustc --version
- Installed Rust Rover from JetBrains
- Installed Git from Git official site

## Project Creation
- Followed the instruction and updated the toml file with the given dependecies
- SetUp Github

## Generating Synthetic Data
- Created (x, y) pairs using \( y = 2x + 1 \) with noise.
- Used `rand` crate for randomness.

## Model Definition
- Utilised `burn` library to define a linear regression model.
- Implemented forward pass and Mean Squared Error (MSE) loss function.

## Training
- Used synthetic data for training.
- Adjusted learning rate to ensure convergence.

## Evaluation
- Tested on unseen data.
- Used `textplots` crate to visualise results.

# Challenges Faced

## 1. Dependency Errors
- Error downloading `burn` due to SSL certificate revocation issues.
- **Attempted solutions:**
  - Used VPN to bypass network restrictions *(did not work)*.
  - Switched to a different network *(also did not work)*.

## 2. Linker Issues
- **Error:** `link.exe not found`
- **Cause:** Missing Visual Studio Build Tools with C++ support.
- **Solution:**
  - Installed Visual Studio Build Tools.
  - Ensured `cl.exe` was available in `PATH`.

## 3. Git Push Rejection
- **Issue:** Remote repository had changes that were not pulled.
- **Solution:**
  ```sh
  git pull origin main --rebase
  git push origin main

# Alternative Solutions Attempted
- Switched IDEs from **Rust Rover** to **VS Code** as an alternative IDE but faced similar issues..
- Followed **YouTube tutorials** for Rust and `burn` Followed guides on setting up Rust and Burn but still faced difficults.
- Used online forums like **Rust Reddit** and **Stack Overflow**.

# Reflection

## AI Assistance
- Used AI to debug errors and improve approach.

## Documentation & Resources
- Official **Rust** and **burn** documentation were helpful.
- Understanding **dependencies and network issues** is crucial.


## Lessons Learned
- Setting up Rust properly requires **Visual Studio Build Tools**.
- **Git workflow** must be managed carefully to avoid conflicts.
- I had no prior knowledge of Rust or Rust Rover, which made it difficult to debug errors efficiently.
- Identifying the missing build tools was a major learning point.
- Although I was unable to successfully complete the linear regression model, this experience provided valuable exposure to Rust, its ecosystem, and troubleshooting development environment issues

