from pathlib import Path
import hashlib,json
r=Path.cwd();review=r/'00_nucleo/materialization/context-occurrences-2026-10-18/review.md';assert hashlib.sha256(review.read_bytes()).hexdigest()=='500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388'
common='''

## Context — identidade de ocorrências ratificada, 2026-10-18

### Medição anterior à decisão e gate

before.json.gz em 00_nucleo/diagnosticos/context-paridade-2026-10-18 conserva
HEAD b7e7c83223e5daa67bbdf124c36b5b7f93200731, fontes, UTC, diffstat, argv e
SHA dos binários; baseline-build-resources.json contém a receita nova. Vanilla
bd5ebec285505889e8ff1d4f68804ab147a699f3. Duas equações dão EQ EQ Q1/Q2;
prefixo Heading H EQ Q0/Q1; it.location observado por query dá 00/01.
context_stabilization.rs:257 reinicia EvalContext por produtor; eval/mod.rs:438
reinicia Locator; realize/mod.rs:41 aloca sem domínio comum; introspect/walk.rs:
375–390 deduplica pela Location. É semântica/identidade de linguagem.
Upstream typst-realize/src/lib.rs:353–418 prepara identidade antes da receita,
mas não exige copiar representação, vtables ou algoritmo.
Hipótese de colisão será refutada por falha de contagem, alias, callback ou replay.

O humano aprovou a proposta review.md sob
00_nucleo/materialization/context-occurrences-2026-10-18, SHA-256
500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388;
recibo approval-receipt.json no mesmo diretório. Gate ADR-0127 §2.1 satisfeito
para a API aditiva especificada. Não autoriza mudança de fase ou API alheia.

### Responsabilidade exclusiva

'''
contracts={
'compiler/eval.md':'''Guardar Option<OccurrenceSlots> crate-private, default None, além do Locator
já existente. OccurrenceSlots pertence ao owner
00_nucleo/prompts/compiler/realize/context_occurrences.md. Não alterar campos
públicos, current_location, IDs de Context ou políticas de observação. O owner
da arena instala e retira esse estado por execução; EvalContext só o armazena.
''',
'compiler/realize/mod.md':'''Declarar o módulo público context_occurrences, legitimado pelo owner
00_nucleo/prompts/compiler/realize/context_occurrences.md. locate mantém
Located existente intacto e delega somente a obtenção da nova Location ao
helper privado next desse módulo; sem frame ativo mantém Locator::next atual.
A reserva de ocorrências existentes continua observando o Locator, sem emitir
nó, executar callback ou inventar identidade por offset/hash.
''',
'infra/pipeline/context_stabilization.md':'''Session possui arena da fachada L1 e Producer possui frame exclusivo emitido
por ela. Criar arena do origin real; sem produtores pode usar None e evitar
uma caminhada redundante. Cada novo Producer recebe frame novo; reexecução
retém o frame do mesmo Producer. Descartar descendente não recicla seus slots.
Envolver a operação já existente apply_func → realize_math_insertions →
realize_table_insertions em arena.evaluate, sem mudar ordem ou executar mais
vezes. L3 não escolhe nem manipula Locations numéricas. Session legacy nova
reconstrói deterministicamente arena/frame do mesmo origin e ordem causal;
não inicializar a arena do snapshot de resultados anteriores. Retenção,
seleção, erros, budgets, estilos, targets, efeitos e callbacks ficam vigentes.
''',
'infra/pipeline/tests.md':'''Testar duas equações em Contexts distintos, prefixo Heading e callback que
localiza seu próprio it na lista query. Incluir produtores iguais, aliases,
produtos frescos, Context aninhado e crescimento/retração em replay selecionado.
Controles PDF/HTML usam texto, contagens e relações de identidade reais; não
sintetizar Locations esperadas nem comparar bytes PDF. Registrar RED funcional
antes do código, conservar todos os controles textuais e a suíte precedente.
'''}
for path,contract in contracts.items():
 f=r/'00_nucleo/prompts'/path;f.write_text(f.read_text()+common+contract)
p=r/'00_nucleo/prompts/compiler/realize/context_occurrences.md'
s='''# L0 — compiler/realize/context_occurrences
Hash do Código: 00000000

Camada L1. Consumer exclusivo: `01_core/src/compiler/realize/context_occurrences.rs`.
Governança: ADR-0107/0108/0127/0129/0130. Estado: ratificado, materialização em curso.
'''+common+'''Arena pura de Locations para os produtores contextuais de uma Session.
API pública aditiva e tipos opacos, sem Clone:

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

new reserva Locations já transportadas pelo origin e as atribuídas pela
introspecção canônica existente desse mesmo origin. Não usar resultados de
outro passe como offset crescente. Ordem de reserva não modifica a identidade:
observe é max, não alocação. Arena tem Locator único; frame tem slots privados
Vec<Location> e cursor de execução. Um token imutável Arc pode verificar que o
frame foi emitido pela mesma arena; seu endereço nunca gera Location.

O helper crate-interno next delega ao Locator normal se não há frame. Com
frame, reutiliza o slot na ordem de alocação efetivamente atingida; caso novo,
aloca pelo Locator único e acrescenta slot. Aliases já Located não chamam next.
Não usar igualdade morfológica, span, texto, hash ou ID de Context para gerar
Location. Slots não usados depois de uma retração permanecem reservados, sem
tags ou contribuições fantasma. Crescimento aloca na cauda livre compartilhada.

Em evaluate, instalar temporariamente Locator e slots no EvalContext existente,
zerar cursor, chamar operation uma vez, recuperar Locator/slots e restaurar
estado anterior do ctx em todo retorno normal, inclusive Result::Err. Não
capturar panic nem mudar a política de unwind do compilador. Frame pertence ao
produtor real e persiste nas reexecuções dele; não cloná-lo para outro produtor.
Não manter frames/arenas entre compilações nem criar estado global, I/O, unsafe
ou despacho dinâmico.

### Aceitação

RED integrado de contagem e it.location, GREEN bilateral no pin vigente.
Testes locais de reserva do origin real, disjunção de produtores, reuso/crescimento/
retração de slots e restauração após Result::Err, usando locatables reais e
introspecção. Controles de aliases, nesting e replay seletivo no pipeline.
Build, suíte completa, Clippy/Rustdoc estritos, formato, linhagens e SARIF sem
novo erro/warning, sob 12 GiB/High8 GiB/swap0/jobs1. Sem concluir paridade geral.
'''
assert not p.exists();p.write_text(s)
