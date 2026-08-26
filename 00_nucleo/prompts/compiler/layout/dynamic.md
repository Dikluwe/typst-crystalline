# Prompt L0 — layout de elemento dinâmico
Hash do Código: 78623cc5

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/dynamic.rs`

## Contrato e aceitação

Resolver `Content::Dynamic` pelo trait Element e registrar o resultado no
Layouter sem alterar despacho estático dos elementos nativos. Erro do elemento
é diagnóstico; conteúdo e morfologia retornados seguem o mesmo pipeline.
