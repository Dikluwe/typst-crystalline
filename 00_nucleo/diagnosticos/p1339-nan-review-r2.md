# P1339 — revisão delimitada de Angle NaN (r2)

Executor: `/root/p1339_nan_review`. Regime: revisão de entradas do protocolo
completo, **executado sem atestação de isolamento**. Ambiente e capacidades
físicas compartilhados; escopo de escrita declarado limitado a este relatório.
Contexto recebido: pedido de revisar hipóteses concorrentes e caminhos permitidos,
sem candidato. Nenhum código cristalino, candidato, L0 ou gate foi lido/editado.
Foram lidos a skill `tekt-materializacao-segregada`, suas referências
`papeis-e-capacidades.md` e `artefatos-e-gates.md`, o passo explicitamente autorizado,
sua cópia histórica, as fontes vanilla indicadas e os recibos de fronteira abaixo.
A busca em `00_nucleo/adr/` não localizou ADR de materialização segregada.

## Proveniência

Inspeção em 2026-09-09, marco UTC `2026-09-09T23:49:55Z`;
HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`.
`git diff HEAD --stat` vazio; `git status --short` apresentou documentos/sondas
P1339 não rastreados, incluindo o passo. Portanto, o passo é entrada de working
tree não commitada, identificado pelo hash, e as fontes vanilla lidas não têm
diferença tracked contra HEAD. Não executei novas sondas de produto: os resultados
citados são os recibos anteriores, com seu estado integral, horários, comandos e
hash do binário em `p1339-full-boundaries-vanilla-runs.json:2` e `:33`.

Hashes SHA-256 aferidos antes da leitura das fontes/recibos correspondentes:

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1339.md` | `817c3a1476897fb0a847c90183e9a9fe690994f60023126997c8022c4e8b86a9` |
| `00_nucleo/diagnosticos/p1339-step-before-angle-retification.md` | `949040e12bdc023bb791ea4205ce5af20f422a6c9bd2c0c0033079f656135140` |
| `lab/typst-original/crates/typst-library/src/layout/angle.rs` | `76edf90e5c2e38c61189644d77ef78655486230723ad1fdce6b624f8259182d5` |
| `lab/typst-original/crates/typst-utils/src/scalar.rs` | `e3b7bcf23af0ae7fedd312c74fa21b0a65cc14f1cfd62bec556155f75c902891` |
| `00_nucleo/diagnosticos/p1339-full-boundaries-manifest.json` | `b1f66474b9a0bb716b591529a2b528c97cd13a7b6d0a41908afa91738a4336d8` |
| `00_nucleo/diagnosticos/p1339-full-boundaries-vanilla-runs.json` | `c3d34f69494caac1ecb808a5c8b83bba2603716e63324f56ee530e8097638cd7` |
| `00_nucleo/diagnosticos/p1339-full-boundaries-crystalline-before-runs.json` | `5bad03dea42bb069690ec4b568ae6b7b843811bc3da459e3d497bef8a3211ba2` |

O baseline normativo é `a51e02804` (`typst-passo-1339.md:7`). Esta revisão não
reconstruiu a correspondência histórica completa entre upstream e a cópia local;
as conclusões de fonte valem para os hashes acima, cuja identidade foi congelada.

## Evidência antes da classificação

1. Em `lab/typst-original/crates/typst-library/src/layout/angle.rs:25`, Angle
   contém um Scalar privado. `raw` e `with_unit` chamam `Scalar::new` (`:34`,
   `:39`); graus/radianos, razões e trigonometria inversa delegam a esses caminhos
   (`:44`, `:49`, `:54`, `:96`, `:101`, `:106`, `:113`). Os produtores por
   negação, soma, multiplicação, divisão por float e soma de iterador delegam
   às operações Scalar (`:180`, `:188`, `:198`, `:222`, `:233`). Subtração e
   atribuição são declaradas pelas macros de operadores (`:193`, `:227`).
2. Em `lab/typst-original/crates/typst-utils/src/scalar.rs:29`, a documentação
   declara explicitamente a normalização de NaN para zero; `:30` a implementa.
   O campo é privado (`:15`), constantes são não-NaN (`:19`, `:22`, `:25`) e
   produtores aritméticos usam `Self::new` (`:143`, `:151`, `:175`, `:199`,
   `:223`, `:247`, `:269`). A construção direta em `PartialEq<f64>` (`:104`)
   cria apenas um temporário da comparação, não devolve Scalar/Angle ao chamador.
3. Dividir Angle por Angle devolve **float** (`angle.rs:211`), logo
   `0deg / 0deg` não fornece receiver Angle NaN. `deg`/`rad` também devolvem
   float (`:144`, `:150`), dividindo por escalas finitas não nulas (`:64`,
   `:249`). Infinito é distinto: Scalar admite a constante (`scalar.rs:25`).
4. O recibo vanilla registra `float("nan") * 1deg` como
   `(angle, 0deg, true, 0.0, true)` (`p1339-full-boundaries-vanilla-runs.json:53`,
   `:60`), e infinito como ângulo infinito (`:139`, `:146`). A contraparte
   cristalina registra Angle NaN para o primeiro caso
   (`p1339-full-boundaries-crystalline-before-runs.json:53`, `:60`). Isso prova
   uma divergência de construção observável, não uma entrada bilateral NaN
   válida para comparar as conversões.
5. A cláusula “NaN/infinito apenas se a sintaxe pública bilateral conseguir
   construí-los; caso contrário classificar como `Unknown`, nunca como sucesso”
   já existia na cópia histórica (`p1339-step-before-angle-retification.md:107`)
   e permanece no passo vigente (`typst-passo-1339.md:121`). A regra geral
   bloqueia `Unknown` **em caso obrigatório** (`:185`); a finalização proíbe
   `Unknown` **indevido** (`:428`). Não dizem que todo `Unknown` é obrigatório.

## Juízo e limites

A hipótese **(a)**, produção pública segura de Angle NaN no vanilla, não encontra
suporte e contradiz o fechamento dos produtores examinados. A hipótese **(b)**
é sustentada pela representação encapsulada e normalização declarada: para as
APIs seguras examinadas, NaN é excluído do domínio de receivers Angle. Essa é
evidência estrutural mais forte que falhar em descobrir uma expressão de teste.
O fato mecânico da representação sustenta uma conclusão sobre o domínio da
linguagem; não exige copiar a representação Rust no cristalino.

Minha leitura favorece **(c)**: a condição explícita delimita a obrigação. Quando
sua premissa é comprovadamente falsa no vanilla, não se deve criar uma obrigação
incondicional de preservar conversão sobre receiver NaN. O registro dessa
fronteira permanece `Unknown`, conforme o texto; ele não rende sucesso,
cobertura positiva nem demonstra equivalência sobre NaN. Infinito não recebe
essa dispensa, pois é construtível. Os resultados zero das expressões com float
NaN tampouco podem ser rebatizados como testes bem-sucedidos de receiver NaN.

A objeção **(d)** tem uma base textual real: A.1 exige cobrir “no mínimo os casos
abaixo” (`:106`) e bloqueia `Unknown` obrigatório. Mas ler todos os itens como
incondicionais elimina o significado de “apenas se”. Aplicar a condição já
escrita não altera a política. Alterá-la para aceitar opacidade desconhecida,
descartar um caso cuja construção existe ou computar `Unknown` como sucesso
seria mudança material e não está autorizada por esta revisão.

Distinção decisiva: desconhecer um produtor não prova exclusão de domínio. Se a
auditoria dos produtores fosse incompleta ou opaca, a condição permaneceria
indeterminada e o bloqueio não estaria resolvido por esta argumentação. Aqui,
a encapsulação e a normalização dos pontos de criação oferecem uma prova
condicionada à fonte congelada e às APIs seguras examinadas.

Refutadores concretos: um produtor seguro dessa versão que devolva Angle
armazenando NaN sem passar pela normalização; expressão pública que construa
Angle NaN no vanilla; fonte/binário diverso dos hashes citados; ou instrução
normativa explícita de que a cláusula condicional também impõe bloqueio quando
o domínio é comprovadamente excluído. Nenhum foi apresentado nas entradas lidas.
Não cabe fabricar NaN com `unsafe`, alterar a referência ou corrigir produtores
fora das rotas autorizadas para satisfazer a sonda.

Este relatório revisa somente a alegada necessidade de bloquear por Angle NaN.
Não escreve contrato, não implementa solução, não sela artefatos e não emite
`PASS_SCOPED` de P1339. Todos os demais gates permanecem exigíveis.
