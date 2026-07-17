---
# P763h — Auditoria de todas as primitivas de desenho (inline vs block-level) + correcção coerente

> **Passo:** 763h
> **Data:** 2026-07-15
> **Foco:** P763g identificou que `line`/`circle` divergem do vanilla porque o vanilla trata essas primitivas como elementos **inline** (posicionadas pela baseline, com leading entre elas), enquanto o cristalino as trata como **elementos de bloco** (`ShapeKind` dentro de `ShapeElem`, empilhadas por altura própria). Isto não foi verificado para o resto das primitivas de desenho (`rect`, `square`, `ellipse`, `polygon`, `path`, `curve`) — pode ser um padrão sistemático, não um caso isolado de `line`/`circle`. Além disso, P763g fechou o AE=2730 como "fora do escopo" sem passar pela regra 1 do handoff ("nenhuma correcção sem decisão explícita registada") — uma tentativa de correcção parcial (centrar `circle` horizontalmente) foi feita e **piorou** o resultado (AE 402→2713 no caso isolado), confirmando que patches pontuais no modelo block-level não resolvem uma divergência de modelo arquitectural. Este passo audita todas as primitivas, regista a decisão explicitamente, e corrige de forma coerente — não patch por patch.
> **Tipo:** Sonda exaustiva + Decisão registada (regra 1) + Implementação coerente.
> **Tamanho:** L — pode ser XL se a decisão for migrar todas as primitivas para o modelo inline real; a sonda determina o tamanho real antes de comprometer a implementação.
> **ADR-0107 EM VIGOR** — a divergência inline/block é diferença de **linguagem** (resultado observável do documento diverge quando primitivas são combinadas), não diferença de implementação aceitável. **Regra 1 do handoff EM VIGOR** — a decisão de como corrigir tem de ficar registada, com justificação, antes do código. **Regra 5 do handoff** — a correcção final tem de passar pelo checklist dos 4 sub-layouts (grid, box, columns, place).
> **Dependências:** P763f (correcção de `place` em sub-frames), P763g (achado do modelo inline vs block, tentativa de patch parcial registada como fracassada).

---

## Parte A — Sonda: classificar cada primitiva no vanilla

Levantar a lista completa de primitivas de desenho do vanilla e confirmar, por leitura directa do código-fonte (não por suposição), se cada uma é elemento inline ou block-level:

```bash
grep -rln "struct.*Elem" lab/typst-original/crates/typst-library/src/visualize/*.rs
```

Para cada ficheiro encontrado (`line.rs`, `curve.rs`, `path.rs`, `polygon.rs`, `shape.rs` — que cobre `rect`/`square`/`circle`/`ellipse`, e quaisquer outros):

```bash
grep -n "#\[elem\|impl Show\|impl Layout\|Packed<.*Elem>.*realize\|BEHAVIOUR\|Behaviour::" lab/typst-original/crates/typst-library/src/visualize/<ficheiro>.rs
```

Confirmar o mecanismo real que decide inline vs block no vanilla — provavelmente em `typst_library::foundations::content` (`Behaviour`, `Content::is_inline` ou equivalente) ou em `typst_library::realize` (como o realizador agrupa conteúdo em parágrafos vs blocos). Não assumir a partir do nome da primitiva — confirmar cada uma:

| Primitiva | Inline ou block no vanilla (confirmar) |
|---|---|
| `line` | — |
| `curve` | — |
| `path` | — |
| `polygon` | — |
| `rect` | — |
| `square` | — |
| `circle` | — |
| `ellipse` | — |

## Parte B — Levantar o estado actual no cristalino

```bash
grep -n "enum ShapeKind\|ShapeElem\|fn layout_shape" 01_core/src/entities/*.rs 01_core/src/rules/layout/*.rs
```

Confirmar se todas as primitivas passam pelo mesmo `ShapeElem`/`ShapeKind` (como P763g indica para `Line`/`Ellipse`) ou se há tratamento diferenciado já existente para alguma. Preencher a mesma tabela da Parte A com a coluna "cristalino (actual)".

## Parte C — Medir a divergência real para cada combinação

Não assumir que todas as primitivas têm o mesmo problema de `line`+`circle`. Medir, para cada par de primitivas relevante (mesmo padrão de P763g: rasterizar com `mutool draw -r 300`, comparar com `compare -metric AE`, confirmar com `mutool trace`):

```bash
# Modelo — repetir para cada primitiva isolada e para combinações de duas primitivas em sequência
cat > /tmp/p763h-<primitiva>-isolada.typ <<EOF
#set page(width: 8cm, height: 4cm)
#<primitiva>(...)
EOF
```

Registar quais combinações realmente divergem (como `line`+`circle`, AE=2730) e quais não (isoladas, já confirmado próximo do baseline por P763g).

---

## Decisão a registar (regra 1 — não prosseguir sem isto)

Com os dados das Partes A-C, registar explicitamente, com justificação:

| Opção | Descrição | Quando escolher |
|---|---|---|
| (1) Migrar para modelo inline real | Reescrever `ShapeElem` para produzir `InlineItem`s quando o vanilla trata a primitiva como inline, participando do fluxo de parágrafo com baseline/leading reais | Se a Parte A confirmar que a maioria/todas as primitivas são inline no vanilla — é a correcção correcta a longo prazo, mas maior |
| (2) Replicar o espaçamento equivalente dentro do modelo block | Ajustar `cursor_y`/`cursor_x` para produzir o mesmo resultado visual sem migrar o modelo | Só se a Parte C mostrar que um ajuste localizado resolve **todas** as combinações medidas — P763g já mostrou que um ajuste parcial (só horizontal) piora outros casos; esta opção só é válida com um ajuste completo validado, não outro patch pontual |
| (3) Scope-out consciente | Registar a divergência como conhecida e não corrigir agora | Só se nenhuma combinação problemática tiver uso real identificado no corpus do projecto (mesmo critério de uso real já aplicado em P766 para símbolos) |

Esta decisão **não é do Claude Code a tomar sozinho na execução** — se a Parte C revelar um padrão amplo (mais de duas ou três primitivas afectadas), parar aqui e devolver os dados das Partes A-C para decisão antes de implementar. Só prosseguir directamente para implementação se o padrão for estreito (confirmado só `line`+`circle`, como P763g sugere) e a opção (1) ou (2) for claramente a única viável pelos dados.

---

## Implementação (condicional à decisão)

Seguir a opção escolhida, aplicada de forma coerente a todas as primitivas afectadas identificadas na Parte C — não uma correcção por primitiva, uma correcção pelo modelo.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Repetir a medição da Parte C depois da correcção, mais o checklist de sub-layouts (regra 5) para o(s) documento(s) combinados que tinham divergência:

```bash
# grid, box, columns, place — mesmo padrão já usado em P763g Parte A
```

Confirmar que a correcção de `line`+`circle` (e quaisquer outras combinações encontradas na Parte C) chega ao baseline (~AE 241-300), com coordenadas confirmadas por `mutool trace`, não só a métrica de pixel.

---

## Critério de fecho do passo

- [ ] Todas as primitivas de desenho do vanilla classificadas (inline/block) por leitura directa do código-fonte.
- [ ] Estado actual do cristalino levantado para todas.
- [ ] Divergências reais medidas para as combinações relevantes, não assumidas por analogia com `line`+`circle`.
- [ ] Decisão registada explicitamente (opção 1, 2 ou 3), com justificação e, se o padrão for amplo, confirmação antes de implementar.
- [ ] Implementação coerente com a decisão — não patch pontual por primitiva.
- [ ] Combinações problemáticas chegam ao baseline, confirmadas por coordenadas.
- [ ] Checklist de sub-layouts (grid, box, columns, place) para os documentos combinados corrigidos.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p763h.md`, com a tabela de classificação completa e a decisão registada por escrito.

---

## Próximo passo

Se a decisão for (1) migração para modelo inline: pode ter impacto em mais código do que só `visualize` (parágrafos, `realize`, medição de linha) — avaliar se merece passo(s) dedicado(s) fora da numeração P763, dado deixar de ser específico do bug encontrado via `cetz`.
Se (2) ou (3): a linha de trabalho `cetz`/download de pacotes (P763–P763h) fecha, com a divergência resolvida ou conscientemente registada.
