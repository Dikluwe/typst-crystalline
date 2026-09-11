# P1339 — juízo focal independente de discriminação

Execuções persistidas: 2026-09-10T03:29:52.001457+00:00 a
2026-09-10T03:29:54.026312+00:00. Verificador `/root/p1311_review`, executado
sem atestação de isolamento. HEAD, diff/stat, status, binários, fontes, env,
argv, canais textuais/base64 e horários por execução estão no recibo
`p1339-verifier-mutant-focal-r1.json`, SHA-256
`ed7c43a101282164ab327bbe17a4a2df45b22ca2a99c968b83829fc8f85ed071`.
Registry: `04d5578d720b4c7fff47954c29d20392b7a221f8b0cb80094a3859409add3677`.
Contrato r3: `c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17`.

## Calibração executada

Vinte expressões, cada uma em vanilla pinado, rebuild puro, mode0 e negativo,
mais a execução do modo inválido 99: 81 registros brutos. Todos os vinte
trios de controle são Observed e iguais integralmente em exit/stdout/stderr.
Dezoito negativos semânticos produzem violação observável, não falha de build.
M20 aborta de fato com sinal 6: o bruto permanece Unknown e o gate devolve
`mandatory_unknown`. O modo inválido falha fechado. Os controles não têm
Unknown; as duas execuções anormais são M20 e o modo 99 deliberados.

Este focal é só perfil default, sem matriz completa de positivos/opacos,
repetição/reordenação ou prova final de preservação. **Zero full runs, sem
score global, sem selo ou GO.**

## M12: válido e rejeitado estruturalmente no focal

Controle puro compilado: SHA-256
`02d2d0601ac90509b6159d7129b6102d896be658384e6d47a1c5296457da953d`,
build `p1339-mutant-build-pure-control-r1.json`, SHA-256
`91879debddfa302ab5ce6b4dc8030fcd76fcf9b205497d05d8aa7717118421b5`.
Negativo separado: `41219b0d3201b14754acebe6fe0eb9d51c3e4eb141c9b655ef1a3921de130934`,
build `p1339-mutant-build-architecture-pure-r1.json`, SHA-256
`3447c864e1774059a060919386a189f96edf832664541b3fcccf831853d446a4`.
Ambos builds reais concluíram com exit 0. O runner independente verificou
os inventários Rust/Cargo completos das três árvores contra os pins.

Entre controle puro e negativo, somente `crates/typst-eval/src/call.rs`
difere: controle `cb2fa9dfa313b60d39aae320d90f161685fe9bdfbc5ac7aa71419b36447f2862`,
negativo `453622472eb62ec166e5e42b1a2911e1fdce71ad772968818690083a173fed98`.
A arquitetura não é deduzida de SHA distinto nem de um nome encontrado.

Grafo efetivo lido e resolvido nas fontes congeladas:

1. `typst-eval/src/call.rs:249` resolve método no scope do tipo; Value::Type
   delega field ao Type. `foundations/ty.rs:119-134` usa o mesmo scope.
2. `typst-macros/src/ty.rs:96-118` liga o scope Angle ao NativeScope gerado;
   `scope.rs:159-182` registra `Angle::to_deg_data` com o parent correto.
3. `typst-macros/src/func.rs:389-421` gera wrapper que parseia e chama
   efetivamente o método proprietário; `foundations/func.rs:326-354` invoca
   a nativa e conserva Args.finish. Não é uma chamada escolhida por repr.
4. `typst-eval/src/call.rs:52-70` faz as rotas estática e ligada convergirem
   em call_func, inserindo o receiver ligado. Ambas chegam a
   `layout/angle.rs:149` → to_unit(Deg), cujo raw_scale é PI/180 em `:249-254`.
5. No negativo, `call.rs:60-76` conserva a validação real e depois substitui
   exclusivamente o resultado ligado de Angle.deg pela função `:175-177`,
   que contém uma segunda divisão `angle.to_raw() / (PI / 180.0)`. A rota
   estática continua no owner original. A duplicação não depende de env.

O focal `(angle.deg(90deg), (90deg).deg())` devolve `[90.0,90.0]` nos quatro
binários, com exit 0 e stderr vazio. Esta igualdade é esperada e preservada;
a rejeição é a segunda causa efetiva comprovada pelo grafo compilado, não
um falso resultado comportamental. M12 satisfaz a elegibilidade estrutural
e viola W10 no recorte focal. Revalidação final ainda é obrigatória.

## Incidente do runner, sem crédito oculto

A primeira tentativa completou os subprocessos, mas o persistidor passou
o patch grande como argumento e falhou com `OSError: [Errno 7] Argument list
too long: apply_patch` (sessão 86112). Seus canais individuais não foram
persistidos, portanto nenhum deles é usado como evidência. A correção foi
somente transporte do patch por stdin, já usado pelo intake. Reexecução
integral do mesmo focal persistiu o recibo acima (sessão 97969). Nenhuma
expressão, expectativa, mutante, contrato ou produto foi alterado. Não se
conta a tentativa perdida como repetição determinística nem como full run.
