# L0 — compiler/realize/recipes
Hash do Código: 2c25e087

Camada: L1. Consumer produtivo exclusivo: `01_core/src/compiler/realize/recipes.rs`.
Contrato ratificado pelo dono; o consumer será selado durante a materialização.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:f225f3039d4663e9bfcb408d26cf1cd6e557f089013bc2d95090ce16c52080c6

## R3-T1 — realização causal ratificada (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/textual-realization-2026-10-18/{before,scope,refuters}.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não
commitado, UTC, status/diffstat, fontes/argv/hashes dos binários e pin vanilla
bd5ebec285505889e8ff1d4f68804ab147a699f3. before-sources.json.gz contém
fontes/L0 vigentes completos. Não há código candidato implementado nesta medição.
matching.rs:118–150 aplica regex reversa e depois literais em declaração;
:194–207 volta a expor produtos a passes subsequentes. textual.rs:102–123
captura snapshot completo. realize/mod.rs:177–217 visita receitas bottom-up.
apply.rs:620–648 e matching.rs:218–225 chamam funções sem capturar os filhos.
expression_dispatch.rs:125–129 já distingue insertion_rules do estado local.

Exemplos medidos: literais idênticos C/B; empate literal/regex R/L; overlap
X/YB; primeiro efeito LATER/FIRST; função encadeada inversa BBBB/BB;
produto separado AB/X; produto+vizinho X/AB; Strong ocultando filho dá
panic HIDDEN/Q; global capturado seguido de global nova dá L/R, mas local
capturado dá L/L. Entrada CJK local 水\n果 dá 水果/X. A fonte pinada
foundations/selector.rs:110–125 unifica literal escapado com regex;
typst-realize/src/lib.rs:1328–1405 escolhe o início e carrega revogação
no produto. Esses dados medem linguagem/efeitos; não ratificam mecânica Rust,
igualdade de buffers ou bytes PDF. É inferência causal que a separação de
escopos e a visita anterior aos filhos expliquem o conjunto; os controles de
acceptance.json, igualdade/fields/estilo/query e erros podem refutá-la.

### Decisão ratificada

Aplicar o núcleo textual-recipe-frontier pinado neste L0. A confirmação humana
do manifesto 47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302
ratificou esta ordem. Ele supersede neste escopo as
cláusulas históricas de execução provisória de show em ContentBlock e da
visita bottom-up de receitas pendentes, sem alterar a ordem de avaliação
de expressões comuns. Não reavaliar o programa, antecipar State/Counter,
assentar render em morfologia nem criar contrato Rust público.

### Responsabilidade exclusiva

Coordenar somente execução causal de receitas pendentes e seus produtos, usando Engine/EvalContext existentes. Dispatch estático e nominal. Instalar o frame lexical causal, compor regras atuais com deltas locais por identidade, realizar a raiz pelos owners Sequence/apply/textual apropriados e descer somente nos filhos emitidos. Nunca visitar Realized.original como conteúdo emitido. Preservar a proveniência do original para query e o namespace causal do produto.

Os mapas gerais de Content mantêm seu contrato vigente. Usar o pre-order existente como mecanismo de reconstrução e retornar Some quando a raiz já foi processada, evitando a segunda descida automática. Se uma API interna de mapa for estritamente necessária, especificar primeiro o L0 dono correspondente; não alterar assinatura pública de mapa nesta aprovação. O coordenador não duplica seleção, normalização de espaços, preparação Table/Cell, matemática, construção de layout ou terminação.

Manter uma fronteira por produto e snapshots compartilhados, com orçamento de profundidade vigente. Conteúdo emitido como uma unidade pode agrupar seus próprios nós textuais, mas produtos de matches distintos permanecem separados. Restaurar ambiente em todo retorno e preservar primeiro efeito, spans e traces causais. O último normalize_insertions não pode apagar informação ainda necessária a realização contextual posterior. Não publicar API, campo, trait, elemento da linguagem ou flag.

### Aceitação

Todas as famílias T de acceptance.json e revalidação descrita nos owners. Testar especificamente erro antes de siblings, pai ocultando filho, produto que devolve alias, produto composto, efeito uma vez e restauracão após erro. Testar ambos targets e contextos/fixpoint, query de originais/produtos, Table/Cell e Equation sem repetir callbacks.

### R3-T1 — integração revelada pela suíte ampla

Um Located Table protegido é delegado ao preparo Table existente para pular
somente sua receita já aplicada; não pular o preparo das Cells nem tratar sua
árvore como conteúdo genérico. Manter as identidades reais. Nos matches de
Content desta unidade escrever os complementos nominais completos: em
realize_children excluir só Located/Sequence/Styled/Equation tratados sem
guard; em visit excluir ShowRecipe/Realized/Styled/Table/TableCell/Sequence,
conservando Located guardado no complemento. Grupos V19 permanecem visíveis,
com responsabilidade única (continuar catálogo de descendentes), sem alegar
cobertura de cada alternativa apenas por executar o corpo agrupado.

### R3-T1 — refinamento por regra e fronteira

Candidato4 concluiu a suíte sem abortar: 7373 passaram, 15 falharam, cinco
ignorados. As falhas de alias/locais mostram que proteger uma Location não pode
pular todas as regras. Para uma Location protegida, excluir somente RuleIds
ativos já aplicados; regras anteriores e locais com identidade distinta continuam
elegíveis. Ocorrências frescas não herdam essa exclusão. Ao revisitar produto
concluído proteger também a Location de seu original sem executar esse original.
Coalescer packets compatíveis do produto genérico antes da visita, como no
produto textual, preservando fronteiras concluídas/efeitos de raiz. Em full_error
manter morph_canon no caminho durante a visita causal, restaurando-o em erro.
Reconhecer Source na entrada e ancorar somente erros detached ao span mais
interno preservado, sem mudar o contrato do mapper. Para packet exclusivamente
de regras Par chamar realize_paragraphs do owner antes de selecionar as raízes
preparadas; não duplicar sua lógica nem sintetizar Par durante layout.
Empty/Sequence explícitos sob regras Sequence passam por decisão nominal antes
de agrupamento textual; slots consumidos continuam removidos no owner markup.

### R3-T1 — integração do fluxo emitido

Located cuja raiz é Table deve passar pela fachada realize_table_insertions
antes de visitar seus filhos; pular a raiz do Located não pode pular o preparo
da Table. O RED candidato6 tem metadata zero e Table ainda não preparada. A
fachada existente decide a receita e prepara placement/Cells; o coordenador
visita o produto pronto sem reexecutar a raiz. Packets antigos de Table também
entram nesta coordenação, mantendo o fallback de composição de regras legado.

### R3-T1 — contribuição bruta e identidade canônica

O coordenador envia ao apply_root somente regras elegíveis que efetivamente
casam a raiz, usando o mesmo selector_matches existente. Regras não casadas não
podem causar alocação antecipada de identidade numa contribuição bruta. Sem
receita efetiva a raiz preserva seus dados e seus filhos emitidos seguem a
coordenação; a finalização canônica de todo o documento mantém o seu owner.
Revalidar aliases/locais/fresh, query, Table e contextos, sem relaxar testes.
