# Syzygy

Syzygy (sĭz′ə-jē) noun : an alignment of three celestial bodies

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/syzygy_dark.png" />
    <img alt="Syzygy Logo" src="assets/syzygy.png" width="300"/>
  </picture>
</p>

<h1 align="center">
  Syzygy
</h1>

<p align="center">
   A modern alternative to Burp Suite built on top of <a href="https://github.com/ppmpreetham/zopra">Zopra</a> and GPUI
</p>

<p align="center">
  Syzygy (sĭz′ə-jē) noun : an alignment of three celestial bodies
</p>

## Installation

### Build from source

1. Clone the repo:
   ```bash
   git clone https://github.com/ppmpreetham/syzygy.git
   ```
2. Build the project:
   ```bash
   cd syzygy
   chmod +x build.sh
   ./scripts/build.sh
   ```
3. The executable will be located in `target/release/syzygy`.

> [!NOTE]
> On Windows, you can use `build.bat` instead of `build.sh`.

```bash
examples/build.bat
```

## Setup

<details>
<summary>Windows</summary>

1. Open Windows Explorer and navigate to that folder path.
2. Press `Win + R` and Go to `%AppData%\Roaming\syzygy\SyZyGy\config\keys\syzygy_ca.crt`.
3. Double click `syzygy_ca.crt` to open it.
4. Click Install Certificate...
5. Select Current User or Local Machine -> Next.
6. Choose "Place all certificates in the following store" -> click Browse.
7. Select Trusted Root Certification Authorities -> click OK -> Next -> Finish.

</details>

<details>
<summary>Linux</summary>

### Linux

1. Open a terminal and navigate to that folder path.
2. Run the following command:
   ```bash
   sudo cp syzygy_ca.crt /usr/local/share/ca-certificates/
   sudo update-ca-certificates
   ```

</details>

<details>
<summary>MacOS</summary>

1. Open a terminal and navigate to that folder path.
2. Run the following command:
   ```bash
   sudo security add-certificates -d /usr/local/share/ca-certificates/
   ```

</details>

## Star History

<a href="https://www.star-history.com/?repos=ppmpreetham%2Fsyzygy&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=ppmpreetham/syzygy&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=ppmpreetham/syzygy&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=ppmpreetham/syzygy&type=date&legend=top-left" />
 </picture>
</a>

## License

SyZyGy is licensed under the GNU Affero General Public License Version 3
(AGPL-3.0-only). See `LICENSE`.

Copyright (C) 2026 Preetham Pemmasani  
Contact: contact@preetham.top
