# Item 3 — memória da estabilização contextual

## Medição anterior à decisão

O fluxo de `context100`, `context300` e `plain300` já foi igualado ao vanilla no diagnóstico `fluxo-numerico-contextual-2026-10-08.md`. O binário Tekt depois do item 2 (`33805371a50becc46d0c9a7339f5b4fc2071023ab12b4e69148e906119da3920`) deu cerca de 95 MiB de pico em `context300`, contra cerca de 31 MiB no vanilla pinado; `plain300` ficou perto de 32 MiB. Não reutilizar os ~125 MiB anteriores como baseline deste código.

`context_stabilization.rs:196` clonava `TagIntrospector` para cada `EvalContext` e guardava o contexto inteiro em `Execution`, inclusive após o fim da avaliação. `context_reads.rs:232-265,354-365` cria um novo contexto com o candidato para replay; os callers de `with_replay_engine` nesta unidade usam observações registradas, não a vista `Execution.ctx.introspector`. A sonda GDB local contou 300 chamadas de `execute` na preparação: RSS cresceu de 40.376 para 79.760 KiB até a saída de `prepare` no binário anterior. Isto é compatível com retenção por produtor, sem atribuir todos os bytes exclusivamente ao clone.

Hipótese rival testada: remover `retired`, que mantém execuções substituídas e as suas entradas, passou no teste de liberação de Arc mas não reduziu o pico. A mediana `context300` foi 94.826 KiB antes e 94.892 KiB depois (seis processos por versão; `retired-ab.json`). A alteração foi revertida. Esse resultado não demonstra que a retenção nunca importa: apenas que não explica o pico deste caso/instante.

## Decisão e mudança

Correção mecânica privada em fluxo contínuo ADR-0127. L0 do owner atualizado antes do teste e do código; `Hash do Código` e `@prompt-hash` ressellados (`be2d2b8d` / `4705a00f`). Depois de `apply_func` e da realização de conteúdo, `Session::execute` substitui somente `ctx.introspector` por `TagIntrospector::empty()` antes de guardar `Execution`. Durante a avaliação, a cópia mutável continua disponível. A entrada imutável em `Arc`, observações, resultados, sinks, históricos I0–I5, `retired`, cinco tentativas e replay permanecem. Nenhum campo público, assinatura, fase ou default mudou.

O teste de liberação falhou antes com `completed execution must release its private introspector` e passou depois. A suite `cargo test --offline -p typst-infra --lib` terminou com 1.254 testes passados e 2 ignorados, incluindo os testes contextuais, HTML e bundle existentes. `cargo build --offline`, build release, `crystalline-lint .` e `git diff --check` passaram; o linter ainda apresenta avisos preexistentes no working tree amplo. Os PDFs Tekt antes/depois de `plain300`, `context100` e `context300` tiveram XML `pdftotext -bbox` e raster PNG a 36 dpi iguais. Isso verifica estes fixtures, não a paridade geométrica global.

## Memória medida

O protocolo CLI em `bench.json` usa processos frescos, oito repetições por caso e versão, ordem Tekt antes/depois alternada, vanilla pinado como controle, Rayon 1, epoch 0, locale/UTC e `--font-path` comuns. `%M` mede pico RSS; os pares não são soma de objetos vivos. Não houve build/teste/GDB concorrente com esta bateria.

| Caso | Tekt antes | Tekt depois | Vanilla | Diferença Tekt antes→depois |
|---|---:|---:|---:|---:|
| `plain300` | 32.116 KiB | 32.420 KiB | 30.048 KiB | +304 KiB (+0,9%) |
| `context100` | 37.344 KiB | 33.266 KiB | 30.404 KiB | −4.078 KiB (−10,9%) |
| `context300` | 95.042 KiB | **46.574 KiB** | 31.562 KiB | **−48.468 KiB (−51,0%)** |

Em `context300`, os oito deltas pareados foram negativos, de −48.000 a −48.968 KiB. `plain300` não executa o caminho alterado; o pequeno sinal positivo mistura variação de RSS e compilação distinta, sem evidência de custo causal nesta mudança. A sonda GDB posterior viu a primeira série de 300 execuções crescer de 40.520 para 41.864 KiB na saída de `prepare`, e o pico ao sair do pipeline em 45.784 KiB. Assim, a cópia própria retida por execução era a maior fonte removível encontrada neste caso.

O Tekt ainda excede o vanilla em ~15.012 KiB na mediana de `context300`. Há clones transitórios para replay de cada leitura e para o histórico/candidato, além de layout e fontes; este trabalho não os atribuiu byte a byte. O histórico I0–I5 e as entradas Arc são necessários para diagnósticos/replay causal sob o contrato atual. Uma nova representação de snapshots ou mudança na assinatura pública L1 requer auditoria separada e, se mudar contrato, gate ADR-0127. Não afirmar que todo o diferencial residual é da estabilização.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`; working tree não commitado. `bench.json` guarda UTC, comandos, amostras, hashes dos três binários, `git status --short` e `git diff HEAD --stat` capturados com as medições. `final-state.json` guarda estado, arquivos e hashes após resselo. Binário Tekt final: `eb761708cf636c2b67e413c60ca2a2947ec57ff7811886b36e1d13f9505b3828`; vanilla: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, pin upstream `9f2b6e8715237cb086899a42873660fe744622e8`. O binário Tekt final é idêntico ao da bateria, pois o resselo de comentários não alterou seus bytes. `profile.gdb` reproduz as paragens; perfis GDB são diagnósticos de fase e não substituem `%M` da CLI sem debugger.
