# Passo 208A — Diagnóstico-primeiro: `Tracked<Context>` análogo cristalino

**Série**: 208 (sub-passo `A` = diagnóstico-primeiro
reduzido).
**Marco**: M9c (Bloco IV — `here()` + `locate()`).
**Tipo**: diagnóstico-primeiro reduzido (zero código
tocado).
**Magnitude**: S-M (~45 min).
**Pré-condição**: P207E concluído; série P207 fechada;
trait 26 métodos; tests 1899 verdes; 0 violations;
ADR-0076 PROPOSTO; blueprint §3.0quater [P207E];
padrões emergentes P207 formalizados.
**Output**: 1 ficheiro (relatório curto consolidando
auditoria + decisões + plano P208B-D).

---

## §1 Trabalho

Mapear empíricamente como cristalino vai dar suporte a
`here() -> Location` na stdlib (per `P207A.div-1` Q1+Q2
fixadas; per relatório P207E §6 padrão 2).

Vanilla materializa `here()` via `Tracked<Context>`
passado a funcs stdlib. Cristalino tem diferenças
arquitectónicas:

- **Single-pass layout** (P190I) vs vanilla multi-pass.
- **`runtime.positions` populated single-pass** (P204D)
  com sealing pós-finish via `SealedPositions` (P205B/C).
- **Stdlib funcs cristalinas** usam pattern `eval_*`
  com Vm + parâmetros explícitos (não `Tracked<Context>`).

P208A produz:

1. Mapeamento empírico do gap entre vanilla
   `Tracked<Context>` e infraestrutura cristalina
   actual.
2. Cláusulas de decisão (C1–Cn) sem condicionais.
3. Plano P208B-D (sub-passos sem ramos).

P208A respeita o padrão diagnóstico-primeiro reduzido
(per reformulação humana 2026-05-12): inventário breve;
1 output em vez de 4.

---

## §2 Cláusulas de auditoria (A1–A6)

Esta secção é executada **primeiro**. Output empírico
alimenta C1+. Cada item reporta CONFIRMADO /
DIVERGÊNCIA / NÃO APLICÁVEL com evidência.

### A1 — Vanilla `here()` + `Tracked<Context>`

Localizar literalmente:

- `lab/typst-original/crates/typst-library/src/foundations/context.rs`
  (esperado).
- `Context { introspector, location, styles }`.
- `here()` impl: usa `Tracked<Context>` arg; retorna
  `context.location()?`.
- Quem passa `Tracked<Context>` aos call-sites de
  `here()`?

Output: 5-8 linhas literais.

### A2 — Stdlib cristalino actual: como funcs recebem
contexto?

Inventário literal:

- Stdlib funcs em `01_core/src/stdlib/`.
- Pattern de assinatura (`eval_*` ou similar).
- Que parâmetros recebem (Vm, args, ...)?
- Há precedente de passar `Tracked<X>` a funcs stdlib?

Output: 3-5 linhas + 1 example.

### A3 — Cristalino `Vm` ou equivalente

Inventário literal:

- Onde `Vm` (ou tipo análogo) está definido.
- Que campos tem (esperado: world, context, scopes,
  ...).
- Pode receber `Tracked<Introspector>` ou similar?

### A4 — Pipeline de call em layout: location actual
acessível?

P204D introduziu `runtime.positions` populated
single-pass. Durante o walk de layout:

- Há ponto onde Layouter chama eval ou funcs stdlib?
- Que `Location` está activa nesse ponto?
- Cristalino tem `current_location` accessor ou
  equivalente?

Output: identificar literalmente o ponto onde `here()`
faria sentido cristalinamente.

### A5 — Cristalino `Context` análogo

Existe tipo `Context` em cristalino?

- Esperado: não. Cristalino single-pass não precisa
  de Context envolvendo Introspector.
- Verificar grep `pub struct Context`.

Se inexistente: P208 precisa de criá-lo (ou usar
alternativa não-Context).

### A6 — Comparação 3 opções de design

Com base em A1–A5, comparar:

- **Opção 1 — Espelhar vanilla**: criar `Context` em
  L1, `Tracked<Context>` passado a stdlib funcs.
  Custo: L (refactor cross-modular stdlib).
- **Opção 2 — Pattern cristalino especializado**:
  `Vm` ou Layouter expõe `current_location()` directo;
  stdlib funcs receberam param explícito (não
  `Tracked<Context>`). Custo: M (extension de Vm/Layouter).
- **Opção 3 — Compute-on-demand**: `here()` consulta
  Introspector via API stateful (ex: thread-local
  current_location). Custo: S (mas anti-pattern;
  thread-local em L1 viola convenção).

Output: tabela 3 linhas + recomendação.

---

## §3 Cláusulas de decisão (C1–C5)

Fixadas **depois** da auditoria. Sem condicionais.

### C1 — Forma do contexto cristalino

Decisão com base em A6:

- **Caminho A — Espelhar vanilla `Context`** (Opção 1).
- **Caminho B — Especializado cristalino** (Opção 2).
- **Caminho C — Compute-on-demand** (Opção 3).

C1 fixa **uma**. Critério: simplicidade + reuso de
patterns cristalinos existentes + custo agregado.

Hipótese provável: Caminho B (especializado) per
divergência arquitectónica `P205A.div-1`. Vanilla
`Tracked<Context>` é específico de multi-pass; single-
pass cristalino não precisa de tracking adicional.

### C2 — `locate(selector)` design

`locate(selector) -> Location` em vanilla: dado Selector,
retorna primeira Location match.

Em cristalino: invoca `Introspector::query(selector)`
+ retorna `Vec::first().copied()`?

Decisão pode ser trivial. C2 fixa forma exacta.

### C3 — Stdlib funcs assinatura

Decisão fixada com base em C1:

- (C1 = A) funcs recebem `Tracked<Context>` explícito.
- (C1 = B) funcs recebem `&Vm` ou similar com
  `current_location` exposto.
- (C1 = C) funcs sem contexto extra; consultam
  thread-local.

C3 fixa forma das assinaturas das funcs `here` +
`locate`.

### C4 — Magnitude agregada P208

Com base em C1+C2+C3:

- Caminho A: L (~5-7h).
- Caminho B: M (~3-5h).
- Caminho C: S (~1-2h) mas com debt arquitectural.

C4 reporta. P208A não pré-fixou magnitude.

### C5 — Plano P208B-D

Sub-passos sem ramos. Quantidade depende de C1.

Hipótese (Caminho B):

- **P208B** — Vm/Layouter expõe `current_location()`
  + Context cristalino minimal (sem Tracked envolvendo).
- **P208C** — stdlib `here()` materialização.
- **P208D** — stdlib `locate(selector)` + encerramento
  série.

---

## §4 Output

1 ficheiro:
`00_nucleo/diagnosticos/typst-passo-208A-relatorio.md`.

Estrutura (~5-8 KB) com 6 §s:

- §1 O que foi auditado (sumário 3-5 linhas).
- §2 Auditoria A1-A6 (tabelas compactas).
- §3 Decisões C1-C5 fixadas.
- §4 Magnitude agregada P208 série.
- §5 Plano P208B-D.
- §6 Próximo sub-passo (P208B).

---

## §5 Não-objectivos

- Materializar `here()` (P208B/C).
- Materializar `locate()` (P208D).
- Tocar em código produção.
- Estender Selector enum (P209).
- Criar ADR-0076 nova ou transitar PROPOSTO.

---

## §6 Riscos a evitar

1. **Inflar para Caminho A por "paridade vanilla"**:
   Caminho A custa L; cristalino single-pass pode não
   beneficiar do Tracked. Per `P205A.div-1` —
   divergências arquitectónicas legítimas. C1 audita
   empíricamente.
2. **Aceitar Caminho C por "simplicidade"**: thread-local
   em L1 viola convenção. Mesmo S é caro se anti-pattern.
3. **Diagnóstico inflado**: P207A foi extenso (4 ficheiros);
   P208A é reduzido per reformulação humana. 1 ficheiro.
4. **Esquecer regra empírica P207B §5**: P208 vai adicionar
   métodos novos? Provavelmente **não** ao trait
   `Introspector` (here/locate são funcs stdlib, não
   trait methods). Mas P208 pode adicionar API nova em
   `Vm` ou Layouter — não propaga a CountingIntrospector
   directamente. Confirmar em A3+A4.
