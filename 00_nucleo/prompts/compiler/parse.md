# Prompt L0 — fachada pública `compiler/parse`
Hash do Código: 56e53f3d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/mod.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O consumer declara seis submódulos privados e expõe apenas `parse`,
`parse_code`, `parse_math` e `parse_anchored`, além dos testes da fachada. As
gramáticas e o motor já estão atomizados.

## Contrato

- `parse` produz raiz `Markup`, `parse_code` raiz `Code` e `parse_math` raiz
  `Math`, sempre como funções puras e tolerantes a erro sintático.
- Cada entrada cria `Parser` no modo correspondente, executa a gramática dona e
  finaliza a raiz.
- `parse_anchored` despacha pelo `SyntaxMode` e sintetiza todos os spans quando
  o anchor não é detached; anchor detached preserva os spans do parser.
- A fachada não contém regras específicas de gramática.

## Aceitação

Preservar raízes, morfologia, erros e anchoring vigentes. Mudança das quatro
funções públicas ou do comportamento default fica sob ADR-0127.
