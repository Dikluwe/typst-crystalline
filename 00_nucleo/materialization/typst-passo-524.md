---
# P524 — Correções pendentes de P523 + Trilha 7: Sonda de Variation Fonts (VF)

> **Passo:** 524
> **Data:** 2026-07-01
> **Foco:** (0) Corrigir a contradição interna do relatório P523 e do handoff quanto ao mecanismo de kerning, que reintroduz a alegação já refutada por P519 ("GPOS/GSUB removidas"); fechar as duas verificações que ficaram por fazer em P523. (1–6) Diagnóstico empírico do estado actual de Variation Fonts no cristalino — zero código de produção nesta parte, apenas medição e decisão.
> **Tipo:** Correcção de documentação + Diagnóstico.
> **Tamanho:** M (~55 min — inclui a Sub-tarefa 0, que a versão anterior deste passo não tinha).
> **ADR-0108 EM VIGOR** — medir antes de decidir; **não copiar afirmações de relatórios anteriores para novos documentos sem verificar se ainda são válidas** — é exactamente isto que a Sub-tarefa 0 corrige.
> **ADR-0114 EM VIGOR** — sonda A.0 antes de spec.
> **ADR-0107 EM VIGOR** — paridade é de linguagem, não de mecânica.
> **ADR-0109 EM VIGOR** — atomização; Sub-tarefa 0 é independente do resto do passo e pode ser aplicada isoladamente.
> **Dependências:** P519 (refutação original), P520/P521 (kerning e ToUnicode validados), P522/P523 (CFF fechado), P515–P517 (fontdb, subsetting).
> **Trilha:** 7 — Variation Fonts (XL declarado no handoff — a confirmar ou refutar, como aconteceu com CFF em P522).

---

## Sub-tarefa 0 — Corrigir a contradição de kerning que reapareceu em P523 e no handoff

### 0.1 — O problema

O relatório de P523 contém, no mesmo documento, duas afirmações incompatíveis:

- Secção "Resultados de medição": `Kerning AV | Aplicado (TJ com valor negativo)`.
- Secção "Pendências e scope-outs": `Kerning no subset | ⏸️ Scope-out | GPOS/GSUB removidas pelo subsetter; posicionamento aplicado previamente pelo shaper`.

A segunda frase é a hipótese original do handoff que **P519 já investigou e refutou empiricamente**: `oxifont-subset` preserva GPOS/GSUB/GDEF (`retain_layout_tables(true)`, confirmado em P519 §3), o shaping acontece antes do subsetting (P519 §2), e o kerning funciona através dos deltas que o shaper já calculou, aplicados no operador `TJ` (P520/P521). Não há scope-out de kerning — está fechado e validado desde P521.

A mesma frase persiste no handoff actualizado, secção 5.1:
```
Kerning no subset | ⏸️ Scope-out | GPOS/GSUB removidas
```

Se esta linha não for corrigida agora, qualquer sessão futura que leia o handoff (que é precisamente o seu propósito) volta a herdar a alegação errada, podendo repetir sondas já feitas — o mesmo ciclo que já aconteceu uma vez com CFF (marcado XL no handoff, refutado em P522).

### 0.2 — Fix

```bash
grep -rln "GPOS/GSUB removidas\|Kerning no subset.*[Ss]cope-out" \
  00_nucleo/ --include="*.md"
```

Para cada ocorrência encontrada (esperado: handoff §5.1, relatório P523, e possivelmente `DEBT.md` se houver entrada relacionada), substituir por:

```
Kerning no subset | ✅ Fechado em P520/P521 | Delta model no operador TJ; validado com corpus dedicado (lab/parity/corpus/p520/)
```

### 0.3 — Duas verificações que P523 não fez, pendentes desde a revisão do passo anterior

A revisão de P523 pediu explicitamente duas coisas que o relatório final não confirma ter feito:

**a) Comparar o sinal do `TJ` de `AV` em CFF (Nimbus Sans) contra o mesmo par já validado em TrueType (Noto Sans, P520/P521).** O relatório de P523 reporta o valor CFF isoladamente ("TJ com valor negativo") sem cruzar com a referência. Fazer agora:

```bash
./target/release/typst lab/parity/corpus/p520/test-kerning.typ /tmp/av-truetype-ref.pdf
mutool show /tmp/av-truetype-ref.pdf 4 | grep TJ
./target/release/typst lab/parity/corpus/p523/test-cff-nimbus.typ /tmp/av-cff.pdf
mutool show /tmp/av-cff.pdf 4 | grep TJ
```

Confirmar mesmo sinal e ordem de grandeza compatível entre os dois. Registar o resultado — se os sinais divergirem entre TrueType e CFF para o mesmo par de letras, é uma inconsistência a investigar antes de fechar Trilha 6 definitivamente.

**b) O teste unitário `p523_subset_cff_nimbus_sans_preserves_cff_table` usa glyph IDs arbitrários (`&[0, 1, 2, 3]`, comentados como "notdef + A + B") que não correspondem a texto real do corpus.** `test-cff-nimbus.typ` nunca isola um "B" sozinho. Corrigir para derivar os glyph IDs do texto real do documento de teste:

```rust
#[test]
fn p523_subset_cff_nimbus_sans_preserves_cff_table() {
    // Glyph IDs derivados do texto real de test-cff-nimbus.typ
    // ("Hello world...", "ffi fl fi AV"), não de um conjunto arbitrário.
    let font_data = /* fixture, como já corrigido em P523 */;
    let text = "Hello world. The five boxing wizards jump quickly. ffi fl fi AV";
    let gids = glyph_ids_for_text(&font_data, text); // reutilizar helper já existente do shaper, se houver
    let result = subset_font_with_mapping(&font_data, &gids).unwrap();
    let font = font::parse(&result.font_data).unwrap();
    assert!(font.has_table(b"CFF "));
}
```

Se não existir um helper `glyph_ids_for_text` reutilizável, é aceitável manter um conjunto pequeno mas **documentado como correspondente a caracteres específicos usados no teste** (ex.: `&[0, gid_de('H'), gid_de('e')]`), não um array sem explicação da origem.

### Critério de fecho da Sub-tarefa 0

- [ ] Todas as ocorrências de "GPOS/GSUB removidas" ou "kerning scope-out" corrigidas (handoff, relatório P523, DEBT.md se aplicável).
- [ ] Comparação de sinal `TJ` entre `AV` em TrueType (referência) e `AV` em CFF (Nimbus Sans) feita e registada.
- [ ] Teste unitário de P523 corrigido para usar glyph IDs derivados do texto real, não arbitrários — ou, no mínimo, documentados.
- [ ] `cargo test --workspace` passa.

---

## Sub-tarefa 1–6 — Sonda de Variation Fonts (Trilha 7)

### Contexto

O handoff declara VF como scope-out XL. Antes de aceitar isso, aplicar o mesmo tratamento que refutou a alegação equivalente sobre CFF em P522: medir, não assumir.

---

### Grupo 1 — Inventário de fontes VF no sistema

```bash
python3 - <<'PY'
import subprocess, os
from fontTools.ttLib import TTFont

result = subprocess.run(['fc-list', ':', 'file'], capture_output=True, text=True)
fonts = [line.strip().rstrip(':') for line in result.stdout.split('\n') if line.strip()]

vf_count = 0
static_count = 0
error_count = 0
vf_fonts = []

for path in fonts[:500]:
    if not os.path.exists(path):
        continue
    try:
        font = TTFont(path)
        if 'fvar' in font:
            vf_count += 1
            axes = [f"{axis.axisTag}={axis.defaultValue}" for axis in font['fvar'].axes]
            vf_fonts.append((path, axes))
        else:
            static_count += 1
    except Exception:
        error_count += 1

print(f"VF: {vf_count}, Static: {static_count}, Error: {error_count}")
if vf_fonts:
    print("Exemplos VF:")
    for path, axes in vf_fonts[:10]:
        print(f"  {path}")
        print(f"    Eixos: {', '.join(axes)}")
PY
```

**Nota:** sintaxe corrigida face à versão anterior deste passo — `split('\n')` numa linha, sem quebra literal dentro da string (o mesmo bug corrigido em P522).

### Critério de fecho

- [ ] Contagem VF vs estáticas no sistema.
- [ ] Lista das 5–10 fontes VF mais comuns (se houver).
- [ ] Classificação por formato: TrueType VF (`glyf`+`gvar`) vs CFF2 VF.

---

### Grupo 2 — Comportamento actual do cristalino com VF

### 2.0 — Fonte VF garantida, sem depender do sistema

Aplicar o mesmo padrão já estabelecido em P522/P523 para CFF: **não depender de o sistema ter uma fonte VF instalada.** Se o Grupo 1 não encontrar nenhuma, obter uma fonte VF open-source conhecida (ex.: Inter, licença OFL, tem variante VF) e adicioná-la como fixture:

```bash
find / -iname "*.ttf" -o -iname "*.otf" 2>/dev/null | xargs -I{} sh -c \
  'python3 -c "from fontTools.ttLib import TTFont; import sys; f=TTFont(sys.argv[1]); sys.exit(0 if \"fvar\" in f else 1)" {} 2>/dev/null && echo {}'
```

Se nenhuma disponível localmente e houver acesso de rede, obter e documentar a licença, seguindo o mesmo procedimento de `03_infra/fixtures/fonts/` estabelecido em P523. Se não houver acesso de rede: **registar a limitação explicitamente**, não pular o grupo em silêncio — a mesma regra aplicada em P522.

### 2.1 — Documento de teste

```typst
#set text(font: "NOME_DA_FONTE_VF_CONFIRMADA", size: 12pt)
Hello world.

#set text(weight: 700)
Bold hello.

#set text(weight: 100)
Thin hello.
```

### 2.2 — Verificar comportamento e exit code

```bash
./target/release/typst /tmp/test-vf.typ /tmp/vf-cristalino.pdf
echo "Exit code: $?"
```

### 2.3 — Tabela de comportamentos possíveis (com a categoria de corrupção que P522 identificou como necessária)

| Comportamento | Classificação | Observação |
|---------------|---------------|------------|
| Compila, fonte VF embebida completa, texto renderiza com peso default | ⚠️ Funciona parcialmente | Sem variação; `weight: 700` ignorado |
| Compila, fonte VF embebida, texto renderiza com peso variado | ✅ Funciona | rustybuzz + oxifont-subset já suportam |
| Compila, mas fonte substituída por fallback estático | ⚠️ Regressão silenciosa | fontdb escolheu instância estática |
| Panic / erro (exit code ≠ 0) | ❌ Bug prioritário | Deve ser zero |
| `.notdef` para todos os glifos | ❌ Bug prioritário | — |
| **Subsetter trata VF (CFF2/gvar) como formato estático — fonte corrompida embebida** | ❌ **Bug prioritário, mais grave que os anteriores** | Mesma classe de risco identificada em P522 para CFF simples; aqui aplica-se em dobro (CFF2 tem a mesma exposição que CFF, mais a tabela `gvar`/`avar` que TrueType estático não tem). Validar a fonte extraída com `fontTools`, não só confirmar que o PDF abre. |

### 2.4 — Verificar se o peso variado é aplicado

1. Visual: comparar com vanilla para o mesmo documento — pesos 100/400/700 devem ser visivelmente diferentes.
2. `pdffonts` confirma a fonte VF (não fallback).
3. `mutool extract` + `fontTools`: a fonte extraída ainda contém `fvar`/`gvar`/`CFF2`, ou foram removidas?

### 2.5 — Comparação com vanilla

```bash
./lab/typst-original/target/release/typst compile /tmp/test-vf.typ /tmp/vf-vanilla.pdf
```

### Critério de fecho

- [ ] Fonte VF de teste garantida (local, bundled, ou limitação registada).
- [ ] Exit code registado.
- [ ] Fonte extraída validada estruturalmente (não só "o PDF abre").
- [ ] Comportamento classificado na tabela de 6 categorias.
- [ ] Comparação com vanilla.

---

### Grupo 3 — Capacidade do rustybuzz com VF

```bash
grep -rn "fvar\|variation\|set_variations\|axis\|wght\|weight" 01_core/src/infra/shaper/ 03_infra/src/ --include="*.rs" | head -30
```

Perguntas:
- O `FontFace`/`FontRef` do cristalino expõe eixos de variação?
- O shaper chama `rustybuzz::Face::set_variations` ou equivalente?
- `text(weight: 700)` traduz para coordenadas de eixo, ou para selecção de instância estática?

```bash
grep -rn "fvar\|variable\|axis\|instance" ~/.cargo/registry/src/*/fontdb-*/src/ 2>/dev/null | head -20
```

### Critério de fecho

- [ ] rustybuzz recebe coordenadas de eixo? Sim/Não + `file:line`.
- [ ] fontdb expõe eixos ou instâncias? Sim/Não + `file:line`.
- [ ] Tradução `weight: 700` → eixo ou instância confirmada no código.

---

### Grupo 4 — Capacidade do oxifont-subset com VF

```bash
grep -rn "fvar\|gvar\|avar\|HVAR\|MVAR\|CFF2\|variable" ~/.cargo/registry/src/*/oxifont-subset-*/src/ 2>/dev/null | head -30
```

Perguntas:
- Preserva `fvar`/`gvar`/`CFF2`?
- Se subsetar uma VF, o resultado é VF funcional ou fonte estática (eixos removidos)?
- **O código do cristalino verifica o formato antes de chamar o subsetter, ou passa cegamente como fez para CFF antes de P522/P523?** (mesma pergunta do Grupo 3 de P522, aplicada aqui — decide se o caso de corrupção do Grupo 2.3 é sequer possível).

### Alternativas, se não suportado

| Opção | Descrição | Complexidade |
|-------|-----------|--------------|
| A | `fontTools.varLib.instancer` via subprocess — instancia para peso específico | M — quebra pureza Rust |
| B | `hb-subset` via FFI | M/L |
| C | Instanciação manual | XL |
| D | Embeber VF completa sem subsetting | XS — PDFs grandes |
| E | Scope-out permanente, **com detecção de formato obrigatória** para nunca corromper (mesma lógica da Opção E de P522) | S |

### Critério de fecho

- [ ] `oxifont-subset`: preserva `fvar`/`gvar`/`CFF2`? Sim/Não + evidência.
- [ ] Detecção de formato antes de subsetar: confirmada ou ausente.
- [ ] Tabela de alternativas preenchida se necessário.

---

### Grupo 5 — Impacto no corpus e em uso típico

```bash
grep -rn "weight\|stretch\|variant" lab/parity/corpus/ 2>/dev/null | grep -v "//\|# "
```

`weight: 700` é uso comum em Typst real (headings, `strong`). Se o cristalino ignora, isso é **regressão de linguagem**, não só mecânica — a semântica do argumento é descartada, o que é diferente do caso CFF (onde a fonte funcionava, só o descritor PDF era "tecnicamente" diferente).

### Critério de fecho

- [ ] Número de documentos do corpus que usam `weight`/`stretch`/`variant`.
- [ ] Se > 0: confirmado como regressão de linguagem, prioridade sobe.

---

### Grupo 6 — Estimativa de complexidade

| Componente | Esforço estimado |
|------------|-------------------|
| A. Coordenadas de eixo (`weight:` → `wght=`) | S |
| B. Shaping com variações (`set_variations`) | S — se rustybuzz já suporta |
| C. Descoberta VF no fontdb | XS–S |
| D. Subsetting VF preservando variação | M–XL — depende do oxifont-subset |
| E. Instanciação como fallback | M |
| F. Descritor PDF para VF | S–M |

**MVP (sem optimizar tamanho):** A+B (S) + embeber completo (XS) + instanciação se necessário (M) = **S–M**.
**Completo:** L–XL, dependente do Grupo 4.

---

## Tabela final de classificação

| Item | Resultado | Impacto |
|------|-----------|---------|
| Sub-tarefa 0: contradição de kerning corrigida | _a preencher_ | Housekeeping, independente de VF |
| Fontes VF no sistema | _a preencher_ | — |
| Fonte VF de teste garantida? | _a preencher_ | Se não, marcar limitação. |
| Comportamento (6 categorias) | _a preencher_ | Corrupção silenciosa = bug prioritário, independente da trilha. |
| rustybuzz recebe coordenadas? | _a preencher_ | — |
| oxifont-subset preserva fvar/gvar? | _a preencher_ | — |
| Detecção de formato antes de subsetar | _a preencher_ | Se ausente, fix XS obrigatório independente da decisão sobre VF completo. |
| Corpus usa weight/stretch? | _a preencher_ | Se sim, regressão de linguagem confirmada, não só mecânica. |
| Esforço MVP / completo | _a preencher_ | — |

---

## Decisão de prosseguimento

- **Se corpus usa `weight` E cristalino ignora:** P525 — MVP de VF (S–M). Trilha 7 não é XL.
- **Se corrupção silenciosa ou ausência de detecção de formato forem confirmadas:** fix de detecção de formato é prioridade **antes** de qualquer decisão sobre VF completo, independentemente do resultado dos outros grupos — mesmo padrão adoptado em P522/P523 para CFF.
- **Se corpus não usa `weight` E VF é raro no sistema:** scope-out confirmado (não assumido). P525 documenta em ADR e retoma opções do handoff.
- **Se Grupo 2 não foi testável (sem fonte VF disponível nem acesso de rede):** registar como limitação da sonda, distinta de "VF é raro" — são conclusões diferentes, mesma regra de P522.

---

## Relatório de execução

`00_nucleo/diagnosticos/sonda-vf-p524.md` — deve incluir explicitamente a secção da Sub-tarefa 0 (correcções de P523/handoff) separada da sonda VF, para que fique claro no histórico que são dois assuntos distintos combinados num só passo por conveniência, não por relação causal.
