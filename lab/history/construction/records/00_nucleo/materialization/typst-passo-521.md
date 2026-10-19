---
# P521 — Fechar DEBT-64: ToUnicode completo para ligatures

> **Passo:** 521
> **Data:** 2026-07-01
> **Foco:** Eliminar DEBT-64 — o ToUnicode CMap mapeia ligatures (`fi`, `fl`, `ffi`) apenas ao primeiro caractere do cluster. Mapear cada ligature à string completa de codepoints, incluindo texto RTL, sem regredir o mecanismo PUA introduzido em P520.
> **Tipo:** Sonda obrigatória + Implementação.
> **Tamanho:** S–M (~45 min — inclui sonda que a versão anterior deste passo não tinha e um caso RTL que a versão anterior faria entrar em panic).
> **ADR-0108 EM VIGOR.** **ADR-0109 EM VIGOR.** **ADR-0114 EM VIGOR.**
> **Dependências:** P520 (DEBT-64 aberto; mecanismo PUA para ligatures em `subset.rs`; `collect_shaped_glyph_mappings` em `fonts.rs`), P484 (RTL/`bidi_runs`, `cluster` absoluto por byte na string original).

---

## Contexto

DEBT-64 (P520): ToUnicode mapeia ligatures ao primeiro caractere do cluster apenas. Para fechar isto correctamente, dois factos do código real (não assumidos) têm de ser confirmados primeiro:

1. **P520 introduziu um mecanismo PUA** (`0xF0000+`) para recuperar o `new_gid` de glifos de ligature no subset, e **excluiu deliberadamente esses codepoints do ToUnicode CMap**. Qualquer fix de ToUnicode para ligatures tem de saber onde essa exclusão acontece e desligá-la ou substituí-la — não apenas adicionar mapeamentos novos ao lado.

2. **P484 implementou RTL** com `bidi_runs`, e o `cluster` de cada `ShapedGlyph` é o byte-index absoluto na string original (`abs_cluster = run.byte_start + info.cluster`, confirmado no relatório P484). Mas dentro de um run RTL, o rustybuzz devolve os glifos em ordem **visual**, que é ordem lógica **inversa** — os valores de `cluster` **decrescem** ao longo do vector para esse trecho. Um algoritmo de fronteira de cluster que assume `next.cluster > current.cluster` sempre vai fazer `text[start..end]` com `start > end` nesse caso, o que entra em **panic** em Rust (slice de string com índices invertidos).

O corpus RTL já existe (`lab/parity/corpus/rtl/arabic_basic.typ`, `hebrew_basic.typ`, de P488) mas nenhum ficheiro combina RTL com ligatures — isso não interessa para o bug em si: o panic acontece com **qualquer texto RTL**, com ou sem ligature, assim que houver mais de um glifo no run.

---

## Sub-tarefa 1 — Sonda: mecanismo PUA e exclusão de ToUnicode

```bash
grep -n "0xF0000\|PUA\|additional_gids\|to_unicode" 03_infra/src/export/subset.rs
grep -n "collect_shaped_glyph_mappings\|to_unicode_mappings" 03_infra/src/export/fonts.rs 03_infra/src/export/builder.rs
```

**Perguntas a responder, com `file:line`:**

- Onde exactamente o codepoint PUA é excluído do CMap? (uma condição `if cp >= 0xF0000 { skip }` ou equivalente).
- `collect_shaped_glyph_mappings` devolve `(old_gid, char)` ou já tem acesso ao `cluster`/`text` original? Se não tiver, é preciso alargar a assinatura, não criar uma função paralela.
- O `new_gid` de um glifo de ligature, depois do subset, é recuperável a partir do `old_gid` real (via `glyph_mapping`) independentemente do PUA lookup, ou o PUA é a **única** via de recuperação nesse caminho? Isto decide se o fix de ToUnicode pode reutilizar o `glyph_mapping` normal ou tem de reconstruir a ligação via PUA também.

### Critério de fecho

- [ ] Local exacto da exclusão de PUA no ToUnicode identificado com `file:line`.
- [ ] Confirmado se `collect_shaped_glyph_mappings` já expõe `cluster`/texto ou precisa de alargamento.
- [ ] Confirmado como o `new_gid` de ligature é recuperável fora do caminho PUA.

---

## Sub-tarefa 2 — Reconstrução de cluster com correcção de direcção

### 2.1 — Algoritmo corrigido (LTR e RTL)

A fronteira de um cluster **não** é simplesmente "o cluster do glifo seguinte no vector". É o **próximo valor de cluster diferente do actual**, na direcção correcta — o que exige normalizar por posição de byte, não por posição no vector.

```rust
fn cluster_text(glyphs: &[ShapedGlyph], text: &str) -> Vec<(u16, String)> {
    if glyphs.is_empty() {
        return Vec::new();
    }

    // Recolher fronteiras de cluster únicas, ordenadas por byte — independente
    // da ordem visual (LTR ou RTL) em que os glyphs aparecem no vector.
    let mut boundaries: Vec<usize> = glyphs.iter().map(|g| g.cluster as usize).collect();
    boundaries.push(text.len());
    boundaries.sort_unstable();
    boundaries.dedup();

    let mut result = Vec::with_capacity(glyphs.len());
    for g in glyphs {
        let start = g.cluster as usize;
        // Fronteira seguinte por VALOR de byte, não por posição no vector —
        // funciona igualmente para runs LTR e RTL.
        let end = boundaries.iter()
            .find(|&&b| b > start)
            .copied()
            .unwrap_or(text.len());

        // Guard: se start >= end (não deveria acontecer após a correcção,
        // mas nunca fazer slice sem verificar) — fallback para string vazia
        // em vez de panic.
        let cluster_str = if start < end && text.is_char_boundary(start) && text.is_char_boundary(end) {
            &text[start..end]
        } else {
            ""
        };

        let hex: String = cluster_str.encode_utf16()
            .map(|u| format!("{:04X}", u))
            .collect();
        result.push((g.glyph_id, hex));
    }
    result
}
```

**Diferença chave face à versão anterior:** as fronteiras são calculadas a partir do **conjunto ordenado de valores de byte**, não da posição sequencial no vector de glifos. Isto resolve LTR e RTL com o mesmo código, sem precisar de ramificar por direcção. O guard `start < end` + `is_char_boundary` evita panic mesmo que apareça um caso não previsto (defesa em profundidade, não confiar só na lógica acima estar perfeita).

### 2.2 — Mark glyphs (mesmo cluster que o base)

Quando dois ou mais glyphs partilham o mesmo `cluster` (diacrítico combinante sobre uma base), o algoritmo acima atribui a **mesma** substring a todos eles — o que duplica o texto no CMap se não for tratado. Regra:

```rust
// Depois de calcular cluster_text para todos os glyphs, marcar apenas
// a PRIMEIRA ocorrência de cada cluster com o hex completo; ocorrências
// subsequentes do mesmo cluster (marks) mapeiam para string vazia.
let mut seen_clusters = std::collections::HashSet::new();
for (gid, hex) in &mut mappings {
    // usar o cluster original do glyph correspondente para a verificação
    if !seen_clusters.insert(cluster_de(*gid)) {
        *hex = String::new();
    }
}
```

**Nota:** "primeira ocorrência" deve ser a primeira na ordem **lógica** (byte crescente), não na ordem visual do vector — para RTL isto pode não coincidir com a primeira posição no vector. Usar a lista `boundaries` já ordenada como referência de qual é o glyph "base" por cluster.

### 2.3 — Testes obrigatórios (incluindo o caso que a versão anterior não cobria)

```rust
#[test]
fn p521_cluster_text_ltr_ligature() {
    // "fi" → 1 glyph, cluster=0; hex esperado = UTF-16BE de "fi"
}

#[test]
fn p521_cluster_text_ltr_simples() {
    // "café" — 'é' é 2 bytes UTF-8, 1 codepoint; confirmar mapeamento correcto por glyph
}

#[test]
fn p521_cluster_text_mark_glyph_nao_duplica() {
    // base + combining acute com mesmo cluster; só o base leva o hex
}

#[test]
fn p521_cluster_text_rtl_nao_entra_panic() {
    // Glyphs simulando ordem visual RTL: cluster decrescente ao longo do vector
    // (ex.: glyphs[0].cluster = 4, glyphs[1].cluster = 2, glyphs[2].cluster = 0
    //  para uma string RTL de 3 clusters). Confirmar que NÃO entra em panic
    // e que o texto reconstruído corresponde à ordem lógica correcta, não à
    // ordem visual.
}

#[test]
fn p521_cluster_text_rtl_extracao_pdftotext() {
    // Teste E2E: compilar lab/parity/corpus/rtl/arabic_basic.typ,
    // extrair com pdftotext, confirmar que não há panic no compilador
    // e que a string extraída não está corrompida (mesmo que a comparação
    // exacta com o vanilla seja scope-out, per ADR-0107).
}
```

O teste `p521_cluster_text_rtl_nao_entra_panic` é o mais importante deste passo — é o que teria apanhado o bug antes de chegar a produção.

### Critério de fecho

- [ ] `cluster_text` reescrita com fronteiras por valor de byte ordenado, não por posição no vector.
- [ ] Guard contra `start >= end` presente (defesa em profundidade).
- [ ] Mark glyphs com cluster repetido não duplicam texto no CMap.
- [ ] 5 testes unitários acima passam, incluindo o caso RTL.
- [ ] `p521_cluster_text_rtl_extracao_pdftotext` corre sobre o corpus RTL já existente (P488) sem panic.

---

## Sub-tarefa 3 — Integrar no CMap builder, desligando a exclusão PUA

Com base no resultado da Sonda (Sub-tarefa 1):

- Se a exclusão de PUA está numa condição isolada (`if cp >= PUA_START { continue }`), substituir essa branch para, em vez de saltar a entrada, usar `cluster_text` para produzir o hex string completo do cluster real (não o codepoint PUA).
- Confirmar que o `new_gid` usado na entrada do CMap é o mesmo `new_gid` que o resto do pipeline associa ao glifo de ligature (via `glyph_mapping`, não via lookup do PUA) — a Sonda 1 já devia ter confirmado isto.

```rust
// Antes (P520): PUA explicitamente saltado no ToUnicode
// if cp.is_pua() { continue; }

// Depois (P521): usar cluster_text em vez de saltar
for (gid, hex) in cluster_text(&shaped.glyphs, &shaped.text) {
    if hex.is_empty() { continue; } // mark glyph — sem entrada própria
    let new_gid = glyph_mapping.get(&gid).copied().unwrap_or(0);
    to_unicode_mappings.push((new_gid, hex));
}
```

### Critério de fecho

- [ ] Exclusão de PUA no ToUnicode substituída pela lógica de `cluster_text`, não apenas complementada.
- [ ] CMap gerado contém `<new_gid> <00660069>` para `fi`.
- [ ] CMap gerado contém `<new_gid> <006600660069>` para `ffi`.
- [ ] Glifos simples continuam com mapeamento single-codepoint.
- [ ] Nenhuma entrada do CMap final aponta para um codepoint PUA (`0xF0000+`) — confirmar com grep no CMap gerado.

---

## Sub-tarefa 4 — Verificação empírica

### 4.1 — Extracção de texto LTR

```bash
cat > /tmp/test-tounicode.typ << 'EOF'
#set text(font: "Noto Sans", size: 12pt)
The five boxing wizards jump quickly. ffi fl fi
EOF
./target/release/typst /tmp/test-tounicode.typ /tmp/p521-tounicode.pdf
pdftotext /tmp/p521-tounicode.pdf -
```

Critério: saída exacta `"The five boxing wizards jump quickly. ffi fl fi"`, sem espaços estranhos nem truncagem.

### 4.2 — Extracção de texto RTL (sem panic)

```bash
./target/release/typst lab/parity/corpus/rtl/arabic_basic.typ /tmp/p521-arabic.pdf
./target/release/typst lab/parity/corpus/rtl/hebrew_basic.typ /tmp/p521-hebrew.pdf
pdftotext /tmp/p521-arabic.pdf -
pdftotext /tmp/p521-hebrew.pdf -
```

Critério: compilação e extracção **sem panic**. Comparação de exactidão do texto extraído é scope-out (ADR-0107 — paridade de linguagem, não de mecânica), mas o processo tem de correr até ao fim.

### 4.3 — Regressão de kerning e ligatures visuais (P520)

```bash
./target/release/typst lab/parity/corpus/p520/test-kerning.typ /tmp/p521-kerning.pdf
./target/release/typst lab/parity/corpus/p520/test-ligatures.typ /tmp/p521-ligatures.pdf
```

Confirmar via content stream que o kerning e as ligatures visuais de P520 não regrediram.

### 4.4 — Bateria de paridade

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ lab/parity/corpus/p520/*.typ lab/parity/corpus/rtl/*.typ; do
  ./target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1 && echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done
```

Nota: o corpus RTL foi classificado SKIP-feature em P488 (query estrutural, não render); aqui o teste é diferente — é sobre o **export** completar sem panic, não sobre paridade de query.

### Critério de fecho

- [ ] `pdftotext` extrai ligatures LTR correctamente.
- [ ] Corpus RTL compila e extrai sem panic.
- [ ] Kerning e ligatures visuais de P520 não regrediram.
- [ ] Bateria de paridade passa (incluindo os ficheiros RTL, agora como teste de "não crasha", não de match estrutural).
- [ ] `cargo test --workspace` passa.
- [ ] `crystalline-lint .` limpo.

---

## Sub-tarefa 5 — L0, DEBT-64, commits

- Actualizar `00_nucleo/prompts/infra/export/font_subset.md` (ou `tounicode.md` se existir como ficheiro próprio): documentar `cluster_text`, o algoritmo de fronteiras por byte ordenado (não por posição no vector), e a nota RTL explicitamente — é o tipo de detalhe que se perde se não ficar escrito, e foi exactamente o que faltou na primeira versão deste passo.
- Fechar DEBT-64 em `00_nucleo/diagnosticos/debt/DEBT.md`:
  ```markdown
  - ~~DEBT-64 — ToUnicode parcial para ligatures~~ → FECHADO em P521.
    Nota: a correcção original assumida (fronteira = cluster do próximo
    glyph no vector) continha um bug de panic para texto RTL, corrigido
    antes de aplicar (fronteiras calculadas por valor de byte, não por
    posição no vector).
  ```
- Dois commits: um para `cluster_text` + testes unitários (incluindo o caso RTL), outro para a integração no builder + verificação empírica. Não juntar num só — o primeiro é isolado e testável sem tocar no pipeline de export; o segundo depende dele.

---

## Critério de fecho do passo

- [ ] Sonda (Sub-tarefa 1) completa, com o mecanismo de exclusão PUA localizado antes de qualquer edição.
- [ ] `cluster_text` corrigida para funcionar em LTR e RTL sem panic, com guard defensivo.
- [ ] Teste `p521_cluster_text_rtl_nao_entra_panic` presente e verde — é o teste que teria apanhado isto antes.
- [ ] Exclusão de PUA no builder substituída pela lógica nova, não apenas complementada ao lado.
- [ ] Corpus RTL (P488) usado como teste de regressão de "não crasha" no export, não só de query estrutural.
- [ ] DEBT-64 fechado com nota sobre o bug de RTL corrigido durante a implementação deste passo (não só "fechado", registar o que quase passou despercebido).
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p521.md`.

---

## Próximo passo

- **Se tudo OK:** retomar as opções do handoff (Lookahead, publicação, CFF subsetting, VF).
- **Se o teste RTL revelar outro problema** (ex.: mark glyphs em ordem inesperada dentro de runs RTL, ou `bidi_runs` a produzir clusters não estritamente ordenáveis por byte em scripts complexos): não aplicar fix improvisado — abrir sonda dedicada antes, seguindo o mesmo padrão deste passo.
