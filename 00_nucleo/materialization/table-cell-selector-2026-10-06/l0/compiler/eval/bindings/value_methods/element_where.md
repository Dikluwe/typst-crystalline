# L0 — compiler/eval/bindings/value_methods/element_where
Hash do Código: e553cf23

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:a37779d03a4ba0f16ebcc5919477bfdcf783021511a28cf8c9203c67cd64a9e5

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/bindings/value_methods/element_where.rs`. Extração interna medida no HEAD `ce27af795` e árvore limpa em 2026-09-30 23:12:44 UTC. A fonte é o consumer pai desta unidade; a extração não é medição bilateral de paridade nem prova de desempenho.

A lógica extraída estava em `01_core/src/compiler/eval/bindings/value_methods.rs:684-1209` no HEAD indicado. A classificação é extração interna de comportamento, não nova decisão de paridade.

## Contrato

Own native element identity, function.where, field catalog and locatability checks. Preserve typed identity, errors and selector morphology. Não alterar contrato público, fase do pipeline, comportamento por defeito ou compatibilidade.

## Aceitação

Os testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho da divisão estrutural.

## D05 — equation.where, correção interna de paridade

### Medição antes da decisão

Coleta equation-where-2026-10-06/before.json.gz registra pin
9f2b6e8715237cb086899a42873660fe744622e8, binários SHA-256, UTC, HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b e diff HEAD --stat/estado exato.
Tekt rejeita equation.where como função não-elemento; vanilla aceita catálogo
público, filtros block/label/body/numbering/alt, alias, composição e show.
Vanilla rejeita campos size/missing, posição, closure e aplicação parcial.
Fonte upstream: foundations/func.rs:433–462 valida field_id e mantém grupo;
math/equation.rs:58–170 separa campos públicos dos internos.
Tekt element_where.rs:63–490 omite equation; selector_matching.rs:269–310
omite Equation no matching nativo. O getter block/body e snapshot já existem.

### Decisão e aceitação

Acrescentar math.equation à tabela estática por identidade do constructor
native_math_equation já existente, via bridge interno de stdlib. Catálogo público:
block, numbering, number-align, supplement, alt, body; label continua campo
comum permitido. Equação é locatável. size e demais campos internos não são
aceitos. Preservar mensagem equation (sem prefixo math), grupo ordenado vazio,
valores não tipados dos filtros, validação de args e aliases por ponteiro.
Closures, homônimos de função e aplicação parcial continuam não-elementos.
Não mudar assinatura, enum, API pública ou fase; não executar constructor.

Fluxo contínuo ADR-0127: correção de tabela interna, sem novo contrato público
ou fase. RED→GREEN dos consumers e revalidação bilateral dos controles;
preservar D04 e não realizar math bare/D06 ou morfologia de matriz/D07.

## D08A — seletor bare de table.cell (PROPOSTO, gate ADR-0127)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
UTC, diffstat, entradas, comandos, hashes das fontes e executáveis em
00_nucleo/diagnosticos/math-residuos-table-cell-2026-10-06/{before,table-additional-before,table-aliases-before}.json.gz.
Vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8.
selector(table.cell) devolve cell, where() devolve cell.where(:), a comparação
é false, query/counter rejeitam cell não locatable. Alias verdadeiro medido
por CodeBlock conserva esses resultados. A fixture anterior que definiu alias
apenas no documento não disponibilizava a variável à expressão eval e não
prova alias positivo; table-aliases-before corrige a fixture.

Fonte: lab/typst-original/crates/typst-library/src/foundations/selector.rs:309–321
distingue dict None de Some; :421–450 valida locatabilidade separadamente.
No Tekt, entities/selector.rs:34–37 só representa grupo Element presente;
stdlib/foundations/selector.rs:69–77,348–354 omite a nativa cell; os demais
limites atuais e hashes estão em d08-owner-audit.json.

Classificação: formato, identidade e rejeição de consulta são linguagem;
o novo carrier fechado é mecânica escolhida para preservar esses observáveis.
A distinção do vanilla é explícita na fonte; não se infere intenção apenas
da saída. Inferência: a nova variante basta à fatia de seleção/locatabilidade;
a hipótese será refutada por um caso público cuja ausência de grupo ou
identidade não caiba nela. Não se presume correção de realização de células.

### Contrato materializável somente após ratificação

Conservar eval_element_where e a tabela native_element_fields: table.cell já tem catálogo e locatable=false. validate_element_locatability visita também Selector::NativeElement pela função efetiva, rejeitando cell com `cell is not locatable` no span recebido; propagar por And/Or/Where/Within/Before/After como hoje. Não converter NativeElement em Element vazio e não validar locatabilidade na construção de where.

### Fronteira incompleta e aceitação

D08A cobre apenas carrier, construção, repr, igualdade, composição e
locatabilidade. D08 permanece incompleto: D08B completa show direto, projeção
de body/campos e preparação de células explícitas/implícitas, incluindo
posições, padding, spans, header/footer e escopo lexical. D08B precisa de
seus L0s medidos e de gate próprio caso mude contrato ou fase; esta ratificação
não o autoriza. Não esconder negativos D08B ou declarar paridade universal.

Após ratificação: promover este conjunto atomicamente; escrever testes
linguísticos e verificar RED válido antes do código; materializar com
despacho estático e Rust seguro; validar 1:1/V26 antes de ressellar somente
owners deste manifesto; build, suíte core e linter sem erros. Revalidar
selector cell bare/where/alias, igualdade true/false, query/counter diretos
e compostos, função homônima, partial/closure; preservar D05, D07 e query.
