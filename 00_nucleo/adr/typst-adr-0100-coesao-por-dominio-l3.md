# ⚖️ ADR-0100: ADR-0037 estendida a L3 — coesão por domínio em `03_infra/`

**Status**: `IMPLEMENTADO`
**Data**: 2026-05-19 (`PROPOSTO` em P307a) / 2026-05-19 (`IMPLEMENTADO` em P307d após validação empírica P307b.1 + P307b.2 + P307c)
**Passo promotor**: P307a (diagnóstico-primeiro), promovido em P307d
**Categoria**: Arquitectural

---

## Contexto

ADR-0037 ("Coesão por domínio") foi formalizada em P96.3 para L1
(`01_core/`). Aplicou-se aos 6 ficheiros >1000 linhas detectados
em L1 e fechou DEBT-46 com decomposição completa (eval.rs, parse.rs,
stdlib.rs, layout/mod.rs, math/layout.rs, lexer/mod.rs).

A ADR-0037 §"Plano de aplicação" lista explicitamente apenas L1.
Não estende-se formalmente a L2/L3/L4. **DEBT-46-L3 nunca foi
aberto** mesmo quando `03_infra/src/export.rs` cresceu para
**2.826 LOC de produção + 7.029 LOC de testes** (P307a diagnóstico).

Auditoria empírica de 2026-05-19 (pós-P306) identificou
`03_infra/src/export.rs` como único ficheiro L3 a exceder
significativamente o limite Regra 2 de ADR-0037 (800 LOC). Os
outros ficheiros L3 são todos <1000 LOC:

| Ficheiro L3 | LOC |
|---|---:|
| `03_infra/src/export.rs` | **9.856** (2.826 prod + 7.029 tests) |
| `03_infra/src/integration_tests.rs` | ~600 |
| `03_infra/src/world.rs` | ~400 |
| `03_infra/src/layout.rs` | 72 |
| `03_infra/src/pipeline.rs` | ~300 |
| `03_infra/src/fonts.rs` | ~200 |
| Outros | <500 |

O problema concentra-se num único ficheiro. P307 propôs decompor.

---

## Decisão

**ADR-0037 aplica-se a todas as camadas L1-L4 (excluindo L0 que
é Markdown).**

Em particular, para L3 (`03_infra/`):

1. **Regra 1 (coesão por domínio)** aplica-se: ficheiros L3 agrupam
   código por domínio técnico ou conceptual coerente. Para L3, os
   domínios típicos são:
   - **Tipo de I/O**: export (PDF), font loading, image decoding,
     filesystem access.
   - **Pipeline de compilação**: world setup, source loading, eval
     orchestration.
   - **Adaptadores**: `SystemWorld` (filesystem-backed `World`
     trait).

2. **Regra 2 (limite 800 linhas)** aplica-se com mesma orientação.

3. **Regra 3 (hierarquia de submódulos)** aplica-se: ponto de
   entrada (`mod.rs`) contém API pública; submódulos são
   `pub(super)`/`pub(crate)` conforme necessidade. Paths
   relativos (`super::X::func()`) preferidos sobre absolutos.

4. **Regra 4 (dispatchers pequenos)** menos aplicável em L3 (L3
   tem menos enums grandes que L1) mas mantém-se: `match`
   exaustivos sobre `FrameItem` em L3 devem delegar armos
   substanciais a submódulos especializados.

5. **Regra 5 (testes seguem o domínio)** aplica-se com **excepção
   importante**: testes de export PDF são intrinsecamente
   cross-cutting (cruzam imagens + gradients + fonts + layout
   numa única invocação `export_pdf`). Per Ajuste C, podem viver
   em `tests.rs` agregador único mesmo se >800 LOC, marcado como
   Regra 6 categoria "infraestrutura de testes E2E".

6. **Regra 6 (excepções)** aplica-se: ficheiros L3 que
   legitimamente excedem 800 LOC (e.g. `tests.rs` agregador,
   tabelas de dados como `font_metrics.rs`) devem documentar a
   categoria no topo.

7. **Regra 7 (reestruturação preserva comportamento)** **estende-se
   a invariante observable verdadeiro**: para L3 export, a invariante
   é **PDF bytes preservados bit-exact** num corpus canónico, não
   hash textual de ficheiros fonte. Ver substituição em §"Relação
   com ADR-0098".

---

## Decisões específicas para `export.rs` (P307b)

O ficheiro `03_infra/src/export.rs` decompõe-se em:

```
03_infra/src/export/
    mod.rs                  # API pública + dispatcher
    builder.rs              # struct PdfBuilder + métodos build_*
    images.rs               # Detecção + zlib + XObjects
    fonts.rs                # CIDFont helpers + collectors + escape
    gradients/
        mod.rs              # scan + pattern_resources + tipos
        linear.rs           # Linear coords + sample_stops RGB
        radial.rs           # Radial coords + sample_stops RGB
        cmyk.rs             # CMYK helpers + emit_function_dict_cmyk
        relative.rs         # resolve_relative + apply_parent_transform
        adaptive.rs         # adaptive_n + perceptual_distance
        conic.rs            # Coons patches + emit_conic_coons_stream
        function_dict.rs    # emit_function_dict (RGB)
    stream/
        mod.rs              # PageContext + FontScenario
        page.rs             # build_page_stream
        text.rs             # emit_text_pdf + emit_glyph_pdf
        shape.rs            # emit_shape_path_local + emit_rounded_rect_ops + stroke
        draw.rs             # draw_item_local
    tests.rs                # ~7029 LOC agregador (Regra 6)
```

Total: **~15 ficheiros + tests.rs** (vs 1 monolito actual).

Esta estrutura emerge do inventário factual (P307a §5), não de
divisão mecânica nem da spec original §4 (que tinha mais
sub-divisão).

---

## Relação com ADR-0098 (SSoT invariante anti-bug)

ADR-0098 actual estabeleceu que **hash textual de `export.rs`
(`66cb8ac3`) preserva-se bit-exact** como invariante operacional.
23 passos consecutivos P282-P306 honraram esta invariante.

**P307 substitui o proxy textual por proxy binário**:

| Invariante | Antes (ADR-0098 P281-P306) | Pós (ADR-0098 + ADR-0100 P307+) |
|---|---|---|
| Forma | Hash do ficheiro `export.rs` constante | PDF bytes constantes para corpus canónico |
| Proxy | Textual (hash do `.rs`) | Binário (hash dos `.pdf` gerados) |
| Validação | Manual: cada passo verifica `@prompt-hash` | Automática: `cargo test snapshot_p307b` |
| False positives | Sim (qualquer refactor benigno quebra) | Não (só mudança de comportamento quebra) |
| False negatives | Não (hash textual capturava tudo) | Improvável (corpus canónico cobre 5 clusters) |

A invariante observable **fortalece-se**. ADR-0098 não é deprecada;
é **anotada** com secção "Evolução pós-P307: proxy textual →
snapshot binário".

---

## Alternativas Consideradas

| Alternativa | Razão rejeitada |
|---|---|
| Manter ADR-0037 só para L1 | Cria precedente de aplicar disciplina apenas a uma camada; viola o espírito da arquitectura cristalina ("disciplina aplica-se uniformemente") |
| Criar ADR distinta para L3 com regras diferentes | Divergência metodológica desnecessária; L3 e L1 têm os mesmos problemas de monolitização |
| Aplicar ADR-0037 implicitamente sem ADR formal | Drift documental — nenhum registo de quando a regra passou a aplicar-se |
| Deixar `export.rs` monolítico (Regra 6) | 2.826 LOC de produção não cabe em "tabela de dados" ou "enum fundamental"; é código composicional decomponível |
| Decomposição em estrutura da spec §4 (25 ficheiros) | Spec foi pre-inventário; granularidade real (P307a §5) é 15 ficheiros sem perder coesão |

---

## Consequências

### Positivas

- Navegação L3 melhorada: encontrar "como Conic Coons é emitido"
  abre `gradients/conic.rs`, não scroll através de 9.856 linhas.
- Habilita discussão de dependências por submódulo: `flate2` em
  `images.rs` é justificável; em `gradients/linear.rs` não é.
- Testes podem decompôr-se gradualmente: tests.rs agregador é
  primeira fase; P307b.2/posterior pode separar por cluster se
  útil.
- Hashes propagados independentemente: alterações a `gradients/`
  não invalidam hash de `images.rs`.

### Negativas

- Esforço inicial P307b mecânico mas alto volume (~3.000 LOC
  movidas).
- Imports cross-submódulo mais longos.
- ADR-0098 perde proxy textual (compensado por snapshot binário).
- L0 `infra/export.md` decompõe-se em ~15 prompts (P307c).

### Neutras

- Git detecta movimentação como rename; história preservada.
- `crystalline-lint --fix-hashes` propaga mecanicamente os novos
  hashes.

---

## Critérios de validação (para promoção a `EM VIGOR/IMPLEMENTADO`)

ADR-0100 promove-se a `EM VIGOR/IMPLEMENTADO` em P307d quando:

1. **P307b** (decomposição) fechado: 15 submódulos criados;
   `crystalline-lint` zero violations; snapshot binário verde
   para 7 ficheiros canónicos.
2. **P307c** (L0 prompts) fechado: cada submódulo tem prompt L0
   com hash propagado.
3. **Métrica L3 pós-P307**: nenhum ficheiro em `03_infra/src/`
   acima de 800 linhas sem justificativa Regra 6 documentada.
4. **Cargo test workspace verde**: incluindo todos os 230 testes
   pré-existentes de `export.rs` (movidos para `tests.rs`).
5. **Performance neutra ou positiva**: compile time de `03_infra`
   não regride significativamente.

Se algum critério falhar, ADR-0100 retrocede para `REVISÃO` com
diagnóstico de causa.

---

## Validação empírica P307d — todos os critérios atingidos

Verificação executada em 2026-05-19 ao fechar P307c:

| # | Critério | Estado | Evidência |
|---:|---|---|---|
| 1 | P307b decomposição + lint zero | ✅ | 14 submódulos em `03_infra/src/export/`; `crystalline-lint .` reporta "No violations found" |
| 1 | Snapshot binário verde (7 fixtures min) | ✅ | **9 fixtures** verde (1 a mais do que estipulado): `cargo test p307b_snapshot` → 9/9 OK |
| 2 | L0 prompts por submódulo + hashes | ✅ | 14 prompts criados em `00_nucleo/prompts/infra/export/`; hashes propagados via `--fix-hashes` |
| 3 | Nenhum ficheiro > 800 LOC sem Regra 6 documentada | ✅ | Maior ficheiro de produção: `builder.rs` (671 LOC), `stream.rs` (685 LOC); `tests.rs` (7039 LOC) documentado como Regra 6 em `tests.md` |
| 4 | Cargo test workspace verde + 230 testes | ✅ | `cargo test -p typst-infra --lib` → 472 passed (230 export + 233 outros + 9 snapshot); workspace total 2919+ |
| 5 | Performance neutra | ✅ | Compile time `cargo build -p typst-infra` ≈ 0.05s incremental (cold ainda ~5s); sem regressão detectada |

**Conclusão**: todos os 5 critérios atingidos. ADR-0100 promovida
de `PROPOSTO → IMPLEMENTADO`. Ver §"Status" no topo.

### Diferenças vs plano original

- **Estrutura final 14 submódulos** (vs ~15 inicialmente estimado em §"Plano de aplicação"). Razão: fusão de clusters menores em ficheiros únicos (e.g., gradient relative+adaptive não eram suficientemente independentes para sub-ficheiros próprios em P307b.1; foram extraídos em P307b.2 onde a granularidade fez sentido).
- **stream.rs (685 LOC) ficou ligeiramente sob 800** — sub-divisão futura em P-stream-decomp possível mas baixa prioridade (per L0 `stream.md` §"Excede limite 800 LOC ADR-0037 Regra 2").
- **conic.rs (~300 LOC) ficou dentro do limite** — sub-divisão prevista em P307b.2 (radial/linear separation interna) não foi necessária.

### Sub-padrão emergente — N=1 "decomposição L→L+1"

P307 é a primeira aplicação de decomposição estrutural em camada
não-L1. ADR-0037 cobriu L1 (P96.x série); ADR-0100 estende L3.
Padrão emergente: **"ADR de camada estende coesão para a camada
seguinte quando ficheiro excede 800 LOC"**.

N=1 — não promovido a meta-padrão per anti-padrão P273.17 §0
(over-formalização). Reavaliar se L2 ou L4 acumular ficheiros
> 800 LOC no futuro.

---

## Plano de aplicação

| Passo | Acção | Output |
|---|---|---|
| P307a | Inventário + ADR PROPOSTA | Diagnóstico + ADR-0100 (este) + anotação ADR-0098 |
| P307b.1 | Extracção mecânica de 15 submódulos | `git mv` + reorganização imports; snapshot binário verde |
| P307b.2 | Refino opcional dentro de `gradients/` | Sub-divisão `conic.rs` se >800 LOC |
| P307b.3 | Hashes propagados | `crystalline-lint --fix-hashes` |
| P307c | L0 prompts por submódulo | ~15 prompts em `00_nucleo/prompts/infra/export/` |
| P307d | Promoção ADR-0100 | `PROPOSTO → IMPLEMENTADO`; anotação ADR-0098 reflectindo transição completa |

---

## Referências

- ADR-0037 (coesão por domínio L1) — esta ADR estende para L3.
- ADR-0098 (SSoT invariante anti-bug) — anotada com substituição
  proxy textual → binário.
- ADR-0033 (paridade funcional vanilla) — invariante observável
  preservada via snapshot binário.
- DEBT-46 (L1) — fechado em P96.x série; DEBT-46-L3 análogo aberto
  e fechado dentro da série P307a-d.
- P156B (Layout) e P154A (Model) — análogos históricos de "spec
  diagnóstico-primeiro antes de implementação".
- `diagnostico-export-passo-307a.md` — inventário factual completo.

---

## Nota histórica

ADR-0100 é a primeira ADR L3-específica do projecto. Antecede a
extensão formal para L2/L4 (se justificada). L2 (`02_shell/`) tem
apenas ~600 LOC totais — não candidato à decomposição. L4
(`04_wiring/`) tem ~500 LOC — idem.

A spec P307 documenta que cada camada pode receber ADR análoga se
e quando ficheiros excederem o limite. ADR-0037 já antecipou isto
em §"Plano de aplicação" ("se outras camadas excederem o limite,
abrir DEBT específico por camada").
