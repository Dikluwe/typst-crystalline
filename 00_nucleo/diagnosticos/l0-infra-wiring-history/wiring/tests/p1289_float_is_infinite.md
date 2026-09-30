# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/tests/p1289_float_is_infinite.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `fd841970efd78f5833aa3be382e19dff31c056a998ba0e4e695e9d92d2c0d5d7`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — teste público de `float.is-infinite`
Hash do Código: 4381390d

**Camada:** L4 — teste de integração
**Ficheiro alvo exclusivo:** `04_wiring/tests/p1289_float_is_infinite.rs`

## Contrato

Executar o binário cristalino e verificar diretamente:

- finitos retornam `false`;
- infinitos positivo e negativo retornam `true`;
- NaN retorna `false`;
- identidade pública, aridade, named desconhecido e casts inválidos preservam
  os diagnósticos da linguagem.

O teste não depende de runner, baseline serializado, score, hash congelado ou
qualquer outro artefato administrativo.
