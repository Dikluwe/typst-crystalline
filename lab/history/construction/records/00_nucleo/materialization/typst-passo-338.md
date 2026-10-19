# Passo 338 — Lote F-4 `Styled`: colapso da dualidade de backing + B2 (com triagem-47 como Estágio 0)

**Pré-condição**: P337 fechado (`1be7083df`) — recon dimensionador feito, ordem
**F-4 → F-realização → F-5 → F-6** decidida e registrada na fila. Suíte 2717
**ou** 2718 (a contabilidade das caronas do P337 ficou sem declaração — C0
abaixo resolve), lint 0/0.
**Tipo**: caronas (commit próprio) + **Estágio 0 (triagem-47, commit próprio)**
+ **Lote F-4** em estágios isoláveis. Content-preserving: **nenhuma asserção
existente alterada** (critério transversal 1 do plano).
**Objetivo** (conforme a correção do recon P337 — o alvo é a **dualidade de
backing**, não uma "2ª chain"): **(a)** colapsar `StyleDelta` ↔ `Styled.Styles`
num backing único que `#set`/`#show`/Layouter leem direto; **(b)** fechar
**B2** (`is_empty` sem arm `Styled`, `content.rs:1566`). F-4 é a fundação que
F-realização, F-5 e F-6 leem.

---

## Caronas (commit próprio)

- **C0 — contabilidade pendente do P337**: declarar no relatório do P337 (ou
  no deste passo, com ponteiro) o que o `1be7083df` fez de fato: o C1
  adicionou o teste-contrato da `FuncRepr::Element` (**+1**) ou apontou um
  existente (**0**)? Suíte resultante exata (2717 ou 2718)? Lint? **A
  pré-condição deste passo é esse número** — fixá-lo antes de qualquer delta.
- **C1 — a condição do tampão F-6 no plano**: uma frase em
  `f-plano-lotes-passo-333.md`, na linha do F-6: *se usado como tampão
  (rotear pela chain mantendo o campo assado), o caminho duplo nasce com
  gatilho de remoção escrito (o F-5 remove o campo) e com teste de paridade
  entre os dois caminhos enquanto coexistirem* — a lição do S5b
  (morto/redundante-mas-alimentado mascara) aplicada antes do fato.
  **Δ testes: 0.**

## Estágio 0 — Triagem dos 47 `is_numbering_active` (commit próprio; lição do S5b)

A pergunta aberta do recon ("vivo ou morto pós-F-2?") não atravessa F-4 e
F-realização dormindo — é exatamente a configuração que mascarou o auto-TOC no
S5b, e a F-realização vai mexer em show/introspecção com esse caminho duplo no
meio se ele não for triado agora.

1. **Mapear (file:line)**: quem **produz** as chaves `numbering_active:*` no
   StateRegistry **hoje** (pós-F-2 S5b: o eval não emite mais os marcadores —
   resta algum produtor? qual?); quem **lê** via
   `is_numbering_active`/API legada das 47 referências em `introspector.rs`.
2. **Prova de mordida**: se houver leitor aparente, teste que muda o estado
   que ele leria e observa o efeito — ou a demonstração de que nenhum input
   de produção alcança o caminho.
3. **Desfecho A — morto**: remover as 47 referências + a API legada + os
   testes que só testavam a plumbing (deltas **exatos**: apagados vs
   migrados, sem "~" — disciplina S5b). Registrar no plano: **o F-5 encolhe**
   (a linha de risco da tabela do recon sai).
4. **Desfecho B — vivo**: registrar **o quê** o mantém vivo (`file:line`, o
   caso de produção que o alcança) e gravar na fila: **o F-5 tem 5 pontos,
   não 4** — o de-bake terá de religar também este consumidor.
5. **Desfecho C — ambíguo** (leitor vivo de chave que ninguém produz, ou
   vice-versa): parar, registrar, checkpoint do dono — ambiguidade aqui é
   sintoma de migração incompleta do F-2, e conserto inline sem decisão é
   como o mascaramento começa.

Suíte verde após o estágio, com o delta declarado. **Este estágio não toca o
backing do Styled** — é triagem do canal de numeração, paralela ao F-4.

## Fase A — reconhecimento do F-4 (confirmar o mapa do recon, `file:line`)

O recon P337 já mapeou; a Fase A **confirma contra o HEAD** (a lição P331: não
desenhar sobre inventário, mesmo recente) e completa o que ficou "a medir":

1. **O mapa confirmado**: produtores diretos de `Content::Styled` em produção
   (**2**: `Content::strong` `content.rs:1035`, `Content::emph` `:1043`, com
   a stdlib passando por eles); preservadores (**3**: `map_content :2023`,
   `map_text :2193`, walk `introspect.rs:348`); consumidores (**~7 → contar
   exato**: push/pop `layout/mod.rs:1248-1256`, `plain_text :1578`,
   `PartialEq :1785`, deteção bold/italic `rules.rs:113,115`,
   `locatable.rs:131`, walk `introspect.rs:1205` — e o que mais o grep
   dirigido achar).
2. **A dualidade em detalhe**: `style_chain.rs:12` ("coexistência intencional
   até o pipeline migrar") — o que exatamente `StyleDelta` carrega (10 campos
   fechados + `custom` do F-2) vs o que `Styled.Styles` (enum) carrega; onde
   um converte no outro hoje; o papel da cache achatada `self.style:
   TextStyle` (`layout/mod.rs:97-98`) — **ler, não tocar** (o TextStyle
   assado é o ponto 4 do F-5, fora deste lote).
3. **A superfície de teste**: contar **exato** os testes que constroem ou
   asseriam `Styled`/`strong`/`emph` (o "a medir no arranque" da tabela do
   recon) — o número entra no relatório antes do primeiro commit de S2.
4. **Vanilla como referência da forma**: `StyleChain<'a>` emprestada +
   `Resolve` a tempo de realize/layout
   (`lab/.../foundations/styles.rs:564-807`) — a forma-alvo conceitual; a
   forma concreta daqui respeita as restrições do cristalino (Arc, E1
   Send+Sync — precedente do S1 do P336).
5. **A direção do colapso — as duas opções para o checkpoint**:
   - **(i)** `Styled` passa a carregar `StyleDelta` (o backing dos accessors
     vira o payload transportado; o enum `Styles` morre);
   - **(ii)** `Styles` permanece como superfície e converte para `StyleDelta`
     **uma vez, na borda** (push), com os accessors lendo só `StyleDelta`.
   O critério de escolha: **um** backing lido por todos os consumidores, zero
   conversão dupla, e o menor churn nos 2+3+N sítios mapeados. **Checkpoint
   do dono com a recomendação fundamentada** se a Fase A mostrar custo
   assimétrico; se for unilateral e óbvio, registrar e seguir (precedente: o
   checkpoint só dispara em contradição/assimetria).

## Fase B — estágios (commit por estágio)

| Estágio | O quê |
|---|---|
| **S1 — B2** | O arm `Content::Styled` no `is_empty` (`content.rs:1566`): styled-de-vazio reporta vazio. **Antes do fix**: grep por dependentes do comportamento errado (algum teste/consumidor conta com `false`?) — se houver, é caracterização a registrar, não asserção a mudar silenciosamente. Teste novo do comportamento correto. Isolado e barato — vai primeiro para não viajar dentro do colapso. |
| **S2 — o colapso** | A direção decidida na Fase A: backing único, push/pop do Layouter (`layout/mod.rs:1248`) e accessors lendo a mesma representação; a conversão (se a opção ii) acontece **uma vez na borda**, com teste de que não há segunda. Migrar os 2 produtores + 3 preservadores + consumidores contados, **estágio a estágio se o churn pedir** (S2a produtores/borda, S2b consumidores — válvula declarada). Asserções existentes **intactas** (content-preserving: strong/emph renderizam idêntico). |
| **S3 — a trava do backing único** | O teste que impede a dualidade de renascer: a representação antiga (o que morreu — o enum `Styles` ou a conversão dupla) **não é construível/alcançável** de produção, ou o varre-consumidores que falha se um arm novo ler outro backing (a forma exata sai do que S2 escolheu; o requisito é que exista — eco da trava S5a do F-2). |
| **S4 — registro e fecho** | `style_chain.rs:12` atualizado (a coexistência "até migrar" terminou — o comentário não pode virar fóssil); B2 marcado fechado no plano; o doc retomável do F-4 fechado sem seção fóssil; F-4 ✅ na fila (próximo: F-realização). |

## Verificação transversal (critérios do plano, todos)

- **Suíte verde, nenhuma asserção existente alterada**; deltas **exatos** por
  estágio (adicionados/apagados/migrados — números, não "~").
- **Lint** `crystalline-lint .` 0/0 a cada commit.
- **Perf — protocolo C1** (P335): par **back-to-back na mesma sessão**
  antes/depois do S2 (o absoluto 0.6518 está superseded); reportar o par. O
  colapso não deve regredir o caminho quente do push/pop.
- **Lente por lote**: `--estrutura` (edges `content→elements::*`) e
  `--comparar` antes/depois do lote — o colapso da dualidade deve aparecer
  (ou a ausência de efeito estrutural, declarada). Registrar
  `tekt-cargo-dsm@<commit>` usado.
- **L0 primeiro** (Trava arquitetural): auditar/atualizar
  `entities/f_fronteira_e1.md` + sincronizar o hash **antes** do código de S2.
- **Caveat de stack**: `RUST_MIN_STACK=33554432`.

## O que NÃO fazer

- **Não tocar o TextStyle assado do `Content::Text`** (cache achatada
  `self.style` incluída) — é o ponto 4 do F-5; este lote só unifica o
  backing que será lido depois.
- **Não des-assar nada** — F-5. **Não tocar show/realização** — F-realização
  (o Estágio 0 tria o canal de numeração do introspector, que é outra coisa).
- **Não mudar asserção existente para passar** — content-preserving; se uma
  asserção depende do bug B2, isso é achado a registrar com decisão, não
  edição silenciosa.
- **Não deixar a representação morta "por segurança"** — S3 existe para o
  contrário; morto-alimentado mascara (S5b).
- **Não estimar com "~" no relatório final** — os "~7" e "a medir" do recon
  viram números na Fase A.

## Critérios de Verificação

```
Dado o C0
Então a suíte-base deste passo declarada com o número exato herdado do P337

Dado o Estágio 0
Então os 47 triados com desfecho único (morto: removidos com deltas exatos e
F-5 encolhido na fila / vivo: 5º ponto do F-5 registrado com file:line /
ambíguo: parado com checkpoint) — nada atravessa para a F-realização sem
veredito

Dado o S1
Então styled-de-vazio reporta vazio; dependentes do comportamento antigo
inexistentes ou registrados com decisão

Dado o S2
Então um backing único lido por todos os consumidores mapeados; conversão no
máximo uma vez na borda; strong/emph content-preserving (asserções intactas)

Dado o S3
Então a trava existe e falha se a dualidade renascer

Dado o fecho
Então lint 0/0; par de perf C1 reportado; lente antes/depois registrada;
L0 auditado antes do código; commits isoláveis (caronas / E0 / S1 / S2[a,b] /
S3 / S4); F-4 ✅ na fila com B2 fechado; style_chain.rs:12 sem fóssil
```

---

## Histórico

| Data | Motivo |
|---|---|
| 2026-06-11 | P338 — Lote F-4 conforme a ordem decidida no P337: **(a)** colapso da dualidade de backing `StyleDelta`↔`Styled.Styles` (a correção do recon: não há "2ª chain"; há um backing duplo com coexistência declarada "até migrar" em `style_chain.rs:12` — este lote é o migrar), direção decidida na Fase A entre payload-vira-StyleDelta e conversão-única-na-borda, com trava S3 contra o renascimento da dualidade; **(b)** B2 fechado (`is_empty` arm Styled, `content.rs:1566`) como S1 isolado com varredura prévia de dependentes do bug. **Estágio 0** (commit próprio): triagem dos 47 `is_numbering_active` do `introspector.rs` — a lição do S5b aplicada antes da F-realização atravessar show/introspecção com caminho duplo não-triado; desfechos morto/vivo/ambíguo com efeito registrado na dimensão do F-5. Caronas: C0 fecha a contabilidade pendente das caronas do P337 (suíte-base exata); C1 grava a condição do tampão F-6 (gatilho de remoção + paridade enquanto coexistir). Content-preserving estrito; TextStyle assado e show fora do lote (F-5/F-realização); perf por protocolo C1; lente antes/depois; L0 primeiro. |
