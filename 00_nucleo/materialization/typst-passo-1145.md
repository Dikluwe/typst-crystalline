# P1145 — auditoria integral dos constructors e transformações de `gradient`

**Data:** 2026-08-24  
**Estado:** `CONCLUÍDO — gate aprovado; scope gradient 16/16`  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Owner primário:** `01_core/src/compiler/stdlib/gradients.rs`  
**Dependência:** P1144 concluído e commitado em
`c65c839ff5544a66683b7e81edba923acb49760e`.

## 1. Objetivo

Auditar integralmente os constructors `gradient.linear`, `gradient.radial` e
`gradient.conic`, incluindo stops, argumentos nomeados, defaults, validações e
erros. Completar simultaneamente a varredura do scope público de `gradient`
antes de declarar a família fechada.

P1145 nasce de duas contraprovas encontradas após P1144:

1. o vanilla aceita `focal-center:` e `focal-radius:`, enquanto o cristalino
   vigente consulta apenas `focal_center` e `focal_radius`;
2. a fonte ratificada exige pelo menos dois stops nas três variantes, enquanto
   o cristalino vigente rejeita apenas zero.

A releitura desde o início do bloco `#[scope] impl Gradient` também encontrou
`sharp` e `repeat` antes de `kind`. Portanto, a afirmação P1144 “11/11 membros”
é incompleta: são onze accessors/operações já materializados por P1144, mas o
scope contém pelo menos essas duas transformações adicionais. P1145 deve medir
o bloco inteiro e corrigir a documentação; não pode inserir apenas dois nomes
num dispatcher.

## 2. Proveniência inicial

- HEAD em `2026-08-24T19:33:20-03:00`:
  `c65c839ff5544a66683b7e81edba923acb49760e`, branch `Tekt`.
- `git status --short` e `git diff HEAD --stat` sem saída: árvore limpa.
- Fonte ratificada inicial:
  `a51e02804:crates/typst-library/src/visualize/gradient.rs:243-875`.
- L0s vigentes a auditar antes de qualquer código:
  `00_nucleo/prompts/compiler/stdlib/gradients.md`,
  `00_nucleo/prompts/entities/gradient.md` e, caso o dispatch de instância seja
  ampliado, `00_nucleo/prompts/compiler/eval/call_dispatch.md`.

Todo número decisório deve registrar novamente HEAD, hora e
`git diff HEAD --stat`.

## 3. Fase A — inventário completo da fonte ratificada

Ler o bloco `#[scope] impl Gradient` inteiro, do primeiro constructor ao fecho,
e produzir tabela com:

```text
membro | kind | assinatura Typst | defaults | validações |
desde | estático | instância | owner semântico
```

O inventário mínimo já comprovado na fonte é:

```text
linear, radial, conic, sharp, repeat,
kind, stops, space, relative, angle, center, radius,
focal-center, focal-radius, sample, samples
```

Essa lista ainda é hipótese até a varredura alcançar literalmente o fim do
scope. Métodos Rust sem `#[func]`, como helpers internos, não entram na
superfície da linguagem.

Para os constructors, extrair da fonte:

- `linear`: `..stops`, `space`, `relative`, `angle` e o argumento externo
  `dir`, incluindo precedência `angle` sobre `dir` e defaults direcionais;
- `radial`: `..stops`, `space`, `relative`, `center`, `radius`,
  `focal-center`, `focal-radius` e validações geométricas;
- `conic`: `..stops`, `angle`, `space`, `relative`, `center`;
- regras comuns de `GradientStop` e `process_stops`, sem inferi-las da
  representação cristalina.

Para `sharp` e `repeat`, registrar transformações de stops, preservação de
variant/fields, argumentos, limites e erros. Distinguir semântica observável de
detalhes mecânicos do algoritmo Rust.

## 4. Fase B — sondas nos dois binários vanilla

Executar todas as sondas em
`lab/typst-original/target/release/typst` e `/usr/local/bin/typst`. As saídas
devem coincidir antes de servirem como baseline.

### 4.1 Constructors e defaults

Medir para cada variante:

1. `type`, `repr`, `kind`, `stops`, `space`, `relative` e accessors próprios;
2. exatamente 0, 1, 2 e 3 stops;
3. stops todos implícitos, todos explícitos e mistura dos dois formatos;
4. offsets ausentes nos extremos, fora de `[0%, 100%]`, decrescentes,
   repetidos e início/fim diferentes de `0%`/`100%`;
5. stop direto `Color`, par `(Color, Ratio)`, arrays de tamanho/tipo errados,
   números sem unidade e spreading;
6. cada named arg isolado, todos combinados e named desconhecido;
7. defaults por variante e igualdade com a forma explicitamente equivalente;
8. `space` como função construtora, `auto` e valores inválidos;
9. `relative` como `auto`, `"self"`, `"parent"` e tipo/string inválidos.

### 4.2 Casos específicos

- Linear: `angle`, `dir`, ambos presentes, quatro direções, tipo inválido.
- Radial: grafia literal `focal-center`/`focal-radius`, defaults focais,
  radius/focal-radius limites, focal circle tangente, interior e exterior.
- Conic: center e angle, incluindo ângulos negativos e maiores que uma volta.

Sondas negativas devem ficar em ficheiros separados, preservando mensagem,
span e hints integrais quando esses forem o observável.

### 4.3 `sharp` e `repeat`

Medir nas três variantes, nas formas estática e de instância:

- `sharp(steps, smoothness:)`: steps 0/1/2/5; smoothness abaixo de 0%, em
  0%/20%/100% e acima de 100%; stops resultantes; fields preservados;
- `repeat(repetitions, mirror:)`: 0/1/2/4; mirror false/true; stops e offsets
  resultantes; fields preservados;
- aridade, named/positional, tipos inválidos, identidade quando aplicável;
- sampling dos resultados em posições discriminantes, como prova semântica
  independente da estrutura interna.

## 5. Fase C — auditoria cristalina

Produzir matriz de divergências com `file:line` para:

- mínimo atual de stops (`is_empty`) versus vanilla `< 2`;
- `parse_stops`: formatos aceites, mistura de offsets, ordenação e extremos;
- grafia underscore versus hífen dos named args focais;
- defaults e casts atualmente mais permissivos (`Float`/`Int`) que o vanilla;
- `space`, `relative`, `angle`, `dir`, center/radii e named whitelist;
- mensagens/hints e spans;
- existência ou ausência de domínio reutilizável para `sharp` e `repeat`;
- dispatcher P1144 e fields de tipo;
- testes históricos que fixem comportamento divergente e não devam ser
  silenciosamente reinterpretados como prova de paridade.

Matriz obrigatória:

```text
caso | vanilla medido | cristalino atual | língua/mecânica |
L0 afetado | contrato público necessário? | decisão
```

Não remover permissividade cristalina automaticamente: aceitar inputs extras
pode ser compatibilidade. Classificar cada extra separadamente antes de
restringir.

## 6. Decisão ADR-0107/0108 e gate ADR-0127

São língua: nomes, grafia, aridade, casts, defaults, validações, stops
resultantes, sampling, fields preservados, mensagens e hints. São mecânica
livre: `Arc`, shape dos enums Rust, loops, alocação e passos internos, salvo se
afetarem um observável.

Atualizar primeiro os L0s realmente afetados. Depois classificar:

- correção de tabela/parser/glue de paridade sem contrato Rust público,
  default novo ou fase nova: fluxo contínuo, L0 ressellado + RED→GREEN;
- novo método/campo/assinatura pública Rust para `sharp`/`repeat`, mudança de
  representação, default, compatibilidade ou fase: **parar no gate ADR-0127**;
- se for possível implementar as transformações como funções privadas do
  owner sobre campos existentes, registrar por que isso não altera contrato
  público Rust antes de continuar.

Em dúvida, parar. Não declarar novamente o scope completo enquanto `sharp` ou
`repeat` permanecerem sem decisão/scope-out nomeado.

## 7. Plano RED→GREEN após L0 e eventual gate

1. RED tabelado para 0/1/2 stops e regras de offsets nas três variantes;
2. RED para grafia focal com hífen e defaults/validações radiais;
3. RED para named args, `dir`/`angle`, spaces e relatives;
4. RED de `sharp` e `repeat` nas três variantes e nas duas formas de chamada;
5. implementar no owner `gradients.rs`, reutilizando entidade existente onde
   suficiente, sem registry genérico ou despacho dinâmico;
6. manter match exaustivo e delegação estática;
7. ressellar headers afetados;
8. executar sondas cristalinas equivalentes às do vanilla;
9. executar regressões de gradient, paint, layout e export;
10. executar `cargo check --workspace`, `cargo build --workspace`,
    `cargo fmt --all -- --check`, `git diff --check` e
    `crystalline-lint .` com zero violations.

## 8. Limites e encerramento

Ficam fora, salvo dependência inevitável medida:

- mudança de estratégia de render PDF/SVG;
- novos tipos de gradient;
- refactor geral de `Args` ou mensagens de todas as nativas;
- remoção de compatibilidade extra sem decisão explícita;
- accessors de `datetime` e demais itens da fila pós-P1140.

P1145 encerra quando o scope público completo tiver contagem reproduzível,
constructors e transformações tiverem decisão caso a caso, L0 e código
coincidirem, e a validação final estiver verde. Se qualquer decisão exigir
contrato público, o estado correto antes da confirmação humana é
`AGUARDA GATE ADR-0127`.

---

## 9. Execução até ao gate

HEAD inicial `c65c839ff5544a66683b7e81edba923acb49760e`, árvore limpa
exceto por este passo ainda não rastreado, em `2026-08-24T19:34:57-03:00`.

A varredura literal de
`a51e02804:crates/typst-library/src/visualize/gradient.rs:243-875` encontrou
dezasseis funções públicas: 3 constructors + `sharp` + `repeat` + os 11
membros P1144. Helpers Rust posteriores ao fecho do `#[scope]` não são fields
da linguagem.

Os dois binários vanilla produziram saída idêntica na sonda positiva e nas
quatorze negativas. Foram medidos defaults, quatro direções, radial completo,
stops de `sharp(3, smoothness: 20%)`, `repeat(2, mirror: true)`, mínimo de
stops, mistura/ordem/extremos de offsets, grafia focal, conflito dir+angle,
limites de sharp/repeat e tangência focal.

As correções de constructors e o despacho são glue/paridade interna. Contudo,
`sharp` define `anti_alias = false` e `repeat` preserva esse estado no vanilla.
As structs públicas cristalinas `Linear`, `Radial` e `Conic` não possuem o
campo. Ignorá-lo não é justificável sem scope-out de morfologia visual;
adicioná-lo é alteração direta de contrato público Rust.

Decisão: atualizar os três L0s afetados e parar. Nenhum teste RED nem código
L1/L3 de P1145 foi escrito. Continuação exige confirmação humana explícita do
contrato proposto no gate ADR-0127.

## 10. Continuação após aprovação do gate

O dono respondeu “Continuar”, aprovando explicitamente o contrato P1145. Só
depois dessa confirmação foram ressellados os L0s e alteradas as structs
públicas.

Materialização:

- `pub anti_alias: bool` em Linear/Radial/Conic;
- constructors e literals históricos usam `true`;
- regras completas de stops, conversão para mixing space e `space: auto`;
- `dir` linear e grafia focal com hífen;
- nativas e despacho fechado de `sharp`/`repeat`;
- sharp usa `false`; repeat preserva o estado;
- sampling de angle normaliza por volta com `rem_euclid`.

RED confirmado: 2 testes P1145 falharam antes da implementação (`dir`
inesperado e um stop aceito). GREEN focado: 2 passaram. Regressão Core
`gradient`: 97 passaram. Regressão Infra filtrada encontrou snapshots PDF
esperadamente alterados pela conversão correta dos stops default para Oklab;
as referências 05, 06 e 07 foram regeneradas e revalidadas.

## 11. Validação final e proveniência

- hashes L0 finais ressellados: `entities/gradient.md` → `6237ed19`;
  `eval/call_dispatch.md` → `93177a13`;
- sonda cristalina de defaults/sharp/repeat reproduziu a morfologia vanilla,
  incluindo stops Oklab e offsets de 31.67%/35%/65%/68.33%;
- `cargo test -p typst-core gradient`: 97 passed, 0 failed;
- `cargo test -p typst-infra gradient`: 10 passed, 0 failed;
- `cargo test -p typst-infra p307b_snapshot`: 9 passed, 0 failed;
- `cargo check --workspace`: sucesso;
- `cargo build --workspace`: sucesso;
- `cargo fmt --all -- --check`: sucesso;
- `git diff --check`: sucesso;
- `crystalline-lint .`: exit 0, zero violations; avisos históricos permanecem
  fora do escopo.

Proveniência final em `2026-08-24T19:50:01-03:00`: HEAD
`c65c839ff5544a66683b7e81edba923acb49760e`, working tree não commitado com
12 ficheiros tracked alterados (`585 insertions`, `28 deletions`) e este passo
untracked. Três desses ficheiros são referências PDF binárias regeneradas.
