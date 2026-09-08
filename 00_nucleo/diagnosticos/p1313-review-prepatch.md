# P1313 — revisão anterior ao patch

**Veredito: `GO_PREPATCH_SCOPED`, em 2026-09-08T11:47:42Z.** Aprovação humana,
ativação e políticas A/B conferidas. Pode iniciar o patch dentro do L0 aprovado.
Este veredito não substitui RED→GREEN, replays, gates finais ou revisão do candidato.

## Aprovação e ativação

A resposta específica do dono «Autorizo» resolveu o gate da ampliação
CSV Path/Str/Bytes apresentada na proposta revisada. O recibo
`p1313-implementation-baseline.json`, SHA-256
`26d2e294e7e66a46b41f8274743adaa4bc285083a4b79a04a82e53f544bd349f`,
registra a aprovação e aponta para os hashes da proposta efetivamente
revisada: L0 `fe8e147fca053d9ebd9f9eeb4b6b4fa1bdf51346e00ea3f8576a6fdb38e9e4ed`
e passo `b88fce4e98f39dcfb774a526d237a253d6187aad2aeb6ee18e0bd378e1cddda1`.

Baseline de implementação em 2026-09-08T11:39:09.889363+00:00, HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado;
diff/stat e hashes integrais encontram-se no recibo identificado acima.
Na primeira conferência de ativação, o owner `loading.rs` ainda tinha SHA-256
`acdb71c8775765a6f54bcbd9925ec76d7493a6aac2b1675dcd7f1fac9e1cd8be`,
igual ao baseline. L0 ativo naquele momento:
`a03be089611106b58cf345fa8ac019ceaec2911b53b31e9f25f132b965407f2c`.

A ativação mantém a obrigação substantiva aprovada, registra o encerramento
do gate e não amplia o recorte. O texto preserva read/P1310/P1311/encoders,
Symbol como dívida separada, ordem de validação existente e política legada
de erros de parsing/opções que Bytes torna alcançáveis. O passo mantém a
paragem como histórico e a identifica expressamente como encerrada.

Foi comunicado ao autor um ajuste puramente documental: os pipes da linha
CSV da tabela devem ser escapados para preservar as colunas Markdown.
Esse ajuste não muda autorização nem contrato. O hash final de entrada A/B
deve corresponder à versão efetivamente congelada após ajustes documentais.

## Papel e limites

Revisor `/root/p1313_review`, artefatos julgados somente leitura. Escritas
restritas aos próprios `p1313-review*`. O revisor consultou baseline e código
legado para conferir a ordem de validação; não é o autor dos testes A/B.
Testador independente `/root/p1313_tests` informado pelo coordenador em
contexto novo, sem leitura de source/diff/testes locais. Filesystem compartilhado:
A/B executado sem atestação de isolamento técnico. Nenhum selo completo.

A aprovação humana já obtida não precisa ser repetida para a implementação
que permaneça dentro do escopo aprovado.

## Freeze revisado antes do patch

`p1313-ab-freeze.json`, SHA-256
`0f7ba52eb0ec820f53754274d51cb7ff4d20aab8d284d3c56161ff3d70b3e998`,
congelado pelo testador em 2026-09-08T11:46:33.577812+00:00. A revisão
recalculou os hashes de todas as entradas listadas, sem divergências.
L0 integral `373766911e4f1d56ea4ae483fdd6b66e4c1d380ba617124b403eee72b2d74bb3`;
L0 normativo `553294f33c131121926207fb203c1e4a4174b03e653afe39e5827b3a773fead3`.
Apenas a linha canônica Hash do Código pode mudar sem invalidar esse pin.
O problema de formatação da tabela foi corrigido antes do freeze.

Foram lidos o catálogo completo, o runner e o algoritmo de freeze/comparação.
O catálogo possui 144 identificadores únicos e o freeze 576 pares id/perfil
únicos, nos perfis default/html/a11y/html+a11y. A origem desses números é
o freeze pinado e seu baseline final
`712c79126b0476e9417803ced4bfc595fed6f8b2df5f6e6a6d6019573f48e746`,
cuja proveniência inclui HEAD, UTC e diff/stat anteriores ao candidato.

As políticas são compatíveis com a cláusula aprovada:

- 27 casos de cast com mensagem/tipo/origem usam observação vanilla; incluem
  named prefix/suffix, alias, With, spread, sink, múltiplas linhas e
  arguments.map com origem detached legítima.
- 21 casos de Bytes válidos usam observação vanilla, incluindo vazio,
  dictionary/header, quoting, Unicode, CRLF, transporte e conteúdo que parece
  caminho. O valor público inteiro é comparado, sem normalização.
- 3 casos Symbol usam mensagem normativa e origem/trace medidos; o oracle
  registra explicitamente que não é igualdade vanilla.
- 14 rotas novas Bytes usam controles Path/Str com os mesmos bytes e opções.
  O texto exibido da expressão é substituído antes do candidato; mensagem,
  origem detached e trace legados permanecem. Há controles de parsing,
  opções, prioridade, named, excesso e duplicatas. Essa derivação foi lida
  e não escolhe a expectativa a partir do candidato.
- 79 controles exigem preservação do baseline, incluindo read, outros
  decoders, encoders, campos de funções e rotas CSV legadas.

O runner fixa binário/argv/cwd/fixtures e perfis, remove variáveis que
alterariam o modo, exige identidade dos executáveis antes/depois e aborta
em timeout. A comparação exige exatamente os pares congelados nas ordens
normal/repeat/reverse; compara exit/stdout/stderr completos. Falta ou
duplicação de observação bloqueia. Crash, saída malformada ou timeout não
produzem aprovação implícita. Não existe fallback para aceitar Unknown.

## Replays e obrigações restantes

As políticas prédeclaradas em `p1313-preservation.py` e
`p1313-p1308-delta.py` foram inspecionadas: P1310 muda exatamente csv.wrong
para o envelope vanilla anterior ao patch; P1311 permanece igual a P1312;
P1308 permite exatamente os novos casos CSV e CSV wrong-type por perfil,
conserva o conjunto anterior de deltas e compara a referência pinada.
Os oráculos históricos permanecem imutáveis. Não há autorização de deltas
por prefixo amplo ou reclassificação retrospectiva de falhas.

Ausência estrita de chamadas ao World, origem sintética Rust e UTF-8 inválido
não ficam provados pelo corpus CLI. O freeze declara esses limites, e o
fechamento exige evidência local e revisão de código para eles. O controle
CLI de Bytes com aparência de caminho não deve ser chamado prova geral
de zero I/O. O revisor ainda não atesta essas obrigações nem o GREEN final.

GO é restrito à implementação e validação previstas, sem mudança de owner,
API Rust, fase, coerção Symbol ou política de parsing. Qualquer divergência
da hipótese local do L0 deve ser diagnosticada antes de ampliar esse recorte.
