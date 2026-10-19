# R2-S — realização de Sequence antes do aplanamento

Status: proposta pendente; não autoriza código da nova ordem antes da confirmação.

## Medição antes da decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado. As coletas before/expanded/controls.json.gz em 00_nucleo/diagnosticos/sequence-constructor-2026-10-17 conservam pin bd5ebec285505889e8ff1d4f68804ab147a699f3, SHA dos dois binários, UTC/status/diffstat/argv/fontes/saídas. red.log registra oito REDs; candidate.log registra cinco controles aprovados e três falhas. Não se contam fixtures incorretas de igualdade de texto vazio como defeito de código.

O vanilla devolve Y para show sequence substituindo a soma [A]+[B], [] e sequência de um filho; mantém AB para alias it e para filtro que não casa; rejeita reconstrução fresca por maximum show rule depth exceeded. O candidato já admite a função no catálogo, mas a soma interpolada ainda imprime AB. Symbol é problema distinto: cast de sym.alpha projeta text no Tekt e symbol no vanilla, com igualdade true/false; esta proposta não introduz Symbol nem certifica essa morfologia.

Fontes atuais: markup_stream.rs:240–260 aplana Sequence; :180–200 só intercepta parágrafos após montar o fluxo. call_dispatch/invoke.rs:47–85 captura chamadas de conteúdo mas a soma não é chamada. realize/mod.rs:170–210 percorre ShowRecipe bottom-up depois de preparar tabelas/math. rules/apply.rs:140–170 e show_rule_termination.rs:63–80 conservam ponto-fixo morfológico para não-localizáveis, inadequado para certificar novas gerações Sequence. Estes são efeitos, escopo e aceitação da linguagem; não se pretende copiar a estrutura upstream.

## Contrato proposto

1. Capturar a inserção de uma Sequence/Empty ainda inteira, com regras, estilos, guards e origem lexicais, antes de append_markup_sequence a aplanar. Tratar tanto valores interpolados quanto sequências formadas pelo markup, respeitando fronteiras de set/show e sem retroatividade ao prefixo. Sem regra Sequence pertinente, conservar a rota atual.
2. Realizar essa fronteira antes de percorrer receitas de seus filhos e antes de preparar tabelas/math que a própria regra possa ocultar. Se a regra substitui o corpo, callbacks dos filhos ocultados não rodam. Se mantém it, os filhos efetivamente emitidos continuam sua realização. A função de show é executada apenas no ambiente lexical correto; leitura de fields/func de um valor guardado não dispara receitas.
3. Usar transporte privado em RAM e os tipos existentes. Se ShowRecipe for reutilizado, sua marca e origem são dados privados no snapshot de estilos, sem novo campo público, mudança de tamanho de Value ou nova identidade na linguagem. Um helper descendente privado compiler/realize/sequences.rs pode possuir a lógica; seu L0 proprietário será criado em 00_nucleo/prompts/compiler/realize/sequences.md após ratificação. Não expor fachada pub externa.
4. Diferenciar alias de it de Sequence nova mesmo quando children são iguais. Guards são por ocorrência, não por igualdade morfológica nem um RuleId global que imunize descendentes. Novas gerações voltam a ser visitadas, com o limite existente de show, mensagens/hints/spans de língua. Não mudar valores dos limites de Route nem a política de outros não-localizáveis neste pacote.
5. Sequência continua não localizável: query/counter rejeitam sequence is not locatable e location() continua none. Não criar Location falsa para resolver proteção. Source e as demais identidades, snapshots e Styled opaco permanecem.
6. Manter código do construtor/catálogo/set já validado independentemente. Não modificar a classificação de homônimos, With, perfis de feature, ordem de argumentos ou schemas alheios.

## L0s e ownership

Após a aprovação, atualizar antes do código os owners de markup_stream, call_dispatch/invoke, realize/mod, rules/apply, show_rule_termination e selector_matching, e criar o owner 1:1 do descendente sequences se usado. Cada um recebe somente sua responsabilidade de captura, composição, aplicação, política ou matching. Shared invariants novas exigem Núcleo Tekt pinado, sem @prompt direto no núcleo. V15/V26 devem passar antes do resselo. Não alterar os contratos públicos de Content/Value/Engine.

## Aceitação concreta

S1 construção e reflexão de zero/um/vários filhos; S2 seleção/counter/query e não localização; S3 show bare/where/miss; S4 alias it; S5 alias em bloco; S6 reconstrução fresca com mesmos filhos rejeita; S7 reconstrução vazia rejeita; S8 codeblock com efeito e nova sequência rejeita; S9 regra externa substitui antes dos efeitos ocultos dos filhos; S10 ordem/quantidade de efeitos de filhos emitidos; S11 prefixo anterior à regra preservado; S12 regras/set locais e aliases sob estilos; S13 outros seletores e regras textuais não repetem; S14 constructor guardado em let não executa show antes da inserção; S15 resultado gerado obedece regras lexicais de outras famílias; S16 erro/span/hints e limite existentes; S17 PDF/HTML com feature e target corretos; S18 regressões de Table/Cell/State/Heading e pilha anterior; S19 workspace/doctests/build/Clippy/Rustdoc/fmt/linhagem e SARIF diferencial; S20 12 GiB/High8GiB/swap0/jobs1 e pico registrado. Medir cada fixture no pin antes de fixar a expectativa; não desativar refutadores nem declarar paridade total se algum continuar aberto.

Sem commit, push, merge, publicação ou release incluídos.

## Por que exige confirmação

ADR-0127 §2.3: “Mudança arquitectural de fase/ordem de pipeline”. A preparação de Sequence passa a anteceder as receitas dos filhos e preparação de Table/math; esta autorização cobre somente essa nova fronteira de realização. É uma classificação conservadora diante da mudança de ordem observável. O gate não é exigido para o construtor, o catálogo, o set ou correções internas independentes já em andamento.
