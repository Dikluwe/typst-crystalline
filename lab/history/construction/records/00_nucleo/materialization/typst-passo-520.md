---
# P520 — Correção de kerning no export PDF e ligatures no subsetting

> **Passo:** 520
> **Data:** 2026-07-01
> **Foco:** Corrigir dois problemas identificados em P519: (1) kerning visual ausente no operador PDF `TJ`; (2) ligatures (`fi`, `fl`) renderizam `.notdef` porque o subsetter constrói o conjunto de glifos a partir de codepoints, não dos `glyph_id` reais produzidos pelo shaper. **Nenhum fix é aplicado antes de a sonda desta secção confirmar a causa exacta.**
> **Tipo:** Sonda obrigatória + Implementação condicional.
> **Tamanho:** M (~75 min — inclui sonda que P519 não fez).
> **ADR-0108 EM VIGOR** — medir antes de decidir; a sonda de P519 não determinou o modelo de largura do CIDFont, o que é pré-condição para saber que sinal está certo.
> **ADR-0109 EM VIGOR** — atomização; os dois fixes são causalmente independentes (kerning é bug de export preexistente; ligatures é regressão de subsetting) e ficam em commits separados.
> **ADR-0114 EM VIGOR** — sonda A.0 antes de spec.
> **Dependências:** P519 (relatório de regressão), P482 (`ShapedGlyph` original — confirmar campos antes de propor novos).

---

## Contexto

P519 confirmou dois problemas de produção, com causas **distintas**:

1. **Kerning visual ausente** — bug no export de `TextShaped` em `03_infra/src/export/stream.rs:215`. Segundo P519, **não foi introduzido por P515–P518** (subsetting); é preexistente aos testes P485/P486, que validaram o comportamento actual sem o comparar contra o vanilla.
2. **Ligatures renderizam `.notdef`** — regressão **introduzida pelo subsetting** (P516): o conjunto de glifos incluído no subset é construído a partir de codepoints (`collect_codepoints` em `builder.rs`), não dos `glyph_id` reais que o rustybuzz produz após substituição GSUB.

P519 não determinou **qual modelo de largura** o CIDFont exportado usa (`/W` array com larguras reais, vs `/DW` fixo e dependência total do `TJ` para posicionamento). Essa é a informação que decide se a correcção proposta em P519/P520 original está certa ou se inverte o bug para pior — texto sobreposto em todo o documento, não só nos pares com kerning.

---

## Grupo de sondas — obrigatório antes de qualquer edição

### Sonda 1 — Modelo de largura do CIDFont exportado

```bash
mutool extract /tmp/kern-cristalino.pdf
strings font-*.pdf 2>/dev/null | grep -A2 "/DW\|/W \["
# ou inspeccionar directamente o objecto CIDFont no PDF:
python3 - <<'PY'
import re
data = open('/tmp/kern-cristalino.pdf', 'rb').read()
for m in re.finditer(rb'/DW\s+(\d+)', data):
    print("DW:", m.group(1))
for m in re.finditer(rb'/W\s*\[', data):
    print("Tem array /W explícito")
PY
```

**Pergunta a responder:** o CIDFont declara `/W` com larguras reais por glifo (correspondentes ao `hmtx` da fonte), ou usa `/DW` fixo (ex.: 0 ou 1000) e depende inteiramente dos números do `TJ` para posicionar cada glifo?

- **Se `/W` real:** o `TJ` deve conter apenas o *delta* entre a largura declarada e o `x_advance` real do shaper. A fórmula de P520 original (remover o sinal negativo do `x_advance` completo) está **errada** — vai somar a largura inteira em cima da que o `/W` já move automaticamente.
- **Se `/DW` fixo / sem `/W`:** o `TJ` tem de fornecer o avanço inteiro. Nesse caso a questão é só o sinal, e a fórmula precisa de seguir exactamente a convenção PDF (ver Sonda 2).

### Sonda 2 — Convenção de sinal do `TJ` (confirmar, não assumir)

Espec PDF §9.4.3: o número no array `TJ` é **subtraído** da coordenada horizontal corrente, em milésimos de unidade de texto. Ou seja:

- Número **positivo** → reduz o avanço (glifos ficam mais próximos).
- Número **negativo** → aumenta o avanço (glifos ficam mais afastados).

Confirmar isto contra a spec instalada ou um PDF de referência gerado pelo vanilla, **não** assumir. O exemplo do vanilla em P519 (`[(...\000\001\000\002)]TJ`) não tem números intercalados — usa `Tj` simples com larguras vindas do `/W`, o que já é uma pista de que o vanilla usa o modelo "largura real + delta", não "avanço total via TJ".

### Sonda 3 — Semântica de kerning no output do rustybuzz

Confirmar com um teste isolado (sem passar pelo export) se o GPOS Pair Adjustment do rustybuzz, para o par "AV", reduz o `x_advance` do glifo "A", desloca o `x_offset` do glifo "V", ou ambos:

```rust
#[test]
fn p520_sonda_av_kern_valores_brutos() {
    let shaped = shape_text("AV", &font, 1000.0 /* upm-normalizado */);
    eprintln!("A: advance={} offset={}", shaped[0].x_advance, shaped[0].x_offset);
    eprintln!("V: advance={} offset={}", shaped[1].x_advance, shaped[1].x_offset);
}
```

Isto elimina a ambiguidade de qual campo (`x_advance` do glifo anterior ou `x_offset` do seguinte) carrega o ajuste de kerning — necessário para saber que fórmula aplicar independentemente do modelo de `/W`.

### Sonda 4 — `collect_glyph_ids` já existe? Está ligado a quê?

P519 §11 refere `03_infra/src/export/fonts.rs:79-102` como possível já tendo uma função que recolhe `glyph_id` reais dos `ShapedGlyph`.

```bash
grep -n "fn collect_glyph_ids\|fn collect_codepoints" 03_infra/src/export/fonts.rs
grep -rn "collect_glyph_ids\b" 03_infra/src/ --include="*.rs"
```

**Se `collect_glyph_ids` já existe e já recolhe os `glyph_id` correctos**, mas o subsetter em `builder.rs` continua a usar `collect_codepoints`, o fix da Sub-tarefa 2 é trocar a chamada — não escrever uma função nova. Confirmar antes de criar `collect_shaped_glyph_ids` como propunha o P520 original (risco de duplicar lógica já existente sob outro nome).

### Sonda 5 — `ShapedGlyph.cluster` já existe?

```bash
grep -n "struct ShapedGlyph" -A 12 01_core/src/entities/*.rs
```

O histórico deste projecto (P482) já adicionou `cluster: u32` ao `ShapedGlyph` como parte da implementação original do shaper — "índice byte no string original". O P520 original assume que este campo pode não existir e propõe adicioná-lo. **Confirmar primeiro.** Se já existe, a Sub-tarefa 2 usa-o directamente; não há mudança de struct a fazer.

---

## Sub-tarefa 1 — Kerning (fórmula condicional ao resultado das sondas 1–3)

### Caso A — `/W` declara larguras reais (delta model)

```rust
// TJ = delta entre largura declarada e x_advance real, com sinal PDF correcto
// (positivo aproxima, negativo afasta — Sonda 2)
let declared_width_tu = get_declared_width(g.glyph_id, upm); // via /W ou hmtx
let real_advance_tu   = g.x_advance as f64 / upm * 1000.0;
let delta_tu           = declared_width_tu - real_advance_tu;
// delta positivo quando o avanço real é MENOR que o declarado (kerning aproxima)
// → precisa de número POSITIVO no TJ para subtrair distância (Sonda 2)
let advance_tu = delta_tu + (g.x_offset as f64 / upm * 1000.0);
```

### Caso B — `/DW` fixo / sem `/W` (avanço total via TJ)

```rust
// TJ fornece o avanço inteiro; sinal deve seguir a convenção PDF (Sonda 2):
// avançar o pen para a direita = AUMENTAR o gap = número NEGATIVO.
let advance_tu = -(g.x_advance as f64 / upm * 1000.0)
    + (g.x_offset as f64 / upm * 1000.0);
```

**Nota:** o Caso B é, formalmente, a fórmula que já está no código (P519 confirma isto é o valor actual). Se a Sonda 1 confirmar Caso B, o bug não está no sinal do `x_advance` — está algures na Sonda 3 (ex.: o rustybuzz pode não estar a aplicar GPOS Pair Adjustment de todo, e o `x_advance` de "A" já vem sem kerning nenhum, sem que o sinal tenha culpa). **Não alterar o sinal sem que a Sonda 3 confirme que o valor bruto de `x_advance`/`x_offset` já reflecte o kerning correcto.**

### 1.x — Verificação com teste empírico ampliado

Não validar só `AV`. Usar um conjunto de pares com kerning conhecido e sinal esperado em ambas as direcções:

```bash
cat > /tmp/test-kern-set.typ << 'EOF'
#set text(font: "Noto Sans", size: 48pt)
AV WA To Va
Texto normal sem pares especiais para confirmar que a mudança não introduz sobreposição.
EOF
```

Critério de aceitação:
- Pares kernados (`AV`, `WA`, `To`, `Va`) mostram gap visualmente menor que o mesmo par sem kerning.
- **O parágrafo de texto normal (sem pares especiais) não apresenta sobreposição de glifos nem espaçamento anormalmente largo** — este é o teste que protege contra o risco descrito no Caso B/Grupo de sondas: se o fix estiver errado, este é onde aparece primeiro e de forma óbvia.

### 1.y — Ajuste de testes unitários

Não assumir o sinal do assert antes da sonda. Escrever o teste **depois** de confirmado o Caso A ou B:

```rust
#[test]
fn p520_tj_advance_sign_para_kerning() {
    // Valor esperado depende do resultado da Sonda 1+2; preencher após confirmação.
    // Documentar no comentário do teste QUAL caso (A ou B) se aplica e porquê,
    // para que uma futura regressão de sinal seja detectável sem repetir a sonda.
}
```

### Critério de fecho da sub-tarefa 1

- [ ] Sonda 1 (modelo `/W` vs `/DW`) resolvida e documentada.
- [ ] Sonda 2 (convenção de sinal PDF) confirmada contra a spec, não assumida.
- [ ] Sonda 3 (semântica de kerning bruto do rustybuzz) confirmada com teste isolado.
- [ ] Fórmula aplicada corresponde ao caso (A ou B) confirmado pelas sondas.
- [ ] Teste com conjunto de pares (`AV WA To Va`) + parágrafo normal, sem sobreposição.
- [ ] Teste unitário com assert documentado (não arbitrário).
- [ ] `cargo test -p typst-wiring` e `cargo test -p typst-core` passam.

---

## Sub-tarefa 2 — Ligatures no subsetting

### 2.0 — Pré-condição: resultado das Sondas 4 e 5

- Se Sonda 4 confirmar que `collect_glyph_ids` já existe e está correcto: o fix é trocar a chamada em `builder.rs` de `collect_codepoints` para `collect_glyph_ids`. **Não** escrever `collect_shaped_glyph_ids` nova.
- Se Sonda 5 confirmar que `ShapedGlyph.cluster` já existe: usar directamente. **Não** adicionar campo novo à struct.

### 2.1 — Implementação (ajustada ao resultado das sondas)

```rust
// Só criar esta função se a Sonda 4 confirmar que não existe equivalente:
fn collect_shaped_glyph_ids(doc: &Document) -> BTreeSet<u16> {
    let mut set = BTreeSet::new();
    set.insert(0); // .notdef
    for page in &doc.pages {
        for elem in &page.elements {
            if let Element::Text(shaped) = elem {
                for g in &shaped.glyphs {
                    set.insert(g.glyph_id);
                }
            }
        }
    }
    set
}
```

Ligar ao subsetter em vez de `char_to_old_gid`.

### 2.2 — ToUnicode para ligatures (scope-out explícito, registado em DEBT.md)

Mapear o glifo de ligature para o primeiro codepoint do `cluster` (já existente, per Sonda 5) — `pdftotext` extrai texto parcialmente correcto (`"f"` em vez de `"fi"`), visual correcto.

**Registar formalmente:**

```
DEBT-XX — ToUnicode CMap para ligatures usa apenas o primeiro codepoint
do cluster, não a string completa. Extracção de texto (`pdftotext`,
accessibility) fica incompleta para documentos com ligatures. Fix
completo requer CMap multi-codepoint por glifo (S-size). Aberto em P520.
```

Adicionar esta entrada a `00_nucleo/diagnosticos/debt/DEBT.md` seguindo o formato já usado no projecto — não deixar como nota solta no relatório do passo.

### 2.3 — Verificação com teste empírico

```bash
cat > /tmp/test-liga.typ << 'EOF'
#set text(font: "Noto Sans", size: 48pt)
fi fl ffi
EOF
```

Confirmar via `fontTools` que o glifo de ligature está presente no subset (não só `.notdef`).

### Critério de fecho da sub-tarefa 2

- [ ] Sonda 4 resolvida: reutilizar `collect_glyph_ids` existente ou confirmar necessidade de nova função.
- [ ] Sonda 5 resolvida: confirmar `ShapedGlyph.cluster` existente antes de qualquer mudança de struct.
- [ ] Subsetter alimentado com `glyph_id` reais, não codepoints.
- [ ] Ligatures `fi`, `fl`, `ffi` renderizam correctamente (glifo real, não `.notdef`).
- [ ] Kerning (sub-tarefa 1) não regride com esta mudança — reexecutar teste de kerning após o fix de ligatures.
- [ ] DEBT.md actualizado com a entrada de ToUnicode parcial.
- [ ] `cargo test -p typst-wiring` e `cargo test -p typst-core` passam.

---

## Sub-tarefa 3 — Fechar a lacuna de cobertura permanente (P519 §4)

P519 confirmou que **nenhum ficheiro do corpus de paridade testa kerning ou ligatures visualmente**. Corrigir o bug sem fechar esta lacuna significa que a mesma regressão pode voltar a passar despercebida.

### 3.1 — Adicionar ficheiros permanentes ao corpus

```bash
lab/parity/corpus/p520/kerning-pairs.typ    # AV WA To Va + parágrafo normal
lab/parity/corpus/p520/ligatures-basic.typ  # fi fl ffi + parágrafo normal
```

### 3.2 — Sentinela de regressão visual (não só compilação sem erro)

A bateria actual (`for f in ...; typst "$f" out.pdf && echo OK`) só confirma que o ficheiro compila — não confirma que o kerning ou as ligatures estão correctos. Adicionar verificação estrutural mínima:

```bash
# Sentinela: extrai o content stream e confirma que o glyph_id de "fi" não é 0
python3 tools/perf/../../lab/parity/check_ligature_glyph.py /tmp/liga-check.pdf
```

Ou, se preferir manter simples nesta fase: um teste unitário `p520_ligature_nao_produz_notdef` no próprio `builder.rs`/`fonts.rs` que verifica directamente na estrutura interna (sem passar pelo PDF), o que é mais barato e mais estável do que parsing de PDF.

### 3.3 — Bateria de paridade de linguagem (sanity check, como no P520 original)

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ lab/parity/corpus/p520/*.typ; do
  ./target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1 && echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done
```

Esperado: 39/39 OK (37 anteriores + 2 novos).

### 3.4 — Benchmark (regressão de performance)

```bash
python3 tools/perf/benchmark-p507.py
```

Esperado: dentro da margem de tolerância do baseline P518. Se `collect_shaped_glyph_ids` percorrer o documento de forma adicional (em vez de reaproveitar um passo já existente), medir o impacto — pode não ser desprezável em documentos grandes.

---

## Critério de fecho do passo

- [ ] Grupo de sondas (1–5) completo, com resultado documentado antes de qualquer edição.
- [ ] Sub-tarefa 1 (kerning) — fórmula aplicada corresponde ao caso confirmado pelas sondas, não a uma hipótese.
- [ ] Sub-tarefa 2 (ligatures) — reutiliza infra existente onde as sondas confirmarem que já existe.
- [ ] Sub-tarefa 3 — corpus de paridade ganha cobertura permanente para kerning/ligatures; a lacuna identificada em P519 §4 fica fechada, não só o sintoma.
- [ ] L0 prompts actualizados (`00_nucleo/prompts/...`) e `crystalline-lint --fix-hashes .` corrido, seguindo a convenção já estabelecida no projecto para qualquer passo de implementação.
- [ ] DEBT.md tem a entrada de ToUnicode parcial de ligatures.
- [ ] Dois commits separados (ADR-0109): um para o fix de kerning, outro para o fix de ligatures — são causalmente independentes.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p520.md` distingue claramente: kerning é bug preexistente (não introduzido por P515–P518); ligatures é regressão do subsetting (P516).

---

## Próximo passo

- **Se as sondas 1–3 revelarem que o modelo de largura já estava certo (Caso B) e o bug é outro** (ex.: rustybuzz não aplica GPOS de todo, ou o `x_offset`/`x_advance` chega zerado ao export por outra razão): P520 muda de âmbito — o fix não é de sinal, é de onde a informação de kerning se perde entre o shaper e o export. Investigar a cadeia completa antes de tocar em `stream.rs`.
- **Se tudo fechar como esperado:** retomar a escolha entre as opções do handoff original (Lookahead, publicação, CFF subsetting, VF, etc.) — mas com mais confiança de que a base de produção está correcta antes de construir inovação em cima dela.
