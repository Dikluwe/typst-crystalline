# Passo 1423 — completar os 43 constructors tipados HTML residuais

## Estado medido antes da decisão

O P1422 reclassificou 44 rotas `html.*`: todas são `PARTIAL`, com forma
qualificada `ABSENT` e alternativa local `MATCH`. A recomendação automática do
backlog chamou as 44 de “funções tipadas”, mas a fonte ratificada refuta essa
homogeneidade:

- `typst-assets` pinado em `94dcb990edb36311ef85162baa26e4a8205e0bf0`,
  `files/html/data.rs`, contém 112 entradas tipadas e exatamente 43 das 44
  rotas P1422;
- `html.frame` não aparece em `data::ELEMS`; em
  `typst-html/src/lib.rs` é uma função separada que converte conteúdo paginado
  em SVG HTML;
- a própria sonda P1420 prova a diferença morfológica: `html.frame([x])`
  produz `<svg ...>`, enquanto `html.elem("frame")` produz `<frame></frame>`.

Logo, materializar `html.frame` como tag seria deriva. Este passo corrige o
recorte para os 43 constructors realmente derivados da tabela tipada e reserva
`html.frame` para P1424, com contrato próprio dentro do mesmo L0.

Estado causal da medição: HEAD
`5b40e9b354e95fbb0195337512e26f1c8c0ab92b`, tree
`9d56477e598613fd7869c1e5100743115c0c7a03`; L0 anterior SHA-256
`c7d6d0271e244a6036150d9aeac130ad12ba30a440b10f8b4c78631069c8b6b6`;
consumer anterior SHA-256
`9194006bd095b49a93c2c482ca8ffe290e596621f3c5b4dbe6a78049bb515d6c`.

## Classificação ADR-0127 e parada

O passo adiciona 43 funções públicas ao módulo de linguagem `html`, incluindo
assinaturas, casts e diagnósticos. É mudança de contrato público e de
compatibilidade no sentido do ADR-0127. Portanto:

1. atualizar primeiro `00_nucleo/prompts/compiler/stdlib/html.md`;
2. commitar este passo e a proposta L0;
3. **PARAR antes de qualquer alteração em L1 ou nos headers de linhagem**;
4. continuar somente após confirmação explícita do dono;
5. depois da confirmação, registrar aprovação no L0, derivar contrato,
   oráculos, ataques e testes segregados, selar e só então implementar.

**Gate confirmado pelo dono em 2026-09-16.** A continuação deve usar o L0
aprovado e não pode reabrir silenciosamente o recorte de 43 constructors.

## Objetivo após confirmação

Sob `Feature::Html`, acrescentar exatamente os 43 bindings tipados definidos
na secção P1423 do L0 proprietário. Reutilizar `HtmlElem`, `GLOBAL_ATTRS`, o
dispatcher estático e o exporter genérico existentes. Não adicionar variantes
de entidade, lógica de browser, rede, CSS, fallback de named ou validação de
parent/content model.

As dez tags void usam `HtmlBody::Unset`; `script` e `style` recebem string raw
posicional opcional; as demais 31 recebem Content posicional opcional e omissão
produz `HtmlBody::None`. Todos aceitam os 76 globais P1168 e somente os
específicos congelados no L0.

## Regime segregado posterior ao gate

Aplicar protocolo completo `tekt-materializacao-segregada`, declarando
“executado sem atestação de isolamento” se os papéis compartilharem workspace:

- autor do contrato/oráculos não lê implementação candidata;
- adversário cria mutantes de binding, body, casts, enums, serialização,
  feature e DOM;
- testador A/B deriva testes do L0/contrato selado sem ler o patch;
- implementador escreve somente consumer e adaptações internas necessárias;
- verificador não corrige o material que julga.

O preseal deve proteger L0 aprovado, passo, HEAD/tree causal, pin vanilla,
binários, contrato, oráculos, mutantes e testes A/B. Qualquer mudança em entrada
protegida invalida a cadeia desde a primeira fase afetada.

## Gates de implementação

Após confirmação e selo:

1. RED de presença: 43 bindings ausentes no baseline cristalino;
2. RED de assinatura: body/void/raw e específicos representativos;
3. RED adversarial: named cruzado, enum fora do conjunto, limites inteiros,
   lista inválida, união inválida e feature off;
4. GREEN L1 focal e integral;
5. reprobe bilateral das 43 rotas nas três ordens, exigindo `MATCH` qualificado;
6. controle negativo: `html.frame` continua ausente;
7. `cargo fmt --all -- --check`, testes focais, `cargo build`,
   `crystalline-lint .` e `git diff --check`.

Mutation score exigido: `1.0`; `Unknown` nunca é sucesso. Timeout, OOM, swap,
truncamento, identidade divergente, assinatura não observada ou DOM incompleto
fecham como `INCONCLUSIVE`.

## Critério de fecho

P1423 fecha quando os 43 bindings possuem assinatura e morfologia bilateral
medidas, as 43 rotas passam de `PARTIAL` para `MATCH`, `html.frame` permanece
fora, todos os mutantes válidos são rejeitados, hashes selados permanecem
íntegros e os gates arquiteturais passam. O veredito cobre apenas os 43
constructors e não declara paridade geral de HTML.
