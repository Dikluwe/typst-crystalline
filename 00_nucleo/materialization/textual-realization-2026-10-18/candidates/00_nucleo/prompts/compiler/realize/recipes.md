# L0 — compiler/realize/recipes — candidato R3-T1

Camada: L1. Consumer produtivo exclusivo planejado: `01_core/src/compiler/realize/recipes.rs`.
Sem Hash do Código enquanto não materializado; instalar/sellar apenas após aprovação e RED→GREEN.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:f225f3039d4663e9bfcb408d26cf1cd6e557f089013bc2d95090ce16c52080c6

## R3-T1 — candidato, NÃO vigente antes do gate de ordem (2026-10-18)

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

### Decisão sujeita a aprovação

Aplicar o núcleo textual-recipe-frontier pinado abaixo, somente depois da
confirmação humana do manifesto da revisão. Ele supersede neste escopo as
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
