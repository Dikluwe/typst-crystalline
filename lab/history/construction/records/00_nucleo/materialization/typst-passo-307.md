# typst-passo-307 — atomização L3: decomposição de `export.rs`

**Tipo**: Passo de Execução (planeamento táctico)
**Data**: 2026-05-19
**Magnitude**: M+ a L (série de sub-passos)
**Pré-requisitos**: decisões P307a + P307b antes de qualquer L1/L3 código
**Bloqueio actual**: ADR-0098 SSoT bit-exact (23 passos consecutivos P282-P306)

---

## 1. Motivação

Auditoria empírica de 2026-05-19 identificou desproporção
qualitativa em L3:

> `03_infra/src/export.rs`: **9.856 LOC num único ficheiro**.
> Vanilla `typst-pdf`: 7.559 LOC distribuído em ~30 ficheiros
> temáticos. Sinal de não-decomposição L3 — viola o espírito
> da arquitectura cristalina dentro da própria L3.

DEBT-46 (ADR-0037) fechou decomposição de L1 mas declarou
explicitamente "se [outras camadas] excederem o limite, abrir
DEBT específico por camada". Esse DEBT nunca foi aberto para L3.

P306 fechou um cluster aditivo trivial (calc); P307 fica
disponível para trabalho estrutural maior.

---

## 2. Objectivo

Decompor `03_infra/src/export.rs` (9.856 LOC) em submódulos
coesos por domínio dentro de `03_infra/src/export/`, mantendo:

- Output binário PDF bit-exact (paridade ADR-0033 reforçada por
  validação de bytes, não só de hash do ficheiro `.rs`).
- API pública inalterada (`export_pdf`, `export_pdf_with_font`,
  `export_pdf_multifont`).
- Zero novas dependências externas (decomposição é mecânica;
  ADRs de autorização ficam fora do escopo P307).

A análise de "organização de dependências" surge naturalmente
**depois** da decomposição: cada submódulo declara o que
realmente usa, podendo justificar adicionar deps focadas em vez
de uma dep monolítica usada em ≤1 sítio (ex: `flate2` para
compressão de streams, `zopfli` para optimização, etc., se
materializadas no futuro).

---

## 3. Tensão arquitectural — ADR-0098

`export.rs` está preservado **bit-exact desde P282** (hash
`66cb8ac3` ao fim de P306 — 23 passos consecutivos). Esta
preservação é registada em cada relatório como invariante
metodológico explícito.

**Decomposição quebra este invariante por definição**: novos
ficheiros, imports redistribuídos, hashes diferentes.

**Resolução proposta**: o invariante é instrumental, não
terminal. Existia para evitar drift acidental durante refactors
L1. P307 introduz uma forma mais forte do invariante:

> **Invariante reforçado**: para um conjunto de inputs
> canónicos (ficheiros `.typ` em corpus), o output binário PDF
> de cada um permanece bit-exact pós-decomposição.

O ficheiro fonte muda; o PDF gerado não. Esta é paridade
observable verdadeira, não paridade textual ao código fonte.

---

## 4. Estrutura proposta — referência vanilla

Vanilla `typst-pdf` divide ~7.559 LOC em ~30 ficheiros temáticos.
Cristalino pode replicar a divisão por domínio sem replicar a
contagem (vanilla tem features que cristalino não cobre).

**Esboço inicial** (a confirmar em P307a sub-passo de inventário):

```
03_infra/src/export/
    mod.rs              # API pública (export_pdf, _with_font, _multifont)
                        # + dispatch entre os 3 caminhos
    pdf/
        objects.rs      # Object IDs, xref, trailer, header
        catalog.rs      # /Catalog, /Pages root
        page.rs         # /Page dict, /MediaBox, /Contents
        stream.rs       # build_page_stream_type1, _cidfont, _multifont
        escape.rs       # escape_pdf_string, helpers de string
    fonts/
        helvetica.rs    # Caminho A — Helvetica Type1 fallback
        cidfont.rs      # Caminho B — CIDFont + Identity-H
        multifont.rs    # Caminho C — N fonts (ADR-0055 decisão 5)
        cmap.rs         # to_unicode_cmap, blocos de 100
        widths.rs       # widths_array
        descriptor.rs   # FontDescriptor, /FontFile2
    images/
        jpeg.rs         # XObject JPEG, /Filter /DCTDecode
        dedup.rs        # Arc::as_ptr dedup
    geometry/
        coords.rs       # Inversão Y, MediaBox cálculo
    collectors/
        codepoints.rs   # collect_codepoints, BTreeSet<char>
        glyph_ids.rs    # collect_glyph_ids
        fonts.rs        # collect_fonts_from_doc (era em pipeline.rs)
```

Esta estrutura é **proposta inicial**. P307a inventário pode
ajustar (clusters reais podem agrupar diferente do que esta
divisão antecipa).

---

## 5. Sequência de sub-passos proposta

Dado o tamanho (~10k LOC), agregar tudo num passo é
irresponsável. Proposta de decomposição em sub-passos:

### P307a — Inventário + ADR (diagnóstico-primeiro, sem código)

**Magnitude**: M documental.

Análogo a P156B (Layout) e P154A (Model).

Output material:
- Diagnóstico em `00_nucleo/diagnosticos/diagnostico-export-passo-307a.md`
  com inventário factual: por função actual em `export.rs`,
  classificação por cluster, identificação de helpers privados
  reusados cross-cluster.
- **ADR-0099 (ou próximo número disponível)**: "ADR-0037
  estendida a L3 — coesão por domínio em `03_infra/`". Status
  `PROPOSTO`.
- **ADR sobre ADR-0098 SSoT**: ou anotação clarificando que o
  invariante bit-exact textual cede ao invariante bit-exact do
  output binário (proposta acima), ou ADR nova que formaliza
  esta substituição. Decisão depende de inventário.

Sem código tocado. Sem hash propagado.

### P307b — Decomposição mecânica (`git mv` + reorganização imports)

**Magnitude**: L (mecânica, mas alto volume).

Pré-requisito: ADRs P307a `PROPOSTO`.

Movimentação semântica do código por cluster definido em P307a.
Cada submódulo:
- Ganha header `@prompt-hash` + `@layer L3`.
- Declara apenas os imports que realmente usa.
- Mantém `pub(super)` ou `pub(crate)` conforme visibilidade.

**Critério de fecho**:
- `cargo test --workspace` verde.
- **Validação binária**: corpus de ficheiros `.typ` produz bytes
  PDF idênticos pré- e pós-P307b. Mecanismo: snapshot test
  novo em `03_infra/src/integration_tests.rs` que faz
  `assert_eq!(export_pdf_pre, export_pdf_pos)` para N inputs
  canónicos.
- `crystalline-lint .` zero violations.
- Hash de cada submódulo registado individualmente.
- ADR-0098 (SSoT) anotada com transição "preservação textual →
  preservação binária".

### P307c — L0 prompts por submódulo

**Magnitude**: M documental.

`export.md` antigo (que era monolítico) decompõe em:
- `infra/export.md` (entry point, dispatch)
- `infra/export/pdf/*.md` (5-6 prompts)
- `infra/export/fonts/*.md` (5-6 prompts)
- `infra/export/images/*.md` (1-2 prompts)
- ...

Cada prompt cumpre o template `00_nucleo/prompts/template-prompts.md`.

### P307d — Promoção das ADRs

ADR-0099 (ou similar) e anotação ADR-0098: transição
`PROPOSTO → EM VIGOR` ou `IMPLEMENTADO` após validação empírica
de P307b+c.

---

## 6. Sobre "organização de dependências"

A decomposição **não adiciona deps por si só**. Mas habilita
discussões focadas que estavam invisíveis num monolito de 10k
LOC:

- Identificar deps usadas em ≤1 submódulo.
- Avaliar se adicionar `flate2` (compressão `/FlateDecode`) ou
  outras crates focadas justifica-se para um cluster específico
  sem contaminar o resto.
- Reagrupar `[dependencies]` de `03_infra/Cargo.toml` por uso
  real, não por convenção alfabética.

Estes são candidatos a sub-passos P307e+ ou passos separados
P308+ — **fora do escopo da decomposição mecânica em si**.

---

## 7. Protocolo de Nucleação — onde estamos

Per `CLAUDE.md` §"Protocolo de Nucleação":

| Fase | P307a | P307b | P307c | P307d |
|---|---|---|---|---|
| 1. Passo planeia tarefas | este doc | aguarda P307a | aguarda P307b | aguarda P307c |
| 2. IA redige L0 | (P307a não toca L0/L1) | aguarda Fase 1 | (esta é a Fase 2 alargada) | (admin) |
| 3. Humano grava + hash | n/a P307a | aguarda Fase 2 | obrigatório por ficheiro | n/a |
| 4. Testes primeiro | n/a P307a | snapshot binário | n/a | n/a |
| 5. Implementação | n/a P307a | `git mv` + reorganização | n/a | n/a |
| 6. Validação | n/a P307a | `crystalline-lint` + cargo test | `--fix-hashes` | n/a |

**Trava arquitectural**: P307b não pode arrancar antes de P307a
estar fechado (com ADRs propostas) e de o humano ter aceite o
plano concreto.

---

## 8. Decisões críticas que P307a precisa fixar

Lista para o diagnóstico decidir explicitamente:

1. **Aceitação da substituição ADR-0098**: invariante textual
   bit-exact dá lugar a invariante binário bit-exact?
2. **Numeração ADR**: 0099 ou outro slot disponível?
3. **Estrutura de submódulos final**: a proposta em §4 sobrevive
   ao inventário ou ajusta-se?
4. **Snapshot binário**: que ficheiros `.typ` do corpus
   `lab/parity/corpus/` formam o conjunto canónico de validação?
5. **Hash de cada submódulo**: convenção de naming dos
   `@prompt-hash` (paths longos podem dar headers extensos).
6. **Granularidade de P307b**: um só passo de movimentação
   completa, ou várias sub-movimentações por cluster (P307b.1
   PDF objects, P307b.2 fonts, etc.)?

A decisão 6 é especialmente sensível dada a magnitude. P307a
deve recomendar com base no inventário factual.

---

## 9. Não-objectivos explícitos

P307 **não**:

- Adiciona features (zero impacto na cobertura de paridade).
- Altera comportamento observable (PDF gerado é bit-exact).
- Introduz novas dependências externas (ficam fora do escopo).
- Aplica decomposição a outros monolitos L3 ou L1 (cada um
  exige passo dedicado se justificado).
- Toca L1, L2, L4 (excepto se imports cross-camada precisarem
  ajuste mecânico — improvável dado que API pública de L3
  permanece).

---

## 10. Risco e mitigação

| Risco | Probabilidade | Mitigação |
|---|---|---|
| PDF binário muda (regressão silenciosa) | Média | Snapshot test obrigatório em P307b |
| Ciclo de imports entre submódulos | Baixa | P307a inventário identifica deps cruzadas antes |
| Quebra de teste pré-existente | Baixa | `cargo test --workspace` em cada sub-passo |
| ADR-0098 substituição rejeitada pelo humano | Possível | P307a separado precisamente para esta decisão |
| Inventário revela que decomposição não vale o custo | Possível | P307a pode concluir "manter monolito + Regra 6" |
| Magnitude estourar (LOC errados na auditoria) | Baixa | Verificar contagem em P307a inventário |

---

## 11. Critério de fecho do passo (série completa P307a-d)

P307 completo quando:

- [ ] P307a fechado: diagnóstico publicado, ADRs PROPOSTAS
- [ ] P307b fechado: `git mv` + reorganização; testes verdes;
      snapshot binário verde; lint zero
- [ ] P307c fechado: L0 prompts por submódulo criados; hashes
      propagados
- [ ] P307d fechado: ADRs `PROPOSTO → EM VIGOR/IMPLEMENTADO`
- [ ] DEBT-46-L3 (novo) aberto e fechado dentro da série, ou
      fechado por declaração que monolito permanece (decisão
      P307a)
- [ ] Métrica: nenhum ficheiro em `03_infra/src/` acima de 800
      linhas sem justificativa Regra 6 documentada no topo

---

## 12. Próxima acção concreta

**Aguardar confirmação humana** sobre:

1. **Avançar com P307 ou não?** Se sim:
2. **Aceitar substituição ADR-0098** (textual → binário bit-exact)?
3. **Arrancar P307a?** Diagnóstico-primeiro é XS-M documental,
   sem código tocado, sem trava arquitectural prévia.

Se as três respostas forem "sim", a IA pode iniciar P307a:
ler `03_infra/src/export.rs` por completo, classificar por
cluster, escrever diagnóstico, redigir ADRs em status `PROPOSTO`.
Continua a parar antes de qualquer L1/L3 movido.

Se alguma resposta for "não" ou "pensar mais", P307 fica adiado
e o estado pós-P306 permanece o ponto de partida para outra
direcção (P307 alternativa: erf, math style functions, etc.).
