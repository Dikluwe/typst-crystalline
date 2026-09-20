# Prompt L0 — teste público de `float.is-infinite`
Hash do Código: cd4d481b

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
