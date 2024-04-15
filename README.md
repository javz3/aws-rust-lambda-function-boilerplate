# aws-rust-lambda-function-boilerplate
A simple boilerplate in Rust for an AWS Lambda Function

## Set-up

### Installing Mysys2
Install https://www.msys2.org/
In the Msys2 terminal 
- `pacman -S mingw-w64-x86_64-gcc`
- `export PATH=$PATH:/mingw64/bin`
- `echo "export PATH=\$PATH:/mingw64/bin" >> ~/.bashrc`
- `source ~/.bashrc`

### Setting up Mysys2 to be used across all terminals in Windows
- From Start, serach `Environment Variables` In the System Properties dialog, click on the "Environment Variables" button.
- Modify PATH: Under `System variables` (which affects all users) or `User variables` (which affects only the current user), find the PATH variable and click- `Edit.`
- Add MSYS2 Path: Add the path to your MSYS2 gcc executable. Typically, this would be something like `C:\msys64\mingw64\bin` for a 64-bit installation.
- Save and Exit
