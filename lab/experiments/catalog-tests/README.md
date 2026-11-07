# Bancadas de desempenho e identidade

Código de laboratório, sem importação pelas camadas produtivas. Os testes
`c01_image_identity` e `c25_plugin_namespace` contêm regressões que falham no
estado medido. Não executar esses casos esperando uma suíte totalmente verde.
Resultados e limites: `00_nucleo/diagnostics/code-performance-tests-2026-11-05.md`.

Todas as compilações, cópias instrumentadas, dumps e saídas devem ficar em um
diretório exclusivo de `/dev/shm`. O checkout recebido é somente leitura durante
as medições. Use o lockfile do checkout para manter as versões das dependências
externas; não aceite uma nova resolução silenciosamente. O package isolado inclui
core, infra e shell reais, sem simuladores desses componentes.

`profile_pipeline --help` documenta cargas e snapshots de alocação.
`profile_query --help` documenta projeção/serialização. Nunca usar o tempo desses
processos com contador como benchmark de desempenho. Execute o controle `empty`
e `tests/allocator_validation.py --binary CAMINHO_ABSOLUTO` para ambos. O contador
encaminha para System; não contabiliza mmap nem alocadores estrangeiros.

`apply_overlay.py --help` prepara instrumentação em cópias separadas em RAM,
mantendo o experimento recebido `joint-font-discovery`. Há dois builds distintos:
timing e consultas detalhadas. Não usar duração do segundo como desempenho.
`tests/c02_dump_validation.py` valida o parser, incluindo dumps reais opcionais;
`tests/probe_behavior.rs` pode ser compilado com `rustc --test --edition=2021`
mantendo sua posição relativa ao diretório `probe` numa cópia em RAM.

`paired_measure.py` reproduz a organização do lote pareado CLI/startup. Defina
`TEKT_CATALOG_RAM` para o diretório exclusivo, contendo:

- `base-target/release/typst`: baseline;
- `timing-target/release/typst`: pipeline instrumentado de timing;
- `fixtures/plain.typ` e dependências;
- `tmp/`: temporários.

O driver fixa um CPU permitido, stack de 64 MiB e Rayon 1, alterna variantes,
aquece e executa 12 rodadas de três repetições. Publicação do PDF no startup é
detectada por EOF; o processo é encerrado e coletado com wait4. A CPU do watch
inclui a vida inteira até esse encerramento, incluindo a espera após publicação.
O diretório deve estar livre de saídas anteriores `c02/`; o script recusa colisões.
Dumps e resultados intermediários são descartáveis. O bootstrap usa mediana dos
deltas pareados de médias por rodada. Escopos internos são inclusivos.

O teste independente do ciclo real é:

```sh
python3 tests/watch_plugin_identity.py --binary /dev/shm/AREA/BINARIO --ram-dir /dev/shm/AREA
```

`public_costs` contém controles e microbancadas `#[ignore]`; execute estas
explicitamente, em processos separados e sem outros profilers/builds ativos.
`c21_png_stack` e `c28_query_serialization` exercitam APIs públicas reais. A
retenção privada tem helper em `tests/support/c01_retention.rs`; sua integração
é exclusivamente numa cópia em RAM e não redefine o contrato produtivo.

Remova somente a área RAM criada para a execução, depois de conservar apenas
fontes, testes e medições de domínio úteis. Não conserve logs, mutantes, binários
ou um novo manifesto de execução no Git.
