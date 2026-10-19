# Passo 978 — escala de tamanho de headings diverge do vanilla (e perde-se com `#set text`)

**Precede este passo**: achado incidental de P975 (a analisar os pares
flagged da secção 28 — as letras de maior |dx| eram de **headings**, não
de math). Duas divergências medidas:

1. **Factores errados**: `heading_scale` (`01_core/src/engine/layout/
   helpers.rs:254`) usa 2.0/1.667/1.333/1.167 (escala tipo HTML); o
   vanilla usa 1.4em para nível 1 e 1.2em para nível 2 (medido: nível 1
   a 15.4pt e nível 2 a 13.2pt sobre corpo de 11pt — `= 15.4/11`,
   `== 13.2/11`; nível 3+ fica no tamanho do corpo, 11pt, em negrito).
2. **Escala perdida com `#set text`**: no documento de 30 secções (que
   tem `#set text(font: "New Computer Modern", size: 11pt)`), os headings
   saem a **11pt** — sem escala nenhuma (medido: `== 28. …` a Tf 11.0 no
   cristalino contra 13.2 no vanilla). Sem o `#set text`, o cristalino
   aplica os seus factores (errados). Ou seja: o `#set text(size:)`
   sobrepõe-se à escala do heading em vez de a multiplicar — o vanilla
   aplica a escala do heading **sobre** o tamanho corrente (em
   relativo).

**Pré-condição de árvore**: `git status`. Confirmar P975-977 presentes.

---

## Fase A — confirmar a fórmula real do vanilla

1. Ler as regras de heading do vanilla (show-set de `HeadingElem` —
   provavelmente `text(1.4em)`/`text(1.2em)` por nível + bold) e confirmar
   os factores por nível para níveis 1-6 (medir nível 1, 2 e 3 num caso
   mínimo com e sem `#set text(size:)`).
2. Confirmar por que o cristalino perde a escala com `#set text`:
   onde a cadeia de estilos do heading é construída (`heading.rs` /
   `native_heading` / rules) e porque o size explícito ganha à escala em
   vez de compor com ela.
3. Verificar se há testes existentes que travam os factores actuais
   (2.0/1.667/…) — se houver, são testes a corrigir (eram calibrados
   contra o comportamento errado), com nota no relatório.

## Fase B — Implementação

TDD directo se pontual (correcção de factores + composição com `#set
text`); protocolo de dois agentes se a cadeia de estilos exigir
reestruturação. Gate ADR-0127 se tocar contrato público.

1. Testes: níveis 1/2/3 com e sem `#set text(size:)` — tamanhos do
   vanilla (1.4em/1.2em/1.0em sobre o tamanho corrente).
2. Implementar.
3. Suíte verde.

## Fase C — Revalidação

1. Casos mínimos medidos contra o vanilla (Tf sizes iguais nos dois).
2. Documento de 30 secções: headings à mesma largura/altura do vanilla
   (os pares de letras de heading flagged em 4/25/28 devem desaparecer do
   compare.py).
3. Benchmark completo, 7 cenários, `depois/antes`, zero regressão.

## Resultado esperado

- Headings com a escala do vanilla (1.4em/1.2em/1.0em, relativa ao
  tamanho corrente) em todos os níveis, com e sem `#set text`.
- Benchmark sem regressão.
