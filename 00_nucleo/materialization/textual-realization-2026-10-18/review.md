# Revisão R3-T1 — escopos, precedência e produtos textuais

**Estado: proposta pronta para revisão; código e L0 vigentes não alterados.**
Gate obrigatório da ADR-0127 §2.3: mudar a ordem de realização das receitas de
conteúdo para decidir a raiz antes dos filhos que ela pode ocultar, e retirar
show textual provisório da mera fabricação de um valor ContentBlock. A aprovação
R2-S cobre Sequence; não cobre esta ordem textual/genérica adicional.

## Evidência anterior à proposta

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, working tree não commitado. Fontes/L0 completos,
UTC, status e `git diff HEAD --stat` em
[before-sources.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/textual-realization-2026-10-18/before-sources.json.gz). Oracle pinado
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore
`cac836c07e4dec2a89219370428292393a3216b5`, binário SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Tekt SHA-256 `57154005875bca986a1c4eb44038c8bd9ad2b3580e5534059574895c94e62f82`.
Fontes exatas, argumentos, stderr, texto e horários em before/scope/refuters.json.gz
no [diagnóstico](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/textual-realization-2026-10-18/relatorio.md). Versão CLI isolada não foi usada como prova.

| Família | Tekt atual | Vanilla pinado |
|---|---|---|
| Dois literais idênticos | C | B, regra mais recente |
| Literal mais recente vs regex anterior | R | L |
| Match AB antigo vs A recente | X | YB |
| Primeiro efeito em ZA | panic LATER | panic FIRST |
| Callback encadeado, ordem inversa | BBBB | BB |
| Produto A seguido de B de outra ocorrência | X | AB |
| Produto composto [A]+[B], como uma unidade | AB | X |
| Strong substitui o filho B | panic HIDDEN | Q |
| Valor sob global antiga, inserido sob global nova | L | R |
| Valor com regra local, inserido sob global nova | L | L |
| Show local sobre CJK com newline | 水果 | X |

São observáveis de linguagem, não exigência de buffers, vtables, igualdade Rust
ou bytes de exportação. Texto igual isolado não demonstra estilo ou paridade geral.
A contagem de sondas não é contagem de bugs. A hipótese causal e os refutadores
estão nos L0s candidatos; implementação será julgada por resultados e efeitos.

## Mudança proposta

1. **Escopo:** frames privados distinguem regras herdadas da inserção e delta
   introduzido pela cauda lexical do valor. Compor global vigente e delta local
   por RuleId em ordem lexical. Uma regra global antiga não congela o valor;
   a regra local acompanha sua cauda. Let não emite conteúdo ao avaliar o binding.
2. **Seleção textual única:** literal/regex competem por menor início UTF-8;
   no empate vence a regra lexicalmente mais interna/recente não revogada.
   Preservar prefixo/sufixo e efeitos em ordem. Não aplicar um selector a todo o
   input antes de considerar os concorrentes.
3. **Produtos:** a guarda acompanha a ocorrência transformada. Dois produtos
   independentes continuam fronteiras, mesmo com guardas iguais; nós emitidos
   juntos dentro de um produto podem agrupar entre si. Constantes, aliases e
   callbacks recebem regras restantes uma vez, conservando proteções concluídas.
4. **Realização causal:** novo owner privado `compiler/realize/recipes.rs`
   coordena raiz e depois apenas filhos emitidos. Substitui a combinação atual
   de prepass Sequence global e map bottom-up de ShowRecipe; delega a lógica aos
   owners Sequence, apply e textual. Função textual avalia seu código uma vez e
   captura as receitas dos filhos, permitindo que um pai os oculte antes de efeitos.
5. **Transporte:** tipos públicos atuais; frames e snapshots compartilhados por Arc.
   Classificação herdada/local via metadado privado em ShowRecipe/styles;
   conclusão por proveniência em Realized.original e visual separado. Nenhum
   campo, variante, trait, assinatura pública ou Location artificial.
6. **Espaços/estilos:** normalizar o fluxo emitido antes do match, conservar
   Text whitespace como texto, e aceitar show-set textual com Styles reais,
   restritos ao match/produto. Conservar Source, labels, Empty explícito e slots.

O núcleo candidato textual-recipe-frontier é compartilhado pelos owners, com
pins SHA-256 completos. Os mapas de domínio mantêm seus contratos; coordenação
fica no compilador. Table→placement→Cell→body, preparação Mathy antes do show da
inserção e State/Counter lazy são restrições obrigatórias, não frentes a redesenhar.

## L0s preparados e limites

[baseline-owners.json](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/textual-realization-2026-10-18/baseline-owners.json) fixa hash dos 11 L0s existentes.
[candidates](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/textual-realization-2026-10-18/candidates) contém versões completas dos 11, mais o novo L0
`compiler/realize/recipes.md` e um núcleo novo. Cada adendo identifica a medição,
a responsabilidade exclusiva, o gate e a aceitação. Esses candidatos **não estão
instalados** em prompts/; hashes produtivos somente serão selados após integração.
Nenhum histórico de L0 é apagado: a supersessão é explícita e restrita a R3-T1.

R3-T2 completa a coerção do retorno de funções show (int/float/none/bool/etc.,
hoje indevidamente rejeitados); não depende desta mudança de ordem. Símbolos,
Styled/Styles públicos, grupos sintéticos em múltiplas caudas, repr de labels,
getters/include, paridade query restante, A2–A5 e V16 de outras unidades seguem
no inventário geral. Não são declarados resolvidos pela aprovação desta proposta.
Os dois controles com show de Sequence e erros distintos continuam diagnóstico
pendente; uma primeira versão de sonda usava identificador inexistente e foi
excluída como prova da feature. Não copiar erro depth do vanilla sem análise.

## Aceitação antes de fechar a implementação

[acceptance.json](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/textual-realization-2026-10-18/acceptance.json) traz 56 linhas,
55 fontes distintas, fontes completas e respostas
congeladas do oracle. É escopo inicial de famílias e controles, não resultado GREEN.

- Testes nos owners antes do código e RED real. Reproduzir todas as famílias;
  adicionar refutadores de estilos, spans/trace, retorno alias/fresh, restauração
  em erro e execução uma vez. Cores/estilos exigem observação visual real.
- Revalidar S1–S20 ratificados, coalescimento/EOF/slots, none/children, prefixo e
  cauda após 1fr, contextos/State/Counter, query/originais/produtos, Table/Cell,
  Equation e PDF/HTML separadamente. Não relaxar expectativas para obter GREEN.
- Build, workspace e doctests separados, Clippy/Rustdoc estritos, fmt,
  V1/V5/V15/V26 e linhagem reversa. Comparar SARIF completo com a baseline:
  106 V16:error continuam pendentes; esta proposta não silencia esse gate.
- MemoryMax 12 GiB, MemoryHigh 8 GiB, swap zero, Cargo jobs1, debug0,
  incremental0, comandos pesados sequenciais. Medir pico/tempo pareados antes
  de alegar ganho. Snapshots por Arc e runs por intervalo, sem estado por byte.

Aprovação autoriza atualizar os L0s candidatos no caminho canônico, escrever
os testes e implementar/validar esta ordem e o transporte privado. Não autoriza
contrato público adicional, commit, push, merge, publicação ou release.
Se um novo contrato público for necessário, preparar revisão específica antes.
