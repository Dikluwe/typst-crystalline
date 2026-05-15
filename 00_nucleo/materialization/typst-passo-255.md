# Passo 255 — Finalizar Math (auditoria + actualização docs + fecho DEBT-8 condicional)

**Data**: 2026-05-15
**Tipo**: passo composto sequencial; magnitude estimada XS-M conforme cenário Fase A
**Pré-requisito leitura obrigatória**: `CLAUDE.md` (Regra de Ouro
+ Protocolo de Nucleação + Ordem testes-primeiro) e
`00_nucleo/diagnosticos/diagnostico-math-passo-254B.md` +
`00_nucleo/diagnosticos/fase-a-checklist-math-passo-254B.md`.

**Outputs canónicos esperados** ao fim do passo:
- `00_nucleo/diagnosticos/diagnostico-math-fase-a-passo-255.md`
  (Fase A executada — tabela preenchida; imutável após criação).
- DEBT-8 actualizado ou encerrado em `00_nucleo/DEBT.md`.
- Prompts L0 obsoletos actualizados (`rules/math/layout.md`
  e `entities/math_constants.md`); hashes propagados.
- Eventual código L1 novo apenas se cenário B2/B3 confirmar
  pendência real e seu prompt L0 estiver actualizado primeiro.
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-255-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

Respeitam **CLAUDE.md** sem excepção:

1. **Regra de Ouro (trava arquitectural)** — código L1 nunca é
   escrito antes do prompt L0 correspondente estar criado ou
   actualizado.
2. **Ordem testes-primeiro** — qualquer código novo: testes
   primeiro a partir dos critérios do L0 (com falha verificada
   antes da implementação), depois código.
3. **Após editar L0** — correr `cargo run -p crystalline-lint --
   --fix-hashes .` (ou comando equivalente do repo).
4. **Materialization é leitura proibida por iniciativa
   própria** — Claude Code não deve ler
   `00_nucleo/materialization/` durante este passo excepto se o
   utilizador fornecer path explícito.
5. **Critério de correcção primário** — `crystalline-lint .`
   deve retornar `✓ No violations found` no fim do passo.
6. **Tests workspace** — não devem regredir; contagem reportada
   antes e depois.

---

## §1 — Sub-passo P255.A: Fase A (auditoria empírica)

**Objectivo**: produzir evidência factual sobre o estado real
das 4 pendências DEBT-8.

**Materialização**: zero código novo. Apenas leitura e produção
de diagnóstico imutável.

### Acções obrigatórias

Executar exactamente os comandos listados em
`00_nucleo/diagnosticos/fase-a-checklist-math-passo-254B.md`,
secção "Comandos Fase A":

**Item 1 — Kern matemático**:
```bash
grep -rn "MathGlyphKern" 01_core/src/rules/math/
grep -rn "math_kern\b" 01_core/src/rules/math/
grep -n "kern\|MathKernRecord\|math_kern" 01_core/src/rules/math/layout/attach.rs
grep -n "kern" 01_core/src/rules/math/layout/mod.rs
```

**Item 2 — OpenType MATH tables + variantes**:
```bash
grep -n "GlyphVariants\|vertical_glyph_variants\|\.select(" 01_core/src/rules/math/layout/stretchy.rs
grep -n "GlyphAssembly\|vertical_glyph_assembly\|GlyphPart" 01_core/src/rules/math/layout/assembly.rs
grep -rn "MathConstants\|math_constants\b" 01_core/src/rules/math/
```

**Item 3 — MathPrimes layout**:
```bash
grep -rn "MathPrimes\|Content::MathPrimes\|primes\b" 01_core/src/rules/math/
grep -n "primes\|Primes" 01_core/src/rules/math/layout/attach.rs
```

**Item 4 — Baseline x-height**:
```bash
grep -n "apply_axis_offset\|axis_height" 01_core/src/rules/math/layout/mod.rs
grep -rn "x_height\|x-height\|axis_height\|baseline" 01_core/src/rules/math/
```

**Verificação dos campos reais de MathConstants** (para
P255.B docs):
```bash
cat 01_core/src/entities/math_constants.rs
diff <(cat 01_core/src/entities/math_constants.rs | grep "pub " | grep ":") \
     <(cat 00_nucleo/prompts/entities/math_constants.md | grep "pub " | grep ":")
```

### Output exigido — ficheiro novo

Criar
`00_nucleo/diagnosticos/diagnostico-math-fase-a-passo-255.md`
com a seguinte estrutura (imutável após criação per ADR-0034):

```markdown
# Diagnóstico Math Fase A — Passo 255 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0034 diagnóstico canónico + ADR-0065
inventariar primeiro critério #5.
**Diagnóstico pai**:
  `diagnostico-math-passo-254B.md`.

---

## §1 — Comandos executados e output literal

(Colar output literal de cada um dos comandos §1 do enunciado
do passo, incluindo hits ou ausência de hits. Não interpretar
aqui — só registar.)

## §2 — Classificação por item

Tabela preenchida com hits factuais:

| # | Pendência DEBT-8 | Hits literais | Classificação | Justificação |
|---|------------------|---------------|---------------|--------------|
| 1 | Kern matemático  | ...           | aberto/parcial/fechado | ... |
| 2 | OpenType MATH    | ...           | ...                    | ... |
| 3 | MathPrimes       | ...           | ...                    | ... |
| 4 | Baseline x-height| ...           | ...                    | ... |

## §3 — Inconsistências documentais detectadas

(Listar discrepâncias factuais entre prompts L0 e código real
encontradas durante Fase A.)

## §4 — Decisão do cenário Fase B

**Contagem fechados/abertos**: `_/4 fechados; _/4 abertos`.

**Cenário escolhido**: ☐ B1 (fecho total) / ☐ B2 (parcial) /
☐ B3 (≥3 abertos).

## §5 — Referências

- Diagnóstico pai P254B.
- Checklist Fase A P254B.
```

### Critério de aceitação P255.A

- Ficheiro
  `diagnostico-math-fase-a-passo-255.md` criado em
  `00_nucleo/diagnosticos/`.
- Tabela §2 preenchida com hits factuais literais (não
  interpretativos).
- Decisão Fase B explicitada em §4.
- Zero alterações em código L1/L2/L3/L4.
- Zero alterações a prompts L0 ou DEBT.md (ainda — vem em
  P255.B).

---

## §2 — Sub-passo P255.B: Actualização documental dos L0 obsoletos

**Objectivo**: reconciliar prompts L0 com o estado real do
código pós-P96.8 (descoberto em P255.A §3).

**Materialização**: edição de prompts L0 + `--fix-hashes`.
Sem código L1.

### Acções obrigatórias

#### B.1 — Actualizar `00_nucleo/prompts/rules/math/layout.md`

Conteúdo a corrigir:

- Substituir secção "Âmbito por passo" (obsoleta — refere
  Passos 36/37+/38+ como futuro) por **secção "Estado
  actual"** listando:
  - Estrutura pós-P96.8: 8 submódulos
    (`attach.rs`, `root.rs`, `frac.rs`, `matrix.rs`, `cases.rs`,
    `stretchy.rs`, `assembly.rs`, `delimited.rs`).
  - `MathLayouter` com métodos coord: `new`,
    `apply_axis_offset`, `layout_equation`, `layout_node`,
    `layout_text_node`, `layout_sequence`, `layout_grid_rows`,
    `layout_grid`, `hconcat`.
  - `MathBox` com 4 campos `pub(super)`.
- Listar consumers reais de cada tipo de domínio descobertos
  em P255.A:
  - `MathConstants` → consumidor (ex: `apply_axis_offset`).
  - `MathGlyphKern` → consumidor (se Item 1 estiver
    fechado).
  - `GlyphVariants` → consumidor em `stretchy.rs`.
  - `GlyphAssembly` → consumidor em `assembly.rs`.
- Manter restrição arquitectural L1 puro + `FontMetrics` trait.

#### B.2 — Actualizar
`00_nucleo/prompts/entities/math_constants.md`

Conteúdo a corrigir:

- Substituir struct exposta (10 campos enumerados) pela
  enumeração completa descoberta em P255.A
  (provavelmente inclui `axis_height` e outros campos
  ausentes).
- Verificar critérios de verificação ainda válidos; ajustar
  se necessário.

#### B.3 — Propagar hashes

Após editar ambos os prompts:

```bash
cargo run -p crystalline-lint -- --fix-hashes .
```

Verificar que nenhum outro prompt fica com hash inconsistente
como efeito colateral.

#### B.4 — Verificação final

```bash
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
cargo test
# Esperado: contagem inalterada vs antes do passo
```

### Critério de aceitação P255.B

- Ambos os prompts L0 reflectem o código real pós-P96.8.
- Hashes propagados via `--fix-hashes`.
- Zero violations do linter.
- Tests workspace inalterados em contagem.
- Zero alterações a código L1/L2/L3/L4 (esta fase é puramente
  documental).

---

## §3 — Sub-passo P255.C: Materialização condicional (apenas se B2 ou B3)

**Executar apenas** se P255.A §4 escolheu cenário B2 (1-2
pendências reais) ou B3 (≥3 pendências reais).

**Se P255.A escolheu B1** (4/4 fechados), saltar P255.C
directamente para P255.D.

### Acções (por pendência real confirmada em P255.A)

#### Caso C.1 — Item 1 (Kern matemático) confirmado aberto

**Pré-requisito**: prompt L0 `rules/math/layout.md`
actualizado em P255.B inclui secção "Kern matemático" com
critérios de verificação. **Se não incluir, criar/editar
antes de qualquer código** (Regra de Ouro).

**Materialização**:

1. **Testes primeiro** (a partir dos critérios do L0):
   ```rust
   // 01_core/src/rules/math/layout/tests.rs (ou attach.rs tests módulo)
   #[test]
   fn hconcat_aplica_kern_top_right() { ... }
   #[test]
   fn attach_aplica_kern_superscript() { ... }
   ```
   Executar `cargo test math::` — verificar que falham.

2. **Implementação** em `hconcat` (ou `attach.rs`):
   - Consultar `metrics.math_kern(c)` para o glifo base.
   - Aplicar kern_value ao offset horizontal do próximo
     MathBox.

3. Executar `cargo test math::` — verificar que passam.

4. Executar `cargo run -p crystalline-lint -- .` — zero
   violations.

**Magnitude esperada**: M (~1-2h; +5-8 tests).

#### Caso C.2 — Item 3 (MathPrimes) confirmado aberto

**Pré-requisito**: prompt L0 `rules/math/layout.md` inclui
secção "MathPrimes layout" com critérios.

**Materialização**:

1. **Testes primeiro**:
   ```rust
   #[test]
   fn primes_single_glifo_2032() { ... }  // U+2032 prime ′
   #[test]
   fn primes_double_glifo_2033() { ... }  // U+2033 ″
   #[test]
   fn primes_triple_glifo_2034() { ... }  // U+2034 ‴
   #[test]
   fn primes_com_superscript() { ... }
   ```
   Executar — verificar falha.

2. **Implementação** em `attach.rs` arm `MathAttach.primes`:
   - count=1 → glifo `′` (U+2032) como superscript.
   - count=2 → `″` (U+2033).
   - count=3 → `‴` (U+2034).
   - count=4 → `⁗` (U+2057).
   - count>=5 → repetição de `′`.
   - Confirmar paridade com vanilla (verificar em
     `lab/typst-original/` antes de fixar comportamento).

3. Executar `cargo test math::` — verificar que passam.

4. Lint zero violations.

**Magnitude esperada**: S+ (~30-60 min; +3-5 tests).

#### Caso C.3 — Item 2 ou 4 confirmados parciais

Decisão caso a caso. Provavelmente classificar como "PARCIAL +
scope-out per ADR-0054 graded" preserva paridade observable
suficiente sem materialização adicional. Documentar em P255.D
no relatório final.

### Critério de aceitação P255.C

- Cada pendência real materializada respeita ordem
  testes-primeiro.
- Cada pendência tem prompt L0 actualizado **antes** do
  código.
- Hashes propagados.
- Tests workspace +N (consoante pendências resolvidas).
- Zero violations do linter.
- Paridade observable preservada (testes E2E não regridem).

---

## §4 — Sub-passo P255.D: Fecho DEBT-8 + relatório

**Objectivo**: actualizar `00_nucleo/DEBT.md` com o resultado
final e produzir relatório do passo.

### Acções obrigatórias

#### D.1 — Actualizar DEBT-8 em `00_nucleo/DEBT.md`

**Se cenário B1** (4/4 fechados em P255.A) ou
**B2 com todas as pendências materializadas em P255.C**:

```markdown
## DEBT-8 — Motor de equações — ENCERRADO (Passo 255) ✓

**Estado**: ENCERRADO em 2026-05-15.

[preservar histórico Passos 36-40]

**Resolvido pós-Passo 40** (auditoria P255.A revelou):
- Kern matemático: integrado em [hconcat/attach] em [passo X]
  ou neste passo (P255.C).
- OpenType MATH tables: consumidores em
  `01_core/src/rules/math/layout/stretchy.rs` e
  `assembly.rs` desde P96.8.
- MathPrimes layout: arm em `attach.rs` em [passo X] ou
  P255.C.
- Baseline x-height: `apply_axis_offset` usa
  `MathConstants.axis_height` desde [passo X].

Ver `00_nucleo/diagnosticos/diagnostico-math-fase-a-passo-255.md`
para evidência detalhada.
```

**Se cenário B2 com pendências adiadas** ou
**B3** (improvável):

Actualizar lista "Ainda pendente" com itens reais
restantes; secção "Resolvido pós-Passo 40" enumera o que caiu;
preservar status `PARCIALMENTE RESOLVIDO`.

#### D.2 — Relatório do passo

Criar
`00_nucleo/materialization/typst-passo-255-relatorio.md`
com seções:

- **§1 Sumário executivo** — cenário Fase A (B1/B2/B3); tests
  delta; ADRs tocadas (provavelmente zero); prompts L0
  actualizados (2).
- **§2 Sub-passo P255.A** — output Fase A resumido.
- **§3 Sub-passo P255.B** — prompts editados; hashes antes
  e depois.
- **§4 Sub-passo P255.C** — código materializado (se
  aplicável) com referências a ficheiros.
- **§5 Sub-passo P255.D** — DEBT-8 fechado ou actualizado.
- **§6 Padrões metodológicos** — ADR-0065 critério #5
  aplicado; subpadrão "auditoria condicional" cresce N=1→2.
- **§7 Limitações e trabalho futuro** — qualquer scope-out
  registado.

### Critério de aceitação P255.D

- DEBT-8 reflecte estado real pós-passo.
- Relatório criado em `00_nucleo/materialization/`.
- Cross-references entre relatório, diagnóstico Fase A, e
  DEBT-8 coerentes.

---

## §5 — Critério de aceitação global P255

Ao fim do passo, todos os seguintes têm de ser verdadeiros:

- [ ] `cargo run -p crystalline-lint -- .` retorna
  `✓ No violations found`.
- [ ] `cargo test` retorna contagem ≥ contagem inicial (sem
  regressão).
- [ ] `00_nucleo/diagnosticos/diagnostico-math-fase-a-passo-255.md`
  existe com tabela §2 preenchida.
- [ ] `00_nucleo/prompts/rules/math/layout.md` reflecte
  estado pós-P96.8 + descobertas Fase A.
- [ ] `00_nucleo/prompts/entities/math_constants.md` enumera
  todos os campos reais da struct.
- [ ] Hashes propagados (zero violations V5 PromptStale).
- [ ] DEBT-8 actualizado ou encerrado conforme cenário Fase A.
- [ ] Relatório do passo criado em
  `00_nucleo/materialization/`.
- [ ] Se cenário B2/B3 e materializaste código: cada
  pendência respeitou ordem testes-primeiro e teve L0
  actualizado antes do código.

---

## §6 — Sequência operacional condensada

Para Claude Code seguir linearmente:

1. **Ler** `CLAUDE.md`,
   `diagnostico-math-passo-254B.md`,
   `fase-a-checklist-math-passo-254B.md`.
2. **Reportar** estado inicial: tests count + lint clean
   (assumido baseline).
3. **P255.A** — executar 4 blocos de `grep`/`view`; criar
   diagnóstico Fase A imutável com decisão B1/B2/B3.
4. **P255.B** — editar 2 prompts L0; `--fix-hashes`; lint
   limpo; tests inalterados.
5. **P255.C condicional** — se B2/B3: para cada pendência,
   verificar/expandir L0 → testes → código → lint → tests.
6. **P255.D** — actualizar DEBT-8; criar relatório.
7. **Verificação final** — todo o checklist §5 satisfeito.
8. **Reportar** ao utilizador: cenário escolhido, tests
   delta, ficheiros criados/editados.

---

## §7 — Política de paragem

Claude Code **deve parar e perguntar ao utilizador** se:

- P255.A revela ambiguidade que não permite classificar
  alguma pendência (ex: hits em comentários `// TODO` mas
  sem evidência de implementação).
- P255.B descobre prompt L0 com critérios contraditórios face
  ao código real que não são meramente desactualizados (i.e.
  exigem decisão arquitectural nova).
- P255.C identifica que materialização de pendência exige
  ADR nova ou DEBT novo (excede scope deste passo).
- `crystalline-lint` reporta violations que não são triviais
  de resolver via `--fix-hashes`.
- Tests regridem sem causa óbvia.

Em qualquer paragem, registar contexto em comentário no
relatório parcial e aguardar instrução do utilizador.

---

## §8 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0033 — Paridade observable vanilla.
- ADR-0034 — Diagnóstico canónico.
- ADR-0054 — Perfil graded.
- ADR-0065 — Inventariar primeiro.
- DEBT-8 (`00_nucleo/DEBT.md`) — alvo deste passo.
- `00_nucleo/diagnosticos/diagnostico-math-passo-254B.md` —
  diagnóstico pai (planeamento das fases A/B).
- `00_nucleo/diagnosticos/fase-a-checklist-math-passo-254B.md`
  — comandos exactos a executar em P255.A.
- P96.8 — reestruturação `math/layout/` em 8 submódulos.
- P199B — `Content::SetEquationNumbering` materializado
  (precedente recente em série Math/Equation).
- P192A — precedente "auditoria condicional" / fecho
  retroactivo.
- P160 — precedente "diagnóstico com recomendação Fase 1".
