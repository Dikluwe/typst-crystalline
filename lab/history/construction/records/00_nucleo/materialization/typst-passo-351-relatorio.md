# Passo 351 — relatório: Fase A + TRAVA — o caso 4 está INVERTIDO no enunciado e JÁ FEITO

> **Resultado: PAROU NA FASE A (contradição medida).** O enunciado do P351 diz que o caso 4
> é "bloco de conteúdo `[]` **vaza** para o irmão; `{}` **confina**". A **fonte contradiz**:
> o spike-2 §B4 diz "regra dentro de bloco **não vaza** para irmão" (confina), e o **vanilla
> compilado confirma** — `[]` **e** `{}` **ambos confinam** (não vazam). Além disso, o caso 4
> (confinamento) **já está implementado e testado** desde o **P340** (`f3s3_caso4_escopo_
> show_confina_no_content_block`, `tests.rs:480`). A frase "`[]` vaza" do P351 é a descrição
> do **bug pré-P340**, não do alvo. Por ADR-0108 (medir produz a decisão; "a fonte vence e a
> divergência é reportada" — o próprio P351) **paro e reporto**: implementar "`[]` vaza" seria
> uma **regressão** (diverge do vanilla + desfaz o P340 + quebra o `f3s3`). **Nenhum código
> escrito.** Caveat de stack: `RUST_MIN_STACK=33554432`.

## C0 / pré-condição
HEAD `02bba712b` (pós-P350c), árvore limpa, lint **0/0**, suíte **2729** (lib). Bate.

## Fase A — as leituras (a fonte, com `file:line` + medição)

### 1. L0 da realização — caso 4 já aterrado (P340)
`entities/f_fronteira_e1.md §3a.7-bis`: "**Aterrado — Caso 4 / `f3s3`** (fatia 2a,
confinamento de escopo). O `#show` num `ContentBlock` `[]` **deixa de vazar**: `eval/mod.rs`
passa a clonar `local_show_rules` (Arc O(1)), espelhando o `CodeBlock` `{}`. … `f3s3` virou
(de 'vaza 2×' a 'confina 1×')." → **o caso 4 (confinamento) já foi implementado**.

### 2. Spike-2 §B4 (a definição canônica do caso 4) — **NÃO VAZA**
`f-spike2-show-passo-333.md:64-70`: "**B4 — Scope: regra dentro de bloco não vaza para
irmão.** Show rules são empacotadas num `StyledElem { child, styles }`
(`content/mod.rs:744-752`); só o `child` carrega esses styles … um irmão fora do `StyledElem`
**não vê** a recipe." E `:159`: "Recipes **confinam-se** à subárvore que embrulham." →
**caso 4 = confinar (não vazar)**, para blocos em geral (`[]` e `{}`).

### 3. Vanilla compilado (o oráculo) — `[]` E `{}` AMBOS CONFINAM
| `.typ` | vanilla | cristalino |
|---|---|---|
| `#[#show "a":"Z"⏎a]a` (conteúdo dentro + irmão fora) | **`Za`** (dentro vira Z, irmão NÃO) | `Z a` (idem — confina) |
| `#{ show "a":"Z"; [a] }⏎a` (code block + irmão) | **`Za`** (confina) | `Za` (confina) |

Os dois blocos **confinam** no vanilla; **nenhum vaza**. O cristalino **bate** (confina nos
dois). → o enunciado "`[]` vaza; `{}` confina" (uma **diferença** entre os dois) é **falso**:
eles se comportam **igual** (ambos confinam).

### 4. O teste do caso 4 — já existe e assere confinamento (P340)
`tests.rs:480` `f3s3_caso4_escopo_show_confina_no_content_block`: comentário "**CASO 4
(escopo) — PARIDADE ALCANÇADA (P340)** … No vanilla, `#show` dentro de um bloco NÃO vaza …
**`f3s3` VIROU** … Antes (eager): `#show` no `[]` … **VAZAVA** → AMBOS … (count 2). Agora …
confina ao bloco: só o callout 'a' (dentro) vira 'DENTRO'; o 'b' (fora) fica intacto." Assere
`matches("DENTRO").count() == 1`. → **o caso 4 está implementado e protegido por teste**.

## A CONTRADIÇÃO (o achado da Fase A)
O P351 (escrito do plano/spike pré-P340) inverte e reabre o caso 4:
1. **Semântica invertida.** P351: "`[]` **vaza**". Fonte (spike §B4 + vanilla): a recipe
   **não vaza** — **confina**, em `[]` e em `{}`. A frase "`[]` vaza" é o **bug pré-P340**
   (citado no próprio `f3s3`: "Antes (eager): … VAZAVA"), não o alvo.
2. **Sem diferença `[]`-vs-`{}`.** P351 enquadra `[]` e `{}` como **diferentes** (um vaza,
   outro confina). Medido: **ambos confinam** — não diferem. (A única diferença foi
   **histórica**: `[]` tinha o bug de vazar; `{}` sempre confinou via `CodeBlock`. O P340
   igualou.)
3. **Já feito.** O caso 4 (confinamento) está implementado (P340, clone de `local_show_rules`)
   e testado (`f3s3`). Não há o que implementar.

**Implicação:** implementar "`[]` vaza" seria **regressão tripla** — divergir do vanilla,
desfazer o P340, e quebrar o `f3s3`. **Não faço.** (ADR-0108 regra 1 + a instrução do próprio
P351: "se a fonte contradisser o plano, a fonte vence e a divergência é reportada".)

## O que de P351 sobra de fato (para o dono re-escopar)
- **Caso 4: FECHADO** (P340). Nada a fazer; o enunciado "vaza" deve ser corrigido para
  "confina (já feito)".
- **Estágio 0 — o transporte `StyledElem`-scoped:** é **fundação real** para os casos 1
  (composição) e 3 (show-set), P352/P353 — **mas o caso 4 NÃO precisou dele** (P340 confinou
  com o mecanismo mais leve: clonar `local_show_rules`, espelhando o `{}`). O próprio spike
  modelou o escopo "por chains separadas (inner/outer), **em vez de** [o StyledElem]"
  (`:220-222`). Logo **construir o transporte agora não é justificado por este lote** (cujo
  alvo declarado — caso 4 — está feito/invertido); a necessidade e o momento do transporte
  são uma decisão a **medir** quando os casos 1/3 forem o lote (P352+), não a assumir aqui
  (disciplina: não construir infra à frente da demanda).

## Recomendação (do agente, marcada)
**Re-escopar o P351.** O caso 4 está feito e seu enunciado está invertido. Opções para o dono:
- **(A)** Encerrar o P351 como "caso 4 já fechado (P340); enunciado corrigido" e fazer o
  próximo lote ser **casos 1/3** (P352), que aí **medem** se o transporte `StyledElem`-scoped
  é necessário (Fase A própria) antes de construí-lo.
- **(B)** Se o objetivo real era **o transporte** (a fundação multi-passe), redigir um P351'
  que o **meça e justifique** contra os casos 1/3 — não sob o pretexto do caso 4.
- **Não recomendado:** implementar "`[]` vaza" — é regressão vs vanilla + P340 + `f3s3`.

## Intactos (confirmado — nenhum código escrito)
Caso 2 (recursão), `morph_canon`/`==` (P345), flag de diagnóstico (P350c), DEBT-59 (CLI),
Marco G (`edges content→elements = 66`): **todos intocados** — a Fase A só leu fonte e
compilou probes descartáveis (removidas; árvore de produto limpa).

## Verificação (gates até a trava)
```
content-preserving (Fase A): zero .rs/.toml. Árvore limpa. Suíte 2729 não re-rodada.
lint: crystalline-lint . = 0/0.
medição: spike §B4 (file:line), L0 §3a.7-bis, vanilla compilado (`[]`/`{}` ambos "Za" =
  confinam), `f3s3` (tests.rs:480). Vanilla 0.14.2 como oráculo. Zero "~".
fronteira: PAROU na Fase A — contradição medida; nenhum código; nenhuma asserção alterada.
ADR-0108 aplicada (3ª vez): a medição pegou o enunciado invertido + a redundância (caso 4
  já feito). O plano/spike pré-P340 não sabia que o P340 fechou o caso 4.
```

## Mapa de filtro (campo)
**Lugar lógico:** o P351 foi redigido do **plano pré-P340** e herdou a descrição do **bug**
(`[]` vaza) como se fosse o **alvo** — quando o P340 já tinha **fechado** o caso 4 (confina,
paridade vanilla, `f3s3`). A **disciplina anti-deriva (ADR-0108, P349)** pegou: a Fase A
**mede da fonte** (spike §B4 + vanilla + o teste) **antes de decidir**, e a medição
**contradiz e refuta** o enunciado — exatamente o modo de falha "decidir da narrativa
acumulada em vez de medir". **Rastro:** P340 fechou o caso 4; o plano/spike (pré-P340)
descreviam-no aberto e com a polaridade do bug; P351 herdou isso; P351-Fase-A mede e reporta.

## Item carregado
`content→elements` aponta para o **Marco G** (P346) — não volta como órfão; inalterado (66).
