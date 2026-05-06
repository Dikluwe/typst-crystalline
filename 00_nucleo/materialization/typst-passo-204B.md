# Passo 204B — `#[comemo::track]` no trait `Introspector` + bounds `Send + Sync`

**Série**: 204 (sub-passo `B` = implementação foundational
após diagnóstico-primeiro de P204A).
**Tipo**: implementação.
**Magnitude planeada**: S–M.
**Pré-condição**: P204A concluído; auditoria
`typst-passo-204A-auditoria-comemo.md` produzida;
diagnóstico `typst-passo-204A-diagnostico.md` com
C1–C14 fixadas; ADR-0073 PROPOSTO em
`typst-adr-0073-comemo-introspector.md`; tests 1824
verdes; 0 violations.
**Output**: 3 ficheiros (inventário + relatório +
alterações de código).

---

## §1 Propósito

Aplicar `#[comemo::track]` directamente ao trait
`Introspector` (Padrão A — paridade vanilla literal,
fixado em C2 de P204A) e adicionar bounds
`Send + Sync`. Verificar que `TagIntrospector` (única
impl actual) satisfaz os requisitos.

P204B é foundational. Não migra Layouter consumers
(P204C). Não materializa Position (P204D). Não toca em
`evict()` wrapper (P204E). Apenas expõe o trait como
trackable.

P204B respeita a convenção P203 §9.1: começa com
inventário empírico antes de qualquer alteração.

---

## §2 Material de partida verificado em P204A

Antes de qualquer alteração, confirmar empíricamente:

- Trait `Introspector` em `01_core/src/contracts/introspector.rs`
  (ou caminho real — confirmar em C1) com 20 métodos
  read-only `&self`.
- `TagIntrospector` em `01_core/src/entities/introspector.rs`
  (ou caminho real) é a única impl em produção.
- `comemo` versão 0.4.0 declarada em `Cargo.toml`
  workspace (per A6 da auditoria).
- `comemo` 0.4.0 suporta `#[comemo::track]` em traits
  não-genéricos (per A6.3).
- Vanilla declara `#[comemo::track] pub trait
  Introspector: Send + Sync` (per A7).

Sem isto, recuar para P204A.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código, listar literalmente:

- Caminho real do trait `Introspector`.
- Caminho real da impl `TagIntrospector`.
- Bounds actuais do trait (provavelmente nenhum).
- Bounds actuais de `TagIntrospector` (Send? Sync?
  Clone?).
- Os 20 métodos com assinatura completa (nome, args,
  retorno).
- Para cada método, classificar:
  - Args satisfazem `ToOwned` (per restrição comemo)?
  - Retorno satisfaz `Hash` ou é `Copy` (per restrição
    comemo)?
  - Há tipos não-tracked-friendly que precisem ajuste?

Output: tabela de 20 linhas (método × bounds satisfeitos
× ajustes necessários).

Critério: cada método com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**. Se houver ajustes não triviais,
registar em `P204B.div-N` e re-fixar C2 com base nos
dados.

### C2 — Forma da alteração

Decisão fixada por P204A C2 (Padrão A — paridade vanilla):

- Adicionar `#[comemo::track]` ao trait.
- Adicionar bounds `Send + Sync`.
- Adicionar `use comemo` se necessário.

Forma literal (notação ilustrativa, não código final):

```text
+ #[comemo::track]
  pub trait Introspector: Send + Sync {
      // 20 métodos existentes inalterados em assinatura.
  }
```

C2 não tem ramos. Caso C1 detecte ajustes em métodos,
P204B trata-os antes de aplicar `#[comemo::track]`,
não como alternativa a aplicar.

### C3 — Verificação `TagIntrospector` satisfaz `Send + Sync`

Confirmar que `TagIntrospector` é `Send + Sync`. Caso não
seja, identificar field obstrutivo:

- Field com tipo não-`Send` (ex: `Rc<...>`, `*const T`).
- Field com tipo não-`Sync` (ex: `RefCell<...>`, `Cell<...>`).

Output: lista de fields obstrutivos ou confirmação
"todos `Send + Sync`".

Se houver obstrução, registar em `P204B.div-N` e
re-fixar C4 com base nos dados.

### C4 — Resolver fields obstrutivos (se aplicável)

Para cada field obstrutivo de C3:

- Tipo substituível trivialmente: substituir
  (`Rc` → `Arc`, `RefCell` → `Mutex` ou refactor para
  ownership).
- Tipo não-substituível trivialmente: registar em
  `P204B.div-N`, recuar para P204A para re-fixar C2/C3.

Pipeline esperada per A4 da auditoria: 9 sub-stores
todos baseados em `HashMap<K, V>` ou `Vec<T>`. Ambos
`Send + Sync` se `K`, `V`, `T` são. Provavelmente
trivial.

### C5 — Aplicar alteração

Edições literais:

- `pub trait Introspector` → `pub trait Introspector: Send + Sync`.
- Adicionar `#[comemo::track]` directamente acima.
- Garantir `use comemo;` ou `use comemo::track;` no topo
  do módulo (depende da resolução do macro).

C5 só é executada quando C1–C4 estão CONFIRMADO ou
ajustados. Não há `if condição` em C5.

### C6 — Compilação

Verificar:

```
cargo build -p typst-core 2>&1 | head -50
```

Critério: compila sem erros. Warnings são aceitáveis se
forem novos e relacionados com `track` (ex: deprecation
de impl manual).

Caso falhe:

- Identificar erro literal.
- Resolver (provavelmente importação faltante ou bound
  satisfeito por add `where T: Hash + Eq` em algum tipo
  retornado).
- Tentar de novo.

C6 termina com compilação verde. Não passa para C7 sem
isto.

### C7 — Tests workspace

```
cargo test --workspace 2>&1 | tail -30
```

Critério: 1824 tests verdes (mesmo número que pré-P204B
— P204B não adiciona tests, apenas modifica trait
declaration). Caso algum test falhe, identificar
literalmente que asserção falhou e investigar.

Hipóteses prováveis de falha:

- Test que constrói mock `Introspector` e que agora
  precisa de `Send + Sync` no mock. Solução: adicionar
  bounds ao mock.
- Test que invoca método via `&dyn Introspector` e que
  agora encontra erro de coerção. Solução: ajustar call
  site.

Cada falha tratada e documentada no inventário.

### C8 — Linter

```
crystalline-lint .
```

Critério: 0 violations.

Caso aumentem:

- Identificar regra violada.
- Verificar se a violação é gerada pelo macro
  `#[comemo::track]` (ex: regra que proíbe certos
  attributes em L1).
- Resolver caso a caso.

### C9 — Tests adicionais (sentinelas)

Adicionar 1 ou 2 tests sentinela que confirmam que o
trait é trackable:

- Test 1: `fn _trait_is_trackable() { fn assert_track<T: comemo::Track>() {} ... }` — falha de compilação se trait perder `#[comemo::track]`.
- Test 2 (opcional): construir um `Tracked<dyn Introspector>` num teste e invocar 1–2 métodos. Garante que o pipeline básico funciona.

Critério: tests verdes; 1824 → 1825 ou 1826.

C9 é opcional (decisão dentro de P204B). Recomendado
para registo histórico e protecção contra regressões.

### C10 — Documentação ADR-0073

Não modificar ADR-0073 PROPOSTO em P204B. ADR transita
para ACEITE estrutural em P204C ou ACEITE final em
P204H.

P204B apenas confirma que a aplicação de
`#[comemo::track]` foi bem sucedida em isolamento. Sem
necessidade de actualização da ADR.

### C11 — Critério de fecho de P204B

P204B está concluído quando:

- C1 inventário completo.
- C2 alteração aplicada.
- C3+C4 `Send + Sync` confirmado.
- C5 edições literais aplicadas.
- C6 compilação verde.
- C7 tests workspace verdes (mínimo 1824; máximo 1826
  se C9).
- C8 linter 0 violations.
- C9 tests sentinela (opcional, mas recomendado).
- Inventário registado.
- Relatório escrito.

### C12 — Sem cláusulas condicionais

C1 produz dados empíricos. C2 aplica decisão fixa de
P204A. C3 verifica. C4 resolve obstruções. C5 executa.
C6–C8 verificam. C9 é decisão dentro de P204B (opcional).

Nenhuma cláusula tem `if A else B` em sentido de ramos
estruturais. Decisões dependem de auditoria empírica
em C1/C3, não de pré-fixação na spec.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204B-inventario.md`.

Conteúdo:
- §1 C1 — tabela 20 métodos.
- §2 C3 — `Send + Sync` de `TagIntrospector`.
- §3 C4 — obstruções (se houve) e resolução.
- §4 Decisões tomadas durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-204B-relatorio.md`.

Conteúdo:
- O que foi feito.
- Tempo de execução.
- Métricas (tests pre/post; LOC delta).
- Decisões durante a leitura.
- Sugestão para próximo sub-passo (P204C).

### Ficheiro 3 — Alterações em código

Não é ficheiro discreto — é o conjunto de alterações em:

- Trait `Introspector` em `01_core/src/...` (caminho
  exacto de C1).
- Possíveis adjustments em `TagIntrospector` se C4
  detectar obstruções.
- Tests sentinela adicionados (se C9 afirmativa).

---

## §5 Critério de progressão para P204C

P204B fechado quando C11 cumprido.

Em caso de divergência empírica relevante (ex: trait
não tem 20 métodos, métodos não satisfazem restrições
comemo, `TagIntrospector` tem field não-`Send + Sync`
não-trivial), registar em `P204B.div-N` e:

- Resolver dentro de P204B (preferido).
- Recuar para P204A re-fixar C2 com novos dados (se
  obstrução for estrutural).

P204C só começa quando P204B fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 3 outputs (inventário + relatório + código).
- Inventário empírico antes de implementação (per
  convenção P203 §9.1).
- Localização canónica: `00_nucleo/diagnosticos/` para
  inventário; `00_nucleo/materialization/` para relatório.
- Distinção fecho estrutural vs final mantida.
- Sem inflação retórica.

---

## §7 Não-objectivos

P204B não:

- Migra Layouter consumers (P204C).
- Adiciona lifetime parameter ao Layouter (P204C).
- Materializa Position (P204D).
- Adiciona `evict()` wrapper (P204E).
- Adiciona ficheiros ao corpus de paridade (P204F).
- Adiciona benchmarks (P204G).
- Transita ADR-0073 para ACEITE (P204H).
- Cria ADR nova.
- Modifica ADR-0066.
- Toca em outros traits que não `Introspector`.

---

## §8 Erro a não repetir

Da série P203 — duas detecções consecutivas de
premissas erradas. Padrão correcto: cada passo começa
com inventário empírico.

P204B aplica isso em C1: lista os 20 métodos e
classifica-os antes de aplicar qualquer macro. Se algum
método não satisfizer constraints comemo, ajusta antes
ou recua para P204A.

Hipótese específica a testar em C1: alguns métodos
podem retornar tipos que precisem de bounds adicionais
(ex: `Hash`). Vanilla resolveu isto declarando bounds
explícitos no trait (`Send + Sync`) e em tipos
auxiliares. Cristalino pode precisar do mesmo.

---

## §9 Particularidade — execução

P204B é trabalho de código focado:

- Modificação de 1 ficheiro principal (trait
  declaration).
- Possíveis adjustments em 1 ficheiro auxiliar
  (`TagIntrospector` impl).
- 1–2 tests sentinela.
- Verificação compilação + tests + linter.

Volume baixo. Magnitude S–M. Pode ser executado pela
sessão actual (Opus, com bash_tool) ou pelo Claude Code.

Recomendado pela sessão actual se houver disponibilidade
— P204B é o sub-passo mais simples de M8 e serve como
validação de que a infraestrutura comemo está realmente
pronta. Caso a sessão actual não esteja disponível,
Claude Code segue padrão habitual.
