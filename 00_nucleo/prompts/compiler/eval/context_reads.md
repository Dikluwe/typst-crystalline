# L0 — compiler/eval/context_reads
Hash do Código: a42e7e5a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:a37779d03a4ba0f16ebcc5919477bfdcf783021511a28cf8c9203c67cd64a9e5

## Medição anterior à decisão

No HEAD f7195cdf6 (árvore limpa, 2026-09-30), o bloco está em `01_core/src/compiler/eval/mod.rs:252-737`. Esta observação estabelece o limite da extração; não mede desempenho ou nova paridade bilateral.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/context_reads.rs`. Captura, observação, replay e validação causal de leituras de contexto; preserva identidade e diagnósticos. Manter contrato público, comportamento por defeito, fase do pipeline, compatibilidade, pureza e despacho estático.

## Aceitação

A suíte L1, o build e crystalline-lint passam. A separação não constitui prova de ganho de desempenho.

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

Reconhecer NativeElement como folha de identidade nativa na classificação recursiva de seletores, sem wildcard silencioso. Conservar observação, replay, snapshots, ordem e os mesmos critérios causais. Uma operação inválida por cell não locatable não publica leitura bem-sucedida, counter ou índice. Não reexecutar corpos contextuais para resolver identidade.

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


## Tamanho dos enums — decisão medida 2026-10-16

### Medição anterior à decisão

00_nucleo/diagnosticos/clippy-enums-2026-10-16/before.json.gz conserva fontes/L0, hashes globais, HEAD, working tree não commitado, UTC e diffstat; callers.txt registra usos no workspace. probe-details.json e measurements.json guardam rustc 1.92/x86_64, comandos/binário/rlib e tamanhos/alocações de candidatos isolados antes de código. O enum privado é espelhado fielmente da fonte; os tipos públicos são medidos diretamente do rlib. O observador unsafe existe somente no executável diagnóstico, nunca na camada produtiva. É medição de representação Rust, não igualdade de linguagem, intenção upstream ou prova de ganho global.

### Responsabilidade exclusiva e aceitação

Conservar ContextReadRequest privado ao crate e seu reexport já existente; não exportar tipo novo no hub. Mover apenas os quatro campos entry Content, prefix Value, inner Value e gap Length para OutlineEntryIndentedRead, tipo pub(crate) no módulo com campos privados, derivando Clone. A variante torna-se OutlineEntryIndented(Box<OutlineEntryIndentedRead>). Constructor pub(crate) ContextReadRequest::outline_entry_indented(entry,prefix,inner,gap) move os mesmos valores para uma única caixa, sem clones adicionais. Os demais requests, ledger Vec<Arc<ContextRead>>, ordem/lifetimes e stores permanecem. No replay, desempacotar o payload emprestado uma vez e manter o corpo de execução anterior; na classificação de counter/state, a variante continua não selecionando essas leituras. Sem Eval/Func novos, novo passo, dedup ou identidade por ponteiro. O Box é somente armazenamento, não parte da igualdade de linguagem. Teste de budget exige que o request real caiba em sizeof(Value)+alignof(ContextReadRequest), sem fixar bytes do ABI; o layout x86_64 medido antes era 432 e a candidata isolada 144. O registro real ContextRead também é medido pelo teste antes/depois. O custo adicional de uma alocação é restrito ao pedido de indentação; não é alegação de ganho global de RSS/tempo.

Fluxo contínuo ADR-0127: esta etapa altera somente representação/constructor privados ao crate e metadados nominais; os contratos externos dos outcomes e entidades permanecem. L0 antes do código, resselo restrito. Gate de tamanho privado RED→GREEN e Clippy large_enum_variant deve confirmar três spans selecionados antes e resolver um com código e dois com expectativas satisfeitas. Não elevar limiar ou silenciar módulo/impl/crate; não estender expectativas a Tag/Transformation ou ao request privado. Preservar fixtures/esperados e testar os controles existentes de outline (incluindo oito casos P1405), contexto/replay, callbacks e pipeline. Validar workspace completo/build/Clippy/expectativas/Rustdoc/fmt/linhagem, sem avisos Tekt novos, sob 12 GiB, swap zero e um job. Tag/Transformation continuam medidos, sem alteração pública autorizada neste pacote.
