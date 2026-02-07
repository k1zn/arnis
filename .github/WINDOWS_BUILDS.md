# Windows x86 Executable Builds

This document explains how Windows x86_64 executables are generated through our CI/CD pipeline.

## Automatic Builds

### CI Builds (Pull Requests and Main Branch)

Windows x86_64 executables are automatically built on:
- Every pull request that modifies relevant code
- Every push to the main branch

**To download Windows builds from CI:**

1. Navigate to the GitHub Actions tab
2. Find the workflow run for your commit/PR
3. Scroll to the bottom of the run page
4. Download the `arnis-windows-x86_64` artifact
5. Extract the ZIP file to get `arnis-windows-x86_64.exe`

**Note:** CI artifacts are retained for 7 days.

### Release Builds

Official Windows builds are created when a new GitHub release is published:
- Navigate to the [Releases page](https://github.com/k1zn/arnis/releases)
- Download `arnis-windows.exe` from the latest release

## Manual Builds

### Triggering a Release Build Manually

You can manually trigger the release workflow to generate all platform builds:

1. Go to the [Actions tab](https://github.com/k1zn/arnis/actions)
2. Select "Build and Release Arnis" workflow
3. Click "Run workflow" button
4. Select the branch to build from
5. Click "Run workflow"

The Windows executable will be available as an artifact once the build completes.

### Building Locally on Windows

To build the Windows executable locally:

```powershell
# Install Rust (if not already installed)
# Visit: https://rustup.rs/

# Build the project
cargo build --release --target x86_64-pc-windows-msvc

# The executable will be at:
# target/x86_64-pc-windows-msvc/release/arnis.exe
```

## Build Specifications

- **Target**: `x86_64-pc-windows-msvc` (64-bit Windows)
- **Architecture**: x86_64 (Intel/AMD 64-bit processors)
- **Compiler**: MSVC (Microsoft Visual C++)
- **Features**: All features enabled (GUI with Tauri, Bedrock support)

## Troubleshooting

### Missing DLL Errors

If you encounter missing DLL errors when running the executable:
- Install [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)
- Install [WebView2 Runtime](https://go.microsoft.com/fwlink/p/?LinkId=2124703) (for GUI version)

### Antivirus Warnings

Some antivirus software may flag the executable as suspicious because it's self-signed:
- The release builds are self-signed with a certificate
- You can verify the build by checking the workflow logs
- Or build the executable locally from source

## Notes

- CI builds generate unsigned executables
- Release builds are self-signed with a certificate for Windows Defender compatibility
- Both 32-bit and 64-bit Windows are supported (x86_64 binaries run on both)
