# L0 — introspect/queries
Hash do Código: 10f3d758

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:8de2c32fbeeee7a30088b4ff3808e15a211f23a182673ef5cdebe54bd3eadad6
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:a37779d03a4ba0f16ebcc5919477bfdcf783021511a28cf8c9203c67cd64a9e5

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/queries.rs`.
No estado `df72a6c4a701917ab56a7c6169764a92ea775693` com árvore limpa, `01_core/src/compiler/introspect.rs:287-673` contém o comportamento que esta unidade recebe. A fonte e o estado estão registrados em `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. A decisão é separá-lo por comportamento, sem alterar a API ou alegar ganho de desempenho.

## Contrato

Implementar Introspector para TagIntrospector usando os índices de ocorrências. Filtros, seletores e posições preservam ordem causal, identidade por Location, limites Before/After e projeções de snapshot conforme o contrato existente.

As invariantes de linguagem, a ordem causal e os limites de paridade continuam definidos em `00_nucleo/prompts/compiler/introspect.md`; este owner não cria uma segunda interpretação. O código usa chamadas estáticas na camada L1, sem vtable, I/O ou estado global mutável.

## Aceitação

Preservar os testes de introspecção e a suíte L1, compilar e passar `crystalline-lint .` com hashes coerentes. Mudanças semânticas futuras exigem medição de linguagem e atualização do L0 antes do código.




## Medição da ordem antes da decisão

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, CounterRegistry::value_at e StateRegistry::value_at comparam Location.as_u128; queries before/after e eventos de contador fazem o mesmo. A realização prepara identidades antes de transformar, portanto o número deixa de ordenar origens e produtos. A sonda CLI e a sentinela single_evaluation_counter_follows_causal_order conservam a evidência no diagnóstico de implementação. Aplicar o núcleo realization-order; manter fallback histórico somente sem ordem registrada.


## Realização com avaliação única

Seletores before/after e ordenação de native-element query usam a ordem causal registrada, conservando Locations como identidades opacas.


## Realização com avaliação única

after compara ordinal causal; não inferir posição no documento do número interno de Location.


## Realização com avaliação única

Ordenar união de selectors nativos pela ordem causal, deduplicando apenas a identidade preparada.

## D05 — label em filtros de elementos com snapshot legado

Medição: after.json.gz registra query equation.where(label:<display>) como zero
contra um no vanilla. walk.rs:242 conserva fields None para Equation;
from_tags/equations.rs:161–166 realiza a vista com fields None. O registro de
labels já identifica a Location, como transportado por query_helpers.rs:544
para os resultados None. Decisão: quando entry.fields é None, filtro label
Value::Label resolve pelo índice LabelRegistry da mesma Location; demais campos
mantêm element_selector_matches e igualdade existente. Some, incluindo vazio,
permanece autoritativo sem completar campo ausente pelo índice. Não fabricar
snapshot nem inferir label do texto; preservar ordem causal e demais seletores.
Não altera assinatura ou fase, fluxo contínuo ADR-0127. Testar positivo e Some
vazio antes do código; valores inválidos/ausência ficam submetidos à medição.

label-oracle.json mede adicionalmente: label none e string "display" não casam;
Label <display> casa e <unknown> não casa. Não equiparar ausência a Value::None,
nem string a Label. O fallback de índice recebe apenas Value::Label. Demais
valores usam projeção/igualdade antiga, sem transformar tipo de filtro.

## D08A — seletor bare de table.cell (PROPOSTO, gate ADR-0127)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
UTC, diffstat, entradas, comandos, hashes das fontes e executáveis em
lab/results/construction/diagnostics/math-residuos-table-cell-2026-10-06/{before,table-additional-before,table-aliases-before}.json.gz.
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

Consumir NativeElement pelo matching de identidade nativa real dos elementos registrados, conservando ordem causal e Locations. Sem filtros adicionais ou snapshot fabricado: ausência de grupo não é filtro vazio. Não hardcodear retorno [] para a nova variante: table.cell não deve entrar em consulta válida, porque a validação antecedente a rejeita. Composições preservam limites e identidade existentes. Não ampliar catálogo indexável ou registrar TableCell.

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
