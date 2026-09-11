# P1339 — medição focal do runtime vanilla de counters

## Proveniência e limites

Medição de 2026-09-10, 01:36:19.991799–01:36:23.537105 UTC, no HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado.
O recibo `p1339-where-counter-runtime-probe-runs.json` conserva o
`git diff HEAD --stat`, lista exata de paths pelo status antes/depois, fontes
integrais, hash de cada entrada, argv, cwd, stdout/stderr integrais e UTC por execução.
Executável `/usr/local/bin/typst`, baseline upstream `a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
SHA-256 do runner: `abf493f664a5f79015a9acf2152d124928ffcf57f6ed64ece0aef9277cdec27a`.
SHA-256 do recibo: `178a8637df7eb75d21131a46ad68005b9f7e35670644c9cb9b7eeb93031d5ae3`.

Papel `/root/p1339_counter_runtime_probe`: medidor vanilla. Leitura permitida:
fontes vanilla, skill e referências, regras do repositório, script de phase probe
e seus dois recibos indicados pelo coordenador. Não foram lidos L0 cristalino,
fonte cristalina ou patch candidato. Escrita limitada a estes três artefatos.
A procura normativa em `00_nucleo/adr` por `segregada|segregado` não encontrou ADR.
Regime exploratório, com disciplina de entradas declarada e filesystem compartilhado;
executado sem atestação de isolamento. Não constitui contrato, selo ou veredito
de implementação. A skill tekt-materializacao-segregada foi lida integralmente
com as referências de papéis/capacidades e artefatos/gates.

Budget: doze casos distintos, sem revisão adaptativa; dois controles repetidos.
Uma primeira execução idêntica ocorreu às 01:36:03.340624–01:36:06.784573 UTC;
o runner foi repetido para persistir o stdout integral pelo mecanismo apply_patch.
O recibo canônico é a segunda execução. Não se mede custo de compilação produtiva.

## Medição

Todos os casos usam `typst query - metadata --field value --format json`.
As formas abaixo são stdout JSON exato sem a quebra final; array externo corresponde
à consulta de metadata. Os erros completos estão no recibo, incluindo localização,
cadeia `while calling get` e warning de depreciação da CLI.

| Caso | Observação | Saída/erro |
|---|---|---|
| `field_order` | Filtros com os mesmos fields, ordem inversa; update apenas no primeiro | `[[false,[12],[2]]]` |
| `int_float_keys` | `level: 1` vs `level: 1.0`; update do primeiro afeta ambos | `[[true,[12],[12],2]]` |
| `nan_keys` | NaN não reflexivo; update(7) não é recuperado nem pelo mesmo binding | `[[false,false,[0],[0],0]]` |
| `interleaved_callbacks` | Updates e headings intercalados de contadores level 1 e level 2 | `[[[24],[20,30],[2]]]` |
| `callback_get_other_outside_context` | Callback tenta `b.get()` | exit 1, `can only be used when context is known` |
| `callback_get_other_created_in_context` | Mesmo callback criado por expressão context | exit 1, `can only be used when context is known` |
| `unqueried_callback` | Update com panic, contador nunca consultado | `[["survived",[1]]]` |
| `get_prefix_before_bad_callback` | `get()` aparece antes do update com panic | exit 1, `panicked with: after-prefix` |
| `counter_captured_outside_context` | Binding e função read definidos fora, chamada em context | `[[12]]` |
| `context_generated_update_assert12` | Update gerado em context, assert12 dependente | `[["passed",[12],[1]]]` |
| `stable_one_page_plain` | Documento de uma página, duas execuções | `[[[12],[12],[1]]]` em ambas |
| `stable_one_page_context` | Uma página, update contextual e assert12, duas execuções | `[[[12],[12],[1]]]` em ambas |

Fonte vanilla que contextualiza as observações:

- `lab/typst-original/crates/typst-library/src/introspection/counter.rs:243`
  seleciona updates pela chave e `:256` combina esses updates com o selector do counter.
- `counter.rs:370` pede localização contextual para get; `:515` produz o elemento
  update sem executar imediatamente a função.
- `counter.rs:612` chama a função update com `Context::none()`.
- `counter.rs:797` resolve a sequência antes de `:798` calcular o prefixo do get.
  Isto explica por que o erro no callback posterior é observável na leitura anterior.

## Classificação posterior à medição

São observáveis da linguagem: igualdade de counters; associação de updates às chaves;
arrays devolvidos; sucesso do assert dependente; erro contextual e panic. A ordem
dos fields afeta a identidade observada do counter mesmo quando ambos selecionam
os mesmos headings. Int/float compartilham updates neste caso. NaN exige preservar
a não reflexividade observada, inclusive a incapacidade de encontrar seu update.

O callback sem consulta não produz erro, mas consultar um prefixo do mesmo contador
torna observável o erro posterior. Portanto, inferir apenas da posição do get que
callbacks posteriores nunca importam é refutado por este corpus. A callback recebe
nenhum contexto utilizável para get de outro counter, inclusive quando foi criada
dentro de context. Capturar o próprio objeto counter fora de context é permitido;
é o local da leitura que precisa de contexto.

Os dois controles de uma página mostram que o update contextual e o assert12
produzem resultado correto com total final de uma página. Isto não mede a sequência
intermediária de totais de páginas nem prova quantas reavaliações foram necessárias.
Inferência limitada: o total final de páginas sozinho não evidencia que a dependência
contextual foi resolvida; o resultado 12 e o sucesso do assert são observáveis
adicionais. O corpus não obriga uma mecânica específica de estabilização.

Limites: nenhum gate cristalino, teste de mutação, prova geral de convergência,
identidade de PDF ou certificação funcional foi executado. Esta evidência cobre
somente os casos registrados e o baseline de hash verificado.
