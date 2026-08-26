# Prompt L0 — gramática matemática
Hash do Código: 4a4fd8ce

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/math.rs`
**ADRs:** ADR-0037, ADR-0107, ADR-0108, ADR-0129.

## Medição anterior à decisão

O consumer implementa expressões math, attachments, frações, roots,
delimitadores e argumentos posicionais/nomeados/arrays. Precedência e classe
math são específicas deste owner.

## Contrato

- Produzir `Math`, `MathAttach`, `MathFrac`, `MathRoot` e `MathDelimited`
  conforme tokens, trivia e precedência vigentes.
- Preservar chains de hat/underscore/primes e chamadas implícitas.
- Detectar argumentos nomeados duplicados e preservar arrays por semicolon.
- Código embutido delega ao owner Code; cursor e mode seguem o núcleo.

## Aceitação

Suíte math/parse GREEN sem alterar CST, precedências ou diagnostics.
