# Tarefa P333 — Fronteira E1 decidida: spike-2 do `#show` + L0 completo do F + plano de lotes (parar na Trava)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P333 (confirmar livre).
**Pré-condição**: P332 fechado — experimento E1/E2/E3, tabela comparativa,
produto intocado, suíte 2708, lint 0. Se não, parar.
**Tipo**: (Parte 1) registro da decisão da fronteira — **decisão do dono,
zero código** + (Parte 2) spike-2 do `#show` — experimento descartável +
(Parte 3) **L0 completo do F sob a fronteira E1** + (Parte 4) plano de
lotes + (Parte 5) baseline estrutural com a lente. **Zero código de produto
em todo o passo.** Execução longa autônoma permitida (regras P331/P332:
bloqueio vira pergunta numerada; progresso em disco
`f-progresso-passo-333.md`; commits por parte; ordem por valor 1→2→3→4→5;
a Parte 5 é independente das 2–4 e pode rodar em paralelo após a 1).
**Fontes**: `f-experimento-extensao-passo-332.md` (tabela + spikes),
inventários 1a/1b/1c (P331), dossiê P331, DEBT 99.E, triagem P329,
`lab/typst-original/` (leitura autorizada para semântica do `#show`; nunca
importar), trait `Element` + 65 módulos (precedente), baseline 10× (P330),
repo do `tekt-cargo-dsm` (a lente — Parte 5; registrar versão/commit).
**Commits**: "Passo 333 — decisão da fronteira (registro)" · "Passo 333 —
spike-2 show" · "Passo 333 — L0 do F" · "Passo 333 — plano de lotes" ·
"Passo 333 — baseline estrutural (lente)".

---

## Parte 1 — A decisão da fronteira (do dono, a gravar; zero código)

Gravar em **ADR nova** (`ADR-01xx-fronteira-de-extensao-e1.md`, numerar
livre) + DEBT 99.E + nota de fecho no `f-experimento-extensao-passo-332.md`
e adendo final no dossiê P331:

1. **Fronteira escolhida: E1** — `Content::Dynamic(Arc<dyn Element>)` como
   única variante de extensão; despacho pelo `trait Element` existente; os
   65 nativos permanecem monomórficos. **Razões** (citar a tabela P332):
   nativos sem imposto (§4*), precedente vivo (mesmo trait dos 65 → menor
   custo-IA), migração aditiva. **E2 rejeitada** (falha silenciosa de tipo;
   fidelidade estrutural não é requisito — P329). **E3 não escolhida**
   (comportamento em `fn` ptrs = menos precedente; teto do `Value` no lugar
   errado), registrada como alternativa.
2. **Valores settáveis**: o `enum Value` das propriedades é **fechado e
   espelha os tipos de valor da linguagem typst** (fidelidade à linguagem —
   o conjunto é fechado pela própria linguagem; tipo de valor novo =
   extensão de linguagem = mudança de core legítima e rara). Uma variante
   de escape `Value::Custom(Arc<dyn …>)` (contrato eq/hash/display; eco do
   `Value::Dyn` vanilla) **só entra se** o spike-2 ou o L0 demonstrarem
   necessidade — não por precaução. A decisão final do escape é item do
   checkpoint da Trava.
3. **Migração incremental**: `Set*`/`Styled`/3 folhas provisórias **não**
   são absorvidas pela fronteira no primeiro movimento. Ordem: fronteira
   (aditiva) → canal único das `Set*` desenhado sobre a fronteira (o F-D na
   forma nova) → `Styled`/folhas por lote quando o desenho os alcançar.
4. **Q1–Q7 do P332**: respondidas por consequência (Q1 custo-IA/precedente;
   Q2 sim com §4*; Q3 resolvida pela linguagem — item 2; Q4 eliminatório;
   Q5 incremental; Q6 spike-2 sim — Parte 2; Q7 sim — Partes 3–4).

## Parte 2 — Spike-2: `#show` real no harness E1 (descartável)

Estender `lab/spikes/f-extensao/e1/` (mesmas regras: fora do gate, zero
import de produto/quarentena). Antes de codificar, **ler a semântica do
`#show` na quarentena** e registrar com `file:line` os comportamentos a
exercitar. Mínimo:

1. **Multi-regra**: 2+ regras `#show` sobre o mesmo elemento — ordem de
   aplicação (a do vanilla: mais recente primeiro? medir lá, registrar).
2. **Multi-passe/recursão**: regra que produz conteúdo que contém o próprio
   elemento — terminação e semântica (o vanilla revoca a regra dentro do
   próprio corpo? `Revocation`/guards, 1b).
3. **Show-set** (`#show heading: set text(...)`): regra que é um set —
   interação com a chain.
4. **Escopo**: regra dentro de bloco não vaza para o irmão.
5. **Sobre elemento de usuário E o `callout` dinâmico** — os dois públicos.

Saída: o spike rodando os 5 casos + nota de medição
(`f-spike2-show-passo-333.md`): o que a semântica do vanilla exige da
fronteira E1 (ex.: a regra precisa de identidade de elemento estável?
de revocação por nó? de ordem na chain?) e **o que isso muda no L0** —
cada achado vira requisito numerado (S1, S2, …) consumido na Parte 3. Se um
achado **contradisser** a fronteira E1 (caso de parada), registrar como
pergunta de Trava e prosseguir com o L0 marcando a dependência.

## Parte 3 — L0 completo do F sob a fronteira E1

Redigir os L0 (convenção do repo; hashes sincronizados; zero código):

### 3a — Lado elemento (`fronteira de extensão`)

- A variante `Content::Dynamic(Arc<dyn Element>)`: contrato do trait
  público (os 7 métodos atuais + o que o spike-2 exigir — ex.: identidade/
  kind dinâmico, props settáveis declaradas), object-safety verificada.
- O **registro** (construtores por nome para o público typst; injetado,
  sem global — pureza L1 do spike E1).
- `ElementKind`/payload dinâmicos (introspecção/query sobre elemento de
  usuário — eco de `to_payload`).
- O que o hub muda: +1 variante, 6 matches ganham o arm `Dynamic(e) =>
  e.método()` (dispatch idêntico aos 65) — aditivo, conferir
  content-preserving.
- Os dois públicos demonstrados no papel: o caminho Rust (implementa trait,
  registra) e o caminho typst (pacote/`#show`/composição — o que é possível
  sem Rust e o que exige Rust, dito explicitamente).

### 3b — Lado estilo (chain + mapa aberto + `Value`)

- A chain única do Layouter estendida: os **10 campos nativos fechados**
  (zero custo aos nativos) + **mapa aberto** `(PropKey dinâmica → Value)`
  para props de usuário; resolução por fallback instância→chain→default
  (C1+C3); fold onde aplicável (C2).
- O `enum Value` fechado espelhando os tipos da linguagem (lista concreta,
  derivada da quarentena com `file:line`); o escape `Custom` avaliado
  contra os achados S* do spike-2 (entra ou não — com a razão).
- **O canal único das `Set*` como primeira aplicação** (o F-D renascido):
  os 4 efeitos atuais como entradas na chain; `SetPage` (o caso difícil,
  D4) e a lacuna do produtor de `SetEquationNumbering` (D1/Q3 do P331)
  resolvidos no desenho; a rede de caracterização (+11, Fase 2 P331) citada
  como spec de paridade.
- `#show` no desenho conforme os S* (não além do que o spike validou;
  o que ficar argumentado, marcado como argumentado).
- Fronteiras declaradas: o que fica para lotes futuros (`Styled`, de-bake
  de `#set text`, 3 folhas — com gatilhos de revisita).

### 3c — Contrato de verificação

C1–C8 aplicáveis + S* do spike-2 + a rede de caracterização existente +
os testes novos que cada lote deverá trazer; a trava de verificação da
ADR-0105 (lint/teste-varre-tabela antes de perder o compilador no caminho
dinâmico) dimensionada com o M2.

## Parte 4 — Plano de lotes (proposta, não execução)

Sequência com custo pelo preditor (largura por grep, refeita): lote(s) da
fronteira (variante `Dynamic` + registro + trait público — aditivo), lote(s)
do canal `Set*` (~108 sites, 1c), e a fila incremental (`Styled`, folhas,
de-bake) com gatilhos. Cada lote na faixa validada (~110–150) ou com a
válvula declarada; baseline 10× citado para o antes/depois.

## Parte 5 — Baseline estrutural com a lente (`tekt-cargo-dsm`)

A lente entra no loop do typst pela primeira vez. **Zero código de produto;
zero conserto** — é medição estrutural, o análogo do baseline 10× para
arquitetura. Três entregas:

1. **Rodar a lente sobre o typst-crystalline** (instalar/buildar do repo do
   `tekt-cargo-dsm`; **registrar versão/commit** da lente usada). Produzir o
   **baseline estrutural pós-lotes**: DSM/grafo de dependência do `01_core`,
   acoplamento do hub (`content.rs`) com o resto, independência dos 65
   módulos `entities/elements/*` (quem importa quem; ciclos = 0 esperado),
   e as métricas que a lente expõe (registrar comandos). Saída:
   `00_nucleo/diagnosticos/baseline-estrutural-lente-passo-333.md` — os
   números que faltam ao balanço da fase ("atomização" e "separação de
   camadas" medidas por computação, não por narrativa).
2. **Limites encontrados viram requisitos de volta**: cada capacidade que a
   lente não tiver para este corpus (multi-crate, granularidade de módulo,
   métrica ausente) é registrada como **requisito numerado (R1, R2, …)** no
   próprio baseline — material para a sessão do `tekt-cargo-dsm` refinar
   (o ciclo projeto-real→ferramenta, precedente 0052 do linter). **Não
   consertar a lente neste passo**; registrar. Achados conhecidos da casa
   dela (ex.: V3=8 do 0053) não bloqueiam o uso como leitora.
3. **Incorporar a lente ao contrato de verificação do F** (amarra com a
   Parte 3c): o critério "separação de camadas" do L0 passa a ser **medido
   pela lente** antes/depois em cada lote do F (a fronteira E1 promete "o
   elemento de usuário importa só o trait + tipos públicos; o core não o
   conhece" — isso vira verificação computada, não argumento). Avaliar e
   registrar (sem executar) se a trava da ADR-0105 (M2: acesso a
   campo/propriedade que o F troca por lookup) é implementável como análise
   da lente em vez de grep mantido à mão — se sim, vira requisito R*; se
   não, registrar o porquê.

Se a lente não buildar/instalar no ambiente, registrar o erro exato como
R0, prosseguir com as outras partes (regra de bloqueio do passo), e o
baseline fica declarado pendente no relatório.

---

## TRAVA ARQUITETURAL — o passo termina aqui

Checkpoint no chat: a decisão gravada (Parte 1), os achados S* do spike-2 e
o que mudaram no desenho, o L0 em ~1 página, o plano de lotes, o item
aberto do `Value::Custom` (entra/não entra, com a evidência), os números do
baseline estrutural (Parte 5: acoplamento do hub, independência dos 65,
ciclos) + os requisitos R* para a lente, e qualquer pergunta acumulada.
**Nenhum código de produto até o dono aprovar o L0.**

## Relatório (`typst-passo-333-relatorio.md` + resumo no chat)

Estado por parte; onde cada registro mora; os 5 casos do spike-2 com
resultado; os S*; os L0 (paths + resumo por seção); o plano de lotes; o
baseline estrutural (versão da lente, comandos, números-chave) e os R*
(ou o R0 se a lente não rodou); produto intocado (`git status` limpo fora
de `lab/` e docs); suíte 2708 verde, lint 0 no produto; caveat do stack
(`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

Implementação de qualquer parte do F (depois da Trava, por lotes);
mudanças em `Set*`/`Styled`/folhas (desenhadas, não tocadas); consertos
B1/B2/B3 (destinos: B1 no canal `Set*`, B2 no lote de `Styled`, B3 no
primeiro lote do F — só desenho aqui); **consertos/refinamentos na lente**
(os R* são material para a sessão do `tekt-cargo-dsm`, não deste passo);
otimizações.
