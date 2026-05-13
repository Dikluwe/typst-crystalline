# Passo 215 — Diagnóstico Layout Fase 3 (columns/colbreak; DEBT-56)

**Série**: 215 (passo único — diagnóstico-primeiro; abre
sub-série materialização se aceite em C8).
**Marco**: nenhum (terceiro passo pós-M9c; primeiro a
abrir nova trajectória de materialização — Layout Fase 3).
**Tipo**: diagnóstico arquitectural amplo + ADR PROPOSTO
column flow algorithm + roadmap sub-passos.
**Magnitude**: M (~2-3h documental — diagnóstico amplo
+ ADR draft + plano N sub-passos).
**Pré-condição**: P214 concluído (Layout §2.1 sincronizado
78%; 4 categorias ≥ 78%; pendência documental fechada);
Layout Fase 1+2 fechadas (P156C-I; 8 features); Fase 3
sub-passo 1 fechado (P156J `repeat`); Fase 3 sub-passo 2
fechado (P156L `pad` refino); ADR-0061 PROPOSTO (caminho
1: 50% concluído — repeat ✓, columns/colbreak pendentes);
DEBT-56 EM ABERTO desde P156B; tests 1939 verdes; 0
violations; humano fixou "focar no Layout até onde der".
**Output**: 1 ficheiro relatório curto (diagnóstico) +
1 ficheiro ADR novo PROPOSTO (column flow algorithm).

---

## §1 Trabalho

P214 confirmou Layout em 78% (sincronização §2.1 pós-P156L).
**Restam 4 entradas em §A.5 do inventário 148** que separam
Layout de 100%:

1. `columns(n)` — **ausente** (Fase 3; DEBT-56 L+).
2. `colbreak()` — **ausente** (depende de columns; DEBT-56).
3. `place(...)` — **parcial** (sem float/clearance; refino
   column scope).
4. `measure(body)` — **parcial** (helper privado; sem stdlib
   expose; ADR-0066 Bloco C).

`pad/corners/sides` listado em §A.5 linha 150 duplica `pad()`
linha 135 — não é entrada distinta per P156B reconta.

**Decisão central de P215**: aplicar padrão diagnóstico-primeiro
(11ª aplicação documentada — P148, P154A, P156B, P157, P158,
P159, P159B, P160, P213, P214, **P215**) para fixar:

- Inventário código actual do Layouter face a multi-region.
- Pré-condições DEBT-56 (5 itens documentados); validar.
- ADR-0078 PROPOSTO column flow algorithm (paridade
  ADR-0061 PROPOSTO de Layout Fase X + ADR-0066 PROPOSTO
  Introspection runtime).
- Estrutura granular sub-passos paridade P156C-L (10 passos
  Layout cumulativos em 2 semanas).
- Reservas P216-Y (NÃO compromisso; documentar opções).

**Decisão alternativa rejeitada**: ir directo a materialização
sem diagnóstico. Rejeitada porque DEBT-56 é o maior refactor
estrutural pendente do projecto (L+ ~5-8h); pular diagnóstico
viola ADR-0034 + ADR-0065. P156B (Layout Fase X amplo) foi
diagnóstico ANTES de P156C série; P215 paridade.

Reuso de dados (sem recolha nova):

- DEBT-56 conteúdo (4 sub-secções: Contexto / Diferença
  vanilla / Pré-requisitos / Plano).
- ADR-0061 §"Aplicações cumulativas" pós-P156L (10 passos
  registados).
- P156B diagnóstico (precedente metodológico directo).
- DEBT-37 fecho P84.6 (`PlaceScope::Parent` para
  columns) — relevante para place refino.
- ADR-0054 graded (perfil scope-out autorizado).
- ADR-0064 (Smart→Option/default — replicável em
  ColumnsElem fields).

---

## §2 Cláusulas (8)

### C1 — Inventário código actual face a multi-region

Auditoria empírica do Layouter actual em
`01_core/src/rules/layout/`:

| Componente | Estado actual | Multi-region? |
|------------|---------------|----------------|
| `Layouter` struct | single-page write-target | Não |
| `current_items: Vec<FrameItem>` | escrita directa | Não — sem `Region` |
| `current_line: Vec<FrameItem>` | acumulador de linha | Não |
| `cursor_x`/`cursor_y` | escalares globais | Não — sem iteração regions |
| `page_config.width`/`height` | dimensões directas | Não — sem `width / count - gutter` |
| `flush_line` / `flush_page` | escreve em `pages` | Não — sem iteração de colunas |

Verificação:
- `grep -n "current_items\|current_line\|cursor_x\|cursor_y"
  01_core/src/rules/layout/mod.rs | wc -l` quantifica
  call-sites afectados pelo refactor.
- Hipótese provável: 40-80 call-sites (escala média; trabalho
  L+ mas tractável em sub-fases).

Confirmar:
- **Sub-fase (a)** DEBT-56: introduzir `Region`/`Regions`
  mantendo comportamento single-column. Magnitude esperada:
  M+ (~3-5h). Risco: médio (refactor cross-modular mas
  comportamento observable preservado).
- **Sub-fase (b)** DEBT-56: consumer multi-column +
  `Content::Columns` + `Content::Colbreak`. Magnitude
  esperada: M (~2-3h). Risco: baixo (sub-fase (a) já
  reduziu).

Total DEBT-56 estimado: **L (~5-8h)** vs L+ original.
Redução por decomposição em sub-fases.

Se inventário revelar > 100 call-sites ou complexidade
inesperada: registar `P215.div-1` e reajustar magnitude.

### C2 — Inventário entradas Fase 3 restantes

Mapear cada entrada restante face a multi-region:

| Entrada | Magnitude | Depende sub-fase (a)? | Depende sub-fase (b)? | Notas |
|---------|-----------|------------------------|------------------------|-------|
| `columns(n)` | M | sim | sim | core feature DEBT-56 |
| `colbreak()` | S+ | sim | sim | depende columns; mixing pagebreak |
| `place` float/clearance | S+ | parcial (column scope) | não | refino DEBT-37 fechado P84.6 |
| `measure(body)` stdlib | S+ | não (helper L1 existe) | não | ADR-0066 Bloco C; isolado |
| `grid` header/footer real | M | sim | sim | DEBT-56 difere repetição em page breaks |
| `TableHeader.repeat` algoritmo | S+ | sim | sim | P157C diferiu em DEBT-56 |

**Distinção crítica**: 4 entradas dependem de sub-fase (a)
multi-region (`columns`, `colbreak`, `grid` header/footer,
`TableHeader.repeat`); 2 entradas isoladas (`place` refino,
`measure` stdlib).

Sub-fase (a) **desbloqueia 4 entradas em cascata**; sub-fase
(b) materializa 2 directas (`columns` + `colbreak`).

`measure` stdlib **NÃO depende de DEBT-56**; pode ser
materializado em paralelo ou primeiro. Confirmar com
auditoria final em C7.

### C3 — ADR-0078 PROPOSTO column flow algorithm

Criar `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
em status PROPOSTO. Conteúdo mínimo (paridade ADR-0061
estrutura):

- **Status**: PROPOSTO 2026-05-12.
- **Diagnóstico prévio**: este P215 relatório.
- **Contexto**: DEBT-56 pré-requisitos + Layouter actual
  single-page write-target.
- **Decisão**: abstracção `Region` / `Regions` (single-column
  como caso especial de 1 region); column flow como N regions
  sequenciais com `width / count - gutter`. Sem balanceamento
  altura per vanilla.
- **Decisão arquitectural**: refactor **em sub-fases** (a)
  introduzir `Region` mantendo single-column + (b) consumer
  multi-column. Risco reduzido.
- **Não-objectivos**: balanceamento altura (vanilla também
  não faz); col span (não no vanilla typst); cross-page
  column flow rules específicas.
- **Plano de materialização**: 6 sub-passos P216-P221
  (paridade P156C-L granular).
- **Reservas**: P216-P221 NÃO compromisso (decisão humana
  pós-P215).

Promoção PROPOSTO → IMPLEMENTADO ocorre quando
`columns/colbreak` materializados (sub-passo P221 ou
equivalente fechamento da série).

### C4 — Roadmap granular sub-passos (estilo P156C-L)

Sequência candidata (paridade ADR-0061 §"Aplicações
cumulativas"):

| Sub-passo | Trabalho | Magnitude | Cumulativo |
|-----------|----------|-----------|------------|
| **P216** | sub-fase (a) — `Region`/`Regions` abstraction; single-column preservado | M+ | refactor estrutural; tests inalterados |
| **P217** | `Content::Columns { count, gutter, body }` variant + arms exhaustivos + `is_empty`/`plain_text`/`map_*`/`materialize_time`/`walk` | S+ | enum 56→57 variants |
| **P218** | `native_columns` stdlib + `extract_count` helper + validação count≥1 | S | stdlib 53→54 funcs |
| **P219** | sub-fase (b) — Consumer multi-column no Layouter; iteração N regions reduzidas; `width / count - gutter` | M+ | tests `columns(2)`/`columns(3)` |
| **P220** | `Content::Colbreak { weak: bool }` + `native_colbreak` + mixing pagebreak tests | S+ | enum 57→58; stdlib 54→55 |
| **P221** | encerramento Fase 3 — ADR-0061 PROPOSTO → IMPLEMENTADO; ADR-0078 PROPOSTO → IMPLEMENTADO; DEBT-56 fecha; inventário 148 actualiza | XS | documental |

**Sub-passos opcionais Bloco C** (paralelos; não bloqueiam):

| Sub-passo opcional | Trabalho | Magnitude | Dependência |
|---------------------|----------|-----------|--------------|
| **P222** (Bloco A Introspection) | `measure(body)` stdlib expose | S+ | nenhuma (sub-fase a opcional) |
| **P223** | `place` float/clearance refino | S+ | sub-fase (a) parcial |
| **P224** | `grid` header/footer real + `TableHeader.repeat` algoritmo | M+ | sub-fases (a)+(b) completas |

Total série: 6 sub-passos núcleo (P216-P221) + 3 opcionais
(P222-P224) = 9 candidatos. Custo agregado núcleo: ~M+M+S+S+M+S =
**L cumulativo (~6-9h)** vs DEBT-56 estimado L+ original.

### C5 — Política "sem novas reservas" (P158)

P158 estabeleceu política "sem novas reservas". P215
documenta P216-P224 como **opções** identificadas, NÃO
reservas. Decisão humana sobre prosseguir fica em aberto.

Distinção crítica:
- **Reserva** = compromisso de numeração + escopo (e.g.
  P156, ADR-0061 antes de uso).
- **Opção identificada** = candidato documentado sem
  compromisso (P216-P224 neste passo).

ADR-0078 PROPOSTO é **decisão arquitectural** (não reserva
de número), análoga a ADR-0061 PROPOSTO em P156B (autorizou
abordagem sem compromisso de quando materializar).

### C6 — Marca blueprint §3.0duodecies

Adicionar marca cirúrgica nova após §3.0undecies P214,
antes de §3.1.

**Decisão sobre forma**:
- **Opção α** — marca paridade encerramentos série
  (§3.0quater-§3.0octies).
- **Opção β** — marca paridade diagnóstico-amplo (P156B/
  P159B não criaram marca blueprint per padrão; P215
  poderia seguir).
- **Opção γ** — marca cirúrgica paridade P213/P214
  (recálculos com marca explícita).

Hipótese provável: **Opção α** — marca §3.0duodecies
"Diagnóstico Layout Fase 3 + ADR-0078 PROPOSTO + roadmap
P216-P224". Consistente com pattern marca-por-fecho
estabelecido pós-P204H.

Conteúdo:
- ADR-0078 PROPOSTO column flow algorithm.
- DEBT-56 com sub-fases decomposto (L+ → L cumulativo).
- 6 sub-passos núcleo (P216-P221) + 3 opcionais
  (P222-P224).
- Pattern emergente "diagnóstico-primeiro" cresce N=10
  → 11.
- Política "sem novas reservas" preservada — P216-P224
  são opções, não reservas.

### C7 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
grep -n "ADR-0078\|P215" 00_nucleo/adr/README.md
grep -n "P215" 00_nucleo/diagnosticos/blueprint-projecto.md
```

Critério:
- Tests 1939 verdes (Δ 0 — sem código tocado).
- 0 violations preservadas.
- ADR-0078 ficheiro existe em status PROPOSTO.
- Blueprint §3.0duodecies adicionada.

### C8 — Decisão humana sobre prosseguir

P215 fecha diagnóstico. Decisão humana sobre próxima sessão:

- **Caminho 1** — prosseguir série materialização P216
  (sub-fase a) imediatamente. Custo cumulativo ~6-9h
  distribuído em 6 sessões pequenas.
- **Caminho 2** — começar pelos Bloco C isolados (P222
  `measure` stdlib expose) primeiro. Win rápido S+; não
  bloqueia DEBT-56. Layout 78% → 89% (1 entrada parcial
  → impl).
- **Caminho 3** — adiar DEBT-56; voltar a outro módulo
  (Model hayagriva P159-resto; outro recálculo de
  categoria; etc.).
- **Caminho 4** — diagnóstico mais profundo se C1
  revelou complexidade inesperada (e.g. > 100 call-sites
  Layouter). Sub-passo P215-bis (diagnóstico-refino).

Hipótese provável humano fixou: **Caminho 1** ("focar no
Layout até onde der" sugere materialização imediata pós-
diagnóstico). Mas fica em aberto literal.

---

## §3 Output

2 ficheiros:

1. **Relatório do passo**:
   `00_nucleo/materialization/typst-passo-215-relatorio.md`.

   Estrutura (~6-8 KB) com 8 §s:
   - §1 O que foi feito (sumário 3-5 linhas).
   - §2 Inventário Layouter face a multi-region (C1
     resultados).
   - §3 Inventário entradas Fase 3 restantes (C2 tabela
     6 entradas + dependências sub-fases).
   - §4 ADR-0078 PROPOSTO sumário (C3 + cross-reference).
   - §5 Roadmap granular P216-P221 + opcionais P222-P224
     (C4).
   - §6 Decisões substantivas (sub-fases vs big-bang;
     política sem reservas).
   - §7 Marca §3.0duodecies blueprint (forma C6).
   - §8 Decisão humana caminhos 1-4 (C8; sem fixar).

2. **ADR novo**:
   `00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`.

   Estrutura paridade ADR-0061 (Status PROPOSTO; Data;
   Diagnóstico prévio; Contexto; Decisão; Não-objectivos;
   Plano materialização; Reservas; Análise paridade vanilla;
   Referências).

---

## §4 Não-objectivos

- Materializar sub-fase (a) ou (b) DEBT-56 — P215 é
  diagnóstico-primeiro puro. Sub-passos P216+ se humano
  prosseguir.
- Materializar `Content::Columns` ou `Content::Colbreak`
  — diferido a P217/P220.
- Materializar `measure` stdlib expose — diferido a P222
  (opcional, paralelo).
- Promover ADR-0078 PROPOSTO → IMPLEMENTADO — só após
  sub-fases (a)+(b) materializadas (P221).
- Promover ADR-0061 PROPOSTO → IMPLEMENTADO — só após
  Fase 3 fechada (P221).
- Fechar DEBT-56 — só após P221 (sub-fase b consumer
  multi-column completo).
- Reabrir DEBT-37 (`PlaceScope::Parent`) — fechado em
  P84.6; refino `place` float/clearance é trabalho novo
  (P223 opcional).
- Tocar em código `.rs` — passo documental puro.
- Tocar em hashes L0 — nenhum prompt L0 alterado.
- Auditar outras categorias além de Layout.

---

## §5 Riscos a evitar

1. **Subestimar complexidade Layouter refactor**: C1 exige
   contar call-sites empíricamente, não assumir. Se > 100
   call-sites: registar `P215.div-1` e propor sub-fase (a)
   mais granular (P216A/P216B).
2. **Acoplar P215 com materialização**: paridade ADR-0034
   diagnóstico-primeiro — P215 só audita + propõe. P156B
   foi diagnóstico antes de P156C série; mesma forma.
3. **ADR-0078 com decisão prematura**: PROPOSTO permite
   ajustes em sub-passos. Não fixar todos os detalhes
   (e.g. `gutter` default value, `count` validação) —
   deferir a P217.
4. **Reservar P216-P224**: política P158 "sem novas
   reservas". Mesmo com 6+3 sub-passos identificados,
   tratá-los como opções no relatório, não reservas.
5. **Marca §3.0duodecies inflada**: preserva pattern
   minimal de marca-por-fecho. Diagnóstico amplo P156B
   não criou marca blueprint per pattern P204H+
   "fora-de-escopo reescrita ampla"; P215 segue mesmo
   pattern.
   Reconsiderar: Opção α pode ser inflação. **Opção β**
   (sem marca blueprint, paridade P156B/P159B) talvez
   mais consistente. Fixar em C6 baseado em precedente.
6. **Confundir diagnóstico vs Bloco C opcional**:
   `measure(body)` stdlib expose (P222 opcional) é
   tentação como "primeiro passo barato". P215 documenta
   mas não materializa. Materialização decide humano em
   C8.
7. **Inventariar entradas Fase 3 incompletamente**: §A.5
   tem 18 entradas; 14 implementadas; 4 restantes. C2
   deve cobrir as 4 + as 2 sub-entradas P157C
   (TableHeader.repeat; grid header/footer) que estão em
   §A.6 Model mas dependem de DEBT-56.

---

## §6 Hipótese provável

C1 confirmará Layouter actual single-page write-target;
40-80 call-sites afectados; refactor sub-fase (a) M+
tractável.

C2 confirmará 6 entradas Fase 3 com dependências variadas;
4 dependentes de DEBT-56 sub-fases; 2 isoladas.

C3 fixará ADR-0078 PROPOSTO column flow algorithm;
paridade estrutural ADR-0061.

C4 fixará roadmap 6 sub-passos núcleo (P216-P221) + 3
opcionais (P222-P224). Custo agregado L cumulativo
(~6-9h).

C5 preservará política "sem novas reservas" — P216-P224
documentados como opções.

C6 fixará Opção β — sem marca blueprint (paridade P156B
diagnóstico-amplo). Reduz inflação documental.
**Reconsiderar em C6**: se humano fixar Caminho 1
imediatamente, marca pode ser útil; se Caminho 3 (adiar
DEBT-56), marca é desnecessária.

C7 reportará tests/lint preservados.

C8 deixará caminhos 1-4 em aberto; recomendação
metodológica subjectiva favorece Caminho 1 ("focar no
Layout até onde der" do humano sugere prosseguir).

Custo real: M (~2-3h documental). Sem código tocado.

Mas é hipótese, não decisão. C1-C8 fixam-se empíricamente.

---

## §7 Particularidade P215

P215 é estruturalmente distinto na trajectória pós-M9c:

- **Primeiro passo pós-M9c que abre série de materialização**
  — pós-P213/P214 (administrativos), P215 prepara terreno
  para P216+ (materialização real).
- **11ª aplicação consecutiva** do padrão
  diagnóstico-primeiro (paridade ADR-0034 + ADR-0065).
  Pattern consolidado.
- **Primeira ADR PROPOSTO pós-M9c** — ADR-0078 é decisão
  arquitectural nova (column flow algorithm). Análoga a
  ADR-0061 PROPOSTO em P156B mas escopo focado (uma
  sub-feature vs roadmap inteiro).
- **Decompõe DEBT estimado L+ em sub-fases L cumulativo**:
  precedente metodológico novo. Permite ataque incremental
  análogo P156C-L mas com refactor estrutural (vs P156C-L
  que era aditivo).
- **Sub-fase (a) refactor sem mudança observable**:
  desafio único — sub-passo M+ que preserva todos os tests
  workspace. Risco gerido por testes existentes como
  regression suite.
- **Bloco C opcional paralelizável** (`measure`, `place`
  refino, grid header/footer): primeira identificação
  explícita de sub-passos não-sequenciais em série Layout.

Por isso §5 risco 5 ("marca blueprint inflada") é
relevante — diagnóstico amplo pode ser tratado sem marca
(paridade P156B) ou com marca (paridade P213/P214).
Decisão a fixar em C6 baseada em precedente metodológico
e custo documental.
