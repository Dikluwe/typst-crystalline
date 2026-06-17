# Passo 347 — relatório: Fase A (4 medições) + TRAVA (M3 = multi-passe — decisão do dono)

> **Resultado: PAROU NA TRAVA.** A Fase A mediu, da fonte + binário, que **reproduzir
> a terminação do vanilla EXATAMENTE** (`m1`→"c" **e** infinito→erro **e** ciclo→erro)
> exige o **guard por-instância** do vanilla (recipe-index carregado numa realização
> recursiva) — a **camada multi-passe** que o P340 estacionou. A via **eager
> morfológica** (ponto-fixo via `morph_canon` do P345) **não** distingue o `m1`
> terminal (`it` identidade → "c") do infinito-de-saída-constante (`[= Z]` → vanilla
> erra): os dois são, morfologicamente, o **mesmo** ponto-fixo `X→X`. Logo o lote
> **não executa código** — o dono decide entre (β) multi-passe agora, (α) divergência
> eager documentada, ou (γ) fatiar. Content-preserving até aqui: zero `.rs`, lint 0/0,
> suíte 2723/3242 intacta.

## C0 / C1
- **C0**: HEAD `04e02f93a` (pós-P346), P345 commitado (`3ebb397fb`), suíte 2723/3242,
  lint 0/0, árvore limpa. Bate.
- **C1 — âncora `m1`**: estado atual `= a` → **"b"** (P345 destravou de "a"→"b" via o
  `==` morfológico; o guard por-regra trunca no nível 1). O alvo deste lote era "b"→"c".

## Fase A — as quatro medições

### M1 — semântica de terminação do vanilla (da fonte + binário)
**Mensagem base (byte-exata):** `"maximum show rule depth exceeded"`
(`engine.rs:350`). **Canal de hint separado** (existe no vanilla): `bail!(…; hint:
"maybe a show rule matches its own output"; hint: "maybe there are too deeply nested
elements")` (`engine.rs:351-352`). **Teto:** `MAX_SHOW_RULE_DEPTH = 64` (`engine.rs:335`),
checado em `check_show_depth` após `route.increase()` (`lib.rs:401-402`).

**Revisitação (a semântica a reproduzir):** `visit_show_rules` (`lib.rs:335`) aplica um
passo de recipe e **re-visita** o output recursivamente (`visit_styled(realized)`,
`:404`). O guard é **por-instância**: o output do recipe nasce `.guarded(index)`
(`:366`) e a seleção de recipe salta se `elem.is_guarded(index)` (`:471-473`). **Conteúdo
NOVO** produzido pelo corpo do recipe nasce **fresco** (sem guard) → re-aplica.

**Casos-oráculo (binário, vanilla vs cristalino atual):**

| caso | `.typ` | vanilla | crist (P345) |
|---|---|---|---|
| converge | `it.body==[a]→[= b]; ==[b]→[= c]; else it` · `= a` | **c** | **b** |
| infinito | `#show heading: it => [= Z]` · `= a` | **erro** `maximum show rule depth exceeded` | **Z** (trunca, sem erro) |
| ciclo | `==[a]→[= b]; else [= a]` · `= a` | **erro** (idem) | **b** |

### M2 — forma de detecção (teto puro vs ponto-fixo+ciclo vs híbrido)
- **Teto puro** (contador, erra em N): barato, é o do vanilla. **Mas, sozinho no eager,
  quebra o `m1`**: sem guard, `= c`→`it`(= c) re-casa o heading rule para sempre
  (identidade) → bate no teto → **`m1` erraria** (deveria dar "c"). Insuficiente.
- **Ponto-fixo morfológico** (P345 `morph_canon`: para quando output ≡ input): resolve o
  `m1` (`= c`→`it`≡`= c` → para → "c"). **Mas converge o infinito-de-saída-constante**:
  `[= Z]` produz `= Z` ≡ `= Z` → ponto-fixo → para em **"Z"** (vanilla **erra**). Não
  distingue no-op real (m1) de rewrite-para-igual (o_inf) — ambos são `X→X`.
- **Híbrido (teto backstop + análise sob flag)**: a análise (ponto-fixo/ciclo/classificação)
  só corre quando o teto dispara e a flag está ligada — **bom para a TERMINAÇÃO/erro
  classificado**, mas **não resolve** a distinção m1-vs-o_inf, que é a do guard por-instância.

**Veredito M2:** nenhuma forma puramente eager (teto puro **ou** ponto-fixo morfológico)
reproduz o vanilla nos três casos. A distinção m1-vs-o_inf é o **guard por-instância** (M3).

### M3 — cabe no eager ou exige multi-passe? **EXIGE multi-passe (ou guard-por-instância equivalente, grande).**
O vanilla separa `m1` (termina em "c") de o_inf (erra) por **identidade de instância**:
em `m1` o recipe devolve `it` (a instância **guardada** → re-visita salta → para); em
o_inf devolve `[= Z]` **fresco** (sem guard → re-aplica → teto → erro). Isto é
**não-morfológico** — e morfologicamente `m1` (c→c) e o_inf (Z→Z) são o **mesmo**
ponto-fixo `X→X`. O `==` morfológico do P345 **não consegue** distingui-los.

Reproduzir o vanilla exige carregar o **recipe-index por instância de `Content`** através
de uma **realização recursiva** (re-visita do output com guard propagado) — exatamente a
**camada multi-passe** que o `Content` cristalino (enum sem campo de guard) e o caminho
eager (`map_content`, passe único, `apply_show_rules` `rules.rs:90-172`) **não têm**. É a
mudança de mecanismo que o **P340 estacionou** por ser grande. **Não improvisei** (trava).

### M4 — interseção com o de-bake (F-5) — **DISJUNTO**
A recursão toca o **caminho de aplicação de show** (`apply_show_rules`/`intercept_content`
+ o mecanismo de guard). O de-bake (F-5) toca a **representação de estilo** (`TextStyle`
em `Content::Text`) e as leituras de numbering em layout/introspect. Superfícies
disjuntas (P337: a recursão muda *quando/se* as regras re-aplicam; o de-bake muda *onde o
estilo é lido*). → o **de-bake fica liberado** para o lote seguinte sem risco de ordem.
(Moot enquanto a trava não resolver o escopo, mas registrado.)

---

## TRAVA ARQUITETURAL — emitida; PAROU (M3 = multi-passe)

Per a regra do passo ("M3 = multi-passe → parar; o dono decide executar agora ou
fatiar"), **paro aqui**. O canal de hint existe (✓, `SourceDiagnostic.hints` +
`with_hint`, `source_result.rs`), a mensagem base é clara (✓), M4 é disjunto (✓) — mas
**M3 exige a camada multi-passe**, que é a fronteira de decisão do dono.

**As três saídas (o dono escolhe UMA):**

- **β — multi-passe agora (paridade exata).** Portar o guard por-instância (recipe-index
  no `Content` + realização recursiva com guard propagado) + teto-64 backstop + flag de
  classificação no canal de hint. `m1`→"c", o_inf→erro, ciclo→erro — **byte-idêntico** ao
  vanilla na mensagem base. **Custo:** mudança de mecanismo grande (a estacionada do
  P340); toca o modelo de realização; superfície de L0 nova; muitos testes. É um lote
  próprio, longo.
- **α — revisitação eager morfológica (divergência documentada).** Loop local até
  ponto-fixo (`morph_canon`) + detecção de ciclo (histórico de morfologias) + teto
  backstop. `m1`→"c" ✓, ciclo→erro ✓; **mas o_inf→"Z"** (converge gracioso, **sem erro**)
  — divergência observável vs vanilla (erro), porque morfologicamente o_inf é um
  ponto-fixo. Registrar como **divergência consciente** (ADR-0054-graded / nota de
  paridade): o cristalino é **mais gracioso** (não erra em rewrite-para-constante), ao
  custo de não reproduzir o erro do vanilla nesse caso patológico. Lote **menor**, cabe
  no eager.
- **γ — fatiar.** Fazer **só a revisitação convergente** (`m1`→"c", a parte que o eager
  resolve sem ambiguidade) agora, como lote menor; **adiar** a paridade exata de
  terminação (β, guard por-instância) para um lote multi-passe dedicado, quando o dono o
  priorizar. A âncora `m1` fecha; o erro-de-infinito fica explicitamente pendente.

**Recomendação do agente (marcada como tal):** **γ** (ou **α**). O `m1`→"c" — o resíduo
nominal deste lote — cabe limpo no eager (revisitação convergente). A paridade de *erro*
de recursão infinita é onde o multi-passe é inevitável; isolá-la (γ) entrega o valor
imediato (composição/recursão convergente fiel) sem pagar a camada grande agora, e nomeia
o resto. **α** é defensável se o dono aceita a divergência gracioso-vs-erro como
intencional (é coerente com "o cristalino termina com saída", precedente do P341 opção a).
**β** é o caminho se a paridade de erro for requisito agora — mas é grande e foi
estacionada uma vez.

**Parei. Nenhum código de produção escrito. Probes descartáveis removidas (tree limpo).**

---

## Verificação (gates até a trava)
```
content-preserving (Fase A): zero .rs/.toml. Suíte 2723/3242 intacta (não re-rodada).
  Árvore de produto não tocada.
lint: crystalline-lint . = 0/0.
medição: M1 (mensagem base + hint + teto, file:line), M2 (formas + custo), M3 (eager vs
  multi-passe, decidido: multi-passe p/ paridade exata), M4 (de-bake disjunto). Vanilla
  0.14.2 como oráculo; casos-oráculo no binário. Zero "~".
```

## Mapa de filtro (campo)
**Lugar lógico:** a semântica de recursão de `#show` mora na **camada de realização**; a
terminação separa **comportamento** (paridade: infinito→erro) de **mecanismo** (o guard
por-instância, o teto). O P347 mediu que **a paridade de terminação exata é inseparável
do mecanismo multi-passe** — não é um detalhe que o eager alcança por loop local.
**Rastro:** eager trunca no nível 1 desde sempre; P340 estacionou o multi-passe ao bater
em content-preservation; P341 mediu que o eager-cascade é fiel na composição mas não na
recursão; P345 destravou o `m1` ("a"→"b") medindo que a igualdade mascarava; **P347 mede
que fechar a recursão exige escolher entre multi-passe (β), divergência eager (α) ou
fatiar (γ)** — decisão do dono.

## Item carregado
`content→elements` aponta para o **Marco G** (P346) — não volta como órfão.
```
