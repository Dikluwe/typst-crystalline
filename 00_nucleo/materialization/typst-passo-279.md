# typst-passo-279 — P279.X-bis-text-image-em-group-emit — bug fix funcional 3 font scenarios

**Magnitude**: S+M com código (cap LOC L3 hard 200 / soft 150; cap testes hard 80 / soft 50).
**Cluster**: Cluster Gradient residual (pendência derivada P278) / Render real Groups / Bug fix.
**Origem**: relatório P278 §2.3 + §6 — sub-op 3 reformulada deferiu bug fix funcional para passo dedicado; pendência específica `P279.X-bis-text-image-em-group-emit` registada em P278 §C.4.
**Tipo**: passo principal P279 — bug fix funcional Text/Image (e potencialmente Glyph/Line) emitidos quando contidos dentro de Group transformado.
**Sequência**: P276 (DEBT-35b OBSOLETED) → P277 (DEBT-33 CLOSED) → P278 (cleanup combinado; sub-op 3 reformulada) → **P279 (bug fix funcional sub-op 3 deferida)**.
**Estratégia decidida**: materializar emit local de Text+Image (e tudo o que `draw_item_local` deveria cobrir) preservando bit-exact os 3 caminhos de export (Helvetica Type1 / CIDFont single-font / multifont). Cap LOC hard 200 protege contra scope creep, mas é mais permissivo que P278 (hard 150) por reconhecer a complexidade real estimada em P278 §2.3 (~100-150 LOC).

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md**: código L3 (zero alteração L1); testes-primeiro; bit-exact preservation dos 3 caminhos export para casos pré-existentes.

2. **ADR-0085** (diagnóstico imutável). Fase A produz `00_nucleo/diagnosticos/diagnostico-p279-text-image-em-group.md` imutável. **33º consumo** (continuação P278 N=37; 32º consumo).

3. **NÃO criar ADRs novas** — bug fix é correcção de comportamento existente, não decisão arquitectural. Os 3 caminhos export já têm ADRs próprias (ADR-0027 CIDFont + Identity-H; ADR-0055 single-font; ADR-0055bis decisão 5 multi-font).

4. **ADR-0029 pureza física L1** preserved absoluto — passo é puramente L3 (`03_infra/src/export.rs`). Zero alteração L1.

5. **ADR-0033 paridade funcional** — verificar em Fase A se vanilla emite Text dentro de Group transformado. Provável que sim (comportamento óbvio); confirmar.

6. **ADR-0094 Pattern 1 cap LOC** aplicado:
   - **Cap L3 hard 200 / soft 150** combinado para os 3 font scenarios + helpers + integração. Justificação: P278 §2.3 estimou "~100-150 LOC L3" para o bug fix; cap hard 200 dá margem de 30% para edge cases descobertos em Fase A.
   - **Cap testes hard 80 / soft 50** — múltiplos casos por font scenario.

7. **Crystalline-lint zero violations** obrigatório. L0 prompt `infra/export.md` actualizado se necessário (provável: secção "draw_item_local — emit local").

8. **Tests workspace 2652 → 2680-2720 esperado** (~30-60 testes novos: 4 stubs P278 substituídos por arms reais + casos exercitando cada font scenario × cada variante).

9. **Sub-padrão "Match exaustivo sem fall-through" N=2 → N=3 cumulativo cross-layer** se P279 confirmar match exaustivo P278 sub-op 3 e substituir os 4 stubs por implementação real. Limiar formalização N≥3-4 **atingido**. Decisão: registar §5 do relatório, **não formalizar ADR** per anti-padrão over-formalização P273.17.

10. **Pattern "Render real Groups" N=2** — P273.13 inaugurou render real de Shape/Group recursivo dentro de Group; P279 estende para Text/Image (e talvez Glyph/Line se Fase A revelar bug similar nesses).

11. **Caps documentais** (ADR-0094 Pattern 1):
    - Diagnóstico Fase A: hard 800 / soft 600.
    - Relatório consolidado: hard 1200 / soft 800.

---

## §1 — Sub-passo P279.A — Fase A diagnóstico empírico

Produz `00_nucleo/diagnosticos/diagnostico-p279-text-image-em-group.md`.

### §A.1 — Confirmar bug reproduzível (regressão pré-fix)

Construir documento `.typ` minimal que provoque o bug:

```typst
#rotate(45deg, [Hello dentro de Group])
```

Render PDF expected vs real. **Expected**: "Hello..." aparece rodado 45°. **Real (pré-fix)**: Group emitido com matriz `cm` mas o Text dentro é **descartado** (P278 stub no-op).

Análogo para Image:

```typst
#rotate(45deg, image("foo.jpg", width: 100pt))
```

**Verificar empiricamente** via bytes do PDF (não rendering visual):

```bash
# 1. Compilar o .typ minimal
cargo run --release -p typst -- /tmp/p279_text_in_group.typ /tmp/p279_text_in_group.pdf

# 2. Procurar string "Hello" nos bytes (caminho Helvetica) ou hex bytes (caminho CIDFont)
strings /tmp/p279_text_in_group.pdf | grep -i "hello"
# Expected (pré-fix): zero matches OU match apenas em metadata (não no stream).

# 3. Confirmar matriz cm está presente (Group emite, item interno não)
grep -c "cm" /tmp/p279_text_in_group.pdf
```

**Output §A.1**: confirmação literal que o bug existe pós-P278.

### §A.2 — Inventário factual dos 3 font scenarios

Localizar e mapear:

```bash
# 1. As 3 funções de stream
rg -n "fn build_page_stream_type1|fn build_page_stream_cidfont|fn build_page_stream_multifont" \
   03_infra/src/export.rs

# 2. Como cada uma emite Text actualmente (top-level, fora de Group)
rg -n -A 50 "fn build_page_stream_type1" 03_infra/src/export.rs | head -60
rg -n -A 50 "fn build_page_stream_cidfont" 03_infra/src/export.rs | head -60
rg -n -A 50 "fn build_page_stream_multifont" 03_infra/src/export.rs | head -60

# 3. draw_item_local actual (pós-P278) — confirmar stubs
rg -n -B 2 -A 40 "fn draw_item_local" 03_infra/src/export.rs

# 4. Parâmetros que cada stream-builder precisa
#    (font index map, char-to-glyph map, image_resources, etc.)
rg -n "image_resources|char_to_gid|font_map|cidfont|HashMap.*FontList" \
   03_infra/src/export.rs | head -30
```

**Output §A.2**: tabela:

| Font scenario | Função | Parâmetros que `draw_item_local` precisa | Como emite Text top-level |
|---|---|---|---|
| Helvetica Type1 | `build_page_stream_type1` | (a confirmar) | `BT ... /F1 ... Tj ET` Latin-1 |
| CIDFont single-font | `build_page_stream_cidfont` | char_to_gid map | `BT ... /F1 ... <hex> Tj ET` Identity-H |
| Multifont | `build_page_stream_multifont` | font_map por FontList | `/F{i+1}` select por match |

### §A.3 — Decidir parameter cascade vs structural threading

**Pergunta arquitectural central**: como passar os parâmetros necessários (char-to-gid, font_map, image_resources) a `draw_item_local` quando este é chamado de dentro do match Group?

Duas abordagens possíveis:

**Opção α — Parameter cascade**: `draw_item_local` ganha 3 versões: `draw_item_local_type1`, `draw_item_local_cidfont`, `draw_item_local_multifont`. Cada uma recebe os params específicos. O caller do Group arm sabe qual chamar (porque está dentro de um dos 3 stream-builders).

Prós: minimal mudança estrutural; cada caminho permanece self-contained.
Contras: 3 versões duplicam lógica de transform + emit; replicação parcial reaparece.

**Opção β — Struct `DrawContext`**: passar um struct que carrega os params necessários para emit local. `draw_item_local(items, ctx: &DrawContext)`. `DrawContext` enum sobre os 3 scenarios.

Prós: unified entry-point; assinatura única.
Contras: enum match dentro de `draw_item_local`; reintroduz dispatch por scenario dentro da função (custo arquitectural por unificação).

**Opção γ — Trait `LocalEmitter`**: trait com método `emit_text`, `emit_image`, etc. Implementado por 3 structs (um por scenario). `draw_item_local<E: LocalEmitter>(items, emitter)`.

Prós: dispatch estático; cada scenario tem ownership do seu emit.
Contras: introduz trait novo em L3 (pode ser excessivo para escopo); generics afectam compilação.

**A spec NÃO escolhe entre α/β/γ a priori** — a Fase A §A.3 deve produzir comparação factual e a Fase A escolhe uma. Princípio "Diagnóstico-primeiro" (ADR-0085) preserved.

**Default recomendado**: **Opção α (parameter cascade)** por minimalismo arquitectural, **a menos que** §A.2 revele que os params são triviais e poderiam caber num struct sem custo (caso β fique natural).

### §A.4 — Cobertura: que variantes de `FrameItem` precisam de fix?

P278 sub-op 3 substituiu o catch-all por 4 stubs: Text/Line/Glyph/Image. Verificar para **cada** se o bug se manifesta:

| Variante | Aparece dentro de Group em prática? | Stub actual no-op causa bug visível? |
|---|---|---|
| Text | (verificar) | (sim/não) |
| Line | (verificar) | (sim/não) |
| Glyph | (verificar) | (sim/não) |
| Image | (verificar) | (sim/não) |

**Critério de inclusão**: variante tem fix prioritário se (a) aparece em testes reais OU (b) é caminho user-facing comum (`rotate(45deg, [text])` é claramente comum).

**Output §A.4**: lista de variantes a corrigir. Provável que **Text + Image** sejam mandatórios; **Line + Glyph** opcionais consoante cap LOC.

### §A.5 — Verificar paridade vanilla

```bash
rg -n "draw_item|fn emit_item|emit.*Group" lab/typst-original/crates/typst-pdf/src/ | head -30
```

Confirmar que vanilla emite Text/Image dentro de Group transformado (provável). Se vanilla tem comportamento diferente (e.g. flattens transforms antes do emit), considerar antes de divergir.

**Output §A.5**: nota factual paridade. Diferença material requer ADR — gate §A.7.

### §A.6 — Casos de teste planeados

Por variante × scenario:

**Text (3 × ≥3 casos cada = 9-15 testes)**:
- `p279_text_em_group_helvetica` — Helvetica Type1; verifica string aparece em PDF bytes Latin-1.
- `p279_text_em_group_cidfont` — CIDFont single-font; verifica hex glyph IDs aparecem.
- `p279_text_em_group_multifont` — Multifont; verifica `/F{i+1}` selection.
- `p279_text_unicode_em_group_cidfont` — UTF-8 dentro de Group rodado.
- `p279_text_em_group_nested` — Text dentro de Group dentro de Group.
- ...

**Image (3 × ≥2 casos cada = 6-12 testes)**:
- `p279_image_em_group_jpeg` — JPEG embed dentro de Group.
- `p279_image_em_group_png` — PNG dentro de Group.
- `p279_image_em_group_dedup` — Mesma imagem em 2 Groups → `image_resources` map reutiliza XObject.
- ...

**Line + Glyph (opcionais; ~6 testes)**:
- `p279_line_em_group_emit` — Line dentro de Group rodado.
- `p279_glyph_em_group_cidfont` — Glyph (delimitador matemático) dentro de Group.
- ...

**Regressão (sanity; 5-10 testes)**:
- Testes existentes que exercitam Group + Text/Image top-level continuam a passar bit-exact.
- Tests P273.13 (render real Group + Shape) preserved.

**Estimativa total**: 30-50 testes (cabe em cap testes hard 80).

### §A.7 — Gates de paragem (§política condição)

Disparam paragem antes de §C:

1. **§A.1 não consegue reproduzir bug** — passo OBSOLETA (sub-op 3 P278 já fixou? improvável dado que P278 stubs são no-op). Verificar com humano.
2. **§A.2 revela arquitectura diferente** das 3 funções stream-builder (e.g. funções consolidadas em uma só desde algum passo intermédio) — reformular spec.
3. **§A.3 escolha α/β/γ requer ADR nova** (e.g. trait `LocalEmitter` é decisão arquitectural cross-cutting) — pausar para humano confirmar criação de ADR ou simplificar para Opção α.
4. **§A.4 revela que TODAS as 4 variantes precisam fix** — cap LOC hard 200 ameaçado. Reformular: P279 cobre só Text+Image; Line+Glyph ficam como `P280.X-bis-line-glyph-em-group`.
5. **§A.5 revela vanilla com comportamento materialmente diferente** — pausar para humano antes de divergir.
6. **Tests workspace ≠ 2652** baseline — regressão pré-existente; investigar.
7. **Cap LOC L3 hard 200 ameaçado durante implementação** — reformular ou extrair sub-cobertura.
8. **Cap doc Fase A hard 800 ameaçado** — reformular spec.

---

## §2 — Sub-passo P279.B — Anotação cumulativa (não aplicável por defeito)

**Default**: §2 não aplicado — bug fix funcional sem decisão arquitectural nova. ADRs existentes (0027/0055/0055bis decisão 5) preserved.

**Excepção**: se §A.3 escolher Opção γ (trait `LocalEmitter`) — esta é decisão arquitectural cross-cutting (introduz abstracção nova em L3 sobre os 3 caminhos export). Nesse caso, criar **ADR nova** "ADR-XXXX: Trait LocalEmitter para emit local em Group dispatched por font scenario" antes de §C. Spec reformulada.

**Probabilidade**: baixa. Opção α (parameter cascade) é mais natural per §A.3 default.

---

## §3 — Sub-passo P279.C — Materialização (testes-primeiro + código)

### §C.1 — Actualizar L0 `prompts/infra/export.md`

Adicionar secção:

```markdown
## Secção: `draw_item_local` — emit local dentro de Group (P279)

`draw_item_local` é chamada pelo arm `Content::Group` em cada um dos
3 stream-builders (type1, cidfont, multifont). A função tem 3
variantes (Opção α P279) ou recebe `DrawContext` (Opção β) ou é
genérica sobre `LocalEmitter` (Opção γ). Decisão fixada em
diagnóstico P279.

Comportamento: emite items contidos no Group (Text, Image, e
todos os outros não-Shape/Group cobertos pelo arm), preservando
o espaço local do Group (após matriz `cm`). Coordenadas dos
items são **relativas ao Group** (não à página), por isso a
inversão Y é feita usando a height local do Group, não da página.

Para Image: `image_resources` é partilhado entre top-level e
local emit — dedup XObject preserved.

Para Text: char-to-gid maps / Helvetica encoding / multifont
selection threadeadas via parameter cascade.

Histórico:
- P273.13 inaugurou render real de Shape/Group recursivo.
- P278 sub-op 3 reformulada: catch-all `_ => {}` substituído por
  4 stubs documentados (Text/Line/Glyph/Image).
- P279 substitui stubs por implementação funcional real para
  Text + Image (e potencialmente Line/Glyph consoante §A.4).
```

Aplicar `crystalline-lint --fix-hashes .` para propagar hash.

### §C.2 — Testes-primeiro

Adicionar testes da §A.6 ao módulo de testes `03_infra` (ou `tests/integration_*.rs` conforme convenção identificada em Fase A).

Confirmar que testes **falham** com o código pós-P278 (stubs no-op) para os casos `p279_text_em_group_*` e `p279_image_em_group_*` — esta falha é a evidência factual do bug que P279 corrige.

### §C.3 — Implementação dos arms reais

Conforme Opção escolhida em §A.3.

**Para Opção α (parameter cascade — default)**:

```rust
// Em build_page_stream_type1:
fn draw_item_local_type1(items: &[FrameItem], group_height: f64, ...) -> String { ... }

// Em build_page_stream_cidfont:
fn draw_item_local_cidfont(
    items: &[FrameItem],
    group_height: f64,
    char_to_gid: &HashMap<char, u16>,
    ...
) -> String { ... }

// Em build_page_stream_multifont:
fn draw_item_local_multifont(
    items: &[FrameItem],
    group_height: f64,
    font_map: &HashMap<FontList, usize>,
    image_resources: &HashMap<usize, ImageResource>,
    ...
) -> String { ... }
```

Cada uma:
1. Itera `items`.
2. Match sobre `FrameItem` — arms para Text/Image (e Line/Glyph se §A.4).
3. Emite operadores PDF apropriados ao scenario, com coordenadas locais.
4. Group nested recursão: chama-se a si própria.
5. Shape: chama a função existente (preserved per P273.13).

Para Image: invocar a mesma lógica de XObject lookup que top-level usa, com `image_resources` como referência partilhada.

### §C.4 — Eliminar/substituir stubs P278

Os 4 stubs documentados criados em P278 sub-op 3 são substituídos por arms reais (para Text+Image obrigatório; Line+Glyph se §A.4 incluir). Match exaustivo preservado.

Se §A.4 deixar Line/Glyph como pendência futura (`P280.X-bis-line-glyph-em-group`), os 2 stubs correspondentes permanecem com comment apontando o passo futuro.

### §C.5 — Actualização DEBT.md

**Não é fecho de DEBT numerado** — é fecho de pendência específica `P279.X-bis-text-image-em-group-emit` registada em P278.

Cabeçalho cumulativo do DEBT.md recebe linha:

```markdown
> **Passo 279 (2026-05-XX)**: bug fix funcional `text-image em
> Group emit` (pendência específica P279.X-bis registada P278
> sub-op 3 reformulada). 3 font scenarios cobertos
> (Helvetica/CIDFont/multifont) via [opção escolhida em §A.3].
> Match exaustivo `draw_item_local` substitui [4|2] stubs
> documentados por arms reais. Tests: 2652 → ~2680-2720.
> Sub-padrão "Render real Groups" N=2 cumulativo (P273.13 +
> P279).
```

### §C.6 — Relatório consolidado

Produz `/mnt/user-data/outputs/typst-passo-279-relatorio.md`. Estrutura:

- §1 — Validação contra spec (tabela critérios §7).
- §2 — Resumo factual:
  - §2.1 — Bug reproduzido empíricamente §A.1.
  - §2.2 — Opção escolhida §A.3 (α/β/γ).
  - §2.3 — Variantes cobertas §A.4.
  - §2.4 — Paridade vanilla §A.5.
- §3 — Operações realizadas:
  - L0 `export.md` actualizado.
  - Testes adicionados (lista por scenario).
  - Funções `draw_item_local_*` adicionadas.
  - Stubs P278 substituídos.
- §4 — Sub-padrões emergentes (sem formalização ADR):
  - "Match exaustivo sem fall-through" N=3 cumulativo (se aplicável).
  - "Render real Groups" N=2 cumulativo.
  - "Pendência específica derivada-fecha-derivada" N=1 inaugural (P278 abriu, P279 fecha).
- §5 — Métricas (tabela pré/pós).
- §6 — Próximo passo: P280+ decisão humana.
- §7 — Referências cross-passos.

---

## §4 — Caps e gates de protecção

- **LOC L3**: hard 200 / soft 150.
- **LOC testes**: hard 80 / soft 50.
- **Modificações `.rs`**: `03_infra/src/export.rs`. Sem outras alterações L1/L2/L4.
- **Modificações L0**: `prompts/infra/export.md` (secção `draw_item_local` nova; hash propagado).
- **Modificações `DEBT.md`**: 1 linha cabeçalho.
- **Tests workspace**: 2652 → 2680-2720 esperado.
- **Lint**: zero violations preserved.

---

## §5 — Sub-padrões esperados aplicados

- **Match exaustivo sem fall-through** — N=2 (P278 sub-op 3 com stubs) → **N=3 cumulativo** se P279 mantiver match exaustivo e substituir stubs por arms reais. Limiar formalização N≥3-4 atingido; **decisão**: registar §5 do relatório, NÃO formalizar ADR.
- **Render real Groups** — N=2 cumulativo (P273.13 inaugural + P279 estende). Aguardar reaplicação para considerar formalização.
- **Pendência específica derivada-fecha-derivada** — N=1 inaugural. P278 abriu `P279.X-bis-...`; P279 fecha. Diferente de "fecho DEBT numerado" — é fecho de pendência operacional.
- **Diagnóstico imutável** — N=37 → N=38 cumulativo (33º consumo).
- **Reformulação de sub-op por cap LOC (P278) → passo dedicado (P279)** — primeira consequência operacional do pattern. Confirma que reformulação não é perda de trabalho — é deferimento honesto.

---

## §6 — Workflow operacional

1. Utilizador upload literal `00_nucleo/DEBT.md` + `00_nucleo/prompts/infra/export.md` + excerpts relevantes de `03_infra/src/export.rs` (especialmente as 3 funções stream-builder + `draw_item_local`).
2. Claude Code executa Fase A:
   - Produz `typst-passo-279A-diagnostico.md` em `/mnt/user-data/outputs/`.
   - §A.1 reprodução bug.
   - §A.2 inventário 3 stream-builders.
   - §A.3 escolha α/β/γ com comparação factual.
   - §A.4 lista variantes a cobrir.
   - §A.5 paridade vanilla.
3. Utilizador valida Fase A. Decisão crítica: §A.3 opção + §A.4 escopo.
4. Claude Code executa §C:
   - L0 update (§C.1) + `--fix-hashes`.
   - Testes-primeiro (§C.2) — confirmar falha pré-fix.
   - Implementação (§C.3).
   - Substituir stubs P278 (§C.4).
   - Confirmar tests workspace passam.
   - Edita DEBT.md cabeçalho (§C.5).
   - Re-corre `crystalline-lint` zero violations.
   - Produz relatório (§C.6).
5. Utilizador valida relatório.
6. Próximo passo: P280+ decisão humana (cluster Gradient totalmente encerrado se P279 cobrir Line+Glyph; caso contrário `P280.X-bis-line-glyph-em-group`).

---

## §7 — Critério de fecho

P279 fecha quando:

- [ ] Fase A produzida; §A.1-A.5 preenchidos empíricamente.
- [ ] §A.1 confirma bug reproduzível com fix antes do código.
- [ ] §A.3 opção α/β/γ escolhida com justificação.
- [ ] §A.4 lista variantes a cobrir confirmada.
- [ ] L0 `export.md` actualizado; hash propagado.
- [ ] Testes pré-fix confirmam falha; pós-fix passam.
- [ ] Funções `draw_item_local_*` (ou equivalente per opção) implementadas.
- [ ] Stubs P278 sub-op 3 substituídos (todos ou apenas Text+Image consoante §A.4).
- [ ] DEBT.md cabeçalho com linha P279.
- [ ] Tests workspace ≥2680 (mínimo +28 com Text+Image; ~+50 se incluir Line+Glyph).
- [ ] Lint zero violations.
- [ ] Cap LOC L3 hard 200 respeitado.
- [ ] Match exaustivo preserved (sem reintroduzir catch-all silencioso).
- [ ] Relatório consolidado §1-§7 completos.

P279 NÃO fecha se:

- §A.1 não reproduz bug (passo OBSOLETA — verificar com humano).
- §A.3 escolher Opção γ sem ADR nova justificada.
- Cap LOC L3 hard 200 estourado.
- Regressão tests baseline 2652.
- Algum teste novo falha após implementação.
- Bug nova introduzida (e.g. duplicação de XObject em Image em Group).
- Paridade vanilla materialmente quebrada sem ADR-0054 graded.

---

## §8 — Referências cross-passos

- **P273.13** — Render real Group + Shape (N=1 do pattern "Render real Groups").
- **P278 §2.3** — origem específica P279 (sub-op 3 reformulada; ~100-150 LOC estimado).
- **P278 §4.4** — sub-padrão "Reformulação de sub-op por cap LOC" N=1 inaugural; P279 é primeira consequência operacional.
- **P278 §C.4** — registo da pendência `P279.X-bis-text-image-em-group-emit`.
- **ADR-0027** — CIDFont + Identity-H (1 dos 3 font scenarios).
- **ADR-0055** — Font consumer single-font (1 dos 3 font scenarios).
- **ADR-0055** decisão 5 (P146) — multifont (1 dos 3 font scenarios).
- **ADR-0029** — Pureza física L1 (preserved absoluto; passo é L3).
- **ADR-0033** — Paridade vanilla (verificar §A.5).
- **ADR-0054** — Critério fecho graded (permite divergência justificada).
- **ADR-0085** — Diagnóstico imutável (33º consumo).
- **ADR-0094** — Meta-operacional specs (Pattern 1 cap LOC).
- **L0 `export.md`** — secção a expandir.
- **L0 `pipeline.md`** — referência ao dispatch font-aware.

---

## §9 — Notas de execução para Claude Code

- **Fase A é crítica**: §A.3 escolha α/β/γ define a arquitectura. NÃO escolher sem comparação factual; default α só se §A.3 confirmar minimalismo.
- **Testes-primeiro obrigatório (§C.2)**: os testes `p279_text_em_group_*` e `p279_image_em_group_*` devem **falhar** com o código pós-P278 (stubs no-op). Esta falha é a evidência factual do bug. Depois passam com implementação.
- **NÃO duplicar XObject** em Image em Group — `image_resources` map é partilhado entre top-level e local emit.
- **Coordenadas locais ≠ coordenadas página**: dentro de Group, items têm `pos` relativo ao Group, não à página. Inversão Y usa height do Group.
- **Match exaustivo preserved**: NÃO reintroduzir `_ => {}` catch-all silencioso. Se Line/Glyph não forem cobertos por P279 (per §A.4), manter como stubs documentados apontando `P280.X-bis-...`.
- **3 font scenarios bit-exact preserved para casos top-level**: testes existentes que exercitam Text top-level (fora de Group) **devem continuar a passar bit-exact**. Qualquer mudança em bytes indica regressão.
- **Anti-padrão a evitar**: NÃO unificar os 3 stream-builders num só por causa de P279. Decisão arquitectural fora escopo; cada um tem rationale histórico.
- **L0 antes do código**: `export.md` actualizado (§C.1) antes de tocar `.rs` (§C.3). Protocolo Nucleação CLAUDE.md.
- **Outputs**: 2 ficheiros em `/mnt/user-data/outputs/` (`typst-passo-279A-diagnostico.md` + `typst-passo-279-relatorio.md`).
- **Tempo estimado**: 120-180 min (passo S+M; 3 scenarios + múltiplas variantes).
- **Confirmação visual final**: render `rotate(45deg, [hello])` produz PDF com "hello" visível rodado.

---

*Spec produzida em 2026-05-XX como fecho da pendência específica `P279.X-bis-text-image-em-group-emit` registada em P278 sub-op 3 reformulada. Bug fix funcional Text+Image (e potencialmente Line+Glyph) dentro de Group transformado, cobrindo os 3 font scenarios (Helvetica Type1 / CIDFont single-font / multifont). Disciplina anti-scope-creep P278 preservada: cap LOC L3 hard 200 com margem de 30% sobre estimativa. Sub-padrão "Pendência específica derivada-fecha-derivada" N=1 inaugural — primeira consequência operacional do pattern "Reformulação de sub-op por cap LOC" P278.*
