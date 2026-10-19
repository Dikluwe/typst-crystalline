# Passo P273.14 — CMYK-ICC paridade (Fase A com verificação viabilidade)

**Tipo**: refino qualitativo opcional — substituir `/DeviceCMYK` por `/ICCBased` em gradients CMYK.
**Magnitude estimada**: **indeterminada** — S-M se viável; **trabalho prévio externo** se inviável (Fase A decide).
**Pré-requisitos**: P273.13 fechado (gradients dentro de Group renderizam via pattern dict).
**Cluster**: Visualize / Gradient (quinto sub-passo na sequência terminar cluster — renumerado por inserção P273.13).
**Aplica ADRs**: ADR-0091 (centro de aplicação ColorSpace; décima quarta anotação cumulativa); ADR-0029 (pureza física L1 preserved — passo L3-only); ADR-0093 (Pattern 2 anotação cumulativa).

---

## §0 — Contexto

P270.2 fechou cluster Gradient L3 8/8 spaces incluindo CMYK directo via `/DeviceCMYK`. Decisão ADR-0091 explícita:

> "ICC profiles scope-out preserved (cristalino DeviceCMYK directo; refino futuro)."

E pesquisa industry P270.2 §A.4:

> "Vanilla actual (via krilla) tem CMYK suporte mais sofisticado que precisamos. Cristalino pode começar simples (DeviceCMYK directo sem ICC profiles) e ICC ficar como refino futuro."

**P273.14 atinge o "refino futuro"** — substituir `/DeviceCMYK` por `/ICCBased` profile embedded.

### Diferença observable

| Componente | `/DeviceCMYK` (actual P270.2) | `/ICCBased` (P273.14 objectivo) |
|---|---|---|
| ColorSpace dict no PDF | `/DeviceCMYK` (1 palavra) | `[/ICCBased N_R 0 R]` (referência a object) |
| Profile data | Nenhum (interpretação device-dependent) | Profile bytes embebidos como stream `/N 4 /Range [...]` |
| Reproducibility entre devices | Variável | Determinística (profile define mapeamento) |
| PDF/A compliance | Falha (PDF/A exige ColorSpace ICC-based) | Possível com profile correcto |
| PDF file size | Mínimo | +30-100 KB por profile embedded |

### Bloqueador externo potencial

Cristalino `03_infra/src/export.rs` é **geração manual de PDF sem crates externas** (declarado em prompt L0 export.md):

> "Este módulo (L3) converte essa geometria pura em bytes estruturados de PDF-1.7, **sem `crates` externas de PDF** — geração manual de objectos, xref e trailer."

Para `/ICCBased` profile, há 3 caminhos possíveis:

1. **Crate adicional para ICC profile parsing/serialize** — quebra invariante "sem crates externas de PDF" (mesmo que crate seja só para ICC, ele torna-se parte do export). Adopta de facto princípio que tornaria krilla-like dependency necessária no futuro.
2. **ICC profile bytes hardcoded** (binary blob embedded como constante) — preserva "sem crates externas". Mas: qual profile? Onde obter os bytes? Licensing?
3. **Não materializar** — ICC profile continua scope-out per ADR-0054 graded; cluster Gradient fecha sem este refino.

### Predição factual da Fase A

Trajectória provável da Fase A (auto-avaliação):

- §A.1 inventário viabilidade → caminho 1 (crate) é **decisão arquitectural maior** que excede escopo S-M; precisa ADR independente.
- §A.2 obtenção bytes profile → caminho 2 (hardcoded) precisa profile específico licenciado para redistribuição (FOGRA39 é proprietário; sRGB CMYK não existe; US Web Coated SWOP requer compra licença).
- §A.3 decisão → caminho 3 (scope-out) **provável**, com relatório de trabalho prévio externo necessário.

**Esta predição não é decisão prévia** — a Fase A deve fazer o seu trabalho empírico. Mas a spec reconhece honestamente o que vai descobrir.

---

## §1 — Sub-passo P273.14.A — Fase A diagnóstico (com decisão go/no-go)

**Magnitude**: S documental (~30-45 min — diagnóstico de viabilidade prioritário sobre materialização).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-14-diagnostico.md`.

### §A.1 — Inventário das opções arquitecturais

Para cada um dos 3 caminhos, listar:

#### Caminho 1: Crate externa

- Crates candidatas: `qcms`, `lcms2`, `icc`. Inventariar:
  - Disponíveis em crates.io? Manutenção activa?
  - Licença compatível com projecto (cristalino license preserved)?
  - API serialize-only ou também parse?
  - Dependências transitivas (e.g. C bindings)? Pureza arquitectural cristalino?
- Decisão arquitectural maior necessária: **ADR nova** sobre admitir crate externa específica para ICC. ADR-0029 + invariante export.md prompt L0 impedem actualmente.

#### Caminho 2: Profile bytes hardcoded

- Profile candidatos:
  - **sRGB IEC61966-2.1** — universal, royalty-free, comum no industry. Mas é RGB, não CMYK — não aplica a este passo.
  - **US Web Coated SWOP v2** — CMYK industry standard mas **proprietário ICC** (requer licença Adobe ou compra).
  - **FOGRA39** — CMYK europeu industry standard, também licenciado.
  - **Generic CMYK no-profile** — não existe profile genérico CMYK royalty-free com qualidade.
- Bytes obtidos onde? GitHub repos como `awesome-icc` não cobrem CMYK royalty-free.
- File size impact: profiles CMYK típicos = 500 KB - 2 MB. Embebido em cada PDF.

#### Caminho 3: Scope-out preserved

- Decisão ADR-0091 mantida literal. `/DeviceCMYK` continua a ser a forma actual.
- Relatório de trabalho prévio externo necessário (para futuro hipotético): decisão sobre se cristalino quer manter "sem crates externas de PDF" como invariante perpétuo ou se há cenário em que se admite excepção.

### §A.2 — Decisão go/no-go primária

A Fase A toma uma decisão binária:

- **GO**: materializa um dos caminhos 1 ou 2. Magnitude S-M dependendo de qual. Fase C executa.
- **NO-GO**: scope-out preserved. Fase B+C reduzidas a documentação:
  - **Anotação cumulativa ADR-0091**: regista que P273.14 foi tentado, viabilidade verificada, scope-out reconfirmado.
  - **Relatório de trabalho prévio externo necessário** (output do passo): documento descrevendo o que precisaria de mudar (e.g. ADR nova autorizando crate específica; aquisição de licença profile; decisão arquitectural sobre PDF size impact).

### §A.3 — Critério para GO

A Fase A só decide GO se:

1. Caminho 1 escolhido E ADR nova produzida em paralelo (não dentro deste passo — pré-requisito declarado).
2. Caminho 2 escolhido E profile concreto identificado E licença confirmada compatível E bytes obtidos.

Caso contrário, **NO-GO** é o resultado correcto.

### §A.4 — Critério para NO-GO

A Fase A decide NO-GO se:

1. Caminho 1 requer ADR-0029 alteração ou export.md invariante revogação — decisão arquitectural fora do escopo P273.14.
2. Caminho 2 não tem profile concreto royalty-free disponível.
3. Caminho 2 tem profile mas file size impact é considerado excessivo (>500 KB embedded por gradient CMYK).

NO-GO **não é falha do passo** — é cumprimento honesto do critério "verificar Fase A" registado em todos os relatórios anteriores.

### §A.5 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Decisão arquitectural maior escondida em refino qualitativo | Caminho 1 requer ADR para crate | §A.3 critério explícito: ADR pré-requisito (não dentro deste passo) |
| Profile licenciado embedded sem licença adequada | Caminho 2 sem verificação | §A.3 critério: licença confirmada |
| File size impact ignorado | Caminho 2 sem cap | §A.4 critério: 500 KB cap por embedded |
| Scope-out parece falha | NO-GO confundido com regressão | §A.4 explicita: NO-GO é cumprimento honesto |
| Trabalho de Fase A descartado | NO-GO produz apenas documentação | Documentação é output legítimo per ADR-0054 graded; cluster avança |

### §A.6 — Decisões a fixar na Fase A

1. **Decisão 1** (caminho): 1 / 2 / 3 consoante §A.1 + §A.3 + §A.4.
2. **Decisão 2 (apenas se GO)**: detalhes de implementação consoante caminho escolhido.
3. **Decisão 3 (sempre)**: documento de trabalho prévio externo necessário se NO-GO.

### §A.7 — Critério de aceitação Fase A

Independente de go/no-go:

- §A.1 inventário dos 3 caminhos com factos empíricos (crate availability, profile licensing, file size).
- §A.2 decisão go/no-go fixada com fundamento literal.
- §A.5 risco mitigado por critério explícito.
- Se NO-GO: documento de trabalho prévio externo produzido como output do passo.

---

## §2 — Sub-passo P273.14.B — Anotação cumulativa ADR-0091

**Magnitude**: XS documental (independente de go/no-go).

Anotar ADR-0091 — décima quarta anotação consecutiva.

### Template se GO

```
## Anotação cumulativa P273.14 — CMYK-ICC profile embedded (refino qualitativo)

**Data**: 2026-05-XX.
**Decisão**: GO via caminho [1/2].
**Profile escolhido**: [nome literal + version].
**Mecanismo embedded**: [hardcoded bytes / crate API].
**File size impact**: [valor concreto em KB].
**Sub-padrão "Refino qualitativo opcional materializado"** N=0 → N=1
inaugural (precedente; cluster Gradient adopta padrão).
**Defaults preservam**: gradient não-CMYK preserved literal;
gradient CMYK pré-P273.14 sem ICC continua a ser tratado como
fallback (`/DeviceCMYK`) para reprodutibilidade.
```

### Template se NO-GO

```
## Anotação cumulativa P273.14 — CMYK-ICC scope-out reconfirmado

**Data**: 2026-05-XX.
**Decisão**: NO-GO via §A.4 critério [literal].
**Razão concreta**: [caminho 1 ADR pré-requisito / caminho 2 profile
licensing / file size impact].
**Trabalho prévio externo necessário**: ver
`00_nucleo/diagnosticos/typst-passo-273-14-trabalho-previo-externo.md`.
**ADR-0091 §"ICC profile scope-out" preserved literal** — decisão
P270.2 reconfirmada por Fase A factual.
**Sub-padrão "Scope-out reconfirmado por Fase A"** N=0 → N=1
inaugural — passo executado até critério go/no-go; trabalho de
diagnóstico legítimo per ADR-0054 graded.
**Cluster Gradient avança sem este refino** — `/DeviceCMYK`
preserved como caminho actual; PDF/A compliance preserved como
pendência inalterada.
```

---

## §3 — Sub-passo P273.14.C — Materialização (só se GO)

**Magnitude**: S-M consoante caminho escolhido na Fase A.

Se GO via caminho 1 (crate):
- ADR nova produzida em paralelo (não dentro deste passo).
- Crate adicionada ao `03_infra/Cargo.toml`.
- Helper L3 `emit_icc_based_colorspace(profile_bytes) -> ColorSpaceRef`.
- Dispatcher gradient CMYK substitui `/DeviceCMYK` por reference ao ColorSpace.

Se GO via caminho 2 (hardcoded bytes):
- Constante `const CMYK_ICC_PROFILE: &[u8] = include_bytes!("...");`.
- Helper L3 análogo a caminho 1 mas sem crate.
- Dispatcher gradient CMYK idem.

### Cap LOC (ADR-0094 Pattern 1; aplicável só se GO)

- **L3 hard cap**: ≤ 100 LOC (Caminho 1) ou ≤ 80 LOC (Caminho 2).
- **L3 soft cap**: ≤ 70 LOC ou ≤ 50 LOC.
- **L1 hard cap**: 0 LOC.
- **Tests hard cap**: ≤ 8.
- **Tests soft cap**: ≤ 5.

### Tests propostos (só se GO)

1. `p273_14_gradient_cmyk_uses_icc_colorspace` — gradient CMYK emit usa `/ICCBased` em vez de `/DeviceCMYK`.
2. `p273_14_icc_profile_embedded_in_pdf` — bytes do profile aparecem como stream no PDF.
3. `p273_14_gradient_rgb_unchanged` — gradients RGB-family preserved P270.1 bit-exact.
4. `p273_14_gradient_radial_cmyk_uses_icc` — paridade Radial.
5. `p273_14_pdf_size_within_cap` — file size impact dentro do esperado (e.g. ≤ 1 MB para profile + algumas gradient).
6. Regressão integrada: 2644 verdes preserved + tests novos.

---

## §4 — Sub-padrões cumulativos pós-P273.14

### Se GO

| Sub-padrão | Pós-P273.13 | Pós-P273.14 (GO) |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 20 | 21 |
| Cap LOC hard vs soft explícito | 16 | 17 |
| Aplicação meta-ADR (ADR-0094) | 12 | 13 |
| Sub-passos consecutivos do mesmo cluster | N=9 | **N=10 cumulativo emergente** |
| Diagnóstico imutável | 29 | 30 (25º consumo) |
| **Refino qualitativo opcional materializado** | N=0 | **N=1 inaugural** |

### Se NO-GO

| Sub-padrão | Pós-P273.13 | Pós-P273.14 (NO-GO) |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | 20 | 21 |
| Aplicação meta-ADR (ADR-0094) | 12 | 12 (preserved — sem cap LOC aplicado) |
| Sub-passos consecutivos do mesmo cluster | N=9 | **N=10 cumulativo emergente** |
| Diagnóstico imutável | 29 | 30 (25º consumo) |
| **Scope-out reconfirmado por Fase A** | N=0 | **N=1 inaugural** |

Note-se: ambos os outcomes inauguram um sub-padrão emergente diferente. Decisão Fase A determina qual.

---

## §5 — Limitações conscientes P273.14

Se GO:
- Profile escolhido é uma escolha entre opções imperfeitas — refino futuro pode trocar profile sem alteração arquitectural.
- File size impact é não-trivial (depende profile).

Se NO-GO:
- `/DeviceCMYK` continua a ser ColorSpace para gradient CMYK — interpretação device-dependent preserved.
- PDF/A compliance preserved como pendência inalterada.
- Trabalho prévio externo documentado para futuro.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico **com decisão go/no-go**.
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.7.
5. **Se GO**: utilizador executa P273.14.B + P273.14.C → relatório com materialização.
6. **Se NO-GO**: utilizador executa P273.14.B + produz documento `trabalho-previo-externo.md` → relatório com NO-GO documentado.
7. Utilizador upload do relatório.
8. Claude web analisa + propõe **P273.15** (Bbox medido pós-layout).

---

## §7 — Pendências preservadas pós-P273.14

Inalteradas vs P273.13:

- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Sequência para fechar cluster Gradient (renumerada vs spec P273.13 §9):

- ✓ P273.10/11/12/13 (fechados).
- **P273.14** — CMYK-ICC paridade (este passo; pode ser último materializado se NO-GO + sequência re-avaliada).
- **P273.15** — Bbox medido pós-layout (M).
- **P273.16** — Bbox.y topo-exacto inline (M-L; bloqueado DEBT-56).

Se P273.14 = NO-GO, pendência **P-Gradient-CMYK-ICC permanece** como item formal aberto mas com trabalho prévio externo documentado. Cluster pode declarar-se feature-complete sem este refino.

Pendência específica descoberta P273.13 §9:
- **P273.X-bis-helper-group-bbox** — extract helper compartilhado 3 sítios.

Pendência implícita exposta P273.13 §9 segundo bullet (nova; fora cluster):
- **P273.X-bis-draw-item-local-text-image** — Text + Image em Groups silenciosamente descartados em `draw_item_local`. **Fora do cluster Gradient** (afecta Text/Image). Registar como pendência cluster Visualize geral.

---

## §8 — Critério de fecho do passo

P273.14 fecha com **IMPLEMENTADO** (GO) ou **SCOPE-OUT-RECONFIRMED** (NO-GO).

### IMPLEMENTADO (GO)

- Fase A produzida + critério §A.7 cumprido + decisão GO.
- ADR-0091 anotada (décima quarta anotação consecutiva).
- L3 alterado dentro do cap LOC; L1 intocado.
- Tests workspace verdes (cap respeitado).
- Lint zero.
- Tests P262-P273.13 inalterados bit-exact.
- DEBT saldo 10 preserved.
- Test E2E confirma `/ICCBased` no PDF para gradient CMYK.

### SCOPE-OUT-RECONFIRMED (NO-GO)

- Fase A produzida + critério §A.7 cumprido + decisão NO-GO.
- ADR-0091 anotada (décima quarta anotação consecutiva — versão NO-GO).
- Documento `trabalho-previo-externo.md` produzido.
- Zero alterações ao código L3/L1.
- Tests workspace verdes 2644 preserved (sem mudança).
- Sub-padrão "Scope-out reconfirmado por Fase A" N=1 inaugural.

Ambos são outcomes legítimos do passo — diferenciados pela predição feita na Fase A.

---

## §9 — Numeração

Spec usa **P273.14** continuando a sequência decimal pós-inserção P273.13. Quinto sub-passo materializado da sub-sequência "terminar cluster Gradient".

Sequência prevista atualizada:

- ✓ P273.5 — relative cross-variant.
- ✓ P273.6 — Block save/restore.
- ✓ P273.7 — Boxed save/restore.
- ✓ P273.8 — Cleanup 4 warnings.
- ✓ P273.9 — Grid + Stack + Pad.
- ✓ P273.10 — Group L3-only scan.
- ✓ P273.11 — Extract Stack helper.
- ✓ P273.12 — Dedup bbox-aware.
- ✓ P273.13 — Fix draw_item_local Group (INSERIDO).
- **P273.14** — CMYK-ICC paridade (este passo; GO ou NO-GO).
- P273.15 — Bbox medido pós-layout (M).
- P273.16 — Bbox.y topo-exacto inline (M-L; bloqueado DEBT-56).

**Predição revisada**: cluster termina entre P273.14 (se NO-GO + sequência re-avaliada) e P273.16 consoante decisões de cada passo.

A sequência tem permitido ao cluster Gradient atravessar 10 sub-passos (P273.5-P273.14) — sub-padrão "Sub-passos consecutivos do mesmo cluster" cresce para N=10 em qualquer outcome.
