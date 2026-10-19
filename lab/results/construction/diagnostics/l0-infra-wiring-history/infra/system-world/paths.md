# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/system-world/paths.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `48cdad3a1b9315cbf474d7c89cba476da7067f6ab7e37b9ee2effe8022904dfc`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — SystemWorld: resolução de caminhos
Hash do Código: aa3d0b20

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/world/paths.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

`03_infra/src/world.rs:472-586` possui acesso a paths, realização física e
bibliografia; `:714-810` implementa paths rooted, read e include.
Raiz lexical/absoluto/relativo é semântica; realização física é mecânica L3.
Mensagens de erro são observáveis e devem permanecer idênticas (ADR-0107).

## Responsabilidade e algoritmo

Possui `root`, `directory_of`, `path_of`, `realize_rooted`,
`resolve_physical_path`, `load_bibliography` e helpers internos para os
métodos de paths/read/include do trait. Não cria cache paralelo.

- Path legado absoluto usa raiz do pacote do current_file, ou raiz do projeto;
  relativo usa seu diretório, com fallback à raiz para ID não registrado.
- `resolve_path` preserva `VirtualRoot::Project` ou identidade completa de
  `VirtualRoot::Package(spec)`; normalização lexical permanece nos DTOs L1.
  ID desconhecido, base fora da raiz, escape e backslash conservam seus erros.
- Realização usa raiz física escolhida pelo helper de pacotes, junta a vpath
  e conserva o teste atual de prefixo. Não acrescentar canonicalização ou
  endurecimento de sandbox neste refactor; isso exige uma alteração separada.
- read/include registram ID canônico e usam os caches de
  `00_nucleo/prompts/infra/system-world/cache.md`. Conservar mensagens,
  inclusive `searched at`, access denied e `include: ficheiro não encontrado`.
- Bibliografia lê pelo trait, valida UTF-8 e delega ao parser BibTeX L1;
  preserva formato de erro e base relativa ao current_file.

## Aceitação

Absolutos e relativos mantêm conteúdo e raiz; caminhos rooted preservam
origem entre arquivos. Escape lexical é rejeitado e falhas mantêm mensagem.
