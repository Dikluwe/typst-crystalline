# ⚖️ ADR-0106: Fronteira de extensão do F — E1 (`Content::Dynamic(Arc<dyn Element>)`)

**Status**: `EM VIGOR`
**Data**: 2026-06-12
**Passo promotor**: P333 (grava a decisão do dono; a implementação é por lotes, pós-Trava)
**Experimento**: P332 (`diagnosticos/f-experimento-extensao-passo-332.md` — spikes E1/E2/E3 + tabela comparativa de 7 critérios)
**Diagnóstico-mãe**: P331 (`diagnosticos/f-dossie-opcoes-passo-331.md` + inventários 1a/1b/1c)
**Categoria**: Arquitectural / Fronteira de extensão de elementos e estilo
**Cross-ref**: ADR-0105 (Modelo D agora, F destino — esta ADR **fixa a forma de F**),
              ADR-0104 (Atomicidade para agentes — o critério de custo-IA que decide),
              ADR-0029/0030 (pureza física / RAM em L1 — o `dyn` na folha respeita-as),
              ADR-0026 (Content enum fechado — **complementada**: o enum continua fechado; `Dynamic` é UMA variante),
              `00_nucleo/debt-stylechain-nao-materializada.md` (DEBT 99.E — F resolve-o sob esta fronteira)

---

## Contexto

A ADR-0105 adoptou o modelo **D** (enum fino com delegação por `trait Element`)
incrementalmente e declarou **F** (propriedades reificadas / PropMap + StyleChain
materializada) como destino, a executar junto com o DEBT 99.E. O roteiro de lotes
de D **encerrou** (65 variantes migradas, P316–P330). O P331 diagnosticou o F
(inventário factual + rede de caracterização + dossiê de 4 opções A/B/C/D) sem
decidir. O dono então fixou um **requisito que reabriu a decisão**: a
extensibilidade é requisito **total** do projeto — utilizadores definem elementos
novos que são **cidadãos plenos** (`#set`/`#show`/`query`/render) **sem tocar o
core** — com dois públicos (programador Rust; autor typst). A Opção A (declarar e
congelar) ficou **rejeitada**. B/C/D foram re-avaliadas **pelo experimento**
(P332), não por argumento.

O P332 construiu 3 spikes descartáveis (`lab/spikes/f-extensao/{e1,e2,e3}/`), cada
um implementando o mesmo `callout{body,title,tone}` de ponta a ponta, e mediu-os
nos 7 critérios fixados antes de medir. **Os três zeram os ficheiros de core por
elemento novo** (o requisito de atomização). A escolha separou-se pelos critérios
secundários do dono (custo-IA, separação de camadas, precedente).

---

## Decisão

### 1. Fronteira escolhida: **E1** — fronteira por trait

`Content::Dynamic(Arc<dyn Element>)` é a **única** variante de extensão. Os 6
matches do hub (`content.rs`) ganham o arm `Self::Dynamic(e) => e.método()` —
despacho **idêntico** ao dos 65 nativos. Os **65 nativos permanecem
monomórficos** (`Variant(Arc<Elem>)`, sem imposto de vtable). Propriedades: os
campos nativos **fechados** + um **mapa aberto** `(PropKey dinâmica → Value)` para
props de utilizador.

**Razões (citando a tabela P332):**

- **Nativos sem imposto** (§4* da tabela): nos três desenhos o caminho dinâmico
  não onera os nativos — `dyn`/downcast/tabela só tocam a folha de utilizador.
  Logo o `dyn` de E1 satisfaz ADR-0029/0030 para os 65 já migrados. Facto
  robusto e comparável (os demais números de perf são stub).
- **Precedente vivo → menor custo-IA** (critério 3 da tabela; ADR-0104): o
  elemento de utilizador implementa o **mesmo `trait Element`** que os 65 módulos
  já instanciam. Uma IA (ou humano) escreve um elemento novo por **imitação
  directa**, com o menor raciocínio não-local da tabela. E1 lidera em custo-IA.
- **Migração aditiva**: `Dynamic` é uma variante **nova**; os 65 nativos
  **ficam**. O movimento da fronteira não reescreve o existente — o custo de
  migração mais barato dos três.

### 2. `enum Value` fechado, espelhando a linguagem typst

O `enum Value` das propriedades é **fechado e espelha os tipos de valor da
linguagem typst**. O conjunto é fechado **pela própria linguagem**: um tipo de
valor novo = extensão de linguagem = mudança de core **legítima e rara** (não é o
caso comum de "elemento/prop novo", que continua a custar 0 ficheiros de core).
Isto coloca o teto medido em E3 (o `Value` fechado) no **lugar certo** — na
fronteira da linguagem, não na do elemento.

Uma variante de escape `Value::Custom(Arc<dyn …>)` (contrato `eq`/`hash`/`display`;
eco do `Value::Dyn` do vanilla) **só entra se** o spike-2 do `#show` (P333 Parte 2)
ou o L0 (Parte 3) demonstrarem necessidade — **não por precaução**. A decisão
final do escape é **item do checkpoint da Trava** (P333), com a evidência na mão.

### 3. Migração incremental — a fronteira não absorve tudo no primeiro movimento

`Set*` (as 4 variantes de set), `Styled` e as 3 folhas provisórias **não** são
absorvidas pela fronteira no primeiro movimento. A ordem gravada:

```text
fronteira (aditiva: variante Dynamic + registro + trait público)
  → canal único das Set* desenhado SOBRE a fronteira (o F-D na forma nova)
  → Styled / folhas por lote, quando o desenho os alcançar
```

### 4. Descartes com razão

- **E2 (type-erased à vanilla)** — **rejeitada**. Traz o **downcast silencioso**
  (`#set` de tipo errado falha **sem erro** — o pior resultado de custo-IA da
  tabela) e maximiza fidelidade **estrutural** ao vanilla, que **não é
  requisito** (a fidelidade é **comportamental**, P329). Migração mais cara
  (~283 sites, de-bake de `#set text`).
- **E3 (registro aberto por kind, sem `dyn`)** — **não escolhida**, registrada
  como alternativa. O comportamento mora em `fn` ptrs (não no trait) → **menos
  precedente** que os 65 módulos; e o teto do `enum Value` ficava no lugar errado
  (na fronteira do elemento). E1 obtém o mesmo "0 ficheiros de core" com mais
  precedente. (Em E1, o `Value` fechado fica na fronteira da **linguagem** —
  item 2 — que é onde ele pertence.)

### 5. Q1–Q7 do P332 respondidas por consequência

- **Q1** (eixo de desempate): custo-IA + precedente vivo → E1.
- **Q2** (`dyn` numa única variante, nativos monomórficos): **sim**, com o §4*.
- **Q3** (teto do `Value`): resolvido pela linguagem — item 2 (fechado por
  espelhar a linguagem; escape `Custom` condicional).
- **Q4** (downcast silencioso de E2): **eliminatório**.
- **Q5** (destino de `Set*`/`Styled`/folhas): **incremental** — item 3.
- **Q6** (`#show` na linguagem): **spike-2 sim** — P333 Parte 2 valida antes de
  fixar o desenho do `#show`.
- **Q7** (a fronteira vira F-D/F-B + próximo passo é o L0): **sim** — P333
  Partes 3–4 redigem o L0; código só pós-Trava, com hash humano.

---

## Relação com ADR-0105 e ADR-0026 (explícita)

- **ADR-0105** declarou F como destino e pediu a forma; esta ADR **fornece a
  forma**: F materializa-se sob a fronteira E1. A trava de verificação da
  ADR-0105 cláusula 3 (repor a verificação mecânica que o compilador deixa de
  dar no caminho dinâmico) **permanece obrigatória** e é dimensionada no L0
  (P333 Parte 3c) com o M2.
- **ADR-0026** (enum `Content` fechado) **continua válida**: o enum permanece
  fechado; `Dynamic` é **uma** variante a mais, não uma abertura geral. A
  exaustividade do compilador sobre as 66 variantes (65 nativas + `Dynamic`)
  mantém-se; o que é dinâmico é **o conteúdo** da variante `Dynamic`, despachado
  pelo trait — o mesmo trait que os nativos usam.

---

## Consequências

**Positivas**: entrega a extensibilidade total (dois públicos, 0 ficheiros de
core por elemento novo) com o **menor custo-IA** e **maior reaproveitamento do
precedente** dos três desenhos; migração aditiva (não reescreve os 65);
`Value` fechado no lugar certo (fronteira da linguagem).

**Negativas**: introduz `dyn` na folha de extensão (aceitável por §4*; os nativos
ficam monomórficos); a trava de verificação da ADR-0105 cláusula 3 continua a ser
trabalho obrigatório no F; `#show` na forma dinâmica precisa de validação
(spike-2) antes de fixar.

**Neutras**: o escape `Value::Custom` fica em aberto até evidência (não se decide
por precaução).

---

## Alternativas Consideradas

| Alternativa | Prós | Contras | Veredito |
|-------------|------|---------|----------|
| **E1 — trait/`dyn`** (esta) | menor custo-IA; precedente vivo; nativos sem imposto; migração aditiva | `dyn` na folha | **escolhida** |
| E2 — type-erased | fidelidade estrutural ao vanilla | downcast silencioso; ~283 sites; fidelidade estrutural não é requisito | rejeitada |
| E3 — kind+PropMap (sem `dyn`) | o mais atómico; sem `dyn` no Content | comportamento em `fn` ptrs (menos precedente); teto do `Value` no lugar errado | não escolhida (alternativa) |
| A — declarar e congelar (P331) | custo 0 | congela contra o requisito de extensibilidade total | rejeitada (Parte 0 P332) |

---

## Nota

P333 **grava** esta decisão e redige o **L0 do F** sob ela; **não** implementa
nenhum código de produto. A implementação é **por lotes, depois da Trava**, com o
L0 aprovado e o hash humano. A decisão final do escape `Value::Custom` é item do
checkpoint da Trava, com a evidência do spike-2 e do L0.

---

## Aprovação da Trava (P334 — checkpoint do dono fechado)

O dono **aprovou** o L0 `entities/f_fronteira_e1.md` no checkpoint do P333. As
respostas que destravam o código (gravadas para o leitor futuro):

1. **L0 aprovado** — blanket `impl<T: Element> DynElement for T`; chain única;
   `Value` fechado; canal `Set*` (F-2); `#show` por S3–S6.
2. **Trava-Q1**: guard/lifecycle vive na **camada de realização** (`rules/`), em
   invólucro **transparente** uniforme nativo+dinâmico — **não** no `Content` nem
   no trait. Condição: a transparência é provada por teste quando o invólucro
   nascer (F-2; F-1 não o traz).
3. **Trava-Q2 confirmada**: `dyn_kind_name()` estável `Eq` + `get_field` são
   contrato obrigatório do elemento dinâmico (S1/S7).
4. **`Value::Custom` fica fora** — gatilho de reabertura registrado (§3b.4 do L0).
5. **Plano de lotes aprovado**: F-1 → F-2 → fila incremental; a trava ADR-0105
   cláusula 3 constrói-se em F-1/F-2 antes de relaxar o compilador.
6. **R4 da lente**: sessão `tekt-cargo-dsm` abre em paralelo (não bloqueia F-1).

O L0 passa de **design-ahead** a **ativo** a partir do lote **F-1 (P334)**: o
código F-1 declara `@prompt entities/f_fronteira_e1.md` e o warning V7 (órfão)
limpa. Ajuste de Fase A registrado: o `dyn_hash` do L0 §3a.3 é **removido** —
`content_hash::hash_content` serializa por `format!("{:?}")` (Debug), logo
`Content::Dynamic` precisa só de `Debug` (que `DynElement` já exige); `dyn_eq` +
`as_any` permanecem (o `eq` do hub é um match, precisa do arm dinâmico).
