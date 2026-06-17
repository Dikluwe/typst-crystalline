# Passo 348 — relatório: recursão de `#show` por ponto-fixo morfológico (modelo α)

> **Resultado.** A element rule cujo output **re-casa** passa a ser **revisitada** até
> **ponto-fixo morfológico** (`Content::morph_canon`, P345), com **teto-64 backstop** para
> recursão não-convergente (→ erro com a mensagem base **byte-idêntica** ao vanilla) — um
> **loop local** em `apply_show_rules` (**M-eager = eager-viável**, sem o multi-passe do
> P340). O **`m1` foi de "b" → "c"**. Divergência consciente registrada (ADR-0107):
> `#show heading: it => [= Z]` **converge para "Z"** onde o vanilla erra. Primeiro lote de
> código desde o P345. **A flag de erro completo (classificação) foi ADIADA** — superfície
> nova de config, cross-layer (a mensagem base já é byte-idêntica ao vanilla sem ela).
> Gates: lint **0/0**, lib **2726** (2723 + 3 novos, 3 comentários evoluídos, 0 quebrados),
> workspace **3245**.

## C0 / C1
- **C0**: HEAD `b408e4629` (pós-P346/347*); nenhum `.rs` mudou desde o P345 (os 347* são
  doc); suíte 2723/3242, lint 0/0, árvore de produto limpa. Bate.
- **C1**: `m1` atual era "b" (P345); alvo deste lote "c" (paridade vanilla). Divergência
  `o_inf` aberta na nota de paridade (ADR-0107): vanilla erra, α converge para "Z".

## Fase A — as medições

### M-eager — **eager-viável** (loop local; NÃO exige multi-passe)
O P347/M3 mediu que reproduzir o vanilla **exato** (terminar por identidade de instância)
exige multi-passe. **α não é exato** — termina por **morfologia**. A revisitação cabe num
**loop local** em `apply_show_rules`: aplica a regra, re-alimenta o output no mesmo
conjunto de regras, compara `morph_canon`, repete. O output re-processado pela própria
travessia dá a mesma composição (P341/p3b já mediu a cascata A→B fiel); a única lacuna era
a recursão da **mesma** regra ao próprio output — agora o loop a resolve. O `active_guards`
permanece (anti-recursão **durante** a chamada do recipe — criação aninhada); a revisitação
é o loop, **após** o recipe devolver. **Não** porta o guard-por-instância nem a realização
recursiva do vanilla.

### M-trigger — o caminho comum é barato (não paga `morph_canon`)
A checagem de ponto-fixo (`morph_canon`) só entra **da 2ª aplicação**. O caso comum — uma
regra aplica uma vez e o output **não** re-casa — faz: aplicação 1 (sem checagem), 2ª
iteração tenta casar o output → falha → sai. **Zero `morph_canon`**. Só recursão genuína
(o output re-casa) paga a comparação, por passe.

### M-oráculo — os casos (vanilla / crist antes / α)

| caso | `.typ` | vanilla | crist antes | α (medido) |
|---|---|---|---|---|
| convergente (`m1`) | `it.body==[a]→[= b]; ==[b]→[= c]; else it` · `= a` | **c** | b | **c** ✓ |
| `o_inf` (constante) | `#show heading: it => [= Z]` · `= a` | **erro** | Z (truncava) | **Z** (converge — divergência consciente) ✓ |
| ciclo | `==[a]→[= b]; else [= a]` · `= a` | **erro** | b | **erro** `maximum show rule depth exceeded` ✓ |
| anti-rec. constante | `#show heading: it => [= X ]` · `= A` | (termina) | X | **X** (ponto-fixo) ✓ |
| composição (muda tipo) | `[Prefixo: ] + it.body` · `= Título` | (1×) | 1× | **1×** (output é Sequence, não re-casa) ✓ |

### M-msg — mensagem base e canal de hint
A mensagem base do vanilla é `"maximum show rule depth exceeded"` (`engine.rs:350`). O
cristalino **já a produzia byte-idêntica** em `check_show_depth` (`world_types.rs:273`),
**com** os dois hints do vanilla (`"maybe a show rule matches its own output"` / `"maybe
there are too deeply nested elements"`). O `SourceDiagnostic` tem canal `hints` separado
(`with_hint`). O teto backstop do loop emite **a mesma** mensagem+hints.

### M-text — text rules não recursam
`map_text` (`rules.rs:175-181`) é passe único; o output de uma text rule **não** é
revisitado. A Revocation (que no vanilla servia só a text rules, P347d) **não tem papel**
no cristalino — este lote **não** toca text rules.

## TRAVA — seguiu sem parar
M-eager = eager-viável; M-oráculo bate com os alvos (só o `o_inf` diverge, abençoado em
C1); mensagem base clara e já byte-idêntica. **Nenhuma** condição de parada (não é
multi-passe; nenhuma divergência nova além do `o_inf`; a mensagem não muda). → executou.

## Fase B — o diff por estágio

- **Estágio R — revisitação até ponto-fixo** (`rules/eval/rules.rs`, `apply_all`): o output
  de uma element rule que re-casa é re-alimentado num **loop**; para quando
  `out.morph_canon() == work.morph_canon()` (no-op morfológico — ponto-fixo) ou quando
  nenhuma regra casa o output. `m1` → "c" (3 aplicações: a→b→c→c-fixo).
- **Estágio T — teto backstop + mensagem base** (mesmo loop): recursão não-convergente
  (ciclo/divergente) nunca atinge ponto-fixo → ao chegar a `MAX_SHOW_RULE_DEPTH` (64,
  mecânica) o loop **erra** com a mensagem base + hints do vanilla (byte-idêntica). O teto
  **não** é o mecanismo de terminação (o ponto-fixo é) — é o corte de runaway.
- **Estágio Flag — ADIADO.** A flag de erro completo (classificar cíclico/divergente/
  converge-fundo num canal de hint) é **superfície nova de config**: L1 **não lê env**
  (pureza), então a flag exige um campo plumbed de L2/L4 (cross-layer) + lógica de
  classificação (histórico de morfologias) + decisão de exposição (CLI). Pela regra do
  próprio passo ("L0/superfície nova → parar para selar") e porque a flag **não muda o
  comportamento padrão** (a mensagem base já é byte-idêntica ao vanilla sem ela), **adiei-a
  a lote próprio**, registrando-a no L0 (`§3a.7-bis`). O core (paridade `m1`→"c",
  terminação) está completo sem ela.
- **Estágio L0** (`f_fronteira_e1.md §3a.7-bis` + `rules/eval.md`): o modelo α, o teto
  backstop, a divergência `o_inf`, a não-reprodução da Revocation, o adiamento da flag.
  `--fix-hashes` (eval.md + f_fronteira mudaram → re-sync dos headers que os referenciam).
- **Estágio F**: `@updated 2026-06-17` em `rules.rs`; `--fix-hashes`; V5/V7 limpas.

## Evolução de testes
- **3 novos** (`rules/eval/tests.rs`): `p348_show_recursao_converge_para_ponto_fixo`
  (`m1`→"c", vanilla colado), `p348_show_recursao_o_inf_converge_divergencia_consciente`
  (**a divergência `o_inf` documentada** — "Z" no crist; nota: vanilla erra), 
  `p348_show_recursao_ciclo_erra_com_mensagem_vanilla` (erro + mensagem base byte-idêntica).
- **3 comentários evoluídos** (asserções **intactas**, mecanismo re-explicado):
  `f3s2_show_callout_anti_recursao_termina` (agora ponto-fixo, antes truncava),
  `show_rule_nao_recursiva_sem_stack_overflow` (X→X ponto-fixo),
  `show_rule_composicao_sem_loop` (output é Sequence, não re-casa).
- **0 testes quebrados** — os anti-recursão **convergem** sob α com o mesmo resultado.

## Aceitação morfológica (ADR-0107)
- convergente: morfologia final == vanilla (`m1` → "c"). ✓
- `o_inf`: converge para "Z" (divergência consciente; vanilla erra). ✓ (registrada)
- ciclo/divergente: **erro** (como o vanilla — ambos erram). ✓
- profundo legítimo: o ponto-fixo para antes do teto → compila. ✓ (por construção)
- mensagem base de erro: **byte-idêntica** ao vanilla (`world_types.rs:273`). ✓

## Verificação (gates)
```
build: limpo. lint: crystalline-lint . = 0/0.
suíte: lib 2726 (2723 + 3 novos), workspace 3245. 3 comentários evoluídos, 0 quebrados.
caminho comum barato: confirmado por construção — morph_canon só da 2ª aplicação; o
  caso comum (output não re-casa) não a paga (M-trigger).
== / dois sistemas intactos: morph_canon (P345) e derive(PartialEq) do Rust inalterados;
  este lote só adiciona o loop em apply_show_rules.
lente: delta de aresta ZERO — nenhum `use` novo em rules.rs (Route por full-path,
  morph_canon por método). Baseline 219|676|[90,4]|66|0 preservado por construção.
perf: caminho comum e revisitação ambos **sub-resolução** (<0.01s nos docs de teste; abaixo
  do /usr/bin/time); suíte inalterada (0,37s). Caminho comum sem custo extra (design);
  revisitação adiciona N passes + (N-1) morph_canon só quando recursa de facto. Sem
  regressão observável. (Medição absoluta release não feita — sub-resolução nos casos reais.)
```

## Divergência `o_inf` registrada (não é falha)
`#show heading: it => [= Z]` → cristalino **"Z"**; vanilla 0.14.2 **erro** `maximum show
rule depth exceeded`. **Divergência consciente** (ADR-0107): o vanilla termina por
identidade de instância (mecânica — GEROU, P347b/c; commit #3327; a Revocation é INTERNA,
P347d), e erra ali só porque o seu anti-loop não enxerga o ponto-fixo. O cristalino
converge por morfologia — mais gracioso. Registrada no teste
`p348_show_recursao_o_inf_converge_divergencia_consciente` e no L0 `§3a.7-bis`.

## Mapa de filtro (dois campos)
1. **Lugar lógico desta fatia:** a recursão de `#show` do cristalino termina por
   **morfologia** (o `==` do P345), não por identidade de instância — é a **ADR-0107 levada
   à terminação**: o cristalino escolhe o modelo que se **explica** (convergiu) em vez de
   reproduzir o mecanismo de identidade do vanilla (que o próprio vanilla trata como
   limitação). A divergência `o_inf` é a marca consciente dessa escolha.
2. **Meta-achado da cadeia P347→P348:** o método, ao investigar se tinha achado uma falha
   do vanilla, **recuou três vezes com evidência** — GEROU→QUIS-com-borda (P347c, commit
   #3327); "achamos uma falha"→"diferença de prioridade" (P347c); "respeitar a Revocation"→
   "Revocation é mecânica, substituir" (P347d). Concluir **"não havia falha, eu estava
   enganado"** e autocorrigir é a evidência mais forte da branch de que o tekt **mede antes
   de decidir**. Candidata a entrada de fundação do documento de visão (junto com a entrada
   do `TextStyle` assado).

## Item carregado
`content→elements` aponta para o **Marco G** (P346) — não volta como órfão.

## Ficheiros tocados
- **Código (L1):** `rules/eval/rules.rs` (loop de revisitação em `apply_all`),
  `rules/eval/tests.rs` (3 testes novos + 3 comentários evoluídos).
- **L0:** `rules/eval.md` (bullet da recursão morfológica), `entities/f_fronteira_e1.md
  §3a.7-bis` (modelo α aterrado + flag adiada).
- **Mecânico:** bump de `@prompt-hash` nos ficheiros que referenciam os 2 L0 tocados.
- **Adiado:** a flag de erro completo (classificação) — lote próprio (superfície de config).
