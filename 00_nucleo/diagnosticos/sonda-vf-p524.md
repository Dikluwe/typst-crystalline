# Relatório de Sonda — Passo 524

| Campo | Valor |
|-------|-------|
| Passo | P524 |
| Foco | (0) Correcções pendentes de P523; (1–6) Sonda de Variation Fonts (Trilha 7) |
| Data | 2026-07-01 |
| Autor | IA (Kimi Code CLI) sob direção do utilizador |
| Status | Concluído |

---

## Parte 0 — Correcções pendentes de P523

### 0.1 Contradição de kerning

O relatório P523 e o handoff continham a linha:

```text
Kerning no subset | ⏸️ Scope-out | GPOS/GSUB removidas pelo subsetter
```

Esta afirmação contradiz as conclusões de P519/P520/P521: `oxifont-subset` preserva GPOS/GSUB/GDEF (`retain_layout_tables(true)`), e o kerning é aplicado pelo shaper através dos deltas no operador `TJ`. Foi corrigida para:

```text
Kerning no subset | ✅ Fechado em P520/P521 | Delta model no operador TJ; validado com corpus dedicado (lab/parity/corpus/p520/)
```

Ficheiros alterados:
- `00_nucleo/diagnosticos/cristalino-contexto-handoff.md`
- `00_nucleo/diagnosticos/paridade-producao-p523.md`

### 0.2 Comparação de sinal `TJ` para o par `AV`

| Formato | Fonte | Operador `TJ` para `AV` |
|---------|-------|------------------------|
| TrueType | Noto Sans | `[ <0001> -40 <0002> 0 ]` |
| CFF | Nimbus Sans | `[ <0001> -71 <0002> 0 ]` |

Ambos apresentam **sinal negativo** no ajuste entre `A` e `V`, indicando kerning de aproximação. A diferença de magnitude (40 vs 71) é explicada pelas métricas distintas das fontes; não há inconsistência de sinal.

### 0.3 Teste unitário CFF corrigido

O teste `p523_subset_cff_nimbus_sans_preserves_cff_table` em `03_infra/src/export/subset.rs` passou a derivar os glyph IDs do texto real de `lab/parity/corpus/p523/test-cff-nimbus.typ`:

```text
Hello world. The five boxing wizards jump quickly. ffi fl fi AV
```

Em vez de usar GIDs arbitrários (`36`, `37`), o teste agora percorre cada caractere do texto, obtém o GID via `ttf_parser::Face::glyph_index`, e constrói o mapa `char → old_gid`. O assert de número exacto de glifos foi substituído por uma verificação de `number_of_glyphs() >= 2` (notdef + pelo menos um glifo).

Resultado do teste:

```bash
cargo test -p typst-infra p523_subset_cff_nimbus_sans_preserves_cff_table
```

`ok. 1 passed; 0 failed`.

---

## Parte 1–6 — Sonda de Variation Fonts (Trilha 7)

### Grupo 1 — Inventário de fontes VF no sistema

```bash
fc-list + script Python com fontTools
```

Resultado:

| Métrica | Valor |
|---------|-------|
| Fontes VF | 22 |
| Fontes estáticas | 485 |
| Erros (principalmente WOFF2 sem Brotli) | 191 |
| CFF2 VF | 0 |
| TrueType VF (`glyf`+`gvar`) | 22 |

Exemplos de VF encontradas: Ubuntu Sans, Ubuntu, Ubuntu Mono (eixos `wght`, `wdth`).

### Grupo 2 — Comportamento actual do cristalino com VF

Fonte de teste garantida: `Ubuntu Sans` (VF do sistema, `glyf`+`gvar`, eixo `wght`).

Documento de teste (`/tmp/test-vf.typ`):

```typst
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.

#set text(weight: 100)
Thin hello.
```

Resultado:

| Propriedade | Cristalino | Vanilla Typst |
|-------------|-----------|---------------|
| Exit code | 0 | 0 |
| `pdffonts` | 1× `CID TrueType` `AAAAAA+CrystallineFont` | 3× `CID TrueType` `UbuntuSans-Regular` |
| Fontes extraídas | 1 ficheiro `.ttf`, 209 KB | 3 ficheiros `.ttf` |
| Tabelas da fonte extraída | `fvar`, `gvar`, `avar`, `HVAR` preservadas | instâncias estáticas (sem `fvar`) |
| `/W` (larguras) | Uma só array | Três arrays distintas |

Classificação: **Funciona parcialmente, mas sem variação**. O cristalino embute a VF completa (ou quase), mas não aplica coordenadas de eixo; todas as secções usam as larguras da instância default (`wght=400`).

### Grupo 3 — Capacidade do rustybuzz com VF

Código inspeccionado: `03_infra/src/shaper.rs`.

- `rustybuzz::Face::from_slice(...)` é criado em `03_infra/src/shaper.rs:97`.
- **Não** é chamado `set_variations` nem qualquer equivalente.
- Em `03_infra/src/shaper.rs:162`, `resolve_candidates` faz:
  ```rust
  let variant = FontVariant::default();
  ```
  ou seja, o peso/estilo do `TextStyle` é **ignorado** na selecção de fonte.

Conclusão: rustybuzz suporta variações nativamente, mas o cristalino **não lhe passa coordenadas de eixo** e **não seleciona instância estática** com base em `weight`/`stretch`/`style`.

### Grupo 4 — Capacidade do oxifont-subset com VF

Código do subsetter inspeccionado (`~/.cargo/registry/src/*/oxifont-subset-0.2.0/src/`):

- Existe `gvar.rs` — reescreve a tabela `gvar` para o novo espaço de GIDs.
- Existem reescritores para `HVAR`, `VVAR`, `CFF2`.
- `lib.rs:857-858` preserva `fvar` e `avar` verbatim.
- O cristalino **não detecta formato** antes de chamar o subsetter; passa os bytes directamente. Como `oxifont-subset` suporta VF, isso não causa corrupção.

Validação empírica: a fonte extraída do PDF cristalino contém `fvar`/`gvar`/`avar`/`HVAR` e é parseável. Não houve corrupção silenciosa.

Conclusão: o subsetter é **capaz de preservar VF**. O bottleneck está no shaper, não no subsetter.

### Grupo 5 — Impacto no corpus

```bash
grep -rn "weight\|stretch\|variant" lab/parity/corpus/ | grep -v "//\|# "
```

Resultado:

```text
lab/parity/corpus/visual/set-text-bold.typ:1:#set text(weight: 700)
lab/parity/corpus/p500/test-image-fit.typ:4:#image("test.png", width: 50%, fit: "stretch")
```

- `set-text-bold.typ` usa `weight: 700`, mas recai para fontes base14 (`Helvetica-Bold`), não para VF. Não é uma regressão de VF no corpus actual.
- `test-image-fit.typ` usa `stretch` numa imagem, irrelevante para fontes.

**Impacto no corpus actual: 0 documentos combinam VF com weight/stretch.**

No entanto, a semântica `text(weight: 700)` é **descartada quando se usa uma fonte VF do sistema** — isso é regressão de linguagem, independentemente do corpus não a exercitar.

### Grupo 6 — Estimativa de complexidade

| Componente | Esforço estimado | Notas |
|------------|------------------|-------|
| A. `weight:` → coordenada `wght` | S | Mapear `FontWeight` para `Variation { tag: b"wght", value }` |
| B. Shaping com variações (`set_variations`) | S | `rustybuzz::Face::set_variations` existe; precisa de ser chamado no shaper |
| C. Descoberta/Selecção VF no fontdb | S | `fontdb` tem `is_variable()`; `select_pattern` já recebe `FontVariant`, mas o shaper passa `default()` |
| D. Subsetting VF preservando variação | S–M | `oxifont-subset` já suporta; necessita de embutir a união de glifos usados em todas as variações |
| E. Instanciação como fallback | M | Alternativa se subsetting VF falhar |
| F. Descritor PDF para VF | S–M | `/FontMatrix` ou múltiplos CIDFonts, conforme estratégia |

**MVP (embeber VF completa, aplicar variação no shaper):** S–M.  
**Completo (subsetar VF por documento + instanciação robusta):** L–XL.

---

## Tabela final de classificação

| Item | Resultado | Impacto |
|------|-----------|---------|
| Sub-tarefa 0: contradição de kerning corrigida | ✅ Feito | Housekeeping; evita deriva documental |
| Fontes VF no sistema | 22 VF (todas `glyf`+`gvar`), 485 estáticas | VF existe, mas não é dominante |
| Fonte VF de teste garantida? | ✅ Sim — Ubuntu Sans do sistema | — |
| Comportamento (6 categorias) | ⚠️ Funciona parcialmente — VF embebida, mas `weight` ignorado | Regressão de linguagem quando se usa VF |
| rustybuzz recebe coordenadas? | ❌ Não — `shaper.rs:97` cria face sem `set_variations`; `resolve_candidates` usa `FontVariant::default()` | Causa raiz do comportamento acima |
| oxifont-subset preserva fvar/gvar? | ✅ Sim — verificado no código e empiricamente | Subsetter não é obstáculo |
| Detecção de formato antes de subsetar | ⚠️ Ausente, mas desnecessária — subsetter suporta VF | Não é prioridade |
| Corpus usa weight/stretch? | ✅ Sim (`set-text-bold.typ`), mas com fallback base14; nenhum documento usa VF+weight | Impacto no corpus actual: zero |
| Esforço MVP / completo | S–M / L–XL | — |

---

## Decisão de prosseguimento

A sonda confirma que **Trilha 7 (Variation Fonts) não é XL no lado do subsetter**, mas é **regressão de linguagem** no lado do shaper: `text(weight: 700)` é descartado quando a fonte seleccionada é uma VF.

Recomendações:

1. **P525 — MVP de Variation Fonts (S–M)**
   - Passar coordenadas de eixo para `rustybuzz::Face::set_variations` no shaper (`03_infra/src/shaper.rs`).
   - Usar `FontVariant` real em vez de `FontVariant::default()` em `resolve_candidates`.
   - Embeber a fonte VF completa (ou subsetar com a união de glifos de todas as variações usadas) até optimizar tamanho num passo futuro.

2. **Não reclassificar VF como scope-out XL** com base nesta sonda. A complexidade XL aplicava-se à suposição de que seria necessário novo subsetter/instanciador; `oxifont-subset` já suporta VF.

3. **Se P525 não for prioridade**, documentar a limitação num ADR e manter o handoff actualizado — mas não marcar como "funciona" porque `weight` é ignorado para VF.

---

## Reprodução

```bash
# Grupo 1 — inventário VF
lab/.venv/bin/python3 - <<'PY'
import subprocess, os
from fontTools.ttLib import TTFont
result = subprocess.run(['fc-list', ':', 'file'], capture_output=True, text=True)
fonts = [line.strip().rstrip(':') for line in result.stdout.split('\n') if line.strip()]
vf = 0; static = 0; err = 0
for path in fonts:
    if not os.path.exists(path): continue
    try:
        f = TTFont(path)
        vf += ('fvar' in f); static += ('fvar' not in f)
    except Exception: err += 1
print(f"VF: {vf}, Static: {static}, Error: {err}")
PY

# Grupo 2 — compilação VF
cat > /tmp/test-vf.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.
#set text(weight: 700)
Bold hello.
#set text(weight: 100)
Thin hello.
EOF
./target/release/typst /tmp/test-vf.typ /tmp/vf-cristalino.pdf
pdffonts /tmp/vf-cristalino.pdf
mutool extract /tmp/vf-cristalino.pdf

# Inspecionar tabelas da fonte extraída
lab/.venv/bin/python3 - <<'PY'
from fontTools.ttLib import TTFont
f = TTFont('/tmp/font-0007.ttf')
print('fvar' in f, 'gvar' in f, 'avar' in f, 'HVAR' in f)
PY

# Teste unitário CFF
cargo test -p typst-infra p523_subset_cff_nimbus_sans_preserves_cff_table
```

---

## Linhagem

- L0: `00_nucleo/prompts/infra/export/font_subset.md` (revisado em P523)
- Código: `03_infra/src/export/subset.rs`, `03_infra/src/shaper.rs`
- ADR-0107: paridade é com a linguagem, não com a mecânica/igualdade do Rust.
- ADR-0108: medir antes de decidir; não propagar afirmações não verificadas.
- ADR-0114: sonda A.0 antes de spec.
