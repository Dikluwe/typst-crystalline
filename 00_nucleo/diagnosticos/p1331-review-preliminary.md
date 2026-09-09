# P1331 — parecer preliminar independente

Revisor: agente `/root/p1331_review`, em ambiente compartilhado com root e A/B.
Regime: A/B executado sem atestação de isolamento, sem refinamento. Autoridade
de escrita limitada a novos `00_nucleo/diagnosticos/p1331-review-*`; não escrevi
produto, L0 nem oráculos. O contexto recebido contém a intenção de corrigir os
diagnósticos dos tipos rejeitados e a fronteira de identidade, sem candidato C.
Não acedi nem listei `materialization/` ou `context/`.

## Entradas e proveniência

Leitura concluída da skill `tekt-materializacao-segregada/SKILL.md` e de ambas
as referências de papéis/capacidades e artefatos/gates, `CLAUDE.md`,
`01_core/CLAUDE.md`, ADR-0107, ADR-0108, ADR-0127, ADR-0129 e ADR-0130.
Busca textual nas ADRs não localizou ADR própria de materialização segregada.
Li integralmente `00_nucleo/prompts/compiler/stdlib/calc.md`, o runtime baseline
de `calc_abs`, APIs públicas de `Value`/`Args` e as fontes vanilla abaixo.

Estado observado em `2026-09-09T13:39:01Z`: HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
O inventário e diff/stat completos ficam em `p1331-baseline.json`:
SHA-256 `7287bfac86e29e8ff0e86955ce96f63e140b745fb781eb4b4b7410215c794b45`.
Baseline público SHA-256:
`621362a26b8d4d2553bbde444dc9bdfea6a0da98496b9e2a0dc9d60faafefe6c`.
Closure anterior P1330 SHA-256:
`e01d3a9eb4a26c7b2bc226fdc53d17ea17c8f56fc4cb7b3d9474a85e47532f0f`.
Prompt lido SHA-256:
`1e13169abebffb491b95d3fa098e3b5cbc55b322dcdb3e08b8f8f6c89897fdc0`;
owner baseline SHA-256:
`84dc0e8d42225e1f5955bf20d8ba3bd8bb7ea3628e03d5df6d03df806f787c36`.
O baseline público começa em `2026-09-09T13:36:39.015737+00:00` e identifica
os binários por caminho/hash, incluindo vanilla ratificado `a51e02804`.

## Medição antes da decisão

`01_core/src/compiler/stdlib/calc.rs:139-200` já separa guards, valores aceitos,
overflow, Length misto e conteúdo. O fallback restante usa `err`, português,
`Value::type_name()` e origem detached. `entities/value.rs:335-374` dá `str`
e `bool`, ao passo que o baseline vanilla dá `string` e `boolean`. Para os
demais tipos construídos medidos, o nome publicado no erro coincide.

A fonte ratificada `lab/typst-original/crates/typst-library/src/foundations/calc.rs:74-94`
declara `ToAbs` e os casts admitidos. `foundations/cast.rs:307-364` produz
`expected ... or decimal, found ...`; `foundations/ty.rs:74-81,179-182`
especifica nomes longos para diagnóstico. Isso confirma que o nome longo é
uma escolha explícita, sem inferir intenção geral do design. Os hints especiais
de cast não são acionados pelos tipos rejeitados observados em ToAbs.

No baseline público, `calc.abs("x")`, `calc.abs(true)` e `calc.abs(sym.alpha)`
rejeitam nos dois sistemas, mas só vanilla ancora o erro no argumento. Alias,
With e spread reproduzem a lacuna. `calc.abs("x", bad: 1)` e
`calc.abs("x", 1)` comprovam a precedência diferente dos guards já existente.
`calc.sqrt("x")` confirma que a dívida fora de abs não pode ser fechada por
esta correção.

`calc.abs(1pt + 2%)` alcança a rejeição de `relative length` nos dois sistemas.
Isso prova esse exemplar, sem provar que todo estado Rust `Relative` tem
identidade equivalente. `calc.abs(path())` falha no construtor em ambos:
é Unknown para abs(Path), não rejeição demonstrada pelo owner. Location
também exige testemunha própria antes de alegar cobertura de linguagem.

## Parecer sobre o escopo

É razoável corrigir no owner a mensagem e a âncora dos tipos restantes já
rejeitados, conservando exatamente aceitação, guards, dispatcher e outras
funções. Trata-se de correção de paridade ADR-0127 em fluxo contínuo: L0
primeiro, resselo, testes congelados e RED real antes de C. Não exige novo
contrato Rust público nem mudança de fase.

O L0 deve declarar os nomes diagnósticos e a fonte da origem: somente
`value_span` da primeira ocorrência posicional, sem substituição por args.span,
occurrence.span ou busca textual. Ocorrência inexistente ou detached deve
permanecer detached. `Args` já oferece essa informação publicamente em
`entities/args.rs:14-33,53-79`; esta suficiência é inferência e seria refutada
por uma rota obrigatória que não transporte a origem ou exija outro owner.

Não cabe declarar paridade de todos os tipos com base no enum. Para estados
sem correspondência de linguagem demonstrada, separar obrigação local e
Unknown de paridade. Se o contrato exigir uma testemunha, Unknown bloqueia
essa aceitação. Dívidas de construção devem ser registradas sem ampliá-las
para operadores, entidades, parser ou dispatcher.

## Migração histórica exigida

O sucessor vigente `p1330-ab-p1328-successor.rs:341-375` contém as expectativas
de str e symbol com mensagem portuguesa, span detached e trace da chamada.
A/B precisa migrar exatamente essas observações para mensagem longa,
argumento ancorado e ausência do trace redundante direto. Não basta trocar
só a string, pois o bloco compartilha asserções de origem/trace com
`calc.sqrt([x])`, que continua dívida e deve conservar as expectativas.

Os controles dimensionais positivos de P1329, overflow P1330, conteúdo
P1328 e guards permanecem vigentes. Os snippets históricos em diagnosticos
não devem ser editados. O novo sucessor deve mostrar um delta justificável
contra o vigente, e não remover controles por incompatibilidade com a nova
implementação. A revisão final deve conferir também corpus CLI acumulado,
perfis, ordens e os campos completos dos diagnósticos.

Parecer preliminar: escopo viável condicionado ao L0 e ao congelamento A/B.
Não é aprovação de C nem veredito de fechamento.
