# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/system-world/packages.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `b3d678bc74e22b00544aa462f5ca1572071a0e3588e7574a6b8efe5b74395a69`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — SystemWorld: pacotes e downloader
Hash do Código: f8907a88

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/world/packages.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

`03_infra/src/world.rs:76-100` define bases e cache padrão; `:278-287`
injeta CA; `:498-542` identifica raiz/pacote; `:625-652` carrega manifesto;
`:812-840` procura offline antes do downloader preview. A ordem é comportamento
existente; não se infere intenção de produto nem se altera a política de rede.

## Responsabilidade

Possui bases de procura, cache padrão, builder `with_custom_ca`, identidade
do pacote, raiz física e leitura do entrypoint, mais helper do trait.
O downloader continua sendo o serviço especificado em
`00_nucleo/prompts/infra/package_downloader.md`; não duplicar seu mecanismo.

- Data dir: XDG_DATA_HOME ou HOME/.local/share; depois cache dir:
  XDG_CACHE_HOME ou HOME/.cache, ambos sob typst/packages.
- Cache padrão é a última base disponível. Sem base não instala downloader.
- Cada candidato usa namespace/name/version. Primeiro diretório existente
  vence; manifesto inválido não muda silenciosamente para outra base.
- Ler typst.toml, extrair [package].entrypoint, registrar arquivo e usar
  source cacheado. Preservar mensagens em todos os caminhos de erro.
- Só namespace preview tenta download após esgotar diretórios locais.
  Falha do downloader propaga seu Display; ausência final retorna
  `package not found (searched for {spec})` (P827, substitui erro PT histórico).
- Identificação física conserva namespace/name/version; a forma de busca
  permanece prefixo do path e ordem atual, sem canonicalização adicional.
- `with_custom_ca` recompõe downloader sem ler certificado antecipadamente;
  preserva injeção lazy e claims do Núcleo pinado.

## Aceitação

Pacote local com manifesto e imports internos carrega; falhas de manifesto,
entrypoint e pacote não-preview conservam erro. CA não é lida pelo builder.
