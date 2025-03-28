#!/bin/bash

# Prompt for the current version
read -p "Current version: " version

# Set the app name without spaces around the equal sign
app_name="sherlock-wiki"

# Remove and create the release directory
rm -rf ~/.tmp/${app_name}-release/
mkdir -p ~/.tmp/${app_name}-release/

# Build the project
cargo build --release

# Copy the binary and LICENSE file to the release folder
cp target/release/${app_name} ~/.tmp/${app_name}-release/
cp LICENSE ~/.tmp/${app_name}-release/LICENSE

# Change to the release directory
cd ~/.tmp/${app_name}-release/

# Create a tarball with the app name and version
tar -czf ${app_name}-v${version}-bin-linux-x86_64.tar.gz ${app_name} LICENSE
