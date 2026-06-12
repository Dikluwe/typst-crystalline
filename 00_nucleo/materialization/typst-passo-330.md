# Tarefa P330 — Lote 15 (`Block`, o último lote) + baseline de performance (hyperfine, pré-F)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P330 (confirmar livre).
**Pré-condição**: P329 fechado — triagem gravada, Lote 14 fechado, lint 0,
suíte verde (typst-core 2693). Se não, parar.
**Tipo**: (Parte 1) Lote 15 da migração D — **instância do modelo, a última
do roteiro** + (Parte 2) baseline de performance pré-F —
**diagnóstico-primeiro, zero código de produto, zero decisão**.
**Fontes**: `modelo-lote-migracao-d.md` (receita + passo mecânico C1…C2),
relatório P329, `medicao-pre-f-passo-318.md` (M3 + caveat C1/P319),
triagem DEBT-58 (P329).
**Commits**: "Passo 330 — lote 15" e "Passo 330 — baseline de performance
pré-F" (isoláveis).

---

## Parte 1 — Lote 15: `Block`, instanciando o modelo pela última vez

Executar `modelo-lote-migracao-d.md` com:

- `N_LOTE = 15`
- `LOTE`: **`Block`(121)** — variante única, o último lote tardio
  (classificação confirmada pelo grep da álgebra, P329).
- `NOTAS_FAMÍLIA`:
  - **Contentor denso** (~14 campos: body + 13 cosméticos) — a mais densa do
    roteiro; mesma família do L12/`Boxed`. `map_*` recursa no body;
    `is_empty`/`plain_text` delegam só se o hub atual delega
    (content-preserving, conferir braço a braço).
  - **`Hash` provável manual** via Debug (`Length`/`Sides`/f64 — regra do
    modelo); conferir campo a campo no checkpoint.
  - **Construtor vs Arc-wrap (regra C2)**: conferir quantos campos
    `block(…)` (`content.rs:~1369`) cobre; construções de campo-completo
    (materialize `..(**e).clone()`, stdlib, testes) → `Arc::new(BlockElem
    {…})`.
  - **Locatabilidade**: confirmar contra `locatable.rs` (esperado: não;
    conferir também se a introspecção consome `Content::Block` diretamente
    em walk, como o achado `Labelled`/P195D — se sim, converter só o lado do
    pattern, mecanismo intocado, e registrar).
  - **`|`-combinados**: inspecionar antes de estimar (regra P320); os arms
    de `Styled` e dos 7 primitivos **permanecem no hub** — qualquer arm
    partilhado com eles separa deixando-os para trás.
  - **Passo mecânico C1…C2 integral**: grep prévio → skip-list aos dois
    transformadores (conferir que `Block` está no dict do transformador —
    crash do P329) → aninhados à mão por classe → verificação pós-passada.
    Zero conversões indevidas é o critério.

Tudo o mais — Fase A, checkpoint humano, Fase B, validação (`cargo build`,
suíte com `RUST_MIN_STACK=33554432`, `crystalline-lint .` = 0), medições vs
preditor, contabilidade — **como está no modelo**. Lembretes: trait não muda
em lote; nenhuma asserção existente alterada; conserto oportunista proibido;
quebra de desenho → parar e voltar ao L0.

---

## Parte 2 — Baseline de performance pré-F (fecha o caveat C1/P319)

O baseline M3 do P318 está **quantizado** (~15%, granularidade 0.01 s do
`/usr/bin/time` — caveat C1/P319): serve só para regressões grosseiras. A
decisão F precisa de baseline real, tirado **com o hub no estado final
pós-lotes** (= após a Parte 1). Zero código; só medição e registro.

1. **Ferramenta**: `hyperfine` (média ± σ, warmup; registrar versão e
   comando completo). Se `hyperfine` indisponível no ambiente, fallback:
   corpus ~10× + `/usr/bin/time` repetido (≥10 execuções, registrar média ±
   σ e a ressalva); registrar qual caminho foi usado.
2. **Corpus e comando**: os **mesmos** do M3/P318 (comparabilidade), mais o
   corpus 10× se o tempo unitário ainda ficar perto da granularidade.
3. **Medições**: o(s) mesmo(s) cenário(s) do M3 (compile/render do corpus),
   no commit pós-L15. Se barato, incluir também `cargo build` incremental
   e tempo de suíte (typst-core) — números que o F pode afetar.
4. **Registro**: `00_nucleo/diagnosticos/medicao-pre-f-passo-330.md` —
   números, comandos, versões, comparação com o M3/P318 (com o caveat da
   quantização explicado), e a regra já gravada: o "depois" do F refaz
   antes+depois **no par de commits** com a mesma ferramenta/corpus.
   Referência cruzada no `medicao-pre-f-passo-318.md` (este novo baseline o
   substitui como referência do F).

**Zero decisão**: nenhuma otimização, nenhuma conclusão sobre o F — só o
número que o diagnóstico vai consumir (medir ≠ mexer).

---

## Relatório (`typst-passo-330-relatorio.md` + resumo no chat)

- Lote 15: forma de `Block` (campos × locatável × is_empty × map_* × Hash ×
  construtor/Arc-wrap); consumo direto pela introspecção (se houver, o
  registro); sites tratados à mão + verificação pós-passada; medições
  ADR-0104 (`content.rs` antes/depois; parte atómica; suíte; lint 0).
- **Balanço final da migração D** (consolidação): 65 migradas em 15 lotes +
  piloto; trajetória completa do hub (5782 → final); estado declarado do que
  resta nos 6 matches (7 primitivos + `Styled` + 4 `Set*` = 12 arms **por
  desenho/escopo-F**, zero dívida não classificada).
- **Contabilidade FINAL**: migradas 64 → **65**; conta: 65 + 4 `Set*` + 7
  primitivos + `Styled` = **77** ✓. **Roteiro de lotes: encerrado.**
- Parte 2: o baseline novo (números ± σ, comandos), a comparação com
  M3/P318, onde mora.
- **Próximo passo do roteiro**: o **diagnóstico do F** (conversa de desenho
  — consome este baseline e absorve `Set*` + `Styled` + revisita
  `Text`/`MathText`/`MathIdent`). Este relatório não o inicia; só deixa
  registrado que tudo que o F precisa está pronto.
- `git log --oneline` (dois commits isoláveis); `git status` limpo (cruft
  `lab/` conhecido).
- Caveat conhecida: stack default em `recursao_infinita_*` — não é regressão
  (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

O **diagnóstico do F em si** e qualquer mudança em `Styled`/`Set*`/
`Text`/`MathText`/`MathIdent` (escopo do F); qualquer migração além de
`Block` (não há — o roteiro encerra); otimizações sugeridas pela medição
(medir ≠ mexer).
