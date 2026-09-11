# P1339 — viabilidade dos gates C/E, revisão r1

Revisão limitada, anterior a contrato, selo e candidato. Regime: `executado sem atestação de isolamento`. Executor `/root/p1339_protocol_r1`, contexto inicial somente da tarefa; capacidade técnica de escrita compartilhada, restrição operacional a este diagnóstico. Nenhum material julgado foi editado. A skill `tekt-materializacao-segregada` e suas duas referências orientaram a separação entre evidência semântica, estrutural e de capacidade.

## Proveniência

HEAD observado: `2f42d64253547734564513a1159ee6b584c1c4b4`; `git diff HEAD --stat` vazio às `2026-09-09T23:30:37Z` e `23:31:45Z`. Working tree com diagnósticos P1339 e passo não rastreados; estes não são produto candidato. Inputs foram hashados antes da leitura, com uma exceção na extensão solicitada: a busca em `compiler/eval/repr.rs` precedeu seu hash, registrado imediatamente depois e antes da leitura detalhada. Essa falha de ordem não é apresentada como conforme ao freeze ex-ante.

Pins principais SHA-256:

- P1339 corrigido: `817c3a1476897fb0a847c90183e9a9fe690994f60023126997c8022c4e8b86a9`.
- `p1339-resume-r1.json`: `657f6eeeee2aa8a30a5443f930125231af38ce0088c63956ce17d38bb112fa02`.
- `p1339-authority-manifest-r1.json`: `074ec7a3cfeea34f0f6a230dcd4cc4b9b2777a860ce439bdad8c774c034f38fc`.
- `p1339-a0.json`: `c70ca7d1f22aa7df222a081da4feddbc056b628eca505449eedb153acd71c361`.
- Revisão preliminar: `506f386dc23c27001a8ce147738356dac1b20308490801124b0f2b1e8fabf4e7`.
- Skill: `66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48`; referências papéis/capacidades `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417`, artefatos/gates `16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d`.

## C não exige implementar o produto antes de selar

Medição normativa: P1339:257 permite ao adversário ler baseline e produzir patches mutantes isolados; :278-303 exige compilação e testemunha, sem restringir a base dos mutantes ao candidato cristalino. A proibição em :329 alcança implementação produtiva, não os negativos explicitamente autorizados em C. A referência pinada já possui as funções: vanilla `layout/angle.rs:140-152`, `foundations/float.rs:32-191`, `foundations/func.rs:373-375,395-450`, `foundations/version.rs:114-137`. Assim, a ausência das rotas cristalinas não cria, por si só, circularidade.

Caminho executável: adversário aplica cada patch a cópia isolada da referência pinada, compila essa cópia, executa os mesmos casos públicos contra referência íntegra e mutante e conserva patch, comando, binário e testemunha. O baseline original continua intacto. Após implementação, F:370 volta a executar ataques, com instanciações no candidato cuja validade precisa ser comprovada; a calibração prévia não substitui F.

Locais concretos para ataques: nomes/unidades em `angle.rs:143-151` (#1-2); constantes, `signum`, endianness, comprimento, size e tipo retornado em `float.rs:34-39,114-115,140-152,177-191` (#3,5-10); stub substituindo corpo de função com descoberta intacta (#11); concatenação preargs/new em `func.rs:373` (#13); validação de elemento e campos em `func.rs:430-450` (#14-15); ramo de índice negativo em `version.rs:121-132` (#17). #4 deve atingir igualdade pública de NaN, nunca apenas igualdade interna de Rust. #16 pode diferenciar a rota estática no dispatcher vanilla (`typst-eval/src/call.rs:57-71`), mantendo a ligada como controle. #18 ataca consumo de argumentos, mantendo o caso inválido concreto medido. #19 pode alterar descoberta de uma rota excluída numa cópia do baseline cristalino, comparando com seu antecedente; alterar a referência e exigir igualdade vanilla onde o cristalino já divergia seria um controle errado.

Viabilidade de compilação, sem alegar builds executados: `rustc --version` retornou `rustc 1.92.0 (ded5c06cf 2025-12-08)`. `cargo metadata --manifest-path lab/typst-original/Cargo.toml --locked --offline --filter-platform x86_64-unknown-linux-gnu --format-version 1 >/dev/null` terminou com exit 0. A consulta sem filtro falhou por `android-tzdata v0.1.1` ausente, mas a resolução do alvo Linux refutou a hipótese de bloqueio necessário por essa dependência. Não há `lab/typst-original/target` neste ambiente; custo de build não foi medido. Nenhum mutante foi escrito ou compilado nesta auditoria; não há score observado.

## #12 e #20 exigem testemunhas de natureza própria

#12: `typst-eval/src/call.rs:57-71` encaminha função estática e método ligado a `call_func`, com receiver inserido no segundo caso; `angle.rs:64-65,143-151` contém a conversão compartilhada. É tecnicamente possível duplicar a expressão em um ramo estático isolado mantendo os valores. Testes de saída não distinguem essa duplicação equivalente. Inferência refutável por uma divergência pública demonstrada do mutante concreto.

A obrigação arquitetural já existe em P1339:233,291,335,427. Logo é permitido construir, antes do selo, testemunha estrutural separada: análise da resolução de chamadas/AST dos dois caminhos, localização do owner e ausência de fórmula no roteamento. Precisa aceitar referência estrutural íntegra, rejeitar o patch compilável que duplica a fórmula e não aceitar uma chamada morta ao owner como prova de delegação. Não basta comparar texto, contar símbolos ou introduzir defeito numérico junto da duplicação. O contrato pode exigir simultaneamente saída e arquitetura; isso não torna a mecânica vanilla critério de paridade (ADR-0107).

#20: `Unknown` é resultado do protocolo, não um valor emitido pelo compilador. O ataque apropriado é uma mutação executável do adaptador/classificador que converte diagnóstico obrigatório em `Unknown`, rejeitada por verificador exterior intacto; o controle opaco deve continuar aceito como `Unknown`. Se for exigido artefato compilado, esse adaptador pode ser um executável compilado. Não simular kill editando somente JSON de expectativas, nem fazer o mutante julgar a si mesmo.

O registro deve identificar classe, alvo, compilação, validade e testemunha de cada ataque. O score semântico da skill permanece separado das obrigações de arquitetura/protocolo; o agregado exigido por P1339 só pode ser 1.0 quando todos os ataques válidos exigidos forem realmente rejeitados. Não remover #12/#20 do gate, não declarar 20/20 sem execuções e não contar falha de compilação como rejeição.

## A capacidade final é realizável, mas ainda não foi instituída

P1339:262-264 admite relatar ausência de atestação global; :434 continua exigindo verificador sem capacidade de corrigir. O manifesto r1 registra `capabilities_enforced: false`; agente instruído a não escrever não satisfaz sozinho :434. A referência da skill permite ferramenta determinística como autoridade verificadora.

Sondas de capacidade, sem writes no produto:

| Comando | Resultado |
|---|---|
| `unshare --user --map-root-user --mount --pid --fork /usr/bin/true` | exit 0 |
| `bwrap --ro-bind / / --unshare-user --unshare-pid --unshare-net --proc /proc --dev /dev -- /usr/bin/true` | exit 1; NETLINK_ROUTE/EPERM |
| `bwrap --ro-bind / / --unshare-user --unshare-pid --proc /proc --dev /dev -- /usr/bin/true` | exit 0 |
| `unshare --user --map-root-user --net --mount --pid --fork bwrap --ro-bind / / --unshare-pid --proc /proc --dev /dev -- /bin/sh -c 'test ! -w /repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/foundations/float.rs'` | exit 0 |

A última sonda demonstra primitivas disponíveis para montagem RO e namespace de rede, não isolamento completo do futuro verificador. Caminho legítimo: executable verificante previamente congelado, subprocesso sem ferramentas de edição, inputs/snapshot RO, diretório exclusivo de recibos RW, montagem por allowlist mínima, sem sockets de host nem descritores herdados com escrita, ambiente/contexto declarados. Montar `/` RO sozinho deixa sockets visíveis e não basta como desenho final. Testar capacidade negativa e registrar configuração efetiva antes de atribuir o papel; o processo verificante emite o veredito, e outro agente pode apenas apresentar esse recibo. O manifesto possui papel final ainda não atribuído; preenchê-lo assim não exige reduzir o regime nem renunciar a :434. Não atesta retroativamente isolamento de autores anteriores.

## E: limitação concreta de representação em `function.where`

Solicitação adicional do operador: conferir alternativas existentes para selectors de `strong`/`emph`/`text`. Leitura: `Value::Selector` usa somente `entities::selector::Selector` (`value.rs:19,160`); seu enum fechado (`selector.rs:28-61`) usa `Kind(ElementKind)`, labels, locations, composição, regex, filtros e ancestralidade. `ElementKind` não contém `Strong`, `Emph` ou `Text`; contém `Raw`. `show::NodeKind::Strong/Emph` existem (`show.rs:27-28`), porém pertencem a outro carrier e não cabem em `Value::Selector`.

`selector_matching.rs:24-99` converte o carrier público para show; :150-168 faz `Where` restringir a base e `And` vazio retornar falso. Composição não cria a identidade de elemento que falta. Embuti-la em label/regex/campo sentinela mudaria o significado dessas variantes. O L0 `entities/selector.md`, cláusula P1284, já trata novas variantes públicas como gate ADR-0127; `entities/element_kind.md`, Invariantes, exige inventário para novas variantes e proíbe catch-all.

Inferência: se A.1 confirmar como obrigatórios `where` para esses elementos, o desenho precisará ampliar um contrato público (por exemplo, identidades explícitas apropriadas no carrier), acionando B:242-244. A ampliação exata deve ser definida a partir das medições e do inventário de consumers antes da pergunta humana; não está autorizada somente pela correção Angle→float. Refutação: demonstrar carrier existente com identidade correta, resultado público de tipo selector e sem reinterpretar variantes. Para `raw` essa objeção não se aplica, pois há kind legítimo.

Hipótese adicional submetida pelo operador, ainda não autorizada: `Selector::Element { function: Func, fields: Dict }`. É representacionalmente plausível; fields é dado do seletor e não registro genérico de propriedades de tipos. Distingue filtro vazio da variante `Kind` e permite um grupo de campos, ao contrário de apenas `Element(Func)` mais nenhum `Where`. O operador informou a medição independente `strong.where(:)` para filtro vazio; esta revisão não a reexecutou. O atual `repr_selector` (`repr.rs:1040-1064`) imprime um grupo por `Where` encadeado, exigindo tratamento explícito da morfologia no desenho novo.

Risco concreto a resolver no L0: `Func::PartialEq/Hash` (`func.rs:384-415`) usa nome para nativas, enquanto a identidade para selectors usa `native_fn_addr`/`fn_addr_eq` (`func.rs:286-337`). Não assumir que igualdade de Func prova identidade de elemento; o conjunto de funções aceitas precisa comprovar unicidade/canonicalização, aliases válidos e rejeição de `With`/closure/nativa não-elemento. Namespaces e nome não são prova de elemento. Não foi demonstrada nesta revisão uma colisão pública real dentro do conjunto aceito; trata-se de condição de validade, não motivo suficiente para rejeitar o desenho. `Dict` guarda o resultado normalizado; ordem de avaliação, duplicate named e diagnóstico continuam derivados de `Args`. O L0 `selector_matching.md` §4 prevê explicitamente reabertura por novo `QuerySelector`.

Pins das fontes utilizadas nas decisões centrais (SHA-256): vanilla `angle.rs` `76edf90e5c2e38c61189644d77ef78655486230723ad1fdce6b624f8259182d5`, `float.rs` `8e5c3d84b0263d6217e0f3d3d73dd114127f87f0d913a40eb57b5be6df74b8b2`, `func.rs` `a119f7e8aae459a8358357f398d44c48bc24783b67891ff3901b8d4b043c6a38`, `version.rs` `778b979c3da8749a60776ce04cdf0cb4a25c7052cf10f56c0e7f687ea76e81f6`, `typst-eval/src/call.rs` `cb2fa9dfa313b60d39aae320d90f161685fe9bdfbc5ac7aa71419b36447f2862`; cristalino `entities/selector.rs` `c2d64d1aa8f54c1b027c8166904b67389c39dcb5ee969d8f7c76f3fdb01d44bd`, `entities/element_kind.rs` `864a13a3903529799d85cab16f308ee254879c7401046c60ff26c34aaa2b560e`, `entities/func.rs` `9bcd2e2b0c79cae5c2478b55d9d902f78983f9119d634a04a846d93210307bde`, `eval/selector_matching.rs` `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9`, `eval/repr.rs` `a54ee85615e458bbc5b102a53769e4c756e9502d69abe2be1feaed5077e78efb`; L0 `entities/selector.md` `8dcd7bfbc6789ed0ef669d8ffd943df63aa06343f0c4ef39f23e797ec276b429`, `entities/element_kind.md` `b9853fad937cbfc2361f32903a653b55d11d3ebfbbc46ebd5b08f121d7e5f6d1`, `entities/func.md` `909cd5a85e4968d42a381b33d343e50a7a37e875817d581a6a2ebe9076057a1a`, `compiler/eval/selector_matching.md` `031ba52f7a6d5870479f0fe699a572950cce594db7688d59fc484175c10505a3`.

Conclusão limitada: não foi demonstrada incompatibilidade inevitável na ordem C→E nem impossibilidade de capacidade final; há caminhos técnicos concretos sem afrouxar gates. A fronteira de contrato público de `where` permanece uma pendência distinta a resolver após A.1. Esta revisão não sela contrato, não autoriza mudança pública, não implementa gates e não emite `PASS_SCOPED`.
