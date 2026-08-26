# Prompt L0 — `compiler/eval/bibliography`
Hash do Código: fcd1d272

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/bibliography.rs`
**Vanilla ratificado:** `a51e02804`

## Medição e contrato

O consumer lê bytes somente por `World`, reconhece BibTeX/YAML, converte
Hayagriva em `BibEntry` e resolve CSL builtin ou local. Extensão, UTF-8, parse e
CSL inválidos geram diagnósticos; paths relativos usam o FileId do documento.

## Aceitação

Fixtures BibTeX/YAML, erros e CSL preservam campos e diagnósticos medidos.
