# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/system-world/fonts.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `25306886db2a58fd02c60faba3536416341dd4717b9c8f2438658916fa6e5e12`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — SystemWorld: fontes e cobertura
Hash do Código: cf980a83

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/world/fonts.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

`03_infra/src/world.rs:293-364` compõe fontes; `:418-469` faz preload;
`:679-711` consulta fontes/coverage. Ordem de fallback e cobertura são
comportamento existente; a localização do código é mecânica de L3.

## Responsabilidade

Possui builders de fontes, preload, união embutida, visita de nós de texto
e helpers de font/candidates_for_char. Usa serviços existentes de fonts,
fontdb e embedded_fonts; não os substitui nem altera contratos L1.

- `with_fonts` filtra slots cuja FontInfo falha e alinha book/slots (P839).
- `with_system_fonts` usa catálogo do sistema; sem fontes não falha.
- `with_embedded_fonts`: texto embutido, depois math/code embutido.
- `with_fonts_and_system`: texto embutido → sistema → math/code embutido
  → projeto, conservando correspondência por índice e prioridade P754.
- Consulta de cobertura prefere coverage não-vazia do book sem abrir bytes.
  Só slots sem coverage válida extraem cmap lazy e cacheiam por índice
  (P938/P1410). Retorna índices com cobertura exata do codepoint, em ordem.
  Preservar também fallback vazio para dados/face inválidos e escopo do lock.
- Preload examina Text, MathText e RawTrimmed no source; união conservadora
  só usa slots marcados `<embedded>`. Primeiro caractere não coberto dispara
  candidates_for_char; conteúdo criado depois de eval permanece lazy.
- Não invalidar coverage nem corrigir reutilização dos builders neste lote.

## Aceitação

Book e slots permanecem alinhados; Nimbus cobre A e não CJK; projeto mantém
sua posição. Coverage pré-materializada não requer leitura física; a lazy
é reutilizada e font fora do índice devolve None.
