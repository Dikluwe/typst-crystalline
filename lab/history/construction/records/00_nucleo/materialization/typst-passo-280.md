# typst-passo-280 — Auditoria walkers top-level (estabilização diagnóstica)

**Magnitude**: passo de auditoria diagnóstica S (cap LOC produção 0; cap LOC fixes oportunistas hard 80 / soft 50; cap testes hard 60 / soft 40).
**Cluster**: Estabilização diagnóstica / Cluster Gradient residual / Bug latent class.
**Origem**: relatório P279 §3 sub-padrão "Scope creep arquitectural por falha latent em walker top-level" N=2 cumulativo (P273.10 `scan_all_gradients` + P279 `scan_all_images` + `xobject_resources_for_page`); §5 hipótese explicita "walkers análogos podem existir para text fonts, glyphs, ou outras estructuras de recurso — auditoria futura"; humano confirma "estabilização diagnóstica" 2026-05-18.
**Tipo**: passo principal P280 — auditoria sistemática de **todos** os walkers top-level em `03_infra/src/export.rs` que iteram `page.items` ou `doc.pages → items`, classificando cada um por: (a) atravessa Group recursivamente; (b) não atravessa mas deveria; (c) não atravessa e não precisa.
**Sequência**: P276 (DEBT-35b OBSOLETED) → P277 (DEBT-33 CLOSED) → P278 (cleanup; 1 sub-op reformulada) → P279 (Image-em-Group narrow) → **P280 (auditoria walkers)** → P281+ (decisão humana com mapa completo de bugs latentes).
**Estratégia decidida**: estabilizar a classe de bugs antes de continuar a fechar pendências individuais. **Auditoria primeiro; fixes oportunistas só para casos triviais XS (mesma estrutura dos fixes P273.10 + P279)**.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: auditoria primeiro; código L3 apenas para fixes oportunistas dentro do cap. ADR-0029 pureza L1 preserved absoluto (passo é L3 + diagnóstico).

2. **ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-auditoria-walkers-passo-280.md` imutável. **34º consumo** (continuação P279 N=38; 33º consumo).

3. **NÃO criar ADRs novas** — auditoria identifica bugs latents existentes; não introduz arquitectura nova.

4. **Sub-padrão alvo da auditoria**: "Scope creep arquitectural por falha latent em walker top-level" — N=2 atingiu limiar interessante mas **não atingiu N=3 ainda**. Se a auditoria revelar mais bugs latents (N=3+), o sub-padrão consolida-se com material empírico abundante. Decisão sobre formalização ADR fica para o relatório §4 com base nos achados.

5. **Pattern P206A "auditoria empírica revela hipótese inválida"** disponível — se a auditoria não encontrar walkers vulneráveis adicionais, a hipótese P279 §3 é parcialmente refutada (bug latent class é mais pequena que sugerido).

6. **ADR-0094 Pattern 1 cap LOC** aplicado:
   - **Cap L3 produção hard 0 / soft 0** para auditoria pura.
   - **Cap L3 fixes oportunistas hard 80 / soft 50** — só se algum walker descoberto for fixável com pattern idêntico a `scan_all_gradients`/`scan_all_images` (helper `walk` recursivo interno). XS por walker; máximo ~3-4 walkers fixáveis dentro do cap.
   - **Decisão crítica**: fixes que requeiram refactoring estrutural ficam **out-of-scope** P280 (registar como pendências `P281.X-bis-*`).

7. **Crystalline-lint zero violations** obrigatório. L0 prompts actualizados se algum walker for fixado.

8. **Tests workspace ≥2611 preserved** (baseline P279). Esperado **2611 → 2611-2640** consoante número de fixes oportunistas (~5-10 testes de regressão por walker fixado).

9. **Sub-padrão "Auditoria sistemática de bug latent class"** — N=1 inaugural P280. Difere de:
   - "Passo administrativo de auditoria" (P125 + P275) — auditoria de DEBTs em geral.
   - "Auditoria empírica vs declaração nominal" (P275) — confronto factual vs documentação.
   - P280 audita uma **classe específica de bug** identificada por sub-padrão emergente.

10. **Caps documentais** (ADR-0094 Pattern 1):
    - Diagnóstico Fase A: hard 1000 / soft 700 (mais permissivo — auditoria sistemática gera inventário detalhado).
    - Relatório consolidado: hard 800 / soft 550.

---

## §1 — Sub-passo P280.A — Fase A diagnóstico empírico

Produz `00_nucleo/diagnosticos/diagnostico-auditoria-walkers-passo-280.md`.

### §A.1 — Inventário sistemático de walkers em `03_infra/src/export.rs`

**Pergunta central**: quais funções em `export.rs` iteram items de uma estrutura (página, frame, group) **antes** de aceder a propriedades de cada item?

```bash
# 1. Funções que iteram doc.pages
rg -n "doc\.pages|pages\.iter|for.*page.*in" 03_infra/src/export.rs

# 2. Funções que iteram page.items
rg -n "page\.items|items\.iter\(\)|for.*item.*in.*items" 03_infra/src/export.rs

# 3. Funções com nome "scan_*", "collect_*", "extract_*" (convenção walker)
rg -n "fn scan_|fn collect_|fn extract_|fn map_|fn gather_|fn find_" 03_infra/src/export.rs

# 4. Funções com nome "*_for_page", "*_resources_*"
rg -n "fn .*_for_page|fn .*_resources_|fn build_.*_resources" 03_infra/src/export.rs

# 5. Listar TODAS as fns públicas e privadas da crate em export.rs
rg -n "^fn |^pub fn |^pub\(crate\) fn |^pub\(super\) fn " 03_infra/src/export.rs
```

**Output esperado §A.1**: lista completa de funções candidatas a serem walkers. Estimativa: 10-25 funções a inspeccionar.

### §A.2 — Classificação de cada candidata

Para cada função identificada em §A.1, inspeccionar corpo:

```bash
# Para cada fn em §A.1, ver corpo + dependentes:
rg -n -A 30 "^fn NAME" 03_infra/src/export.rs
```

Tabela de classificação (output principal §A.2):

| Walker | Linha | O que itera | Atravessa Group? | Classificação |
|---|---|---|---|---|
| `scan_all_gradients` | (linha) | items para gradients | ✓ Sim (P273.10) | A — Fixado |
| `scan_all_images` | (linha) | items para Image XObjects | ✓ Sim (P279) | A — Fixado |
| `xobject_resources_for_page` | (linha) | items para resource dict | ✓ Sim (P279) | A — Fixado |
| `pattern_resources_for_page` | (linha) | items para pattern dict | (verificar) | (A/B/C) |
| `collect_codepoints` | (linha) | items.Text.text chars | (verificar) | (A/B/C) |
| `collect_glyph_ids` | (linha) | items.Glyph IDs | (verificar) | (A/B/C) |
| `collect_fonts_from_doc` | (linha) | items.Text.style.font | ✓ Sim (P146; L0 declara) | A — Já recursivo |
| `build_page_stream_type1` | (linha) | items para emit Type1 | (verificar) | (A/B/C) |
| `build_page_stream_cidfont` | (linha) | items para emit CIDFont | (verificar) | (A/B/C) |
| `build_page_stream_multifont` | (linha) | items para emit multifont | (verificar) | (A/B/C) |
| ... | ... | ... | ... | ... |

**Classificação**:
- **A**: atravessa Group recursivamente — **correcto**.
- **B**: NÃO atravessa Group mas **deveria** — **bug latent** (mesma classe P273.10/P279).
- **C**: NÃO atravessa Group e **não precisa** (e.g. emit top-level que já chama `draw_item_local` recursivo no arm Group).

### §A.3 — Análise específica dos candidatos prioritários

#### §A.3.1 — `collect_codepoints` e `collect_glyph_ids` (CIDFont path)

Estes são **candidatos críticos** porque alimentam:
- `char_to_gid` map em CIDFont single-font.
- `ToUnicode CMap`.
- `widths_array`.

Se algum não atravessa Group, então:
- **Text dentro de Group em CIDFont**: char fica fora do `char_to_gid` → render incorrecto.
- **Glyph dentro de Group em CIDFont**: glyph_id fora do CMap → ToUnicode incompleto.

Análise empírica:

```bash
rg -n -A 20 "fn collect_codepoints|fn collect_glyph_ids" 03_infra/src/export.rs
```

**Output §A.3.1**: confirmação binária por função. Se **B (bug latent)**, propor fix oportunista em §C.

#### §A.3.2 — `build_page_stream_type1`/`_cidfont`/`_multifont`

Estes são os 3 stream-builders que **chamam** `draw_item_local` no arm Group. P279 confirmou que cascadeiam `&ptr_to_idx, &img_refs` para `draw_item_local`. A questão é: o **próprio loop top-level** itera só `page.items` ou também atravessa?

Esperado: top-level itera `page.items`; arm Group chama `draw_item_local` recursivo. Comportamento **correcto** porque a recursão acontece dentro de `draw_item_local`. Classificação **C** (não precisa atravessar no top-level).

Confirmar empíricamente.

#### §A.3.3 — `pattern_resources_for_page`

P278 sub-op 2 consolidou 6 sítios incluindo `pattern_resources_for_page.walk arm Group`. O nome do método já sugere recursão (`.walk arm Group`). Confirmar empíricamente que **é** recursivo.

```bash
rg -n -A 20 "fn pattern_resources_for_page" 03_infra/src/export.rs
```

Esperado: **A — já recursivo** desde antes de P278. Confirmar.

#### §A.3.4 — Outros candidatos descobertos em §A.1

Para cada walker novo identificado, inspecção análoga.

### §A.4 — Inventário de walkers em outros ficheiros L3

Auditoria expandida — não só `export.rs`:

```bash
# Walkers em outros módulos L3
rg -n "doc\.pages|page\.items|FrameItem::Group" 03_infra/src/ | grep -v export.rs

# Walkers em L1 sobre PagedDocument?
rg -n "doc\.pages|page\.items|FrameItem::Group" 01_core/src/

# Walkers em L2/L4 (improvável mas verificar)?
rg -n "doc\.pages|page\.items|FrameItem::Group" 02_shell/src/ 04_wiring/src/
```

**Output §A.4**: lista de walkers fora `export.rs`. Esperado: poucos (L1/L2/L4 não devem ter walkers de FrameItem; PagedDocument é tipo L1 mas iteração é L3-territory).

### §A.5 — Análise estrutural: por que walkers falham?

Documentar o **mecanismo arquitectural** da classe de bug:

1. `FrameItem` é enum com variant `Group { items: Vec<FrameItem>, .. }`.
2. Iteração natural top-level via `page.items.iter()` produz `&FrameItem`.
3. Para o caso `FrameItem::Group { items: child, .. }`, o iterador top-level **não desce** para `child`.
4. Walker que precisa de visitar **todos** os items (recursivo) tem de implementar recursão explícita — typically via helper interno `fn walk(items: &[FrameItem], ...)`.
5. Walker que apenas precisa de visitar items top-level (e.g. emit que delega Group para função recursiva) **não precisa** descer — comportamento correcto.

**Output §A.5**: parágrafo conciso documentando o mecanismo. Insumo para registo no L0 e (potencialmente) ADR futura.

### §A.6 — Hipótese reformulada

P279 §3 hipotetizou: *"walkers análogos podem existir para text fonts, glyphs, ou outras estructuras de recurso"*. P280 §A.5 refina:

**Hipótese refinada**: walkers em `03_infra/src/export.rs` que (i) iteram `page.items` directamente E (ii) precisam de visitar **todos** os items (não só top-level) são vulneráveis. Classe não-vazia (P273.10 + P279); cardinalidade actual a determinar (§A.2 + §A.4).

**Predição testável**:
- Se **B = 0**: classe está estabilizada com fixes P273.10 + P279.
- Se **B ≥ 1**: cada novo B é candidato a fix oportunista P280 (XS) ou pendência P281+.

**Refutável**: confirmação de **B = 0** após §A.2 + §A.4 + §A.3 refuta a hipótese inicial (sub-padrão fica em N=2 cumulativo permanente).

### §A.7 — Gates de paragem (§política condição)

Disparam paragem antes de §C:

1. **§A.1 retorna > 30 candidatos** — escopo de auditoria explode; reformular para "auditoria por sub-categoria" (e.g. só walkers `scan_*` primeiro).
2. **§A.2 detecta > 5 walkers classe B** — fixes oportunistas excedem cap LOC 80; transitar para pendências `P281.X-bis-*` em vez de fixar.
3. **§A.3.1 confirma `collect_codepoints`/`collect_glyph_ids` classe B** — bug funcional em CIDFont. **Decisão crítica humana**: fixar agora oportunisticamente (XS) ou abrir pendência dedicada com testes E2E?
4. **§A.4 detecta walkers em outros módulos L3 fora `export.rs`** — auditoria expande para módulos não previstos. Reformular.
5. **§A.6 hipótese refutada (B = 0)** — auditoria conclui rapidamente; passo encolhe para administrativo XS (cleanup das pendências hipotéticas em P279).
6. **Cap LOC L3 fixes hard 80 ameaçado** — limitar fixes; restantes ficam pendências P281+.
7. **Cap doc Fase A hard 1000 ameaçado** — reformular para auditoria sumária (lista compacta sem corpo de cada função).
8. **Tests workspace ≠ 2611 baseline P279** — regressão pré-existente; investigar.

---

## §2 — Sub-passo P280.B — Anotação cumulativa (condicional)

**Default**: §2 não aplicado — auditoria não introduz ADR nova.

**Excepção**: se §A.2 + §A.3 confirmar N ≥ 3 walkers vulneráveis e o sub-padrão "Scope creep arquitectural por falha latent em walker top-level" atingir N ≥ 4 cumulativo (P273.10 gradient + P279 image+xobject_resources + ≥1 novo P280), pode considerar-se:

- **Opção A**: registar em §4 do relatório como sub-padrão consolidado, **sem formalizar ADR**. Per anti-padrão over-formalização P273.17.
- **Opção B**: criar ADR "ADR-XXXX: Walkers que iteram FrameItem têm de atravessar Group recursivamente" — formaliza invariante arquitectural.

**A spec recomenda Opção A** com possível upgrade a B se a auditoria revelar > 5 casos e a invariante for genuinamente fácil de violar em código futuro. Decisão fica para §4 do relatório com base na materialidade dos achados.

§2 do relatório regista: "B sub-passo aplicado em modo decisional — opção A/B fixada com base em achados §A.2".

---

## §3 — Sub-passo P280.C — Materialização (auditoria + fixes oportunistas)

### §C.1 — L0 `prompts/infra/export.md` (estabilização documental)

Adicionar nova secção:

```markdown
## Secção: Walkers top-level — invariante arquitectural (P280)

Walkers em `export.rs` que iteram items de páginas têm duas
classes de comportamento legítimo:

**Classe A — Walkers recursivos**: têm de visitar todos os items
(top-level + dentro de Groups). Implementam recursão explícita
via helper interno `fn walk(items: &[FrameItem], ...)`.

Exemplos: `scan_all_gradients` (P273.10), `scan_all_images`
(P279), `xobject_resources_for_page` (P279),
`pattern_resources_for_page` (P278), `collect_fonts_from_doc`
(P146; já em pipeline.rs, declarado recursivo em L0).

**Classe C — Walkers top-level que delegam**: iteram apenas
`page.items` porque o arm Group da própria função chama uma
sub-função recursiva (e.g. `draw_item_local` em
`build_page_stream_*`).

Anti-padrão a evitar: walker que **precisa** de visitar todos
os items mas itera apenas top-level — bug latente que só
manifesta quando feature dependente passa a renderizar dentro
de Group. Pattern instanciado em P273.10 + P279.

Auditoria sistemática realizada em P280; ver relatório para
inventário completo (data 2026-05-XX).
```

Aplicar `crystalline-lint --fix-hashes .` se necessário.

### §C.2 — Fixes oportunistas (condicional)

Para cada walker classificado como **B** em §A.2 e dentro do cap LOC 80 combinado:

#### §C.2.X — Walker `<nome>`

1. Helper interno `fn walk(items: &[FrameItem], ...)` adicionado.
2. Lógica per-item extraída para função privada (se complexa) ou inline.
3. Chamada top-level itera `page.items` mas dispatch para `walk()` que recurse em `FrameItem::Group { items: child, .. } => walk(child, ...)`.
4. Padrão **idêntico** a P273.10 (`scan_all_gradients`) e P279 (`scan_all_images`).

**Pattern**: Extract helper de replicação inline (N=3 atingido em P278; N=4 cumulativo se fixes oportunistas materializarem).

### §C.3 — Testes de regressão (condicional)

Para cada fix oportunista, 2-3 testes de regressão:
- Caso top-level (sanidade, comportamento preserved).
- Caso dentro de Group (verifica recursão activa).
- Opcional: caso Group dentro de Group (recursão profunda).

Cap testes hard 60 — confortável para até 3-4 walkers fixados.

### §C.4 — Pendências para passos futuros

Walkers classificados como **B** mas que **excedem cap LOC** ou requerem refactoring estrutural ficam como pendências:

| ID nominal | Walker | Razão |
|---|---|---|
| `P281.X-bis-<walker>-recurse-group` | (nome) | (e.g. fix requer alteração de signature; ou ≥ 50 LOC; ou tem múltiplos callers que precisam adaptação) |

### §C.5 — Actualização DEBT.md

**Não é fecho de DEBT numerado** — auditoria diagnóstica.

Cabeçalho cumulativo do DEBT.md recebe linha:

```markdown
> **Passo 280 (2026-05-XX)**: auditoria walkers top-level em
> `03_infra/src/export.rs` (estabilização diagnóstica per P279
> §3 hipótese). N walkers inventariados; M classe A (correctos),
> K classe B (bug latent), L classe C (delegam). [K fixes
> oportunistas materializados | K bugs documentados como
> pendências P281.X-bis]. Sub-padrão "Scope creep arquitectural
> por walker top-level" N=[X] cumulativo. Hipótese P279 §3
> [confirmada empíricamente | parcialmente refutada].
```

### §C.6 — Relatório consolidado

Produz `/mnt/user-data/outputs/typst-passo-280-relatorio.md`. Estrutura:

- §1 — Validação contra spec (tabela critérios §7).
- §2 — Resumo factual auditoria:
  - §2.1 — Inventário walkers (N total).
  - §2.2 — Classificação A/B/C com contagens.
  - §2.3 — Hipótese P279 confirmada/refutada.
- §3 — Operações realizadas:
  - L0 `export.md` actualizado com invariante.
  - Fixes oportunistas materializados (lista).
  - Pendências `P281.X-bis-*` registadas.
- §4 — Sub-padrões emergentes (Opção A/B fixada).
- §5 — Métricas (tabela pré/pós).
- §6 — Próximos passos: P281+ decisão humana com mapa completo.
- §7 — Referências cross-passos.
- §8 — Inventário completo (anexo, se cabe; senão referência ao diagnóstico).

---

## §4 — Caps e gates de protecção

- **LOC L3 produção (auditoria pura)**: hard 0 / soft 0.
- **LOC L3 fixes oportunistas**: hard 80 / soft 50 (combinado).
- **LOC testes**: hard 60 / soft 40 (combinado).
- **Modificações L0**: `prompts/infra/export.md` (secção invariante; hash propagado se ficheiros L1 tocados — improvável).
- **Modificações `.rs`**: `03_infra/src/export.rs` apenas para fixes oportunistas.
- **Modificações `DEBT.md`**: 1 linha cabeçalho.
- **Tests workspace**: 2611 baseline preserved + ~2-15 novos testes consoante fixes.
- **Lint**: zero violations preserved.

---

## §5 — Sub-padrões esperados aplicados

- **Auditoria sistemática de bug latent class** — N=1 inaugural P280. Aguardar reaplicação para considerar formalização.
- **Scope creep arquitectural por walker top-level** — N=2 cumulativo entrando P280; pode subir para N=3+ consoante achados.
- **Extract helper de replicação inline** — N=3 cumulativo (P273.11 + P277 + P278). Pode subir para N=4+ se fixes oportunistas materializarem o pattern `walk` recursivo.
- **Diagnóstico imutável** — N=38 → N=39 cumulativo (34º consumo).
- **Hipótese refutável documentada e testada** — N=1 inaugural P280 (formato científico: hipótese P279 §3 + predição testável + critério de refutação).
- **Pattern P206A "auditoria empírica revela hipótese inválida"** — aplicável se §A.6 refutar.

---

## §6 — Workflow operacional

1. Utilizador upload literal `00_nucleo/DEBT.md` + `00_nucleo/prompts/infra/export.md` + **`03_infra/src/export.rs` completo ou excerpts substanciais**.
2. Claude Code executa Fase A:
   - §A.1 inventário walkers (todos identificados).
   - §A.2 classificação A/B/C de cada um.
   - §A.3 análise específica candidatos prioritários (collect_codepoints, collect_glyph_ids, stream-builders, pattern_resources).
   - §A.4 expansão para outros módulos.
   - §A.5 mecanismo arquitectural documentado.
   - §A.6 hipótese confirmada/refutada.
   - Produz `typst-passo-280A-diagnostico.md` em `/mnt/user-data/outputs/`.
3. Utilizador valida Fase A. Decisão crítica: §A.7 gates não dispararam; conta de walkers classe B.
4. Claude Code executa §C:
   - L0 update §C.1.
   - Fixes oportunistas §C.2 (se algum walker classe B é XS dentro do cap).
   - Testes regressão §C.3.
   - Registo pendências §C.4 para walkers classe B fora do cap.
   - DEBT.md §C.5.
   - Relatório §C.6.
5. Utilizador valida relatório.
6. Próximo passo: P281+ decisão humana com **mapa completo de bugs latents** ou cluster estabilizado.

---

## §7 — Critério de fecho

P280 fecha quando:

- [ ] Fase A produzida; §A.1-A.6 preenchidos empíricamente.
- [ ] Inventário walkers completo (sem omissões conhecidas).
- [ ] Classificação A/B/C definitiva para cada walker.
- [ ] Hipótese P279 confirmada empíricamente ou refutada.
- [ ] L0 `export.md` actualizado com secção invariante arquitectural.
- [ ] Fixes oportunistas (se algum) implementados dentro do cap.
- [ ] Pendências `P281.X-bis-*` registadas para walkers classe B não-fixados.
- [ ] DEBT.md cabeçalho com linha P280.
- [ ] Tests workspace ≥2611 (baseline preserved + novos testes).
- [ ] Lint zero violations.
- [ ] Cap LOC L3 hard 80 respeitado.
- [ ] Relatório consolidado §1-§8 completos.

P280 NÃO fecha se:

- Auditoria incompleta (algum walker não classificado).
- Cap LOC L3 hard 80 estourado.
- Regressão tests baseline 2611.
- Fix oportunista introduz bug nova.

---

## §8 — Referências cross-passos

- **P273.10** — `scan_all_gradients` fix recursivo (N=1 do sub-padrão "Scope creep arquitectural por walker top-level").
- **P273.13** — render real Group + Shape; consume `scan_all_gradients` no contexto Group.
- **P273.17** — encerramento cluster Gradient principal.
- **P278 sub-op 2** — `pattern_resources_for_page` consolidação (helper `group_bbox_from_fields` extraído de 6 sítios; um dos sítios é o walker pattern_resources).
- **P279** — `scan_all_images` + `xobject_resources_for_page` fix recursivo (N=2); hipótese §3 registada (P280 testa).
- **L0 `infra/export.md`** — actualizado neste passo com invariante arquitectural.
- **L0 `infra/pipeline.md`** — declara explicitamente `collect_fonts_from_doc` recursivo (referência para classe A correcta).
- **ADR-0029** — Pureza física L1 (preserved absoluto; passo é L3 + auditoria).
- **ADR-0085** — Diagnóstico imutável (34º consumo).
- **ADR-0094** — Meta-operacional specs (Pattern 1 cap LOC; aplicado per categoria auditoria vs fixes).
- **P206A pattern "auditoria empírica revela hipótese inválida"** — aplicável se hipótese refutada.
- **P275** — auditoria empírica pós-cluster Gradient (precedente metodológico de "auditoria sistemática"; diferente porque P275 audita DEBTs em geral, P280 audita classe específica de bug).

---

## §9 — Notas de execução para Claude Code

- **Fase A é o coração do passo**: §A.1 + §A.2 + §A.3 são o produto principal. NÃO saltar para fixes sem auditoria completa.
- **Honestidade epistémica**: se §A.2 classifica um walker como B (bug latent) mas não consegues confirmar empíricamente (e.g. precisa de teste E2E para certeza), marca como **B?** e regista no diagnóstico que requer verificação adicional. NÃO converter B? em fix.
- **Fixes oportunistas só para casos triviais XS** — mesmo pattern de P273.10 + P279 (helper `walk` interno; extracção opcional de helper per-item). Se um walker pede refactoring estrutural, **NÃO** fixar; registar como pendência `P281.X-bis-*`.
- **Reaplicação de pattern N=3 → N=4+ cumulativo** se materializar — registar em §4 do relatório.
- **Anti-padrão a evitar**: NÃO criar ADR nova "walkers têm de atravessar Group" sem evidência empírica de ≥ 5 casos. Anti-over-formalização P273.17 preserved.
- **Confirmação visual final**: `rg "P280" 00_nucleo/DEBT.md` deve mostrar linha cumulativa; `cargo test --workspace` baseline preserved + novos verdes.
- **Outputs**: 2 ficheiros em `/mnt/user-data/outputs/` (`typst-passo-280A-diagnostico.md` + `typst-passo-280-relatorio.md`).
- **Tempo estimado**: 90-180 min (auditoria sistemática + 0-3 fixes oportunistas).
- **Inventário pode ser longo** — diagnóstico hard 1000 dá folga; se ainda excedido, transitar para auditoria sumária (lista compacta) per §A.7 gate 7.

---

*Spec produzida em 2026-05-XX como auditoria sistemática da classe de bug latent identificada por sub-padrão emergente "Scope creep arquitectural por falha latent em walker top-level" (N=2 cumulativo P273.10 + P279). Estabilização diagnóstica antes de continuar a fechar pendências individuais — produz mapa completo de bugs latents (se existirem) com classificação A/B/C. Fixes oportunistas só para casos triviais XS dentro do cap LOC; restantes ficam pendências `P281.X-bis-*`. Hipótese P279 §3 testável e refutável.*
