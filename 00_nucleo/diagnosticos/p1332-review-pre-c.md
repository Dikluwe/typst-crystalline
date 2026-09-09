# P1332 — parecer sobre freeze e integração A/B

**Favorável ao freeze e à integração pré-C; candidato ainda depende de RED
compilado revisado.** Regime A/B sem atestação de isolamento, sem selo de
refinamento. Nenhum input julgado foi editado pelo revisor.

Manifesto `b32fa0ac39b79d7a1b7f23f62540011b58ac68e20f0c850009f688b68b412084`;
norma `ac26a9a0492f250e4a59175256b8b1b7a4a173546dc483131a704f01262b84ad`.
Freeze `p1332-ab-freeze.json`:
`7e2e2350cefbf5316131cd8ff9ccc0baf53be0efb36d3a0a4ffe059990c21719`.
Integração `p1332-test-integration.json`:
`e77f14a829afa0e69dfe1dfb0df3315cd1bc62d3eb4c8f048576b6d2ad74f2c6`.

## Verificação da cadeia

`node 00_nucleo/diagnosticos/p1332-review-pre-c-audit.cjs` terminou com
exit 0 em `2026-09-09T14:42:35.604Z`. Conferiu hashes de todos os artefatos
e inputs no freeze, dos snippets/progenitores na integração, do manifesto
e da norma. Cada snippet entregue está presente uma única vez no owner.
Revertendo em memória somente os sucessores e retirando o novo snippet,
o owner é idêntico ao baseline inteiro, salvo header de linhagem. Nenhum
C foi encontrado ou julgado. A integração preservou os testes existentes
e não moveu código para fora dos módulos cfg(test).

Os números abaixo provêm desse script e dos recibos pinados. Baseline
P1332 registra HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working
tree não commitado, com inventário/diff/stat. A integração registra
estado antes/depois em `2026-09-09T14:41:01.282719+00:00`. Os diffs já
existentes estão registrados no parecer de escopo; sobre esse baseline,
apenas calc.md e calc.rs mudaram, por L0/header e testes integrados.

## Migração histórica e observação pública

O ledger descreve exatamente sete linhas alteradas nos quatro sucessores:
quatro no sucessor P1328 e uma em cada P1329/P1330/P1331. A reconstrução
linha a linha confirmou o ledger sem mudanças adicionais. As quatro
asserções de Tracepoint passam a exigir abs. No bloco string/symbol de
P1328, os dois nomes e sua condição auxiliar são migrados juntos; o braço
sqrt e suas asserções continuam intactos. Não houve substituição em fonte
de expressão, mensagem de aridade ou nome de outra função.

O corpus congelado possui 616 células, das quais 504 históricas e 112
novas, nos quatro perfis. Cada BASE histórico coincide integralmente com
a expectativa final P1331. A revisão reconstruiu independentemente toda
expectativa P1332 a partir da saída BASE:

- 84 células históricas mudam somente o nome de trace.
- 60 células novas mudam por nome: traces e 16 interpolações primárias de
  gradientes/show. Nenhum outro campo dessas observações mudou.
- Todas as demais células mantêm exit/stdout/stderr completos do BASE.

O runner compara a observação literal do candidato com a expectativa
congelada. A função de transformação é usada na autoria pré-C do oráculo,
não na saída do candidato. O próprio stderr preserva a expressão
`calc.abs`, o texto primário `calc.abs()` de aridade e nomes alheios.

## Testes novos e dívida de math

O snippet novo foi lido integralmente. Verifica nome via lookup/std/alias/
import e With aninhado; identidade e desigualdade na linguagem; resultados
inteiros exatos; repr; erros completos das famílias com origem externa;
warnings, UTF-8 e guards. São quatro testes Rust que percorrem perfis e
casos internamente. A cobertura transitiva gradient/show está no corpus
CLI congelado. O GREEN posterior deve percorrer todos esses casos; o RED
de um loop que para no primeiro erro não prova execução de seus restantes.

A sonda `{import calc: abs; $abs(-1)$}` refutou a hipótese inicial de
paridade: BASE publica equation/math.delimited, vanilla rejeita content.
As quatro células foram classificadas como
`preserved-debt-imported-math-resolution` antes do freeze. Seus outputs
esperados continuam literalmente BASE, não o resultado vanilla inventado.
O raw baseline preserva a classificação inicial `preserved-parity` como
histórico; o freeze descreve a correção e os resultados brutos a sustentam.
Essa classificação não autoriza corrigir resolução de math, fase ou
coerção em P1332. As rotas math históricas continuam protegidas.

O hash do runner usado na coleta raw difere do runner congelado após essa
correção pré-C; o recibo mantém o hash anterior e o freeze identifica o
novo. A decisão está temporalmente documentada e não houve contato com C.

## Gate restante

Ainda não há aprovação para iniciar C por este parecer: aguarda-se
`p1332-unit-red.json` com compilação bem-sucedida e falhas exclusivamente
nas obrigações de nome. Falha de fixture, importação, compilação ou
controle preservado exigirá reabertura. Não se exige alteração de outros
consumers nem se declara paridade geral de abs.
