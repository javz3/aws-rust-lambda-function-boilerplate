# aws_rust_lambda_function_boilerplate
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
- Open System Properties: Right-click on the Start button and select "System." Then click on "Advanced system settings" to open the System Properties dialog.
- Environment Variables: In the System Properties dialog, click on the "Environment Variables" button.
- Modify PATH: Under "System variables" (which affects all users) or "User variables" (which affects only the current user), find the PATH variable and click "Edit."
- Add MSYS2 Path: Add the path to your MSYS2 gcc executable. Typically, this would be something like C:\msys64\mingw64\bin for a 64-bit installation. Ensure that you separate it from other entries with a semicolon (;).
- Save and Exit: Click "OK" to close each dialog.
