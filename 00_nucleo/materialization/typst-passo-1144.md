# P1144 — auditoria da família pública de `gradient`, iniciada por `kind`

**Data:** 2026-08-24  
**Estado:** `CONCLUÍDO — 11/11 membros materializados; sem gate público`  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Owners:** `01_core/src/compiler/stdlib/gradients.rs` e `01_core/src/entities/gradient.rs`  
**Dependência:** P1143 concluído e commitado; working tree limpo ao abrir este passo.

## 1. Objetivo

Auditar a superfície pública de métodos do tipo `gradient`, tomando
`gradient.kind` como sentinela, mas sem assumir que uma correção isolada seja a
unidade arquitetural correta. Medir a família inteira no vanilla ratificado,
confrontá-la com o domínio cristalino já existente e só então decidir se P1144
materializa um cluster coeso ou se redige L0 com scope-outs divididos em passos
nomeados.

O passo não autoriza inserir apenas `gradient.kind` no field access nem criar
um dispatcher paralelo fora do owner.

## 2. Proveniência inicial

- HEAD cristalino em `2026-08-24T19:18:23-03:00`:
  `40e54791aa930f70412e059dbb966b15e1a6bc45`, branch `Tekt`.
- `git diff HEAD --stat` sem saída e `git status --short` sem entradas: working
  tree limpo.
- L0 vigente:
  `00_nucleo/prompts/compiler/stdlib/gradients.md`; entidade subjacente:
  `00_nucleo/prompts/entities/gradient.md`.
- Estado cristalino inicial: `gradient_type_field` expõe somente `linear`,
  `radial` e `conic`; não há dispatcher de métodos de instância para
  `Value::Gradient`.
- Fonte ratificada inicialmente localizada em
  `crates/typst-library/src/visualize/gradient.rs`, especialmente o bloco que
  começa em `kind`. Conferir sempre pelo objeto Git `a51e02804`.

Qualquer número usado para decidir ou fechar deve registrar novamente HEAD,
hora e estado exato da árvore.

## 3. Fase A — inventário na fonte ratificada

Antes das sondas, extrair do objeto Git `a51e02804` todos os métodos públicos
do scope de `Gradient`, suas anotações, defaults e tipos. O inventário inicial
encontrou pelo menos:

```text
kind, stops, space, relative, angle, center,
radius, focal-center, focal-radius, sample, samples
```

Essa lista é hipótese inicial, não conjunto fechado: continuar até o fim do
bloco `#[scope] impl Gradient` e registrar qualquer método adicional. Distinguir
constructors do valor-tipo, accessors, transformações e operações que exigem
contexto.

Para cada membro, registrar `file:line`, assinatura pública Typst, variantes às
quais se aplica, forma de ausência (`none`, erro ou valor), e se a função é
exposta tanto de forma estática (`gradient.kind(g)`) quanto como método
(`g.kind()`).

## 4. Fase B — sondas observáveis no vanilla

Executar as sondas nos dois binários vanilla ratificados para gradientes
linear, radial e conic. Medir, no mínimo:

1. `type` e `repr` de cada membro da família;
2. equivalência entre forma estática e forma de instância;
3. `kind()` nas três variantes e a natureza do retorno: constructor/função,
   sua identidade com `gradient.linear|radial|conic`, `type` e `repr`;
4. `stops()` com offsets implícitos e explícitos, inclusive morfologia do
   retorno;
5. `space()` e `relative()` para defaults e valores explícitos;
6. matriz variante × accessor para `angle`, `center`, `radius`,
   `focal-center` e `focal-radius`, incluindo todos os casos que devolvem
   `none`;
7. `sample()` para ratio e angle, limites, clamp, stops repetidos e erros de
   tipo;
8. aridade, positional/named args, tipo de `self` incorreto e mensagens de erro
   observáveis;
9. igualdade/identidade apenas no nível da linguagem, nunca pelo `PartialEq`
   Rust ou pela estrutura dos `Arc`.

Sonda mínima para `kind`:

```typ
#let linear = gradient.linear(red, blue)
#let radial = gradient.radial(red, blue)
#let conic = gradient.conic(red, blue)
#metadata((
  linear-kind: repr(linear.kind()),
  radial-kind: repr(radial.kind()),
  conic-kind: repr(conic.kind()),
  linear-id: linear.kind() == gradient.linear,
  radial-id: gradient.kind(radial) == gradient.radial,
  conic-type: type(gradient.kind(conic)),
)) <probe>
```

Cada negativa deve ficar em ficheiro separado para que um erro não esconda
outro. Preservar as saídas integrais quando a mensagem for o observável.

## 5. Fase C — auditoria cristalina

Confrontar as medições com:

- `Gradient::{Linear, Radial, Conic}` e os campos já existentes;
- cálculo de offsets efetivos dos stops;
- `ColorSpace`, `RelativeTo`, `Angle`, `Axes<Ratio>` e representação de
  `GradientStop`;
- funções de sampling existentes na entidade e suas diferenças de domínio;
- `gradient_type_field`, field access de `Type::Gradient` e o dispatcher geral
  de métodos de instância;
- L0s vigentes de gradients e entidade, incluindo scope-outs antigos de render
  PDF que não devem ser confundidos com superfície de linguagem.

Produzir uma matriz por membro:

```text
membro | vanilla medido | domínio L1 existente | glue ausente |
mudança pública necessária? | owner | decisão/scope-out
```

Não mover lógica de domínio para field access. Accessors e sampling devem
delegar à entidade/owner apropriado; field access apenas resolve a função.

## 6. Classificação ADR-0107/0108 e decisão de lote

Somente depois das fases A–C, classificar:

- nomes, assinaturas Typst, retornos por variante, `none`, identidade de
  constructors, stops efetivos, cores amostradas e erros são língua;
- enum Rust, `Arc`, passos do algoritmo e formato interno dos stops são
  mecânica livre;
- inferência inicial: `kind` pode ser pequeno, mas a superfície correta é uma
  família homogênea. Essa inferência seria refutada se a fonte mostrar owners,
  dependências ou gates incompatíveis entre os membros.

Se toda a família reutilizar domínio já nuclearizado e exigir apenas glue de
paridade, P1144 pode materializá-la como cluster único. Se `sample` ou outro
membro exigir contrato/representação nova, separar explicitamente:

1. atualizar o L0 com a parte completa e os scope-outs nomeados;
2. materializar somente o subconjunto independente e homogêneo;
3. criar o número do passo que completa cada subconjunto adiado.

Não deixar o L0 declarar a família completa quando o código materializar só
`kind`.

## 7. Gate L0 e ADR-0127

Antes do código, atualizar primeiro os L0s realmente afetados:

- sempre `compiler/stdlib/gradients.md`;
- `entities/gradient.md` somente se a decisão alterar ou completar semântica
  de domínio;
- outros L0s apenas mediante consumer comprovado.

Se houver novo campo/método/assinatura pública Rust, alteração de representação
pública, compatibilidade, default ou fase do pipeline, **parar no gate
ADR-0127**. Em dúvida, parar.

Se for apenas correção de paridade por glue interno e tabela de fields, seguir
em fluxo contínuo: L0 primeiro, resselo, RED→GREEN e revalidação. Registrar no
passo a classificação que autorizou continuar.

## 8. Plano RED→GREEN após L0 e eventual gate

1. escrever RED tabelado de existência estática e de instância para todo o
   subconjunto decidido;
2. escrever RED por variante para retornos, ausências e identidade;
3. escrever RED morfológico de stops e sampling conforme as sondas;
4. escrever negativos de aridade/tipo/named args;
5. implementar helpers no owner e um despacho estático/exaustivo, sem hub
   genérico ou despacho dinâmico;
6. manter constructors `linear`/`radial`/`conic` e render existentes sem
   regressão;
7. ressellar todos os headers afetados;
8. executar testes focados e regressões de gradient, sampling, paint e field
   access;
9. executar `cargo check --workspace`, `cargo build --workspace`,
   `crystalline-lint .` e `git diff --check`.

## 9. Limites e encerramento

Ficam fora de P1144, salvo se a medição provar dependência inevitável:

- implementação de shading/gradientes no PDF;
- mudança de defaults dos constructors;
- novos tipos de gradiente;
- accessors de `datetime`, bitwise de `int`, `counter.get`, `content.fields`,
  numbering, emoji e HTML;
- limpeza de warnings históricos.

P1144 encerra quando a família tiver sido medida integralmente, cada membro
tiver decisão explícita, o subconjunto materializado coincidir semanticamente
com o vanilla nas três variantes, os adiamentos estiverem nomeados no L0 e a
validação final tiver zero violations. Se houver mudança enquadrada pelo
ADR-0127, o estado correto é `AGUARDA GATE`, não implementação parcial.

---

## 10. Execução e decisão

**Estado final:** `CONCLUÍDO — 11/11 membros materializados; sem gate público`.

### Medição ratificada

Estado inicial: HEAD `40e54791aa930f70412e059dbb966b15e1a6bc45`, árvore
limpa, medido em `2026-08-24T19:18:23-03:00`. A fonte exata
`a51e02804:crates/typst-library/src/visualize/gradient.rs:719-874` fixou onze
métodos: `kind`, `stops`, `space`, `relative`, `angle`, `center`, `radius`,
`focal-center`, `focal-radius`, `sample`, `samples`. O inventário inicial de
dez foi corrigido antes da decisão.

`lab/typst-original/target/release/typst` e `/usr/local/bin/typst` produziram
saída idêntica na sonda positiva. Resultados decisivos: kinds
`linear/radial/conic`; stops com offsets efetivos; spaces `rgb/luma/hsl`;
relative `"self"/"parent"/auto`; ausências por variante como `none`; angle em
`sample` convertido para volta; `samples()` vazio como `()`; forma estática
igual à forma de instância. Sete sondas negativas também coincidiram nos dois
binários para self errado, falta/excesso/named e tipos inválidos.

### Auditoria cristalina e ADR-0127

Todos os dados necessários já estavam nuclearizados em
`entities/gradient.rs`: três variants, fields, `effective_offsets` e `sample`.
A matriz inteira exigiu apenas glue interno no owner stdlib e uma intercepção
fechada em call dispatch. Nenhum campo, trait ou assinatura pública Rust foi
adicionado; defaults, compatibilidade e fase do pipeline não mudaram.
Classificação: correção de paridade em fluxo contínuo pelo ADR-0127, com L0
atualizado e ressellado antes do código; não houve gate que exigisse paragem.

### RED→GREEN e validação

- RED em `cargo test -p typst-core p1144_gradient_`: 2/2 falharam antes da
  implementação (`type gradient has no method 'sample'` e ausência dos fields).
- GREEN focado: 2/2 passaram.
- Regressão `cargo test -p typst-core gradient`: 95 passaram, 0 falharam.
- Sonda do binário cristalino confirmou os valores centrais do vanilla,
  incluindo `repr(kind())`, stops, space, relative, accessors e sampling.
- `cargo check --workspace`: sucesso; apenas warnings históricos.
- `cargo build --workspace`: sucesso; apenas warnings históricos.
- `cargo fmt --all -- --check`: sucesso após formatação.
- `git diff --check`: sucesso.
- `crystalline-lint .`: exit 0, zero violations; avisos informativos
  históricos permanecem fora do escopo.

Proveniência final medida em `2026-08-24T19:29:41-03:00`: HEAD ainda
`40e54791aa930f70412e059dbb966b15e1a6bc45`, working tree não commitado com
seis ficheiros tracked alterados, `421 insertions(+), 20 deletions(-)`, mais
este passo untracked. A árvore exata deve ser obtida por `git diff HEAD --stat`
no momento do commit.
