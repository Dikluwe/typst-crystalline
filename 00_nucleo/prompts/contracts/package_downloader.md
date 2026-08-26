# Prompt L0 — `contracts/package_downloader` — porta pura

Hash do Código: ee741d0d

**Camada:** L1
**Ficheiro proprietário:** `01_core/src/contracts/package_downloader.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/packages/downloader-contract.toml sha256:1fe7c4c5a8dff2081396620797bf0f94b051128c75fcaf306d8a691a50d4d91a

## Contrato

`PackageDownloader: Send + Sync` recebe `PackageSpec` e devolve path local ou
`PackageDownloadError`. O enum distingue NotFound, VersionNotFound, NetworkError
e IoError com mensagens observáveis, sem importar bibliotecas HTTP/I/O externas.
Implementação, cache, TLS e rede pertencem exclusivamente a L3.
