# Passo 204F — Corpus paridade reduzido (5 core + 2 opcionais; validação dual)

**Série**: 204 (sub-passo `F` = corpus paridade após
P204E wrapper).
**Tipo**: implementação de validação (corpus + harness).
**Magnitude planeada**: M (M+ se harness estrutural não
existir).
**Pré-condição**: P204E concluído; `crystalline_evict`
exposto em L4; tests 1838 verdes; 0 violations; 9
sentinelas activas; ADR-0073 PROPOSTO em vigor (5/7
sub-passos materializados).
**Output**: 3 ficheiros (inventário + relatório +
artefactos de corpus).

---

## §1 Propósito

Estender o corpus `lab/parity/` com 7 ficheiros `.typ`
cobrindo features de introspecção (5 core + 2 opcionais)
e validar **paridade dual** — saída observable (PDF
render diff) + queries estruturais (`Introspector`
queries idênticas entre cristalino e vanilla).

A decisão de validação dual aumenta cobertura e dá sinal
mais cedo. Saída observable apanha regressões de layout;
queries estruturais apanham regressões de tracking
comemo.

P204F respeita a convenção: começa com inventário
empírico antes de qualquer alteração.

---

## §2 Material de partida verificado em P204E

Antes de qualquer alteração, confirmar empíricamente:

- Pasta `lab/parity/` existe com estrutura previsível —
  caminho real a confirmar.
- Corpus actual em `lab/parity/corpus/` (per P204A A15:
  ~30 `.typ`; 0 cobrem introspecção).
- Harness de paridade actual — onde está, o que produz,
  como é executado.
- Harness para validação observable (PDF diff) — existe?
  Como funciona? Que tolerância?
- Harness para validação estrutural (queries) — existe?
  Caso não exista, P204F precisa de criar.
- Vanilla typst binário acessível — caminho do binário
  ou comando de execução.

Sem isto, recuar para P204E.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código, listar literalmente:

1. **Estrutura `lab/parity/`** — sub-pastas, ficheiros
   chave, README se existir.
2. **Corpus actual** — listar 30 `.typ` actuais
   (categoria, features cobertas).
3. **Harness paridade observable** — caminho do
   driver, comando para executar 1 caso, output
   esperado, tolerância de diff.
4. **Harness paridade estrutural** — existe? Caso não:
   identificar como queries `Introspector` podem ser
   serializadas para comparação.
5. **Vanilla typst** — versão acessível, comando para
   compilar `.typ` standalone, comando para executar
   queries via vanilla `typst query`.
6. **Documentação inline** — `.toml` ou `.json` que
   acompanha cada `.typ` no corpus (expectativas).

Output: 6 sub-secções com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**.

Se C1.4 revelar que harness estrutural não existe,
**P204F.div-1** registada e C2 decide: implementar
harness mínimo dentro de P204F, ou pivotar para
validação observable apenas.

### C2 — Decisão sobre validação dual

Com base em C1.3 + C1.4:

- **Caminho A — dual completa** — harness observable
  existente reutilizado; harness estrutural criado em
  P204F (ou estendido se já existir parcial).
- **Caminho B — observable apenas** — caso harness
  estrutural exija infraestrutura desproporcional, P204F
  cobre apenas observable; queries estruturais ficam
  para sub-passo dedicado pós-M8.
- **Caminho C — estrutural apenas** — caso harness
  observable não cubra documentos com TOC dinâmico ou
  contadores, P204F cobre apenas estrutural;
  observable fica para corpus existente expandir
  separadamente.

Critério: simetria com vanilla + custo. Caminho A é
preferido. Caminho B é fallback aceitável (não
inflaciona). Caminho C é improvável (observable
geralmente cobre tudo).

C2 fixa **uma** alternativa.

### C3 — Especificação dos 7 ficheiros `.typ`

Para cada ficheiro, fixar:

- Nome literal (`.typ`).
- Conteúdo mínimo (10–30 linhas).
- Features exercitadas.
- Asserções esperadas (queries estruturais + observable
  diff target).

#### Core (5)

- **`outline-toc.typ`** — Documento com 5 headings
  numerados; `#outline()` no início. Asserções:
  `query(<heading>)` retorna 5 entries; PDF diff zero
  contra vanilla; `headings_for_toc` populado com 5
  entradas.
- **`counter-heading.typ`** — 5 headings com numeração
  customizada (ex: `set heading(numbering: "1.1")`).
  Asserções: counter values 1, 1.1, 2, 2.1, 3
  (consoante estrutura); `flat_counter_at` retorna
  valores correctos.
- **`figure-ref.typ`** — 3 figures numeradas com label
  e `@ref` cruzado. Asserções: `figure_number_for_label`
  retorna 1, 2, 3; PDF mostra "Figure 1", etc.
- **`equation-ref.typ`** — 3 equations numeradas com
  label e `@ref`. Asserções: numeração consistente;
  PDF mostra "Equation 1", etc.
- **`cite-bibliography.typ`** — 1 ficheiro `.bib` ou
  `.yaml` referenciado; 3 citações no corpo;
  `#bibliography()` no fim. Asserções:
  `bib_number_for_key` retorna 1, 2, 3;
  `bib_entry_for_key` retorna entries correctas; PDF
  mostra citações numeradas.

#### Opcionais (2)

- **`here-locate.typ`** — Uso de `here()` e `locate(...)`
  para captura de Position. Asserções: `position_of`
  retorna `Some(Position)` com `page` correcto;
  observable mostra texto de Position renderizado
  consistentemente.
- **`query-metadata.typ`** — Uso de `metadata` e
  `query(<metadata>)`. Asserções: `query_metadata`
  retorna lista esperada; PDF observable consistente.

Cada ficheiro também ganha um companion (`.toml` ou
`.json`) com expectativas serializadas para o harness
ler.

C3 fixa o que está acima. Detalhes de sintaxe (ex:
exact `set` rules) ficam para o ficheiro produzido —
não pré-definidos.

### C4 — Harness observable

Reutilizar harness existente identificado em C1.3:

- Cada `.typ` é compilado por cristalino e vanilla.
- PDFs comparados via PDF diff existente (ImageMagick,
  pdftocairo, ou outro).
- Tolerância: 0 (idealmente) ou pixel-tolerance
  pré-existente.

Sem trabalho adicional em P204F além de adicionar os
7 casos ao corpus.

### C5 — Harness estrutural

Decisão dependente de C2:

- **Se C2 = Caminho A**: criar harness mínimo em
  `lab/parity/structural/`. Mecanismo:
  - Cristalino: invocar `Introspector` queries via
    binding ou função de teste.
  - Vanilla: invocar `typst query` CLI com selector
    correspondente.
  - Comparar output JSON ou serializado.
- **Se C2 = Caminho B**: skip C5.

Forma do harness mínimo (caso C2 = A):

```text
lab/parity/structural/
  driver.rs                  // ou .py / shell
  expectations/              // JSON ou TOML por caso
    outline-toc.json
    counter-heading.json
    ...
```

Conteúdo de `outline-toc.json`:

```json
{
  "queries": [
    {"selector": "<heading>", "count": 5},
    {"name": "headings_for_toc.len", "value": 5}
  ]
}
```

Detalhes da serialização ficam para output de P204F.

### C6 — Adicionar 7 ficheiros ao corpus

Edições literais:

- 7 ficheiros `.typ` em `lab/parity/corpus/visual/`
  (ou caminho real confirmado em C1).
- 7 companions (`.toml` / `.json`) com expectativas.
- Caso C2 = A: 7 ficheiros expectations no harness
  estrutural.

C6 não tem ramos. Executa lista de C3.

### C7 — Bibliography asset

`cite-bibliography.typ` precisa de ficheiro `.bib` ou
`.yaml`. Decidir:

- Reutilizar asset existente em corpus se houver.
- Criar `lab/parity/corpus/visual/refs.bib` mínimo (3
  entries).

Decisão dentro de P204F com base em C1.

### C8 — Compilação

```
cargo build --workspace 2>&1 | tail -10
```

Critério: verde. P204F adiciona ficheiros de dados +
possivelmente código de harness — pode triggerar
warnings novos.

### C9 — Tests workspace

```
cargo test --workspace 2>&1 | tail -10
```

Critério: 1838+ tests verdes (mais 0–14 de C10).

### C10 — Tests dedicados

Adicionar tests em `lab/parity/` ou em
`03_infra/tests/` que executem o harness para os 7 casos:

- 7 tests observable (cada caso: cristalino vs
  vanilla PDF diff).
- 7 tests estruturais (cada caso: cristalino vs
  vanilla queries) — só se C2 = A.

Total esperado: 7 ou 14 tests. Decisão sobre granulosidade
(1 test por ficheiro) vs (1 test agregado) fica para
output.

### C11 — Linter

```
crystalline-lint .
```

Critério: 0 violations.

Hipóteses prováveis de violação:

- Ficheiros `.typ` no corpus não exigem linter
  cristalino (são documentos, não código).
- Harness em código pode gerar regras (importação,
  visibility).

### C12 — Documentação ADR-0073

ADR-0073 mantém PROPOSTO. Anotar secção "P204F" no plano
de materialização com `✅ MATERIALIZADO` + sumário de
ficheiros adicionados (1 linha).

### C13 — Critério de fecho de P204F

P204F concluído quando:

- C1 inventário completo.
- C2 caminho fixado.
- C3 7 ficheiros especificados.
- C4 harness observable confirmado funcional.
- C5 harness estrutural criado ou skip declarado.
- C6 7 `.typ` + companions adicionados.
- C7 bibliography asset resolvido.
- C8 compilação verde.
- C9 tests workspace verdes.
- C10 tests dedicados (7 ou 14) verdes.
- C11 linter 0 violations.
- C12 ADR-0073 anotada.
- Inventário registado.
- Relatório escrito.

### C14 — Sem cláusulas condicionais

C1 produz dados. C2 fixa **uma** alternativa. C3–C13
executam.

Decisões internas (C7 bibliography asset; granulosidade
de tests em C10) resolvem-se dentro do passo sem ramos
estruturais na spec.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204F-inventario.md`.

Conteúdo:
- §1 C1 — inventário (6 sub-secções).
- §2 C2 — caminho fixado.
- §3 C3 — especificações dos 7 ficheiros.
- §4 C5 — harness estrutural (forma + decisões).
- §5 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-204F-relatorio.md`.

Conteúdo:
- O que foi feito.
- Tempo de execução.
- Métricas (tests pre/post; LOC delta; corpus delta).
- Decisões.
- Sugestão para próximo sub-passo (P204G).

### Ficheiro 3 — Artefactos do corpus

Não é ficheiro discreto. Conjunto de artefactos:

- 7 `.typ` em `lab/parity/corpus/visual/` (ou caminho
  real).
- 7 companions de expectativa.
- Possível bibliography asset.
- Harness estrutural (se C2 = A).
- Tests dedicados.

---

## §5 Critério de progressão para P204G

P204F fechado quando C13 cumprido.

Em caso de divergência empírica relevante (ex: harness
observable não funcional, vanilla binário inacessível,
features específicas que cristalino ainda não suporta —
o que seria descoberta crítica), registar em
`P204F.div-N` e:

- Resolver dentro de P204F (preferido).
- Recuar para P204A se for obstrução de baseline (ex:
  feature ainda não implementada).

Hipótese de obstrução: `here()` ou `locate()` podem
ainda não ser suportados em cristalino. Se for o caso,
ficheiro `here-locate.typ` é skipped (opcional, justifica
remoção). Documentar em divergência.

P204G só começa quando P204F fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 3 outputs (inventário + relatório + artefactos).
- Inventário empírico antes de implementação.
- Localização canónica:
  `00_nucleo/diagnosticos/` para inventário;
  `00_nucleo/materialization/` para relatório.
- Sem inflação retórica.

---

## §7 Não-objectivos

P204F não:

- Adiciona benchmarks (P204G).
- Transita ADR-0073 para ACEITE (P204H).
- Transita ADR-0066 para superseded (P204H).
- Cria ADR nova.
- Implementa features `.typ` ainda não suportadas em
  cristalino — caso descoberto, reporta como
  divergência (não inflaciona escopo).
- Modifica trait `Introspector` ou impl
  `TagIntrospector`.
- Modifica Layouter ou consumers.
- Toca em loops fixpoint.
- Expande corpus além dos 7 ficheiros (5 core + 2
  opcionais).

---

## §8 Erro a não repetir

P204A C9 estimou validação reduzida com 5–7 ficheiros.
P204F materializa essa estimativa. Risco: features
específicas (ex: `here()`, `locate()`, `query` com
selectores complexos) podem ainda não ter implementação
completa em cristalino — descoberta empírica em P204F.

C1 verifica antes de C2 decidir. Caso uma feature falhe,
**não inflaciona escopo** — regista divergência e
continua com os outros casos. P204G/H podem decidir se
endereçar feature em sub-passo dedicado.

Hipótese específica: `here-locate.typ` (opcional) pode
falhar se Position concrete em cristalino ainda não
estiver acessível via stdlib `here()`. P204D
materializou Position em runtime mas não modificou
stdlib. Caso falhe, `here-locate.typ` é skip-able (é
opcional) e a divergência é documentada.

---

## §9 Particularidade — execução

P204F é trabalho de validação:

- 7 ficheiros `.typ` (10–30 linhas cada).
- 7 companions de expectativa.
- Possível harness estrutural (~50–100 LOC se C2 = A).
- 7 ou 14 tests.
- Verificação compilação + tests + linter.

Volume médio. Magnitude M.

Recomendado Claude Code dado:

- Execução de comandos vanilla typst para validar
  output esperado.
- Comparação PDF diff com ferramenta externa.
- Iteração sobre features que podem revelar gaps em
  cristalino.

Sessão actual viável se houver disponibilidade, mas o
padrão dos sub-passos M favorece Claude Code.
