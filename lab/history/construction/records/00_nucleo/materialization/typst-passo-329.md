# Tarefa P329 — Triagem do DEBT-58 (gravar as decisões) + Lote 14 (Labelled, Boxed)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P329 (confirmar livre).
**Pré-condição**: Lote 13 (P328) fechado — lint 0, suíte verde (typst-core
2685); dossiê em `00_nucleo/materialization/dossie-triagem-debt-58.md`. Se
não, parar.
**Tipo**: (Parte 1) Triagem do DEBT-58 — **registro de decisão de desenho do
dono, zero código** + (Parte 2) Lote 14 da migração D — instância do modelo.
**Fontes**: dossiê da triagem (P328), DEBT-58, `modelo-lote-migracao-d.md`
(receita + Contabilidade + passo mecânico C1…C2), relatório P328.
**Commits**: "Passo 329 — triagem DEBT-58" (primeiro, tree limpo, zero
código) e "Passo 329 — lote 14".

---

## Parte 1 — Triagem do DEBT-58 (decisões do dono, a gravar)

**Critério decidido pelo dono** (gravar como cabeçalho da triagem):
fidelidade ao typst vanilla é **de comportamento** — saída renderizada e
semântica da linguagem —, **não de estrutura Rust**. A estrutura interna
decide-se por atomicidade, performance e manutenção por IA. Decisões do
vanilla que impõem forma ao código não vinculam.

### Verificação mecânica prévia (única pendência factual)

Grep em `content.rs` (e registrar o comando): **quais variantes as operações
internas do próprio `Content` constroem** (concatenação, default,
normalização — a "álgebra"). Esperado: `Sequence`/`Empty` sim. **A dúvida é
`Block`**: se a álgebra o constrói, reclassifica para primitivo e o L15
(planejado para ele) cancela; se for só elemento denso (construído por
eval/stdlib como os demais), confirma lote tardio. O grep decide; registrar
o resultado e a classificação resultante. (Surpresa fora desse binário →
reportar antes de gravar.)

### As decisões (com a verificação feita)

1. **Primitivos definitivos (4)** — `Sequence`, `MathSequence`, `Empty`,
   `Space`: álgebra/cola do próprio `Content`, sem campos de usuário;
   permanecem no hub **por desenho declarado** (arm próprio deixa de ser
   dívida). Razão: migração não compra atomicidade (não têm semântica de
   utilizador nem campos por crescer) e custa o topo da largura
   (`Sequence` 216, `Empty` 156).
2. **Primitivos provisórios (3)** — `Text`, `MathText`, `MathIdent`:
   permanecem no hub **com revisita marcada no diagnóstico do F**. Razão: os
   campos que o vanilla lhes dá são estilo (StyleChain) — território do F;
   decidir agora desenharia o F por acidente (mesmo argumento das `Set*`,
   L3).
3. **Lote tardio (3)** — `Labelled`(57) · `Boxed`(69) · `Block`(121):
   element-shaped densos; o modelo provado os come (precedente L12 +
   Arc-wrap C2). Soma ~247 excede a faixa validada → **dois lotes**:
   **L14 = `Labelled`+`Boxed` (~126)** e **L15 = `Block` (~121)** — sujeito
   à verificação de `Block` acima.
4. **Transferido ao F (1)** — `Styled(Box<Content>, Styles)`: carrega
   `Styles`, a superfície que o F/99.E redesenha; sai da triagem e entra no
   escopo do diagnóstico do F, junto das 4 `Set*`.

Conta: 7 primitivos + 3 lote tardio + 1 ao F = 11 ✓ (e 62 + 4 `Set*` + 11 =
77 ✓).

### Onde gravar

- **DEBT-58**: as classificações acima + o critério do dono; a parte
  "primitivos" do débito encerra (vira desenho declarado); a parte
  `Styled` transfere para o 99.E/F com referência cruzada; `Block`/`Boxed`/
  `Labelled` saem do débito para o roteiro de lotes.
- **L0 do `content`** (`entities/content.md`): declarar os 7 primitivos
  (4 definitivos + 3 provisórios com a marca "revisitar no F") como parte do
  desenho do hub — arms próprios são intencionais, não pendência.
- **Contabilidade do modelo**: roteiro atualizado (L14, L15, depois
  verificação de performance + diagnóstico do F).
- **Dossiê**: nota de fecho apontando para as decisões (o dossiê era o
  material; a triagem é esta).

Zero código nesta parte. Diff só de registro.

---

## Parte 2 — Lote 14, instanciando o modelo

Executar `00_nucleo/modelo-lote-migracao-d.md` com:

- `N_LOTE = 14`
- `LOTE` (a confirmar no checkpoint), ordem por largura crescente:
  **`Labelled`(57) · `Boxed`(69)** = **~126 sites** — na faixa.
- `NOTAS_FAMÍLIA`:
  - **`Labelled` `{ target, label }`** — wrapper de label: `map_*` esperado
    recursar no `target`; **verificar no checkpoint** como a introspecção
    consome labels (esperado: via payload/tags, inalterado pela migração —
    precedente dos locatáveis; se o consumo matchear `Content::Labelled`
    diretamente, listar os sites no plano de toque). `Hash`: conferir o tipo
    de `label` (provável derive).
  - **`Boxed`** — contentor denso (~10 campos, body + cosméticos): mesma
    família do L12; `Hash` provável **manual** (`Length`/f64); construtor só
    se cobrir todos os campos, senão **Arc-wrap** (regra C2). `is_empty`/
    `plain_text` delegam só se o hub atual delega (content-preserving).
  - **Locatabilidade**: confirmar ambas contra `locatable.rs` (esperado:
    nenhuma; se alguma for, precedente Heading).
  - **Passo mecânico C1…C2 integral**: grep prévio → skip-list aos dois
    transformadores → aninhados à mão por classe → verificação pós-passada.
    Zero conversões indevidas é o critério.
  - Inspecionar `|`-combinados envolvendo as 2 antes de estimar (regra
    P320); `Styled`/`Boxed`/`Labelled` podem partilhar arms de wrapper —
    com binding → split (e `Styled` **não** migra: o arm dele permanece).
- **Validação intermediária** (precedente L12): build + suíte verdes após
  cada variante.

Tudo o mais — Fase A, checkpoint humano, Fase B, validação final, medições
vs preditor, contabilidade — **como está no modelo**. Lembretes: trait não
muda em lote; nenhuma asserção existente alterada; conserto oportunista
proibido; quebra de desenho → parar e voltar ao L0.

---

## Relatório (`typst-passo-329-relatorio.md` + resumo no chat)

- **Triagem**: o resultado do grep da álgebra (e a classificação de `Block`
  que dele segue); onde cada decisão foi gravada (DEBT-58, L0 do content,
  Contabilidade, dossiê); o critério do dono citado.
- Lote 14: composição/forma confirmadas; como a introspecção consome labels
  (o achado provável); sites tratados à mão + verificação pós-passada;
  validação intermediária.
- Medições ADR-0104: `content.rs` antes/depois (trajetória desde 5782),
  parte atómica, suíte antes/depois, lint 0.
- **Contabilidade atualizada**: migradas 62 → 64; DEBT-58 encerrado
  (7 primitivos declarados + `Styled` → F); resta **L15 = `Block`** (se a
  verificação confirmar) e depois: verificação de performance (`hyperfine`,
  caveat C1/P319) → diagnóstico do F (absorve `Set*` + `Styled` + revisita
  `Text`/`MathText`/`MathIdent`).
- `git log --oneline` (dois commits isoláveis); `git status` limpo.
- Caveat conhecida: stack default em `recursao_infinita_*` — não é regressão
  (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

Lote 15 (`Block`); a verificação de performance e o **diagnóstico do F**
(próximas etapas do roteiro, nesta ordem); qualquer mudança em `Styled`,
`Set*`, `Text`/`MathText`/`MathIdent` (escopo do F); otimizações sugeridas
por medição (medir ≠ mexer).
