# Installation Guide

> OpenDAW is provided as a single executable of a few megabytes and can be launched instantly without installation.

## Overview
This is a guide for preparing the runtime environment for OpenDAW.
Depending on your needs, you can choose to use the portable binary or build from source.

## Running the Binary (Recommended)
This is the easiest way to try out OpenDAW.

1. Go to the [Releases](https://github.com/adasima/opendaw/releases) page of the GitHub repository.
2. Download the latest ZIP file that matches your OS (e.g., `opendaw-portable-windows.zip`).
3. Extract the downloaded ZIP file to a folder of your choice.
4. Double-click `opendaw` (or `opendaw.exe` on Windows) in the extracted folder to launch the application.

## Building from Source
If you want to try the latest development version or modify the code, you can build from source.

1. Ensure that you have the Rust build environment installed.
2. Clone the repository by running the following command:
   `git clone https://github.com/adasima/opendaw.git`
3. Navigate to the cloned directory:
   `cd opendaw`
4. Build and run the application using the following command:
   `cargo run --release`

## Detailed Settings
Currently, no special configuration files are required for installation or the first startup.
Settings are managed within the application.

## Related Topics
- [First Project](first-project.md)
- [UI Overview](ui-overview.md)

## Notes and Limitations
> - When building from source on a Linux environment, you need to install dependencies such as `libasound2-dev` and `libudev-dev` beforehand.
