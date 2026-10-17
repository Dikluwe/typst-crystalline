# L0 — Package download boundary
Hash do Código: 2aa6b6f8

**Layer:** L1
**Exclusive consumer:** `01_core/src/contracts/package_downloader.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/packages/downloader-contract.toml sha256:1fe7c4c5a8dff2081396620797bf0f94b051128c75fcaf306d8a691a50d4d91a

## Contract

`PackageDownloader: Send + Sync` exposes
`download(&self, spec: &PackageSpec) -> Result<PathBuf, PackageDownloadError>`.
Preserve the requested namespace, name and version. The result identifies the
local extracted package root. `PackageDownloadError` distinguishes
`NotFound(PackageSpec)`, `VersionNotFound { spec, latest }`,
`NetworkError { url, cause }` and `IoError { path, cause }`.
Its observable `Display` messages must not be translated by this documentary
change. No HTTP/runtime error type crosses the boundary.
Network, filesystem operations, caching, extraction and TLS belong to L3;
this owner declares their pure interface, not their implementation.

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/contracts/package_downloader.rs` nas linhas 93 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `00_nucleo/diagnosticos/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Remover somente DummyDownloader e sua implementação local nunca instanciada no módulo de testes. Preservar o trait público, seus bounds, assinatura, erros e os testes de Display; não criar um teste que apenas confirme o próprio mock.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.
