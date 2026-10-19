# Relatório — avaliação única de chamadas

Etapa concluída. Receivers, callee e argumentos já resolvidos são conservados entre interceptores e fallback. As sondas agora mostram uma avaliação por efeito; métodos ausentes e callee não chamável falham antes de executar argumentos comuns. Métodos mutantes conservam sua ordem especial de argumentos antes do local.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Baseline `2026-10-17T16:58:14.610986+00:00`, fechamento `2026-10-17T17:47:36.308190+00:00`. before.json.gz, red-state.json.gz e final-state.json.gz conservam fontes/L0, hashes, UTC, status e diff HEAD --stat. Os 2124 hashes da etapa anterior foram conferidos antes do baseline. Artefatos de sondas guardam argv, fixtures, saídas e binários. Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; fontes call.rs/methods.rs pinadas em pinned-source.json.gz.

## Correção e L0

Dezesseis pares consumer/L0 e núcleo `eval/call-once.toml` pinado. L0 antes dos testes/código, fluxo contínuo ADR-0127; sem campo, trait, assinatura ou reexport Rust público novo, default ou mudança de fase. public-contract-before/after confere interfaces; integrity-result confere fontes fora do escopo. Não houve commit/publicação/release.

A chamada guarda receiver/callee/Args no próprio stack e compartilha valores por empréstimo/ownership entre interceptores. Lookup completo usa o receiver já resolvido e preserva AST/spans, features, a11y, estilos e deprecações. Seletores/contexto/conteúdo não reavaliam a expressão quando recusam interceptar.

CollectionDispatch distingue Handled/Unhandled e devolve o receiver original no fallback. FnOnce adia Args até reconhecer o método na tabela estática única; o adapter antigo continua aceitando Args avaliados. Não há enum novo de callable, vtable, registry de métodos ou tabela de nomes duplicada. Bytes len/at/slice usam os mesmos primitives. O catálogo Type, wrappers estáticos, algoritmos bytes e writeback mutante conservam os corpos anteriores, conferidos no fechamento. As extensões Tekt declaradas permanecem.

MutatingCall transporta receiver e Args depois de argumentos→local; a invocação final reutiliza esses Args. O callee final é classificado antes de preparar argumentos. O complemento explícito de Value rejeita não chamáveis com diagnóstico causal, sem executar panic de argumento. Type::Array ausente agora usa backticks medidos do pin e o field-only span, como confirma legacy-array-probe.json com stderr idêntico nos dois binários. As compensações por nome mat, cópia isolada de scopes e cortes por prefix/array_lengths foram removidas; não se pode apagar efeitos legítimos para compensar execução duplicada.

## Evidência

- RED funcional: cinco falhas reais e um controle aprovado antes da implementação.
- GREEN: oito testes novos, incluindo duas matrizes de 39 discriminantes distintos de Value. Elas verificam Unhandled sem callback/sem trocar payload ou ponteiro do Vec e a classificação dos 37 não chamáveis antes de args, além dos dois caminhos Func/Type.
- Corpus principal: 36 casos bilaterais. Corpus matemático: sete casos. Resultado agregado: {'language-parity': 40, 'unchanged-negative-diagnostic-residual': 3}; 30 resultados corrigidos. Mensagens/hints/âncoras/traces são comparados integralmente, descontando somente o aviso Tekt já conhecido de arquivo vazio por literal exato; dados brutos permanecem. Negativos ainda divergentes não recebem crédito de paridade.
- calc.abs/array.len/str.from-unicode: receiver/args 3/2→1/1. Interceptores não aplicáveis podem medir 4/2→1/1; fallback mutante args 3→1. math.mat com array prévio (1,) produz (1,1), preservando o efeito legítimo; não (1,1,1) nem corte para (1,).
- Suíte workspace: 8913 aprovados, 0 falhas e 7 ignorados. Build, formato, Clippy all-targets estrito, Rustdoc estrito/quatro índices, V1/V5/V15/V26, reverse lineage e diff --check passaram.
- Diagnóstico Tekt: {'V16:error': 117, 'V19:note': 1085, 'V20:note': 873}; uma nota V19 nova do complemento de 37 não chamáveis está ligada às matrizes de 39 variantes. V20 não aumentou. Consultar architecture-delta.json para a justificação. Não se presume redução numérica de V16 por endurecer o match final: o baseline não reportava V16 em invoke.rs. Configuração e severidades não foram relaxadas.
- Recursos sequenciais: MemoryMax12GiB/High10GiB/swap0/Cargo jobs1. Pico 10.003387 GiB (10741055488 bytes); eventos {'low': 0, 'high': 935, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}.

## Tentativas e pendências

A fixture inicial de import usava a palavra reservada or como nome de função; foi substituída para o corpus válido e preservada com o panic do Tekt para correção própria. O primeiro GREEN teve cinco controles passando e falhou na grafia antiga de Type::Array; o esperado do pin foi mantido e o literal produtivo corrigido. Uma tentativa da matriz usava path() num World em memória sem resolução filesystem; a fixture passou a RootedPath puro, sem implementar I/O em L1. As tentativas e recursos ficam arquivados. Três bindings de Args ignorados nas extensões de string foram explicitados com prefixo _, conservando sua avaliação e sem supressões. Dois padrões com captura @ foram achatados para conservar V20 sem aumento. A suíte completa encontrou um único esperado antigo de #array.nope; legacy-array-probe.json confirmou o diagnóstico e a âncora no pin, e somente esse esperado foi atualizado, sem retirar assertivas.

O objetivo global continua ativo; follow-ups.md registra parsing/import inválido, erros de campos/negativos matemáticos e os contratos anteriores que permanecem abertos. Esta etapa não afirma paridade universal de chamadas, todos os casts, schemas, hints ou conteúdo renderizado.
