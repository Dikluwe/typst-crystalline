# Prompt L0 — integração black-box de linguagem e exportação
Hash do Código: c13d309a

**Camada:** L4 — teste de integração
**Ficheiro alvo exclusivo:** `04_wiring/tests/p1293_contract.rs`

## Contrato

Comparar observáveis públicos do cristalino com o vanilla ratificado, sem
congelar hash do executável e sem repetir casos para fiscalizar ordem de
execução.

- `float.is-nan`: identidade, chamadas ligadas, casts e diagnósticos.
- Matemática: `attach`, `binom`, `mono` e `script`, incluindo geometria
  semântica e ausência da barra em `binom`.
- HTML: identidade, fields, diagnósticos, DOM, atributos, escaping, elementos
  void, nesting e independência entre feature e target.
- `grid`/`table`: nomes curtos, aliases públicos, `with`, aridade e ausência
  dos aliases não expostos.

Temporários são exclusivos e processos têm timeout finito. A aceitação usa
semântica, morfologia, diagnósticos, geometria e DOM; não usa catálogos de
mutantes, autoria, ordem de agentes, confirmações, registros ou hashes de implementação.

## P1466 — export PNG não pode mascarar OOM

O caso black-box de PNG default exige sucesso, IHDR semanticamente correto e
processo dentro do limite de memória. O default documental 144 PPI corresponde
a 2 pixels por ponto; não comparar bytes comprimidos nem aceitar ausência de
arquivo, signal ou timeout como resultado esperado.
