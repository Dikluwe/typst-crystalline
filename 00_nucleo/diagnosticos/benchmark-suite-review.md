# Revisão da suíte de microbenchmarks

## Inspeção antes da classificação

`benches/stream_bench.rs:125` (`bench_stream`) chama
`export_pdf_with_tags_and_serialization` dentro de cada `b.iter`, com
`PdfSerializationMode::Pretty`. A closure inclui descoberta de recursos,
builder, emissão e serialização PDF. As famílias page/text/shape/draw
descrevem fixtures; não delimitam o trabalho cronometrado.

`benches/pdf_builder_bench.rs:47` (`bench_builder`) também chama exportadores
completos. O sweep de páginas vazias não isola serialização e o sweep
Compact/Pretty não isola streams. Os nomes anteriores sugeriam fases com
mais precisão do que as APIs medidas permitem.

`01_core/src/compiler/eval/observation/mod.rs:187` (`observation_microbenchmarks`)
não possui guarda debug. O runner de stdlib em
`01_core/src/compiler/stdlib/registry/microbenchmarks.rs:205` rejeita
`cfg!(debug_assertions)`. Ambos são testes ignorados privados e medem
as closures Criterion dentro do executável de testes.

`01_core/src/compiler/layout/measurement.rs:367` (`measure_content_real_with_baseline`)
cria um Layouter temporário e executa layout de sub-frame.
`01_core/src/compiler/layout/mod.rs:2388` (`layout`) inclui introspecção.
O benchmark de estilos mede o entrypoint completo sobre fixtures de estilo.

## Correções

- Stream classificado como exportação PDF completa sob
  `pdf_export_cohorts/pretty_serialization`; Pretty e fixtures preservados.
  Não há estimativa de custo isolado page/text/shape/draw.
- Builder classificado sob `pdf_export_cohorts/builder`, com grupos
  `text_serialization_modes` e `empty_pages`. Os nomes indicam o input/configuração,
  não uma fase interna isolada.
- Runners `observation_bench` e `stdlib_bench` em benches compilam testes core
  release, verificam o perfil do artefato JSON e executam somente o teste
  ignorado exato. Artefatos sem otimização, com debug assertions, ausentes,
  incompletos ou ambíguos são recusados antes de executar medições.
- Todos os entrypoints Rust possuem guarda de perfil antes de registrar
  Criterion. Flags do compilador são neutralizadas no build filho dos runners
  privados para preservar a veracidade do perfil verificado.
- Scanner aplica black_box ao input e aos tokens consumidos.
- A classificação de famílias do catálogo Content permanece uma checagem
  de cobertura antes de medir; deixou de aparecer como benchmark de código
  auxiliar do próprio harness.
- Diagnósticos preservam tempos históricos e passam a indicar os runners
  suportados e os limites de isolamento. Nenhuma API produtiva foi adicionada.

Instruções e limites operacionais: [benches/README.md](../../benches/README.md).
O teste ignorado original de observação permanece sem guarda interna:
invocá-lo diretamente não é o caminho suportado para registrar performance.
Corrigir esse teste em L1 fica fora do escopo autorizado; a suíte usa a
proteção no launcher e no artefato realmente executado.

## Validação e proveniência

Esta revisão não coleta novos tempos de desempenho nem fecha ganho/regressão.
HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado.
Snapshot após a validação, com UTC, `git diff HEAD --stat`, lista exata de
arquivos dirty/untracked, SHA-256 das fontes benchmark e identidade dos
executáveis: [benchmark-suite-review-state.json](benchmark-suite-review-state.json).
A árvore já continha alterações extensas de outras frentes; esta revisão
editou somente benches e trechos de diagnósticos de benchmarks.

- `cargo bench --manifest-path benches/Cargo.toml --no-run`: sucesso;
  todos os oito targets compilados com perfil bench otimizado.
- `cargo bench --manifest-path benches/Cargo.toml -- --test`: sucesso em
  todos os targets. Nos runners privados, é verificação de build/perfil/filtro,
  sem execução das fixtures Criterion nem amostragem.
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s benches/tests -p 'test_*.py' -v`:
  oito testes passaram, incluindo execução real dos launchers compilados sem
  otimização ou com debug assertions; ambos recusam antes de iniciar Python.
- Artefato core com perfil debug: teste confirma recusa antes de listar ou
  executar testes. Perfil incompleto/ambíguo e filtro vazio também falham.
- Launcher stdlib executado com `RUSTFLAGS=--benchmark-invalid-flag`:
  smoke passou, confirmando neutralização de flags ambientais no build filho.
- `git diff --check -- benches 00_nucleo/diagnosticos`: sucesso.
- Formatação restrita aos arquivos Rust editados, com `skip_children=true`;
  nenhuma formatação global foi executada. Cargo.lock manteve o SHA-256
  `d26620fd50b4ef73430d779ae55feed8f730e9e27c8ac06a04100f6195cc5a14`
  observado antes dos builds desta revisão.

A compilação emite warnings existentes em core/infra. Não se reivindica um
lint global limpo nem isolamento de fases privadas. Os números históricos
permanecem históricos, sem conclusão nova de performance.
