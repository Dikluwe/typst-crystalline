# Passo 1306 — nome de módulo ordinário no diagnóstico de campo ausente

## 1. Problema medido

Um arquivo ordinário chamado `std.typ` já aparece como `<module std>` depois
do P1305, mas o erro de campo ausente ainda o chama de `global`. O próximo
resultado útil é eliminar essa inconsistência sem alterar o global real,
imports, representação, spans ou lookup.

Este é um **passo de execução**, não um Prompt L0. Sua redação não implementa
a correção, não atualiza o contrato produtivo nem constitui RED independente.

A sondagem preparatória está em
`00_nucleo/diagnosticos/p1306-planning-measurement.json`, SHA-256
`eda5237e4dfafbcd1ac5d61c16a4be0172ea1a0090dff992d7dbd3d66f610cf8`.
Foi executada entre `2026-09-07T14:52:36.331705+00:00` e
`2026-09-07T14:52:42.497148+00:00`, sobre o commit
`b303f1f15b610e09872b567027e0d806387fde8c`, sem alterações tracked/staged
ou untracked no início. `git diff HEAD --stat` estava vazio. O artefato
preserva fixtures, script reproduzível, comandos, cwd, horários, saídas
integrais, hashes e estados antes/depois da medição.

Referência: vanilla ratificado upstream/main `a51e02804`, binário
`/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
O cristalino utilizado foi o binário certificado do P1305 em
`/dev/shm/p1305-r2-target.F7bquUC2/release/typst`, SHA-256
`be51045f1df75ac42ee081801ddf7f738f709029e3694b9205473ba5fa8b31d4`.
Ele foi construído antes do commit de consolidação, a partir dos mesmos
arquivos produtivos; não foi reconstruído nem identificado pela string de
versão. A relação está registrada em `p1305-commit-notes.md` e no certificado
`p1305-r2-certificate.json` de SHA-256
`414479e20c93251488e1840f1298118fc35e3b4698ba73ae35b7e923c769dccc`.

Foram oito probes nos quatro perfis, dois binários e duas ordens: 128
execuções, com repetição estável. O recorte observado foi:

| Caso | Cristalino atual | Vanilla |
|---|---|---|
| Import ordinário `std.typ`, `.nope` | Nome `global` no erro | Nome `std` |
| Alias do import ordinário, `.absent` | Nome `global` no erro | Nome `std` |
| `std.typ` que reexporta `std: *`, `.nope` | Nome `global` no erro | Nome `std` |
| Import ordinário `map.typ`, `.nope` | Nome `map` | Nome `map` |
| Global real e seu alias, `.nope` | Nome `global` | Nome `global` |
| Repr e lookup do import ordinário/reexport | `["<module std>",7]` | Mesmo valor |

Em `01_core/src/compiler/eval/bindings/field_access.rs:376-382`, a falha do
lookup converte nominalmente todo `m.name() == "std"` para `global`. O span
de `Module` já é field-only em `:106-112`. No vanilla,
`lab/typst-original/crates/typst-library/src/foundations/module.rs:139-150`
projeta o nome guardado no módulo nomeado. Os constructors do global
cristalino já guardam `global` em `eval/mod.rs:324,562` e
`eval/modules.rs:66`; arquivos ordinários conservam seu nome próprio.

## 2. Decisão e limite da inferência

Selecionar a coorte **`named-module-missing-field-identity`**: corrigir o nome
público na mensagem de field ausente para módulos nomeados. Mensagem e span
são observáveis da linguagem, inclusive texto exato do erro (ADR-0107/0108);
não se exige igualdade da estrutura Rust ou do algoritmo upstream.

É inferência, a confirmar pela medição independente antes do candidato, que
o nome já armazenado é informação suficiente para todo esse recorte. O caso
de reexport integral refuta reconhecimento por conteúdo do scope; o alias
refuta usar o nome lexical da variável como identidade do módulo. Refutariam
a solução estreita um módulo nomeado legítimo sem informação suficiente,
uma rota que ainda guarde nome incorreto, ou necessidade de outro owner/API.
Não se infere intenção histórica do vanilla da simples observação do erro.

Esta é dívida já registrada no P1305, **não regressão temporal atribuída ao
P1305**. A prioridade aqui é a inconsistência local comprovada com uma rota
ordinária funcional. Não se alega uma nova classificação exaustiva de todos
os residuais. Encoders JSON/TOML/YAML, anonimato de plugin e outras ausências
continuam pendentes; não entram como segunda coorte.

Gate esperado: `ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`, condicionado ao
desenho continuar interno, sem entidade/trait/assinatura pública, default,
compatibilidade ou mudança de fase. Na dúvida ou necessidade de expansão,
parar e pedir decisão ao dono. Este passo não autoriza tal expansão.

## 3. L0 primeiro; owners permitidos

L0 vigentes lidos antes desta proposta:

| Prompt proprietário | SHA-256 inicial | Consumer |
|---|---|---|
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `92aa897909abfb6095ab59191614b0fd87c0d648e8e593ed8f0702176c2a542e` | `01_core/src/compiler/eval/bindings/field_access.rs` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `0d90e29edd7c540a8605235f8a5b7930d827243e86539deb0a25138a5ed77e7a` | `01_core/src/compiler/eval/tests.rs` |

Antes de qualquer Rust, a execução deve atualizar esses L0 com a medição
independente e a decisão perene. O L0 de field access ainda descreve em
P1301 o global internamente chamado `std`, e P1303 proíbe mudar sua projeção.
Essas cláusulas não podem ser ignoradas: registrar explicitamente a revisão
estreita, substituindo a premissa superada pela construção P1305 e
preservando o comportamento público do global real. Não reescrever medições
históricas como se tivessem observado o novo estado.

O contrato novo deve exigir nome armazenado para módulo nomeado; o global
real continuará dizendo `global`, e o arquivo ordinário `std.typ`, `std`.
A regra diagnóstica geral, o field-only e as proteções P1303 continuam.
No L0 de testes, distinguir o global real do import ordinário nos oráculos,
sem apagar ou enfraquecer as regressões P1300/P1301/P1303/P1305.

Confirmar ownership 1:1 e Núcleos com V15/V26 antes de resselo, segundo
ADR-0129. Depois de L0-first e gate aplicável, a implementação pode mudar
somente o consumer de field access; o segundo consumer recebe apenas testes.
Não alterar `eval/mod.rs`, `eval/modules.rs`, `repr.rs`, entidades, Núcleos,
stdlib, CLI, wiring, lab ou provas históricas. A construção e a repr já
certificadas são entradas de preservação, não outro local para corrigir o erro.

Em particular, não aproveitar o mesmo arquivo para mudar os reconhecimentos
nominais `pdf`/`sym`, warnings, gates de features, erro de dict ou de float.
Se esses recortes exigirem trabalho próprio, registrar diagnóstico separado.

## 4. O que precisa ser provado

Na execução, usar a skill `tekt-materializacao-segregada`, com contrato,
oráculos/ataques, implementação e veredito sob autoridades separadas. Ficam
autorizados subagentes com essas tarefas delimitadas. O autor de oráculos
recebe spec/contrato/baseline, não patch candidato; o verificador não corrige
os artefatos que julga. Declarar ausência de atestação técnica quando for o
caso. A sondagem preparatória acima não substitui essa autoria independente.

Congelar um corpus **focal** que cubra, nos quatro perfis:

- imports ordinários `std.typ`, `global.typ`, `map.typ` e nome não reservado;
  aliases, sombrear lexicalmente `std`, acesso aninhado e reexport integral;
- global real e aliases, módulos calc/sym/color.map e fields existentes;
- fields ausentes distintos, incluindo `nope` e outro identificador, com
  deslocamento de coluna e de linha para discriminar nome e span;
- erro de severidade error, mensagem exata
  ``module `<nome>` does not contain `<field>` ``, zero hints/laterais novos,
  stdout vazio, exit correto e span resolvível somente no field;
- preservação dos spans/hints/gates PDF P1303, erro de dict total,
  `float("NaN").is-nan` field-only e dos comportamentos de imports/repr P1305.

Mensurar aliases/reexports com fixtures válidas e chamadas/lookup positivos
pareados. Não aceitar erro de import, arquivo ausente ou falha de adaptador
como RED. Não remover warnings globalmente do comparador: quaisquer dívidas
laterais preexistentes devem ser individualizadas antes do selo. Diagnóstico
dentro de arquivo importado pode exigir separar path de apresentação e
âncora da fonte; não normalizar caminhos para ocultar mudança real.

Os oráculos precisam discriminar ao menos: manter a conversão nominal antiga;
chamar todo módulo de global; chamar o global real de std; usar o nome do
alias; corrigir somente `nope` ou somente um perfil; regredir span para alvo
ou expressão inteira; acrescentar/perder diagnóstico; apagar ou alterar um
lookup bem-sucedido. Executar mutantes reais aplicáveis em cópia descartável,
com testemunha correta e restauração verificada. Selecionar âncoras pelo
owner/função, não por uma ocorrência ordinal ambígua. Falha de compilação ou
aplicação inválida não é mutante morto; preservar a tentativa e corrigir a
instrumentação sem mudar o oracle para favorecer o candidato.

RED deve mostrar a divergência de nome contratada antes da correção. Depois,
exigir GREEN dos mesmos testes e dos controles, todos os mutantes válidos
rejeitados e nenhum `Unknown` obrigatório. Não inventar caso opaco só para
completar matriz. Arrays, mapas e representação permanecem intocados.

## 5. Execução proporcional e gates

Congelar manifesto com HEAD, status/diff/stat, binários, L0, contrato, suites,
autoridades e paths exatos de saída antes da implementação. Evidências novas
ficam em `00_nucleo/diagnosticos/p1306-*`; declarar os nomes concretos no
manifesto antes de criá-los. Não editar a sondagem preparatória, este passo
ou os documentos P1303/P1304/P1305 durante a execução sem reabertura registrada.

O binário RAM pode ser reutilizado como baseline somente após reconferir seu
hash e a identidade das fontes. Se não existir, reconstruir em cópia isolada
do commit indicado, registrando o novo hash. `/dev/shm/` está autorizado para
temporários, sujeito às permissões reais; usar diretório exclusivo criado
por `mktemp -d` e nunca sobrescrever o target certificado.

Orçamento: até duas revisões focais com hipótese e delta observável; corrigir
primeiro o recorte que falhou. Duas revisões sem ganho na mesma causa exigem
reabrir o desenho. Após GREEN, um build candidato fresco e uma matriz focal
completa, com repetição normal e ordem invertida, são suficientes para este
recorte. Não repetir automaticamente os 627 probes de disponibilidade nem
os 15 mapas integrais: eles não exercitam essa colisão e seus owners/dados
não mudam. A preservação fica coberta pelos hashes dos owners imutáveis e
pelas regressões P1305, sem alegar nova auditoria do corpus inteiro.

Gates finais obrigatórios, com comandos e outputs integrais registrados:

```text
cargo fmt --all -- --check
cargo test -p typst-core p1306 -- --test-threads=1
cargo test -p typst-core p1290 -- --test-threads=1
cargo test -p typst-core p1300 -- --test-threads=1
cargo test -p typst-core p1301 -- --test-threads=1
cargo test -p typst-core p1303 -- --test-threads=1
cargo test -p typst-core p1305 -- --test-threads=1
cargo test --workspace
cargo build
crystalline-lint .
crystalline-lint --fail-on warning --checks v3,v4,v5,v13,v14,v15,v26 .
crystalline-lint --fix-hashes --dry-run .
git diff --check
```

Filtros precisam executar casos. Lint estrito exige zero violações e dry-run
sem reparos; reportar separadamente a dívida do lint geral, sem chamar exit 0
de ausência de warnings. Artefatos ainda untracked precisam de check próprio
antes de selar/commitar: o diff tracked sozinho não os verifica. Não alterar
evidência pinada para eliminar tabs estruturais ou whitespace histórico.

## 6. Fechamento útil, não apenas protocolar

O relatório deve começar mostrando antes/depois do erro de `std.typ`, explicar
por que o global verdadeiro continua correto, apresentar os controles que
poderiam refutar a mudança e declarar a dívida remanescente. Autoria e hashes
sustentam essa explicação; não a substituem.

Veredito positivo esperado: `P1306_PASS_NAMED_MODULE_DIAGNOSTIC_IDENTITY`.
Só emitir após correção observável, preservações, mutantes e gates completos;
caso contrário, `P1306_BLOCKED` com causa concreta. Certificado pina evidências
e recibos, relatório pina certificado, sem auto-hash ou ciclo. Registrar custos
e falhas de instrumentação sem contá-las como sucesso.

O recorte é suplementar ao catálogo P1305: não deduzir que fechar esses erros
reduz os 504 pares históricos de diferenças de disponibilidade, nem alterar
o ledger antigo. Encoders e demais coortes exigem decisão posterior própria.

A solicitação atual autoriza escrever este plano, não executá-lo.
