# P1308 — autoria independente dos testes e replay

Estado: congelado antes de candidato P1308; sem veredito. Regime: executado
sem atestação de isolamento técnico. A skill tekt-materializacao-segregada
determinou a separação de autoria, os pins antes do candidato, a medição
literal anterior às expectativas e a preservação integral dos predecessores.

## Artefatos selados deste papel

- `p1308-tests.patch`: `6f9bcb4c7180a8f6eaf16b680c44555247bffc5a59fa5e996f0c1789e000c55a`.
- `p1308-tests-supplement.patch`: `5a6fb49bcf77ac5a3f71ffba9dd02e80520d37212d31db09800508b5035b7892`.
- `p1308-measure.json`: `ce758b5c2a18288bf9c8433178f577b50c52df80cedcddcb1f4daa4e785573fc`.
- `p1308-measure-supplement.json`: `97b0bf7e6e71574067aa79fc7a205f7969509587bd8f7898a29740a4e844d479`.
- `p1308-contract-replay.py`: `e7ec23a6435c7b68613e1da9783031725dcd6230c98986bb09fc977d23c8034b`.
- `p1308-contract-note.md`: `a526f6e2583fc5d22644d7fbeae033d3046827b4bfdf47b9f19a48c8cae5d246`.

O suplemento aplica-se **depois** do patch principal. Ambos passaram
`git apply --check` em seus estados predecessores, exit 0. Nenhum Rust foi
escrito por este papel. Root recebeu os patches imutáveis para integração,
compilação RED e posterior implementação independente. Os novos testes usam
somente APIs que já existiam no baseline, não nomes/layout do futuro overlay.

Baseline exato: `p1308-baseline.json` SHA-256
`62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`,
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree e diff/stat
ali preservados. Os três consumers antes do novo resselo tinham hashes:

- eval/tests.rs: `0fe05afeb845159103f9ee69a90d05a4bc3846b20cce3c6ebbbbb61238f3b819`;
- eval/repr.rs: `6d81d4e0047f802dc564bdd03046a3438bfc6c77a1785bea39d29f7a5d2d4f19`;
- wiring/tests/p1293_contract.rs: `2505aa34538067c743530f4a15feaf0a1b937c24bb51e2abdd64c1bc4151c7cb`.

A geração do patch conferiu os bytes completos desses três arquivos após
repor em memória exclusivamente o prompt-hash anterior; a árvore já tinha
recebido resselo mecânico, não candidato. O suplemento lê somente os helpers
de testes públicos/preexistentes e o trait World pinado, não implementação.

## Origem das expectativas

A matriz principal guarda 59 casos, 1416 processos, quatro perfis e três
ordens, sem instabilidade. Reutiliza os 14 casos pendentes do R6 sem editar
seu oracle. O suplemento mediu literalmente mais oito processos default
(quatro fixtures × dois bins): as expressões exatas históricas Args41 e
With10, mais None/Args nos dois lados seguido de erro cuja ocorrência está
antes da chamada. Não se atribuem quatro perfis ou três ordens a esse focal.

O patch principal acrescenta 15 funções de teste; o suplemento acrescenta
três. São grupos de fixtures, não 18 observações únicas. Diagnostics exigem
um erro, severidade error, mensagem inteira, hints vazios, source/range
half-open e todas as entradas de trace com nome/range. O World base do
harness retorna NotFound para Source, deliberadamente: preencher seu source
com o código testado ocultaria o bloqueio do entrypoint.

Os três casos named de panic **não são paridade vanilla**: conservam a
mensagem portuguesa e span primário detached medidos no baseline. O trace
da chamada resolvível é derivado da regra L0 de trace_call e da nova obrigação
Source, conforme decisão explícita do dono anterior ao candidato. Não
derivamos esse delta de execução candidata nem preservamos ausência acidental
de trace. O replay publica essa política por caso.

Os controles de delegação são obrigações do trait/L0, não novos resultados
de medição vanilla: World custom fornece input `audit=preserved` e um source
externo por include_source; o resultado mantém ambos dentro/fora do import.
O callback externo exige os parâmetros em 16..23 da Source externa e trace
filter em 35..71 do code local. Essa âncora é aplicação da regra lexical já
medida para closures, não lookup por valor/formatter. Estes testes não
pretendem cobrir cada método de World nem filesystem, fonte real ou plugin.

None/Args erro posterior foi medido em ambos os lados: occurrence 27..34,
trace encode 37..74. A identidade deve preservar a origem, além da string
repr. Arrays, None vazio, callback vazio, native type sintetizado e operação
não suportada são controles explícitos.

As três migrações não removem testes:
Location para `location(..)` em repr_value_complex_types;
Args41 multiline integral em p1305_args_fields_remain_integral;
With10 anônimo em p1293_d_ten_short_names_with_calls_and_flat_aliases_preserved.
A referência a With “depois da primeira falha” da nota inicial significa a
execução posterior da suíte; **não** é um segundo assert dentro de
repr_value_complex_types. O local correto é o teste wiring acima.

## Replay e discriminação

Interface:

```text
python3 00_nucleo/diagnosticos/p1308-contract-replay.py \
  --binary PATH --case REGEX --profile default|html|a11y|combined|all [--reverse]
```

Não escreve arquivos; stdout JSON contém binário/hash, casos/fontes/argv,
observable raw, expected, policy e verdict. Reverse só muda a ordem, nunca
comparação. Exit/stdout/stderr são comparados integralmente; exit fora de
0/1, timeout ou falha de execução é Unknown. R6 runner permanece intacto.

Autocontrole do adapter em `2026-09-07T20:53:09.962954+00:00` contra o baseline imutável,
casos filter.alias, panic.named-invalid, repr.args-width-48/49, default e
reverse: 2 Preserved, 2 Violated, 0 Unknown, como previsto. Não é GREEN de
candidato. O trace named esperado foi explícito e anterior ao candidato.

Mapeamento mínimo para os seis mutantes independentes:

| Família | Testemunhas |
|---|---|
| Args/None rejeitado ou origem perdida | p1308_none_identity; p1308_none_identity_preserves_error_origin |
| panic lista em vez de call | p1308_panic_call_origins; p1308_panic_callback_origins |
| boolcast nome curto | p1308_filter_direct; R6 filter-non-bool |
| boolcast origem errada | p1308_filter_alias/with/spread_sources/synthetic_native |
| Source/trace indisponível | p1308_r6_frozen_diagnostic_obligations; external callback/source |
| Args longo inline/elidido | p1308_args_width_boundaries; p1308_args_integral_unicode_and_long; Args41 migrado |

A morte de mutantes Rust, RED/GREEN, cobertura exaustiva de World e veredito
final pertencem aos outros papéis. Nenhum resultado deles é reivindicado.

## L0s lidos integralmente antes dos testes

Também foram lidos 01_core/CLAUDE.md e a skill com suas duas referências.
Pins dos owners relevantes após os refinamentos documentais do dono:

```text
fd0a7d7d5d1d264e33e4d1bd344aeb588899ee142e3419cdadf1d720ef12ce60  00_nucleo/prompts/compiler/eval/tests.md
5b6e94a1f49fd9471bb4f79734bd430dcf3ae9ea525c78b96457f8b3bcf0b30a  00_nucleo/prompts/compiler/eval/repr.md
01c2f977e9ca5ba75d3b4e506487a0cf765b7a8510b2da4e70c52ceb31925bc4  00_nucleo/prompts/wiring/tests/p1293_contract.md
15c8c3ab6443fd0796f590bdabf06dc7b3e77d7ad6f0a2aad103855f7b081dd1  00_nucleo/prompts/compiler/eval.md
5be09eca7734b8e76530176ac0cfce83282fe758fc7099f1dd8f465d509adaa2  00_nucleo/prompts/compiler/stdlib/collections.md
74ff72307542485d9ab428de42dfc74c9d993393cd3b7213c6127819fa97089f  00_nucleo/prompts/compiler/stdlib/panic.md
304b7768a6be5a81af507ad45e2f6d784eb5d96d571bb323655a18a713d9d289  00_nucleo/prompts/entities/func.md
650ff91457330250d7b983c052b5cbd246061ef85da7ee5a7484d0295c702a4e  00_nucleo/prompts/compiler/eval/closures.md
9ea195d7c1a96d592fde11c58024188a9857e8a973c2c63b1a127f98eeb6da8e  00_nucleo/prompts/compiler/eval/call_dispatch.md
3231311640907491de66e32e1779c4dfec2036dad0cba42e502d863b699e8499  00_nucleo/prompts/compiler/eval/operators/arithmetic.md
cd2b0d775bf3dd759908f3440eb90758cfef8f0ed8d8529f0cce18be9ef12078  00_nucleo/prompts/contracts/world.md
```

