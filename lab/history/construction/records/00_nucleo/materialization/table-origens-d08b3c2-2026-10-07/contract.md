# Contrato candidato D08B3-C2 — origem e forma lexical de Content

Estado: proposta sujeita ao gate ADR-0127; não legitima código enquanto não ratificada e instalada nos owners L0. Nenhum código produtivo ou L0 vigente foi alterado nesta preparação.

## Medição anterior à decisão

`00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz` registra oito sondas completas: fontes, comandos, UTC, HEAD, working tree/diffstat, hashes e binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8. Executável Tekt congelado do C validado; `source-verification.json` confirma hashes produtivos/L0 iguais ao snapshot tested-final do C. O target da raiz não estava disponível nesta retomada; não foi fingido um rebuild.

bare/alias/spread-conflict: mensagens e hints iguais, caret/trace divergentes. No alias o vanilla usa a definição [a] (linha 1), não a chamada (linha 3). No spread usa [a], não o spread inteiro. local-body publica sequence com metadata+Cell preparada no Tekt, styled contendo Cell crua no vanilla; cores/efeitos coincidem no fixture. plain-func coincide cell; set-func/show-func/set-show-func medem cell/sequence contra styled. A primeira rodada `before` tinha str(function), foi invalidada para decidir morfologia; somente baseline usa repr(function).

Fonte: vanilla `typst-eval/src/code.rs:148` aplica Value.spanned; `typst-library/src/foundations/value.rs:210–216` só assenta Content/Func; `foundations/content/mod.rs:109–114` preserva span já assentado. `model/table.rs:436–437` transporta value.span ao fabricar Cell. `typst-eval/src/code.rs:26–57` conserva set/show como styled sobre a cauda. Tekt `eval/expression_dispatch.rs:125–136` realiza Cell no join; `stdlib/structural/table_grid.rs:160–175` guarda children sem origem; `realize/table_cells.rs:176–183` fabrica Cell com span detached; `layout/grid/row_groups.rs:109` usa esse span. É língua (diagnóstico/morfologia/efeitos), não igualdade Rust ou bytes PDF. Não inferir intenção upstream.

## Contrato público proposto

No owner `00_nucleo/prompts/entities/content.md`, acrescentar exatamente:

```rust
#[derive(Debug, Clone)]
pub struct SourceContent {
    pub body: Content,
    pub span: crate::entities::span::Span,
}
// Variante adicional no enum público Content:
// Source(Arc<SourceContent>),
```

Sem método/trait/assinatura/flag nova. A variante é metadado de origem transparente para a linguagem, não um elemento novo nem uma localização introspectável. Novo discriminante de enum público quebra matches externos exaustivos: gate ADR-0127 §2.1/2.4 obrigatório. TableElem e TableCellElem não recebem campos novos.

A captura é no eval existente: somente Content sem origem assentada recebe span causal da expressão produtora; referência/clone/spread preservam o primeiro span. Conteúdo sintético permanece detached se não houver origem comprovada. Constructor Cell conserva seu source_span existente; wrapper de origem nunca deve transformar uma Cell explícita em Cell externa. Ao fabricar Cell implícita a Table lê a origem real do corpo, preservando o corpo lexical para getters. Não calcular origem por igualdade, nome, busca textual, último span da chamada ou Args.span. Trace causal da chamada deve permanecer; simples caret sem trace não fecha alias-conflict.

Observação/equality/hash/morfologia/func/fields/repr/serialização delegam ao body e não expõem Source; observação de cache inclui span conservadoramente. Travessia e transformação mantêm span e visitam cada ocorrência uma vez. Layout/introspecção/export percorrem body sem geometria/localização nova, e nunca executam um segundo conteúdo. Nenhum acesso a I/O em L1.

## Correção local, sem contrato público adicional

CodeBlock deve conservar valor lexical quando set/show local modifica a cauda: Styled sobre conteúdo cru, com captura causal existente ShowRecipe para execução posterior na mesma fase eval. Não conservar metadata emitida pela receita como fonte do valor. A execução de show continua na fronteira de inserção autorizada, uma vez, com regras/guards/styles/file restaurados. Plain block continua Cell, set/show local observa styled; aliases antigos, regra externa, preparação Table/Cell e hidden body conservam ordem. Não alterar globalmente Realized para observar original: seus owners atuais escolhem visual. Supersede explicitamente a cláusula suplementar A de expression_dispatch que exige realização imediata da Cell no join; a medição nova demonstra perda de morfologia nesse atalho. A realização continua em eval, mas muda a fronteira causal do join para a inserção do valor; a aprovação deve também cobrir esta mudança de ordem, ADR-0127 §2.3. O núcleo cell-realization permanece: Table → placement → Cell → body, efeitos apenas no conteúdo sobrevivente. Esta parte é hipótese de implementação, refutável pelas sondas de LOCAL/OUTER/NEW e recursão existentes; não permite mover eval↔layout.

## Instalação e aceitação após aprovação

Auditar cada owner afetado antes de instalar seu L0 e escrever código. Os candidatos iniciais completos abaixo são a base dos owners centrais; adaptar e individualizar os demais owners transparentes existentes antes de qualquer edição produtiva correspondente. Não usar este documento tático como @prompt, não criar ownership 1:N, não ressellar globalmente. Alterações públicas adicionais ou mudança de fase exigem novo gate, não ficam implicitamente autorizadas.

RED→GREEN: oito sondas baseline, mais alias em outra Source/import, múltiplas Cells/headers, reutilização/spread/forwarded Args, origem synthetic detached, equivalência/raw fields; source de constructor explícito; Cell substituída/hidden body; contagem e ordem metadata; local/global regras e guard/depth. Preservar geometria/texto/tintas do C e macros Table/Raw/math/matrix/query. Builds debug/release, suítes apropriadas e crystalline-lint sem erros, owners sem V5/V6/V15/V26. Registrar estado/hashes/UTC por medição; nenhum commit/reset/limpeza automática.

Esse pacote ainda não foi implementado ou validado. Os oito casos só descrevem o defeito atual; não representam oito correções. Métricas horizontais antigas, Table geral/casts ricos/multipágina/PDFUA continuam abertos.
