# Passo 151 — Fecho de DEBT-53 (vanilla integration em `lab/parity`)

**Série**: 151 (passo **substantivo** em `lab/parity/`;
desbloqueia matriz P3 real e prepara P151+/P152+ subsequentes).
**Precondição**: Passo 150 encerrado; infraestrutura `FrameDTO`
+ `report.rs` + `tests/layout_parity.rs` + corpus visual
materializada; matriz baseline com 19/19 compila em cristalino,
colunas materiais `N/A`; DEBT-53 aberto (Passo 150);
`from_vanilla_stub()` devolve DTO vazio.

**Numeração**: 151 era reservado para P2 (`value_dto.rs`)
segundo §9 dos documentos de paridade. Reformulação **4 da
série paridade**: P2 fica para 152 quando vanilla integration
estiver concluída — mesmo bloqueio reapareceria. **Coerente
com recomendação humana** + lógica de prioridade.

**Natureza**: passo **substantivo** em `lab/parity/`. Toca:
- `lab/parity/Cargo.toml`: dep nova `typst` (vanilla
  compilador inteiro) via path-dep para
  `lab/typst-original/crates/typst`.
- `lab/parity/src/world_adapter.rs` (novo): `World` trait
  adapter ou setup duplo.
- `lab/parity/src/frame_dto.rs`: `from_vanilla_stub` →
  `from_vanilla` real (consome
  `typst_layout::PagedDocument`).
- `lab/parity/tests/layout_parity.rs`: chama vanilla além
  de cristalino; popula matriz com números reais.
- `lab/parity/reports/latest.md` + nova entrada em
  `history/`: matriz com números reais.

**Cristalino L1/L2/L3/L4**: **intactos**. `lab/parity/`
continua fora do workspace cristalino (per Passo 9).

**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional) — operacionalização da
  comparação observacional.
- **ADR-0054** (perfil observacional graded) — define o que
  conta como "passa" em P3.
- **ADR-0034** (diagnóstico obrigatório) — espírito cumprido
  por inventário 148 + arqueologia 149 + baseline 150.

---

## Contexto

Passo 150 entregou infraestrutura de medição mas detectou
**pré-condição não satisfeita** durante materialização:
`lab/parity/Cargo.toml` só importava `typst-syntax`
(sub-crate de parse). Adicionar `typst` (vanilla compilador
inteiro) + setup de `World` adapter excedeu escopo prático
do passo. Decisão correcta: parar, materializar
infraestrutura, abrir **DEBT-53** ("Integração de pipeline
vanilla em `lab/parity` para medição P3").

Sub-passo 150.2.4 do relatório 150 detalha 3 obstáculos
concretos:

1. **Vanilla `World` trait diverge de cristalino**:
   `typst_library::World` (vanilla) tem assinaturas e
   semântica diferentes de
   `typst_core::contracts::world::World` (cristalino). Setup
   duplo é mandatório — não há reuso directo.
2. **Setup de fonts diverge**: vanilla espera fonts
   embebidas via crate dedicada ou descobertas no sistema;
   cristalino usa `SystemWorld::with_fonts(paths)`.
3. **`from_vanilla` requer inspecção da estrutura interna
   do `typst_layout::PagedDocument`**: forma diverge do
   cristalino (mesmo nome de tipo, tipos internos
   diferentes).

Este passo endereça os 3 obstáculos numa unidade. Resultado
esperado: matriz P3 com **números reais** nas colunas
`text_content` e `structural`; números brutos em
`geometric` (experimental).

---

## Objectivo

Ao fim do passo:

1. **`lab/parity/Cargo.toml` actualizado**:
   - Path-dep para `typst` (vanilla compilador) referenciando
     `lab/typst-original/crates/typst`.
   - Dependências auxiliares se necessário (ex:
     `typst-library`, `typst-syntax` versão vanilla,
     `typst-layout`).
   - Confirmação que `typst-core` cristalino e `typst`
     vanilla coexistem sem conflito (nomes de crate
     distintos).

2. **`lab/parity/src/world_adapter.rs` materializado**:
   - Função ou struct que constrói **dois** worlds equivalentes
     (cristalino + vanilla) a partir do mesmo source `.typ`.
   - Adapter cobre: source loading, font discovery, package
     resolution stub.
   - Setup paralelo: ambos os worlds vêem o mesmo source +
     mesmas fonts (provavelmente via fonts embebidas no test
     harness, não system fonts — para reprodutibilidade CI
     futura).

3. **`from_vanilla` real**:
   - Substitui `from_vanilla_stub()` em `frame_dto.rs`.
   - Consome `&typst_layout::PagedDocument` real.
   - Mapping `vanilla::FrameItem` → `ItemDTO` (cristalino):
     - `Vanilla::Text` → `ItemDTO::Text`.
     - `Vanilla::Group` → `ItemDTO::Group`.
     - Variants vanilla sem equivalente cristalino →
       `ItemDTO::Other("vanilla_variant_name")`.
     - Variants cristalinos sem equivalente vanilla → não
       aparecem em `from_vanilla` (impossível — vanilla é
       quem está a ser consumido).

4. **`tests/layout_parity.rs` actualizado**:
   - Para cada ficheiro do corpus, corre **ambas** as
     pipelines (cristalino + vanilla).
   - Para cada par, faz `compare(&crist_dto, &vanilla_dto,
     tolerance)`.
   - Popula matriz com `text_content_passed`,
     `structural_passed`, `geometric_*` reais.

5. **Matriz com números reais**:
   - `lab/parity/reports/latest.md` actualizado.
   - Cópia em `lab/parity/reports/history/2026-04-25-passo-151.md`.
   - Cópia anterior `2026-04-25-passo-150.md` **preservada**
     (histórico imutável).

6. **Avaliação empírica do output**:
   - Se `text_content` < 50%: investigar ANTES de fechar
     DEBT-53. Pode ser bug do harness, divergência
     fundamental, ou ambos. Registar no relatório.
   - Se `text_content` ≥ 50% e `structural` razoável: fechar
     DEBT-53.
   - Se `geometric` produz números brutos coerentes (não
     `+inf` ou `NaN`): aceitar para calibração futura.

7. **DEBT-53 movido para Secção 2 (encerrado)** ou
   actualizado in-place consoante avaliação:
   - Se matriz tem números reais: encerrar com referência
     ao relatório 151.
   - Se matriz tem números mas avaliação revela bugs:
     manter aberto; relatório regista trabalho futuro.

8. **Documentos de paridade actualizados**:
   - `typst-paridade-plano-medicao.md` §9: Passo 151 muda de
     "P2" para "fecho DEBT-53"; P2 passa a 152; restantes
     renumerados.

9. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-151-relatorio.md`.

Este passo **não**:

- Toca código em L1/L2/L3/L4 cristalino.
- Implementa P2 (`value_dto.rs`) ou P4 (`pdf_compare.rs`).
- Importa corpus oficial vanilla (`lab/typst-original/tests/`).
- Calibra tolerâncias por ficheiro.
- Cria ADRs novas.
- Materializa Opção A de P4 (visual via `pdftoppm`).
- Resolve a divergência arquitectural FixedMetrics vs
  FontBookMetrics — esta é estrutural e fica reflectida em
  `geometric` experimental.

---

## Decisões já tomadas

1. **Setup duplo** (cristalino World + vanilla World) — não
   há reuso directo possível.
2. **Fonts embebidas no test harness** — para
   reprodutibilidade CI futura. Sem dependência de system
   fonts (DejaVu/Liberation podem variar entre máquinas).
3. **`geometric` continua experimental** — divergência
   FixedMetrics/FontBookMetrics não é objectivo deste passo
   resolver.
4. **Matriz é resposta substantiva**: o passo só fecha quando
   a matriz tem números reais nas colunas materiais.
5. **DEBT-53 fecha por defeito**, excepto se avaliação
   empírica revelar problema.

## Decisões diferidas (resolvidas neste passo)

6. **Versão vanilla a importar**: depende do estado de
   `lab/typst-original/` (commit
   `ba61529986e0a5a916cbf937c3c65117cd450683` per inventário
   148). Confirmar em 151.1 que é versão estável + crates
   `typst-layout` + `typst-library` + `typst-syntax` +
   `typst` exportam o necessário.

7. **Estrutura do `world_adapter.rs`**:
   - Opção A: dois adapters separados (`CristalinoWorld`,
     `VanillaWorld`) — mais explícito.
   - Opção B: trait genérico abstraindo over the two — mais
     elegante mas talvez supérfluo dado que tipos são
     concretos e diferentes.
   - **Default A**: explicitação supera abstração para infra
     de medição.

8. **Fonts a embebir**: fonts livres + cobertura ASCII/Latin
   suficiente para o corpus actual (markup PT/EN; math
   básico). Candidatos:
   - DejaVu Sans (livre; cobre Latin extended).
   - DejaVu Serif Math.
   - Inria Serif (multi-font test).
   Decisão final em 151.1 conforme disponibilidade real.

9. **Sample budget para `geometric`**: documento `definicoes.md`
   diz `tolerance_pt = 5.0` default. Para experimental, sample
   _all_ positions ou subsample? Decisão default: **all** —
   harness é offline; sample completo é tractable para 19
   ficheiros.

10. **Tratamento de divergências em `ItemDTO`**: vanilla pode
    ter variants sem equivalente cristalino (ex: `FrameItem::Path`
    se existir em vanilla, ausente em cristalino). Mapeadas
    para `ItemDTO::Other("path")`. Tabela de mapeamento
    documentada no relatório.

11. **Falha no setup**: se vanilla não compila / setup não
    funciona / fonts embebidas falham, **pausar** e abrir
    sub-DEBT (`DEBT-54`). DEBT-53 permanece aberto. Decisão
    de continuar com cristalino-only matriz não é tomada
    aqui.

---

## Escopo

**Dentro**:

- `lab/parity/Cargo.toml`: deps vanilla.
- `lab/parity/src/world_adapter.rs`: novo.
- `lab/parity/src/frame_dto.rs`: `from_vanilla` real.
- `lab/parity/tests/layout_parity.rs`: dual pipeline.
- Fonts embebidas em `lab/parity/fonts/` ou `lab/parity/tests/fonts/`
  consoante convenção.
- `lab/parity/reports/latest.md` actualizado.
- `lab/parity/reports/history/2026-04-25-passo-151.md` novo.
- `00_nucleo/DEBT.md`: DEBT-53 fechado (ou actualizado).
- `00_nucleo/diagnosticos/typst-paridade-plano-medicao.md`
  §9 renumerado.
- Relatório do passo.

**Fora**:

- L1/L2/L3/L4 cristalino.
- ADRs (sem ADR nova).
- Outros DEBTs.
- README dos ADRs (sem mudança — sem ADR nova).
- Corpus oficial vanilla.
- `value_dto.rs`, `pdf_compare.rs`, tests P2/P4.
- Calibração de tolerâncias por ficheiro.
- Promoção de `geometric` para `production`.
- Importação de corpus oficial.
- CI integration ou GitHub Actions.

---

## Sub-passos

### 151.1 — Inventário pré-materialização

**A.1.1 — Estado de `lab/typst-original/crates/`**:

```bash
ls lab/typst-original/crates/
ls lab/typst-original/crates/typst/Cargo.toml 2>/dev/null
ls lab/typst-original/crates/typst-layout/Cargo.toml 2>/dev/null
ls lab/typst-original/crates/typst-library/Cargo.toml 2>/dev/null
ls lab/typst-original/crates/typst-syntax/Cargo.toml 2>/dev/null
```

Confirmar nomes de crate exactos e estrutura.

**A.1.2 — API pública vanilla**:

```bash
grep -nE "^pub use|^pub fn compile|^pub struct (Library|World|PagedDocument)" \
  lab/typst-original/crates/typst/src/lib.rs
grep -nE "^pub use|^pub struct PagedDocument" \
  lab/typst-original/crates/typst-layout/src/lib.rs
grep -nE "^pub use|^pub trait World" \
  lab/typst-original/crates/typst-library/src/world.rs 2>/dev/null
```

Registar:
- Função top-level de compilação vanilla (provavelmente
  `typst::compile(world) -> SourceResult<PagedDocument>` ou similar).
- Trait `World` vanilla — assinatura exacta (`source`, `font`,
  `book`, `today`).
- Forma de `PagedDocument` vanilla.

**A.1.3 — Conflito de nomes**:

`typst-core` (cristalino) e `typst` (vanilla) são nomes de
crate diferentes — sem conflito directo. Mas ambos podem
expor `World`, `PagedDocument`, `FrameItem`. Fully-qualified
paths obrigatórios em `frame_dto.rs` e `world_adapter.rs`.

**A.1.4 — Fonts disponíveis**:

```bash
find lab/typst-original/ -name "*.ttf" -o -name "*.otf" 2>/dev/null | head
find lab/parity/ -name "*.ttf" -o -name "*.otf" 2>/dev/null
```

Se vanilla repo tem fonts embebidas em `lab/typst-original/assets/`
ou similar, reusar. Senão, descarregar fonts livres
(DejaVu) ou usar fonts já em `01_core` se houver fixture.

### 151.2 — `Cargo.toml` actualizado

```diff
[dependencies]
typst-syntax = { path = "../typst-original/crates/typst-syntax" }
typst-core   = { path = "../../01_core" }
+ typst        = { path = "../typst-original/crates/typst" }
+ typst-layout = { path = "../typst-original/crates/typst-layout" }
+ typst-library = { path = "../typst-original/crates/typst-library" }
+ # outros sub-crates conforme necessário (typst-eval? typst-utils?)
```

Confirmar `cargo build` em `lab/parity/` antes de avançar.

### 151.3 — `world_adapter.rs`

**Estrutura proposta**:

```rust
//! Setup duplo: cristalino + vanilla worlds para o mesmo source.

pub struct ParityWorlds {
    pub cristalino: typst_core::SystemWorld,
    pub vanilla:    VanillaWorldImpl,
}

pub struct VanillaWorldImpl {
    main_source: typst_syntax::Source,
    library:     typst_library::Library,
    book:        typst_library::FontBook,
    fonts:       Vec<typst_library::Font>,
}

impl typst_library::World for VanillaWorldImpl {
    fn library(&self) -> ...;
    fn book(&self) -> ...;
    fn main(&self) -> ...;
    fn source(&self, id: FileId) -> ...;
    fn file(&self, id: FileId) -> ...;
    fn font(&self, index: usize) -> ...;
    fn today(&self, offset: ...) -> ...;
}

pub fn build_worlds(source: &str, fonts: &[Vec<u8>]) -> ParityWorlds;
```

Notas:
- `VanillaWorldImpl` implementa `typst_library::World`
  (vanilla) — assinaturas confirmadas em 151.1.A.2.
- `build_worlds` recebe source + fonts comuns; constrói os
  dois worlds com setup paralelo.
- Erros de setup (font invalid, source parse error) →
  panic com mensagem clara (testes de paridade são offline;
  panic é informação útil).

### 151.4 — `from_vanilla` real

```diff
- pub fn from_vanilla_stub() -> Self {
-     FrameDTO { pages: vec![] }
- }
+ pub fn from_vanilla(doc: &typst_layout::PagedDocument) -> Self {
+     let pages = doc.pages.iter().map(|p| PageDTO {
+         text:           extract_text_vanilla(&p.frame),
+         items:          extract_items_vanilla(&p.frame),
+         item_positions: extract_positions_vanilla(&p.frame),
+         width:          p.frame.size().x.to_pt(),
+         height:         p.frame.size().y.to_pt(),
+     }).collect();
+     FrameDTO { pages }
+ }
```

Helpers `extract_*_vanilla` consomem `&typst_layout::Frame`
(ou equivalente) e produzem os campos do DTO. Mapeamento de
`FrameItem` vanilla → `ItemDTO` cristalino fica documentado
no relatório (Tabela em 151.10).

### 151.5 — `tests/layout_parity.rs` actualizado

```diff
  for entry in corpus {
-     let crist_doc = typst_core::compile(&entry.source);
-     let crist_dto = FrameDTO::from_cristalino(&crist_doc);
-     // vanilla integration: DEBT-53
-     let vanilla_dto = FrameDTO::from_vanilla_stub();
+     let worlds = world_adapter::build_worlds(&entry.source, FONTS_EMBEDDED);
+     let crist_doc = typst_core::compile(&worlds.cristalino);
+     let vanilla_doc = typst::compile(&worlds.vanilla);
+     let crist_dto = FrameDTO::from_cristalino(&crist_doc);
+     let vanilla_dto = FrameDTO::from_vanilla(&vanilla_doc);
      let results = crist_dto.compare(&vanilla_dto, entry.tolerance);
      matrix_builder.record(entry.category, results);
  }
```

`FONTS_EMBEDDED` é constante definida em 151.3 com bytes das
fonts embebidas (via `include_bytes!`).

**Sem `assert!` global**: mantém-se conforme P150. Se um
ficheiro falha compilação numa das pipelines, regista-se na
matriz como linha extra (`failed_to_run_*`).

### 151.6 — Geração da matriz com números reais

`cargo test --test layout_parity` produz `latest.md` com
preenchimento real:

```markdown
| Categoria | Total | Compila (crist) | Compila (vanilla) | text_content | structural | geometric (experimental) |
|-----------|------:|----------------:|------------------:|-------------:|-----------:|:------------------------:|
| code      |     2 |             2/2 |               2/2 |          P/2 |        Q/2 | dx=Xpt; dy=Ypt           |
| markup    |     6 |             6/6 |               6/6 |          M/6 |        N/6 | dx=Xpt; dy=Ypt           |
| ...
```

Interpretação:
- "Compila" colunas: ambas devem ser 19/19 (corpus filtrado
  por features `implementado⁺`+ no inventário 148 — todas
  as features devem compilar em vanilla por construção).
- `text_content`: número absoluto de ficheiros onde texto
  extraído bate exactamente.
- `structural`: número onde structure (counts por tipo de
  item) bate.
- `geometric`: números brutos (max_dx, max_dy, mean_dx,
  mean_dy) por categoria.

### 151.7 — Avaliação empírica + decisão sobre DEBT-53

**Cenários esperados**:

| text_content | structural | Decisão |
|--------------|------------|---------|
| ≥ 80% | ≥ 50% | **fechar DEBT-53**; matriz é resposta substantiva |
| ≥ 50% e < 80% | qualquer | **fechar DEBT-53** mas registar no relatório que cobertura observacional está abaixo de cobertura declarada |
| < 50% | qualquer | **investigar**: harness bug? divergência fundamental? Pode exigir 151b ou novo DEBT |

Se `geometric` produz `NaN` ou `+inf`: sinal de bug no
harness ou em `extract_positions_vanilla`. Investigar antes
de fechar.

### 151.8 — Actualizar DEBT-53

Cenário cumulativo (fechar):

```diff
## Secção 1 — DEBTs em aberto
- 11 DEBTs abertos
+ 10 DEBTs abertos

## Secção 2 — DEBTs encerrados
+ ## DEBT-53 — Integração de pipeline vanilla em `lab/parity` — ENCERRADO (Passo 151) ✓
+ Aberto em: Passo 150.
+ Fechado em: Passo 151 (2026-04-25).
+ Resolução: `world_adapter` materializado; `from_vanilla` real;
+ matriz P3 com números reais (text_content X%, structural Y%,
+ geometric experimental).
+ Critério de fecho cumprido: matriz substitui `N/A`.
```

Cenário não-cumulativo (manter aberto): adicionar
"Actualização Passo 151" com bug encontrado + plano.

### 151.9 — Actualizar documentos de paridade

`typst-paridade-plano-medicao.md` §9:

```diff
  ## 9 — Próximas acções concretas
  
  1. **Passo 148** — Inventário ...
  2. **Passo 149** — Arqueologia ...
  3. **Passo 150** — `frame_dto.rs` baseline ...
- 4. **Passo 151+** — Implementar P2 (`value_dto.rs`) ...
- 5. **Passo 152+** — Implementar P4 (`pdf_compare.rs`) ...
+ 4. **Passo 151** — Fecho de DEBT-53 (vanilla integration);
+    matriz P3 com números reais.
+ 5. **Passo 152+** — Implementar P2 (`value_dto.rs`) ...
+ 6. **Passo 153+** — Implementar P4 (`pdf_compare.rs`) ...
```

### 151.10 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-151-relatorio.md`.

Secções:
1. Sumário executivo (estatística agregada).
2. Inventário pré-materialização (resultado de 151.1).
3. `Cargo.toml` — diff dos deps adicionados.
4. `world_adapter.rs` — estrutura final + mapping
   trait→trait.
5. `from_vanilla` — assinatura final + tabela de mapeamento
   `FrameItem` vanilla → `ItemDTO` cristalino.
6. Tests actualizados — estratégia + resultados (compila
   crist/vanilla; text_content; structural; geometric).
7. Matriz primeira-com-números (cópia integral de
   `latest.md`).
8. Avaliação empírica (151.7) — decisão sobre DEBT-53.
9. DEBT-53 actualizado (fechado ou actualizado).
10. Documentos de paridade actualizados.
11. Próximo passo: 152 (P2) ou outro consoante prioridade.
12. Verificação final.

---

## Verificação

1. ✅ `lab/parity/Cargo.toml` com deps vanilla.
2. ✅ `lab/parity/src/world_adapter.rs` materializado.
3. ✅ `from_vanilla` real em `frame_dto.rs` (substitui stub).
4. ✅ `tests/layout_parity.rs` corre dual pipeline.
5. ✅ Matriz `latest.md` com números reais (não `N/A`).
6. ✅ Cópia em `history/2026-04-25-passo-151.md`.
7. ✅ Cópia 150 preservada (`history/2026-04-25-passo-150.md`).
8. ✅ DEBT-53 actualizado (fechado ou nota se mantido).
9. ✅ §9 dos documentos de paridade renumerado.
10. ✅ Nenhum ficheiro tocado em L1/L2/L3/L4 cristalino.
11. ✅ Nenhuma ADR criada.
12. ✅ `crystalline-lint .` zero violations.
13. ✅ `cargo test --workspace --lib`: 1113 inalterado.
14. ✅ `cd lab/parity && cargo test --test layout_parity`
    corre limpo (sem panic do harness).
15. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Matriz P3 tem números reais nas colunas `text_content` e
   `structural`.
2. `geometric` tem números brutos coerentes (não NaN/inf).
3. DEBT-53 fechado **ou** actualizado com plano se
   avaliação revelar bug.
4. Setup vanilla reproduzível (fonts embebidas; sem
   dependência system fonts).
5. Documentos de paridade actualizados.
6. Próximo passo (152 = P2) tem âncora.
7. Sem código tocado em L1/L2/L3/L4 cristalino.
8. Relatório do passo escrito.

---

## O que pode sair errado

- **Vanilla não compila**: `lab/typst-original/` no commit
  `ba61529986e0a5a916cbf937c3c65117cd450683` pode ter deps
  externas que cargo não resolve, ou versão de Rust
  incompatível. Confirmar `cargo build` em
  `lab/typst-original/` antes de adicionar como dep em
  `lab/parity/`. Se não compila: pausar, abrir DEBT-54
  ("Vanilla repo broken at frozen commit"), reverter Cargo
  ao stub.

- **Conflito de versão de deps transitivas**: vanilla puxa
  `ttf-parser` v0.X; cristalino puxa v0.Y. Cargo resolve
  mas pode produzir warnings ou incompatibilidades subtis.
  Investigar; pode exigir feature flags.

- **Setup de fonts embebidas falha**: bytes inválidos,
  formato não suportado por uma das pipelines, ou tabelas
  faltam. Substituir por fonts diferentes; documentar
  selecção.

- **`World` trait vanilla mudou** desde último contacto:
  método novo ou removido. Ler trait actualmente em
  `lab/typst-original/crates/typst-library/src/world.rs`;
  ajustar `VanillaWorldImpl` conforme.

- **`from_vanilla` produz `FrameDTO` malformed**: ex,
  positions com NaN ou texts vazios quando deveriam ter
  conteúdo. Bug no harness; investigar antes de gerar
  matriz. Se não resolúvel rapidamente: pausar e abrir
  sub-DEBT.

- **`text_content` baixo (< 50%)**: três hipóteses:
  1. Bug no `extract_text_*`: comparar manualmente um
     ficheiro simples (ex: `markup/hello.typ`) entre as
     duas pipelines.
  2. Divergência fundamental cristalino vs vanilla: ex,
     cristalino emite `\u{00A0}` (NBSP) onde vanilla emite
     space normal — diferença textual mínima mas detectada
     como divergência.
  3. Corpus filtrado mal: ficheiros usam features `parcial`
     que vanilla expressa diferentemente.
  Decisão: investigar; se for (1) corrigir e re-correr; se
  (2) ou (3), documentar e aceitar abaixo de threshold com
  nota.

- **`structural` muito baixo**: provavelmente mapping
  `ItemDTO` está a colapsar variants vanilla
  inadequadamente. Refinar mapping; documentar tabela.

- **`geometric` produz números muito altos** (esperado):
  FixedMetrics vs FontBookMetrics. Aceitar; é precisamente o
  que `experimental` exprime.

- **`cargo test --test layout_parity` corre durante
  minutos**: vanilla compilação inteira é lenta. Se
  excessivo, aceitar; harness é offline. Não vale a pena
  optimizar nesta iteração.

- **Tamanho do `Cargo.lock`**: adicionar vanilla puxa muitas
  deps; `Cargo.lock` cresce. Aceitar.

- **`lab/parity/` deixa de "ser fora do workspace
  cristalino"**: adicionar `typst` (vanilla) não muda isto.
  `lab/parity/` continua workspace separado; cristalino
  workspace continua a não ver vanilla. Confirmação por
  `cargo build` no workspace cristalino — deve continuar a
  funcionar igual.

- **Aberto sub-DEBT em vez de fechar DEBT-53**: aceitável.
  Se setup é tractable mas avaliação revela bug não-trivial
  no comparator ou no extract, abrir DEBT-54 com plano
  específico; DEBT-53 pode ainda fechar ("setup
  materializado") ou ficar aberto consoante natureza do
  bug.

---

## Notas operacionais

- **Reformulação 4 da série paridade**: 148 inventário; 149
  arqueologia; 150 baseline; **151 fecho DEBT-53**. P2 e
  P4 deslocam-se para 152+/153+. Padrão de reformulação por
  descoberta confirma-se.

- **Modelo: passo de fecho de DEBT específico**, análogo a
  Passo 92 (fecho DEBT-44) ou Passo 95 (fecho DEBT-39).
  Diferença material: DEBT-53 tem critério explícito ("matriz
  com números reais") — fecho mecânico se critério é
  cumprido.

- **Setup duplo é overhead aceite**: alternativa seria mock
  do vanilla pipeline (impossível — semântica diverge);
  alternativa seria comparar contra screenshots PNG (Opção A
  do P4 — exige `pdftoppm`/`mupdf`; fora deste passo).

- **Fonts embebidas vs system fonts**: decisão por
  reprodutibilidade. Se CI futuro for considerado (per §8 dos
  documentos de paridade — "decidir quando estiver
  implementado"), fonts embebidas são pré-requisito. Custo
  agora paga-se uma vez.

- **Resposta substantiva à pergunta original** acontece
  neste passo. Inventário 148 deu cobertura declarada
  (54%/72%); P150 deu estrutura; **P151 dá os números
  observacionais**. Só após P151 é que o utilizador pode
  comparar "afirmamos cobrir 54%; medimos X%
  observacionalmente".

- **Próximas reformulações antecipáveis**:
  - P152 (P2 = `value_dto.rs`): **`Value` em vanilla tem 30
    variants vs cristalino 18**. Mapeamento exige decisão
    análoga a P149 — pode arrastar arqueologia. Antecipar.
  - P153 (P4 = `pdf_compare.rs`): Opção B textual primeiro;
    Opção A visual exige `pdftoppm`/`mupdf` em CI. Fora do
    escopo de P153.

- **Matriz pode revelar surpresas**:
  1. Cobertura observacional **maior** que declarada
     (aceitável; cobertura declarada é conservadora).
  2. Cobertura observacional **menor** que declarada
     (informação valiosa; investigar via novo passo).
  3. Cobertura observacional **igual** à declarada
     (validação directa do inventário 148).

  Qualquer caso é aceite; o resultado em si é informação.

- **Pós-151**: utilizador tem **resposta substantiva** à
  pergunta "em que paridade estamos?". Resposta tem 3
  dimensões: cobertura declarada (148), classificação
  factual (148+149), paridade observacional (151). Matriz
  combina as 3.
