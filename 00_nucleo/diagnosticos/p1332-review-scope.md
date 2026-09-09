# P1332 — revisão prévia do alcance do nome intrínseco de calc.abs

Revisor: agente `p1332_review`, com autoridade de escrita limitada a
`00_nucleo/diagnosticos/p1332-review-*`. Regime solicitado: A/B, executado
sem atestação de isolamento. Esta revisão não escreve L0, produto, testes,
expectativas ou manifesto; nenhum candidato P1332 foi recebido ou julgado.
A skill `tekt-materializacao-segregada`, suas duas referências operacionais,
CLAUDE raiz/L1 e ADR-0107/0108/0127/0129 foram lidas. Não foi encontrada ADR
local específica de segregação na busca textual em `00_nucleo/adr/`.

## Estado e entradas

Inspeção de fonte sobre HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`,
**working tree não commitado**, entre `2026-09-09T11:22:27-03:00` e
`2026-09-09T11:29:11-03:00`. O L0 calc foi lido integralmente. Seus hashes
canônicos foram recomputados removendo somente a linha recíproca:

- `00_nucleo/prompts/compiler/stdlib/calc.md`: normativo
  `ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a`;
  arquivo integral `29ea339d98998d79cf590d21f8bee8b006ac608376427b3e35625b8f319ee8be`.
- `01_core/src/compiler/stdlib/calc.rs`: canônico
  `8d7359a9585e354c370c032b573a7133d004806b3a3c84cd881f8665bb992521`;
  arquivo integral `23b511df001a4184e2d3d02e99f1ec35c129231070411bb47188f106760fd2d1`.
- `01_core/src/entities/func.rs`: integral
  `9bcd2e2b0c79cae5c2478b55d9d902f78983f9119d634a04a846d93210307bde`.
- `p1331-final-report.md`: integral
  `b36ea5065c3d7a88e154bc87bb2e4f35f4025cc5270fb1c638e1394b4f66ca0b`.
- vanilla local `foundations/calc.rs`: integral
  `0eb0836dc8bf17ba214d04be2c14f137350766ec44b6f5afe238cf467f78f484`;
  `foundations/func.rs`: integral
  `a119f7e8aae459a8358357f398d44c48bc24783b67891ff3901b8d4b043c6a38`.

O alvo é upstream/main ratificado `a51e02804`, conforme diretriz vigente;
não foi feita sincronização do lab. Não se atribui identidade à string
de versão nem se apresentam os números de P1331 como nova medição.

Diff HEAD --stat observado (paths produtivos e L0):

```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md | 187 ++-
00_nucleo/prompts/compiler/eval/call_dispatch.md         | 53 +-
00_nucleo/prompts/compiler/eval/modules.md               | 47 +-
00_nucleo/prompts/compiler/eval/tests.md                 | 78 +-
00_nucleo/prompts/compiler/stdlib/calc.md                | 325 +++-
00_nucleo/prompts/compiler/stdlib/loading.md             | 198 ++-
00_nucleo/prompts/wiring.md                             | 104 +-
01_core/src/compiler/eval/bindings/field_access.rs       | 731 +++++++-
01_core/src/compiler/eval/call_dispatch.rs               | 92 +-
01_core/src/compiler/eval/modules.rs                    | 5 +-
01_core/src/compiler/eval/tests.rs                      | 236 ++-
01_core/src/compiler/stdlib/calc.rs                     | 1776 +++++++++++++++++++-
01_core/src/compiler/stdlib/loading.rs                  | 466 ++++-
04_wiring/src/main.rs                                  | 50 +-
14 files changed, 4239 insertions(+), 109 deletions(-)
```

## Medição da fonte anterior à classificação

O registro `calc.rs:58` mantém separadas a chave de escopo `abs` e a
identificação intrínseca `Func::native("calc.abs", calc_abs)`. O módulo
é construído como `calc` em `calc.rs:128`. O corpo `calc_abs`, a partir
de `:131`, seleciona valores/guards/âncoras sem consultar esse nome.
O header aponta para calc.md; a busca de `@prompt` nas camadas produtivas
encontrou somente esse consumer para o owner. Metadata bidirecional
confere com os prefixos canônicos acima; isso não substitui V15/V26 global.

No vanilla, `foundations/calc.rs:17` registra `scope.define_func::<abs>()`
e `:74-79` declara a função `abs`, sem override de nome. A macro
`typst-macros/src/func.rs:140-142,343-346` deriva e grava o nome; a regra
`typst-macros/src/util.rs:170-183` usa o identificador em kebab case.
Portanto o valor intrínseco é `abs`. `foundations/func.rs:166-173` retorna
esse nome e o preserva através de With. A fonte permite afirmar a origem
do nome, sem inferir uma intenção geral de arquitetura a partir do output.

Consumidores relevantes da representação cristalina:

| Superfície | Fonte | Consequência da troca no registro |
| --- | --- | --- |
| Execução e With | `eval/call_dispatch.rs:510-521` | Chama o ponteiro e funde Args; nome não decide cálculo |
| Nome e aliases/With | `entities/func.rs:291-300` | `abs` propaga naturalmente inclusive em With aninhado |
| Trace de chamada externa | `eval/call_dispatch.rs:1648` | Usa Func::name; pode corrigir trace sem editar dispatcher |
| Repr | `eval/repr.rs:76-90` | Já remove prefixo com rsplit; nome apresentado tende a permanecer abs |
| Campo inexistente | `eval/bindings/field_access.rs:427-438,681-693` | Já usa sufixo público; erro tende a permanecer igual |
| Igualdade/hash nativos | `entities/func.rs:383-413` | O nome participa da identidade local e do hash |
| Seletores | `eval/rules.rs:2479-2488` | Identidade por ponteiro; mensagem de rejeição interpola nome completo |
| Gradientes | `stdlib/gradients.rs:638-649` | Erro de `space: calc.abs` interpola Func::name e muda |
| Cor/counter/especializações | `stdlib/color.rs:275-288`, `stdlib/counter.rs:57`, `eval/call_dispatch.rs:129-134,300-310,1590,1614` | Comparações especiais não incluem abs; não ativam novos braços |

A frase “Apenas para apresentação” em `Func::name` não dispensa a auditoria
de identidade: o mesmo campo é explicitamente utilizado por Eq e Hash.
O L0 `entities/func.md` foi lido integralmente e mantém essa norma para
nativas do mesmo kind. Não há proposta de alterar essa entidade.

A busca literal em `01_core/src/compiler/stdlib/**/*.rs` por `"abs"` e
`"math.abs"` encontrou apenas o registro calc no estado observado. Não
foi identificada colisão nativa produtiva homônima que faria duas funções
distintas passarem a comparar iguais. Isso é evidência estática limitada;
não constitui prova universal contra construção dinâmica futura. O contrato
deve confirmar identidade na linguagem entre lookups/aliases/imports e
desigualdade com outras funções pertinentes. Não usar comparação de endereços
ou bits do hash como critério de paridade com vanilla.

Vanilla `Func::repr` em `foundations/func.rs:460-469` imprime With como
`(..) => ..`; o cristalino acompanha seu algoritmo de repr vigente.
Qualquer discrepância anterior em With continua dívida independente, não
se resolve alterando repr neste recorte de registro.

## Parecer de escopo e gate

**Owner calc suficiente como hipótese fundamentada.** Alterar somente o
valor intrínseco do registro, preservando a chave de lookup, ponteiro,
guards e corpo aritmético, é compatível com o desenho vigente. A atribuição
histórica da dívida de nome ao dispatcher, escrita nas seções P1329–P1331,
é imprecisa: o dispatcher publica o nome que recebe; sua origem está no
registro calc. A norma P1332 deve substituir expressamente a preservação
histórica do nome sem reescrever os recibos antigos.

**ADR-0127: fluxo contínuo, L0 primeiro e resselo, RED→GREEN e revalidação.**
A cláusula de correção de entradas de nomes e a exceção explícita de paridade
em superfície visível cobrem a convergência `calc.abs` → `abs`. Não há
assinatura/campo/trait novo, novo default deliberado, mudança de fase ou
dependência nova que exija parada. Esta classificação não autoriza mudar
outro registro, reconstruir identidade ou reformar dispatcher/entidades.

**Restrição para a próxima norma:** não prometer “todos os demais diagnósticos
ficam literalmente iguais”. O nome também aparece no erro de seleção de
gradiente e na rejeição de show selector. Esses efeitos transitivos devem ser
medidos e classificados antes de C, conservando o restante do diagnóstico
quando aplicável. Mudar a exibição do nome em uma dívida maior não a converte
em paridade integral. Named/aridade também podem ter trace alterado enquanto
suas mensagens primárias e precedência permanecem divergentes.

Recomenda-se que os casos independentes incluam lookup qualificado/importado,
alias/With/nested With, erros cuja origem fica fora da chamada, guards e
diagnósticos colaterais acima, repr/campos inexistentes e igualdade na
linguagem. As expectativas históricas só podem migrar no campo de nome
abrangido pela nova obrigação; controles de cálculo, sem coerção de math,
NaN dimensional anterior à chamada e diferenças já conhecidas preservam
sua classificação. `Unknown` obrigatório impede fechamento.

Refutariam a suficiência do recorte: uma colisão nativa real em lookup
produtivo; algum consumidor usar abs para mudar despacho; necessidade de
editar outro owner; ou um efeito observável incompatível com a obrigação
congelada que não seja explicado pelo nome. Nessas hipóteses é necessário
reabrir escopo e norma antes de implementar além do registro.

Este parecer é prévio, favorável ao recorte com as condições acima. Não
aprova candidato, RED, migração histórica nem fechamento: dependerão dos
manifestos e recibos posteriores, julgados sem corrigir os artefatos.

## Limitação operacional registrada

Um `git status --short` geral emitiu incidentalmente nomes de untracked em
`00_nucleo/materialization/`. Foi uma violação da restrição de listagem;
nenhum conteúdo dessas pastas foi aberto ou lido. O fato foi comunicado
ao root, e as consultas seguintes foram limitadas a paths permitidos.
O presente parecer não usa conteúdo dessas pastas como evidência.
