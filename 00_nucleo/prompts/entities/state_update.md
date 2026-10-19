# Prompt L0 — `entities/state_update`
Hash do Código: 2f78e34c

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/state_update.rs`
**Criado em**: 2026-04-30 (P171 sub-passo .B — feature `state(key, init)` M9)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`StateUpdate` é o enum que representa uma operação a aplicar ao runtime mutable state. Vanilla `StateUpdate { Set(Value), Func(Func) }`.

- **P171 (M9 sub-passo 3)**: `Set(Box<Value>)` apenas.
- **P172 (M9 sub-passo 4)**: `Func(Func)` adicionada. **Eval real é stub** — `Func::call` requer `Engine + EvalContext` que não estão disponíveis em `from_tags` (Engine só existe durante eval; from_tags corre depois). `from_tags` reconhece a variant mas **silenciosamente ignora** em `apply_update`. Eval real requer pipeline restructuring (M7+ fixpoint ou refactor dedicado).

Padrão `Box<Value>` consistente com `Content::Metadata` (P169) — evita ciclo Content→Value→Content em compilação.

---

## Restrições Estruturais

- Camada **L1**: enum puro.
- `Clone` derivado.
- `PartialEq` derivado; `Eq` via marker impl manual (Value não impl Eq por f64 NaN); `Hash` manual via `format!("{:?}", self).hash()` — padrão estabelecido em `entities/element_payload.rs` (P169).

---

## Interface pública

```rust
use crate::entities::func::Func;
use crate::entities::value::Value;

#[derive(Debug, Clone)]
pub enum StateUpdate {
    Set(Box<Value>),
    /// **P172** — callback. **Stub** — `from_tags` ignora silenciosamente.
    Func(Func),
}

impl PartialEq for StateUpdate { /* manual: Set por valor; Func por Arc::ptr_eq */ }
impl Eq for StateUpdate {}
impl std::hash::Hash for StateUpdate { /* via format!("{:?}", self) */ }
```

**Nota sobre derives** (P172): `PartialEq` deixou de ser derive porque `Func` interno é `Arc<FuncRepr>` sem `PartialEq`. Comparação `Func` é por ponteiro Arc (`Arc::ptr_eq`) — duas Funcs distintas com mesmo comportamento são `!=` (paridade vanilla).

---

## Semântica

- `Set(value)`: define o novo valor do state. Substitui valor anterior (se houver).
- `Func(fn)`: callback que receberia valor actual e retornaria novo. **Stub em P172** — `from_tags::apply_update` arm `Func(_)` é no-op. Pendência: eval real requer pipeline restructuring para passar `Engine + EvalContext` a `from_tags`.

---

## Tests obrigatórios

- Set com mesmo Value: igualdade preservada.
- Set com Values distintos: desiguais.
- Hash determinístico: hash duas vezes, igual.

---

## Consumers

- `Content::StateUpdate { key, update }`.
- `ElementPayload::StateUpdate { key, update }`.
- `entities/state_registry.rs::StateRegistry::apply_update`.

## Sobre paridade

Vanilla aceita função em `state("key").update(fn)`. Cristalino P171 adia para passo dedicado quando Func eval em walk context for resolvido. `Set` cobre o caso literal (`state("key").update(value)`).

> **Fonte de paridade (P1031)** — **citação literal**, vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/introspection/state.rs:336-337`, doc comment do parâmetro de
> `state.update`, publicado em
> `typst.app/docs/reference/introspection/state/#definitions-update`:
>
> *"- If given a function, that function receives the state's previous value and has to
> return the state's new value."*
>
> Confirma que a forma-função existe na linguagem, e a forma de superfície é
> `state("key").update(f => …)` (método sobre o estado), não `state.update(key, fn)` como a
> redacção anterior sugeria — corrigido acima.
>
> **A preferência é do lado da linguagem, não só conveniência** — `state.rs:339-349`:
> *"When updating the state based on its previous value, you should **prefer the function
> form** instead of retrieving the previous value from the context. This allows the compiler
> to resolve the final state efficiently, minimizing the number of layout iterations
> required."* Ou seja, a variante diferida é a **recomendada** pela documentação, o que
> agrava o scope-out face à estimativa "`Set` cobre 80% dos casos".
>
> **Sobre os "80%"**: era número sem medição. Removido — não há contagem de corpus que o
> sustente, e a documentação empurra os utilizadores para a forma que falta. O scope-out
> mantém-se, mas sem a quantificação não fundamentada.

---

## Resultado Esperado

- `01_core/src/entities/state_update.rs` — enum + impl Hash + impl Eq + 3 tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P171: enum minimal Set para suportar runtime mutable state | `state_update.rs`, `state_update.md` |


## Exaustividade de representação e identidade — V16, 2026-10-16

### Medição anterior à decisão

01_core/src/entities/state_update.rs:58. Fonte e owner vigentes lidos e conferidos por hash antes da decisão. `lab/results/construction/diagnostics/tekt-enums-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, status e git diff HEAD --stat. O SARIF anterior registra onze avisos e uma nota V16 nestes sete owners. Foram também medidos os fallbacks sem emissão; estes fazem parte do mesmo recorte. Classificação: padrão Rust e evolução do enum são mecânica; resultados/campos/identidade observáveis e erros devem permanecer. Não se infere intenção upstream ou ganho para IA.

### Responsabilidade exclusiva

StateUpdate::eq: conservar os dois pares iguais Set/Set (Value) e Func/Func (Arc::ptr_eq); false passa a cobrir explicitamente (Set,Func)|(Func,Set). Não usar Func::eq no lugar de identidade de Arc, pois nativas de mesmo nome em Arcs distintos podem comparar iguais como Func e permanecer diferentes como StateUpdate. Hash, Eq marker, enum, fields e execução de callbacks permanecem. Adicionar controle dessa distinção e simetria dos pares cruzados antes do código. Qualificar o histórico de stub: este owner representa a operação; execução pertence a 00_nucleo/prompts/compiler/introspect/from_tags/state_updates.md e o caminho legacy sem Engine continua distinto. Não alterar nenhum desses consumers.

A fonte `01_core/src/compiler/introspect/from_tags/state_updates.rs:9–47` possui pós-passe com Engine para Func; `01_core/src/compiler/introspect/entrypoints.rs:29–35` qualifica o caminho legacy. O stub descrito em P172 é histórico, não contrato global atual deste carrier.

### Aceitação e gate

Manutenção interna ADR-0127 em fluxo contínuo: nenhuma definição de enum, campo, trait, assinatura pública, default, dependência, compatibilidade ou fase muda. Conservar cada corpo, ordem e guard; não mover lógica entre owners nem criar helper para esconder o catchall. Padrões nominais devem forçar erro de compilação ao surgir variante nova; comentários de neutralidade não servem como supressão.

Antes do código, testes funcionais locais e gate V16 RED por escopo. Executar sonda de evolução em cópia descartável do workspace: acrescentar somente uma variante sentinela a Color, ColorSpace, MathStyleKind, FuncRepr e StateUpdate, sem alterar as definições produtivas. Usar cargo check --offline --locked -p typst-core com um job e cache de artefatos compartilhado; registrar fontes/hashes/argv/UTC/diagnósticos. Para cada um dos 21 matches selecionados, a baseline deve demonstrar a ausência de E0004 naquele match; GREEN exige E0004 ali, independentemente de erros de exaustividade já emitidos por outros matches na cópia. O controle de compilação sem sentinelas e os testes de comportamento aprovam o código real; não usar a sonda negativa como prova de paridade. Não editar source produtivo durante comandos.

Resselo restrito, direto e reverso, sem reparo global da working tree. Revalidar build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, suíte workspace completa, Rustdoc -D warnings e quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF sem avisos novos e diff --check. Limite MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1, tarefas pesadas sequenciais. Nenhuma tolerância/fixture histórica é relaxada. As pendências V16 externas, inclusive T027/T158/T160 rastreadas sem emissão, continuam abertas.


## A1-S2 — documentação do contrato vigente

Medição anterior: state-counterproof.json.gz/state-extended.json.gz do diagnóstico
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17, HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree/UTC/diffstat ali registrados,
pin bd5ebec285505889e8ff1d4f68804ab147a699f3. Set/Func já são públicos; a cláusula
stub de P172 é histórica e não descreve o runtime materializado. A1-S2 aprovado
no review de state-reader-runtime-2026-10-17 transfere a aplicação automática
para a leitura. Este owner continua somente dado puro Set(Box<Value>)/Func(Func),
sem executar callbacks. Corrigir comentários para distinguir histórico e contrato
vigente; preservar todas as definições, igualdade Rust por identidade da Func e
hash atuais. Não alegar equivalência de igualdade Rust à igualdade da linguagem.
