# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/system-world/cache.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `e5a2dd941b14d6a28d677199835760523e13fae9ca66f6262ea37fe6dd23543d`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — SystemWorld: caches e identidade física
Hash do Código: a6bcdcdd

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/world/cache.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

No consumer anterior `03_infra/src/world.rs:37-71`, `SourceSlot` usa
`OnceLock<FileResult<Source>>`; `:368-404` registra IDs e dependências;
`:589-622` mantém o cache de bytes; `:668-677` implementa source/file.
São mecanismos de L3, não mudança na linguagem. Esta extração não corrige
concorrência ou política de invalidação incidentalmente.

## Responsabilidade

Possui `SourceSlot`, `register_file`, `dependencies`, leitura física de bytes,
cache por ID e os helpers internos chamados pelo adapter de World.
`SourceSlot` e operações usadas pela composição têm visibilidade somente
`pub(super)`. Os campos de estado continuam na fachada.

- Source sintético nasce materializado sem I/O; source físico é lazy,
  exceto a leitura eager do main feita pelo construtor.
- `OnceLock` guarda também erros; cada slot conserva seu primeiro resultado.
- Registro canonicaliza quando possível e usa o path original caso contrário.
  Preservar exatamente a alocação de IDs, wrapping e ordem/escopo dos locks.
- Bytes são cacheados somente após leitura bem-sucedida. Chamadas repetidas
  de `read_bytes` e `read_path` partilham `Arc<Vec<u8>>` pelo ID canônico
  (P876); `file` mantém a conversão atual para `Bytes`, sem otimização nova.
- Dependências incluem main, sources, assets e tentativas registradas com erro;
  devolvem paths ordenados e sem duplicados, sem incluir fontes do sistema.
- `NotFound` e demais erros conservam a conversão atual para `FileError`.

## Aceitação

Leituras repetidas conservam conteúdo mesmo após alteração do disco; source
mantém erro já materializado e leitura de bytes falhada permite nova tentativa.
Paths equivalentes registrados conservam ID; dependências mantêm inventário.
