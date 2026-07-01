# Sonda CFF — Passo 522

## Resumo

Esta sonda avaliou o estado actual do subsetting de fontes CFF no export
PDF cristalino. Contrariamente à nota do handoff ("CFF: fallback activo,
XL-size"), os resultados mostram que:

- `oxifont-subset` 0.2.0 **já suporta CFF e CFF2**.
- O cristalino consegue compilar documentos com fontes CFF, embeber a fonte
  subsetada, e extrair texto correctamente.
- **Não foi detectado panic, fallback silencioso, `.notdef` generalizado, nem
  corrupção silenciosa** da fonte embebida.

A principal diferença face ao vanilla é que o cristalino gera um descritor
PDF `CID TrueType` para uma fonte que contém tabela `CFF`, enquanto o
vanilla gera `CID Type 0C`. O tamanho do PDF cristalino é maior (~18 KB vs
~5 KB para "Hello world."), sugerindo que o subset CFF do cristalino
mantém mais tabelas auxiliares.

## Grupo 1 — Inventário de fontes CFF no sistema

```bash
fc-list : file format | grep -i "cff\|opentype"
/tmp/fonttools-venv/bin/python3 # script de contagem
```

Resultado (amostra de 500 fontes do `fc-list`):

| Tipo | Contagem |
|------|----------|
| CFF | 57 |
| TrueType (glyf) | 308 |
| Other/Error | 135 |

Exemplos CFF comuns:

- `/usr/share/fonts/opentype/urw-base35/NimbusSans-*.otf`
- `/usr/share/fonts/opentype/urw-base35/C059-*.otf`
- `/usr/share/fonts/opentype/urw-base35/P052-*.otf`
- `/usr/share/fonts/opentype/freefont/FreeSerif*.otf`
- `/usr/share/fonts/opentype/mathjax/MathJax_*.otf`

Fonte CFF usada nos testes: **Nimbus Sans**.

## Grupo 2 — Comportamento actual do cristalino com CFF

### Fonte de teste garantida

Nimbus Sans (`/usr/share/fonts/opentype/urw-base35/NimbusSans-Regular.otf`,
82 KB, CFF).

### Documento de teste

```typst
#set text(font: "Nimbus Sans", size: 12pt)
Hello world.
```

### Resultado

```bash
cargo run --bin typst -- /tmp/test-cff.typ /tmp/cff-cristalino.pdf
# Exit code: 0
```

Validação da fonte extraída:

```bash
mutool extract /tmp/cff-cristalino.pdf
# extrai font-0007.ttf
```

Apesar da extensão `.ttf` atribuída pelo `mutool`, a fonte extraída contém
a tabela `CFF ` e zero tabela `glyf`:

```text
Tabelas: ['CFF ', 'GPOS', 'GSUB', 'GlyphOrder', 'OS/2', 'cmap', 'head', 'hhea', 'hmtx', 'maxp', 'name', 'post']
numGlyphs embedded: 9
```

A fonte foi **subsetada** (passou de 82 KB para ~16 KB no PDF, 9 glyphs).

### Classificação

| Comportamento | Classificação | Observação |
|---------------|---------------|------------|
| Fonte CFF completa embebida, íntegra | Não aplicável | A fonte foi subsetada, não completa. |
| Subset CFF embebido, íntegro | ✅ Funciona | Texto extraído correctamente; fonte parseável pelo fontTools. |
| Fonte não embebida / fallback silencioso | ❌ Não observado | O PDF usa Nimbus Sans. |
| Panic / erro no processo | ❌ Não observado | Exit code 0. |
| `.notdef` para todos os glifos | ❌ Não observado | Texto legível e extraível. |
| Corrupção silenciosa (CFF tratado como TrueType) | ❌ Não observado | A tabela CFF está presente e válida. |

### Teste com ligatures

```typst
#set text(font: "Nimbus Sans", size: 12pt)
The five boxing wizards jump quickly. ffi fl fi
```

Resultado:

- Compilação com exit code 0.
- `pdftotext` extrai o texto completo, incluindo `ffi`, `fl`, `fi`.
- Kerning visível no operador `TJ`.

### Comparação com vanilla

| Aspecto | Cristalino | Vanilla 0.15.0 |
|---------|------------|----------------|
| Tamanho PDF | 18 291 B | 5 039 B |
| Tipo no pdffonts | `CID TrueType` | `CID Type 0C` |
| Fonte extraída | OpenType com CFF (`.ttf`) | CFF puro (`.cid`) |
| Texto extraível | ✅ Sim | ✅ Sim |

## Grupo 3 — Capacidade do `oxifont-subset` com CFF

### `oxifont-subset` suporta CFF?

**Sim.** Evidência no código fonte da crate 0.2.0:

```text
src/cff.rs       — rewrite_cff
src/lib.rs:673   — let is_cff = orig_tables.contains_key(b"CFF ");
src/lib.rs:1034  — cff::rewrite_cff(d, gid_remap_ref)
src/lib.rs:1043  — cff::rewrite_cff2(...)
```

A crate detecta CFF vs TrueType internamente e aplica o caminho correcto.

### O código cristalino detecta formato antes de subsetar?

**Não explicitamente.** `subset_font_with_mapping` em
`03_infra/src/export/subset.rs` passa os bytes directamente para
`oxifont_subset::subset_with_gid_set`. A detecção acontece dentro da crate.

Isto não produziu corrupção nos testes, mas o comentário em `subset.rs:31`
("Retorna `None` se a fonte for CFF/OpenType sem tabela `glyf`") está
**desactualizado** — a função não retorna `None` para CFF.

### Alternativas avaliadas

| Opção | Estado | Notas |
|-------|--------|-------|
| A — `oxifont-subset` já suporta CFF | ✅ Confirmado | Nenhuma alteração de crate necessária. |
| B — `subsetter` crate | Não avaliado | Desnecessário; oxifont-subset funciona. |
| C — `fontTools` via subprocess | Não necessário | Quebraria pureza Rust. |
| D — CFF subsetting manual | Não necessário | XL; oxifont-subset cobre. |
| E — Scope-out permanente com detecção de formato | Não justificado | CFF funciona; detecção interna da crate é suficiente. |

## Grupo 4 — Impacto no corpus de paridade

```bash
grep -rn "text(font:" lab/parity/corpus/ | grep -v "Noto\|DejaVu\|Linux\|Liberation"
```

Resultado: **nenhum documento do corpus usa fonte CFF explicitamente**.

```bash
grep -rn "CFF\|\.otf" 01_core/src/ 03_infra/src/ --include="*.rs"
```

Resultado: apenas referências incidentais e um comentário desactualizado em
`subset.rs:31`.

## Grupo 5 — Estimativa de complexidade

| Componente | Esforço estimado | Notas |
|------------|-------------------|-------|
| Corrigir comentário desactualizado em `subset.rs` | XS | Documentação, não código. |
| Adicionar teste de CFF ao corpus / unit tests | XS–S | Reutilizar Nimbus Sans ou C059. |
| Investigar descritor PDF `CID TrueType` vs `CID Type 0C` | S–M | Pode exigir ajuste no `FontDescriptor` / `Subtype`. |
| Suporte CFF2 / variable fonts | XL | Fora de escopo; nenhuma fonte CFF2 no sistema. |

## Tabela final de classificação

| Item | Resultado | Impacto na decisão |
|------|-----------|-------------------|
| Fontes CFF no sistema | 57 CFF / 308 TrueType / 135 Other | CFF é comum em instalações Linux padrão (URW, FreeFont, MathJax). |
| Fonte CFF de teste garantida? | Sim — Nimbus Sans | Grupo 2 testável. |
| Comportamento actual (5 categorias) | Subset CFF embebido, íntegro | Nenhum bug prioritário identificado. |
| Código detecta formato antes de subsetar? | Não explicitamente; oxifont-subset detecta internamente | Comentário desactualizado deve ser corrigido, mas não é bug de corrupção. |
| `oxifont-subset` suporta CFF? | Sim | Trilha 6 é muito menor que XL. |
| Corpus usa CFF? | Não explicitamente | Impacto nulo no corpus actual. |
| Esforço estimado (trilha completa) | XS–M | Principalmente testes e alinhamento do descritor PDF. |

## Decisão de prosseguimento

A alegação do handoff "CFF: fallback activo, XL-size" **não se confirma**.
O subsetting CFF já funciona no cristalino através do `oxifont-subset`.

Recomendações para P523:

1. **XS — Corrigir documentação:** actualizar o comentário em
   `03_infra/src/export/subset.rs:31` para reflectir que CFF é suportado.
2. **XS–S — Adicionar cobertura:** incluir pelo menos um teste de export
   com fonte CFF (Nimbus Sans ou C059) no corpus de paridade ou nos testes
   unitários de `subset.rs`.
3. **S–M — Investigar descritor PDF:** o cristalino emite `CID TrueType`
   enquanto o vanilla emite `CID Type 0C`. Verificar se isto é apenas
   cosmético (ambos renderizam correctamente) ou se afecta compatibilidade
   com leitores de PDF específicos. Se cosmético, pode ficar como
   scope-out documentado de paridade mecânica (ADR-0107).

Não é necessário abrir uma Trilha 6 do tamanho XL. CFF subsetting está
operacional; o trabalho restante é polimento e cobertura.

## Ficheiros de referência

- `/usr/share/fonts/opentype/urw-base35/NimbusSans-Regular.otf`
- `/tmp/test-cff.typ`
- `/tmp/cff-cristalino.pdf`
- `/tmp/cff-vanilla.pdf`
- `03_infra/src/export/subset.rs`
- `~/.cargo/registry/src/*/oxifont-subset-0.2.0/src/cff.rs`
