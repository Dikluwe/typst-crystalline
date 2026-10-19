# Context: domínio de identidade de ocorrências — proposta para ADR-0127

Estado: **não implementado; requer aprovação de API pública L1→L3**.
A correção textual independente continua em validação.

## Medição que precede a proposta

HEAD b7e7c83223e5daa67bbdf124c36b5b7f93200731, branch Tekt, fonte produtiva
limpa no build baseline. Fontes, UTC, argv, diffstat, binários e recursos:
`00_nucleo/diagnosticos/context-paridade-2026-10-18/{before-sources.json.gz,before.json.gz,baseline-build-resources.json,before-probe-resources.json}`.
Vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3, SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.

- Duas equações: EQ EQ Q1 no Tekt; EQ EQ Q2 no vanilla.
- Prefixo Heading: H EQ Q0 no Tekt; H EQ Q1 no vanilla.
- Callback consulta posição na lista por `it.location()`: 00 contra 01.
- Uma equação: EQ Q1 nos dois.

`03_infra/src/pipeline/context_stabilization.rs:257` cria EvalContext por
execução; `01_core/src/compiler/eval/mod.rs:438` inicia seu Locator em zero;
`01_core/src/compiler/realize/mod.rs:36–43` usa esse alocador para Located;
`01_core/src/compiler/introspect/walk.rs:375–390` deduplica Located pela
Location. A perda é semântica, não diferença aceitável de estrutura Rust.
A fonte upstream `typst-realize/src/lib.rs:353–418` prepara identidade antes
de chamar a receita e conserva tags do original. Não se infere daí obrigação
de copiar seu algoritmo, representação ou mecanismo de memoização.

Hipótese causal: os alocadores independentes colidem entre produtores e com
originais já preparados. Refutadores: falha da sonda de it.location após
partilha, aliases contados duas vezes, instabilidade de replay ou mudança de
páginas/posições nos controles de Context. A proposta não declara a hipótese
já validada por código candidato.

## API proposta, aditiva

Novo owner `00_nucleo/prompts/compiler/realize/context_occurrences.md`, consumer
`01_core/src/compiler/realize/context_occurrences.rs`. Tipos opacos, sem campos
públicos, sem Clone do alocador e sem acesso numérico à Location:

```rust
pub struct ContextOccurrenceArena { /* privado */ }
pub struct ContextOccurrenceFrame { /* privado */ }

impl ContextOccurrenceArena {
    pub fn new(origin: &Content) -> Self;
    pub fn frame(&mut self) -> ContextOccurrenceFrame;
    pub fn evaluate<R>(
        &mut self,
        frame: &mut ContextOccurrenceFrame,
        ctx: &mut EvalContext,
        operation: impl FnOnce(&mut EvalContext) -> R,
    ) -> R;
}
```

O módulo será público para o pipeline L3; o hub só declara o módulo. As
fachadas existentes permanecem com suas assinaturas. Aprovação permite
instalar estes contratos L0, resselo e materialização com RED→GREEN; não
ratifica outputs ainda não medidos.

## Contrato para o novo L0

A arena pertence a uma Session de compilação, recebe o conteúdo original real
e reserva as Locations canônicas já presentes ou atribuídas pela introspecção
existente desse conteúdo. Não usa o snapshot de resultados antigos como uma
fonte crescente de offsets. Não calcula Location por hash, texto, span, endereço
ou ID numérico de Context.

Cada Producer possui um frame exclusivo emitido pela arena. O frame guarda a
sequência de Locations alocadas nas inserções efetivas de sua execução; não é
uma faixa de tamanho arbitrariamente estimado. Reexecução do mesmo produtor
reinicia o cursor e reutiliza essas mesmas posições de alocação. Se precisar
crescer, solicita novas Locations ao Locator único da arena. Slots excedentes
ficam reservados, mas não criam tags nem elementos. Produtores distintos não
compartilham frames. Descendente novo recebe frame novo; aliases Located
continuam reutilizando a identidade que já possuem.

`evaluate` instala temporariamente o alocador e o frame no estado privado do
EvalContext, chama a operação uma vez e devolve o estado à arena/frame e ao
contexto anterior em retornos normais, inclusive Result::Err. Não captura ou
repete callback. Sem I/O, globais, Mutex global, vtables ou unsafe. A sequência
de alocações mede ocorrências reais da execução; não usa igualdade morfológica
para decidir identidade.

## Alterações dos owners existentes a ratificar

- `00_nucleo/prompts/compiler/eval.md`: estado **privado** para a sequência
  contextual de alocações, separado de current_location e next_context_id.
- `00_nucleo/prompts/compiler/realize/mod.md`: locate delega ao alocador
  contextual quando instalado; Located existente continua intacto. Os demais
  callers conservam o Locator atual. Observar uma Location não emite ocorrência.
- `00_nucleo/prompts/infra/pipeline/context_stabilization.md`: Session possui
  arena e cada Producer seu frame; envolver a operação existente apply_func →
  realização da contribuição em evaluate. Retenção/replay/descendentes e
  budgets existentes permanecem. Uma nova Session legacy reconstrói a arena
  deterministicamente do mesmo origin, sem importar os slots da tentativa velha.
- `00_nucleo/prompts/infra/pipeline/tests.md`: acrescentar regressões de
  identidade, callbacks, prefixo, aliases, crescimento/retração e replay.

Essas cláusulas são rascunhos L0 concretos, ainda em materialization para não
introduzir owner órfão em prompts. Após aprovação serão instaladas nos caminhos
canônicos acima antes de escrever o código.

## Aceitação e limites

RED→GREEN das três sondas de identidade e preservação da sonda de uma equação.
Ampliar para query/it.location em PDF e HTML, equações iguais em produtores
distintos, alias de it, elementos novos emitidos pelo callback, Context aninhado,
produtor com crescimento/retração e controles State/Counter/Heading/Table.
Estabilidade deve ser demonstrada por observáveis e efeitos reais, não por
igualdade dos bytes PDF ou por fabricar Location esperada em teste.

Reexecutar build, suíte completa/doctests separados, Clippy/Rustdoc estritos,
fmt, linhagem direta/reversa, V15/V26 e diferencial SARIF. 12 GiB máximo,
8 GiB high, swap zero, Cargo jobs=1/debug=0/incremental=0. Sem push ou release.
Se replay ou a identidade canônica refutar o desenho, não afrouxar testes:
revisar o L0 e reapresentar mudança pública/fase que exceda esta proposta.

Não abrange R3-T2, novos tipos de retorno de show, fase de eval/layout,
morfologia pública, RG1 ou paridade universal. A ausência de bugs novos é
critério de materialização, não afirmação já obtida.
