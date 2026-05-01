# Passo P180 — Inventário de bib state (lacuna #6)

Passo documental — sem código de produção tocado. Output:
diagnóstico `00_nucleo/diagnosticos/inventario-bib-state.md`
com análise factual da bibliografia em cristalino actual,
comparação com paridade vanilla, e recomendação para
implementação subsequente.

Padrão estabelecido por P167 (inventário de consumers
`CounterStateLegacy`). P180 segue mesma estrutura.

**Pré-condição**: P179 concluído. M9 9/11 features. Lacuna
#6 é única lacuna não resolvida onde infraestrutura
arquitectural está ausente.

**Restrições**:
- Sem código de produção tocado.
- Sem L0+L1 novos (ficheiros L1).
- Sem alteração de tipos existentes.
- Apenas trabalho documental.
- Output observable não muda.

---

## Sub-passos

### .A Inventário de bib state em `CounterStateLegacy`

1. **Localizar fields legacy**:
   - `01_core/src/entities/counter_state_legacy.rs`.
   - `grep -n "bib_entries\|bib_numbers" 01_core/src/`.
   - Confirmar tipos exactos:
     - `bib_entries: ?` — provavelmente `HashMap<Label, BibEntry>` ou `Vec<BibEntry>`.
     - `bib_numbers: ?` — provavelmente `HashMap<Label, usize>`.
   - Outros fields relacionados (lang, has_bibliography,
     etc.).

2. **Identificar populadores**:
   - `grep -n "bib_entries\." 01_core/src/`.
   - `grep -n "bib_numbers\." 01_core/src/`.
   - Em particular, walk arm em `rules/introspect.rs::walk`
     para `Content::Bibliography` ou `Content::Cite`.
   - Documentar:
     - Quem chama qual setter.
     - Que dados são extraídos do Content para popular.

3. **Identificar consumers**:
   - `grep -rn "\.bib_entries\|\.bib_numbers" 01_core/src/ 03_infra/src/`.
   - Categorizar por consumer:
     - Layouter (provavelmente — para renderizar bib).
     - `references.rs::layout_ref` (cite-arm — bloqueada
       per P167).
     - Outros.
   - Documentar exactamente que field é lido onde.

### .B Inventário de Content variants relacionados

1. **`Content::Bibliography`**:
   - Existe? `grep -n "Bibliography" 01_core/src/entities/content.rs`.
   - Campos: provavelmente `path`, `style`, `title`,
     `full`, etc.
   - Walk arm em layout: como renderiza?

2. **`Content::Cite`**:
   - Já é locatable (P162). Confirmar.
   - `ElementPayload::Citation` capturada (P162).
   - Walk arm popula `bib_numbers`? Provavelmente sim.

3. **Outros**: Hayagriva ou outra dependência externa?
   - `grep -rn "hayagriva\|biblatex\|csl" 01_core/Cargo.toml 01_core/src/`.

### .C Comparação com vanilla

1. **Identificar como typst-vanilla organiza bib state**:
   - `lab/typst-original/.../introspection/` — procura
     `BibIntrospection` ou similar.
   - `lab/typst-original/.../math/` ou `model/` —
     `Bibliography` element.
   - Documentar componentes vanilla:
     - Tipo de armazenamento.
     - Como queries `bib_*` são resolvidas.
     - Stdlib funcs (`bibliography(...)`, `cite(...)`).

2. **Comparar com cristalino**:
   - O que cristalino tem que vanilla também tem.
   - O que cristalino tem que vanilla não tem.
   - O que vanilla tem que cristalino não tem.

3. **Decidir alinhamento**:
   - Bib é Introspection-style (sub-store paralelo a
     Metadata) — consistente com desenho actual.
   - Ou bib é domínio próprio (lib externa, formatação
     complicada) — escapa ao espaço Introspector.

### .D Análise de magnitude

Com base em .A, .B, .C, estimar magnitude para
implementação:

1. **Pequena (S-M)** se:
   - Bib state é apenas "lista de Cite labels" + "tabela
     label→número".
   - Walk arm para Cite já popula `bib_numbers` — só
     falta migrar para sub-store.
   - Sem dependência externa.
   - **Implicação**: P181 pode ser implementação directa.

2. **Média (M-L)** se:
   - Bib state requer parsing de ficheiros (`.bib`).
   - Stdlib `bibliography(...)` carrega ficheiro externo.
   - Hayagriva ou similar como dependência.
   - **Implicação**: P181 = inventário detalhado de
     parsing; P182 = implementação.

3. **Grande (L-XL)** se:
   - Bib é domínio próprio com formatação CSL, idioma,
     etc.
   - Múltiplos formatos de citação.
   - **Implicação**: feature grande; pode justificar
     decompor em vários passos com objectivos cumulativos.

### .E Recomendação

Output final em
`00_nucleo/diagnosticos/inventario-bib-state.md`:

```markdown
# Inventário Bib State — Lacuna #6

## Componentes em cristalino
- bib_entries: <tipo>
- bib_numbers: <tipo>
- Content::Bibliography: <campos>
- Walk arm: <comportamento>

## Consumers
| Consumer | Field lido | Localização |
|----------|------------|-------------|
| ... | ... | ... |

## Comparação com vanilla
| Componente | Vanilla | Cristalino | Diferença |
|------------|---------|------------|-----------|
| ... | ... | ... | ... |

## Magnitude estimada
**S-M / M-L / L-XL** com justificação.

## Recomendação para implementação
- Caminho A: implementação directa em P181 (S-M).
- Caminho B: inventário detalhado P181 + implementação P182 (M-L).
- Caminho C: decomposição em N passos (L-XL).

## Decisões a tomar antes de P181
- Lista de decisões pendentes.
```

### .F Verificação estrutural

1. `cargo check --workspace` passa (não tocámos código).
2. `cargo test --workspace` passa sem mudança de
   contagem.
3. `crystalline-lint`: zero violations (não modificámos
   L0/L1).
4. Diagnóstico `00_nucleo/diagnosticos/inventario-bib-state.md`
   existe com 5 secções (componentes, consumers,
   comparação vanilla, magnitude, recomendação).
5. Nenhum L0 modificado.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-180-relatorio.md` com:

- Resumo: inventário de bib state feito; magnitude
  estimada; recomendação para P181+.
- Confirmação de cada verificação .F.
- Resumo numérico do inventário:
  - Número de fields legacy de bib.
  - Número de consumers.
  - Magnitude estimada com justificação.
- Decisão recomendada para P181:
  - Implementação directa, ou
  - Inventário detalhado com pré-implementação, ou
  - Decomposição em N passos.
- Estado pós-passo: P180 concluído. P181 desbloqueado.
- Pendências cumulativas (sem alteração — P180 documenta
  apenas).

---

## Critério de conclusão

Todas em conjunto:

1. .A: inventário literal de fields bib em
   `CounterStateLegacy`.
2. .B: inventário de Content variants relacionados.
3. .C: comparação com vanilla.
4. .D: análise de magnitude.
5. .E: recomendação escrita em diagnóstico.
6. .F: verificações 1-5 passam.
7. .G: relatório escrito.
8. Sem código de produção tocado.
9. Sem L0+L1 novos.

---

## O que pode sair errado

- **Bib state mais ramificado do que esperado**: pode
  haver múltiplos sub-sistemas (parsing, formatting,
  output). Documentar exhaustivamente.
- **Vanilla typst usa lib externa (hayagriva)**:
  cristalino pode ou não. Se sim, magnitude L-XL
  provável. Se não, magnitude menor.
- **`Content::Bibliography` ausente em cristalino**:
  improvável, mas possível. Verificar.
- **`bib_numbers` lookup é mais complexo que cite-by-label**:
  pode envolver ordering, grouping, etc. Documentar.
- **Bib state tem dependências em runtime data**: e.g.
  ficheiros `.bib` no filesystem, lookup de network,
  etc. Cristalino isolation level pode rejeitar essa
  forma. Decidir.
- **Hayagriva ou similar não está em
  `[l1_allowed_external]`**: dependência externa precisa
  ser permitida. Documentar gate.

---

## Notas operacionais

- **Tamanho**: S. Trabalho documental puro. Sem código.
- **Output principal**: diagnóstico
  `inventario-bib-state.md`. Será referência para P181+.
- **Padrão P167**: P180 segue mesma estrutura. P167
  produziu `inventario-consumers-counter-state-legacy.md`;
  P180 produz equivalente para bib.
- **Cláusula gate trivial**: aplicável a decisões
  documentais (formato exacto do diagnóstico,
  granularidade da comparação vanilla).
- **Risco de descoberta tarde**: P180 é onde podemos
  saber se lacuna #6 é Introspection-compatible ou
  domínio próprio. Preferível descobrir agora que
  começar implementação e descobrir incompatibilidade
  arquitectural a meio.
- **`m1-lacunas-captura.md` actualização**: lacuna #6
  pode ganhar nota "P180 inventário concluído;
  recomendação registada".
