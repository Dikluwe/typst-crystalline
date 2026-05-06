# Passo 204E — Wrapper `crystalline_evict()` em L4

**Série**: 204 (sub-passo `E` = wrapper L4 após P204D
Position concrete).
**Tipo**: implementação trivial (wrapper).
**Magnitude planeada**: S.
**Pré-condição**: P204D concluído; tipo `Position` em
L1; trait `Introspector::position_of -> Option<Position>`;
runtime.positions populado durante layout; tests 1836
verdes; 0 violations; 7 sentinelas activas (3 P204B +
2 P204C + 2 P204D); ADR-0073 PROPOSTO em vigor.
**Output**: 3 ficheiros (inventário + relatório +
alterações de código).

---

## §1 Propósito

Expor `comemo::evict` via wrapper cristalino em L4
wiring, conforme decisão C6 do diagnóstico P204A
(política de invalidação tracking-based intra-compilation
+ `evict()` exposed para callers, paridade vanilla).

Trabalho concreto:

- 1 função wrapper em L4 (provavelmente
  `04_wiring/src/...`).
- Re-export ou chamada directa a `comemo::evict`.
- Sem integração CLI (fica para pós-M8).
- Sem alterações em outros consumers existentes.

P204E é o sub-passo mais simples de M8 (per estimativa
P204A C12). Magnitude S.

---

## §2 Material de partida verificado em P204D

Antes de qualquer alteração, confirmar empíricamente:

- Crate L4 wiring existe — caminho real (provavelmente
  `04_wiring/src/lib.rs` ou similar).
- Comemo `evict` API exposta — confirmar assinatura no
  crate (provavelmente `pub fn evict(max_age: usize)` ou
  similar).
- Política de visibilidade do crate L4 — pública para
  consumo CLI/external?

Sem isto, recuar para P204D.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código:

1. **Crate L4 wiring** — confirmar:
   - Caminho real (esperado `04_wiring/`).
   - Estrutura dos módulos.
   - Existem ficheiros relacionados a comemo/cache?
   - API pública actual (`pub fn ...` em `lib.rs`).
2. **`comemo::evict` API** — confirmar:
   - Assinatura exacta.
   - Parâmetros e semântica (`max_age` é número de
     iterações antes de invalidar?).
   - Documentação no crate.
3. **Vanilla typst — `evict` exposure** — confirmar
   (per A8 da auditoria P204A):
   - Vanilla expõe `comemo::evict` directamente?
   - Wrapper em CLI ou crate intermédio?
   - Convenção de naming em vanilla.
4. **Localização canónica do wrapper** — decidir:
   - `04_wiring/src/eviction.rs` (módulo dedicado).
   - `04_wiring/src/lib.rs` (função top-level).
   - Outro nome se inventário sugerir convenção
     diferente.
5. **Visibilidade** — decidir:
   - `pub` (consumo external).
   - `pub(crate)` (consumo interno apenas).

Output: 5 sub-secções com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**. Cada item com evidência.

### C2 — Forma do wrapper

Decisão fixada com base em C1.2 + C1.3 + C1.4. Duas
alternativas canónicas:

- **Wrapper passthrough** — função 1-linha que delega:
  ```text
  pub fn crystalline_evict(max_age: usize) {
      comemo::evict(max_age)
  }
  ```
- **Wrapper com policy** — função que encapsula uma
  política específica (ex: `pub fn evict_all() {
  comemo::evict(0) }`).

Critério para escolha: simetria com vanilla. Se vanilla
expõe directamente sem policy, escolher passthrough. Se
vanilla aplica policy, replicar.

C2 fixa **uma** alternativa.

### C3 — Edição literal

Aplicar a alternativa fixada em C2. Edições prováveis:

- Novo ficheiro `04_wiring/src/eviction.rs` (se C1.4
  escolher módulo dedicado) ou edição de
  `04_wiring/src/lib.rs`.
- Re-export ou chamada a `comemo::evict`.
- `pub use` ou `pub fn` conforme C1.5.

C3 não tem ramos. Executa decisões fixas.

### C4 — Documentação inline

Adicionar comentário ao wrapper:

- Referência a ADR-0073 (mecanismo aplicado).
- Referência a P204E (passo que criou).
- Documentar semântica do `max_age` (quando invalidar).

Edição cirúrgica: 4–6 linhas de doc-comment.

### C5 — Tests

Adicionar 1 test sentinela:

- **`p204e_crystalline_evict_existe`** — falha de
  compilação se wrapper for removido. Apenas chama
  `crystalline_evict(0)` e confirma que compila.

Decisão dentro de P204E: 1 sentinela é suficiente.
Tests de comportamento real do `evict` requerem
infraestrutura de medição não existente (per C10 do
diagnóstico P204A).

### C6 — Compilação

```
cargo build --workspace 2>&1 | tail -10
```

Critério: verde. Hipóteses prováveis de erro:

- Crate `04_wiring` não tem `comemo` em dependências —
  adicionar a `Cargo.toml`.
- Visibilidade incorrecta — ajustar.

### C7 — Tests workspace

```
cargo test --workspace 2>&1 | tail -10
```

Critério: 1836+ tests verdes (mais 1 sentinela de C5).

### C8 — Linter

```
crystalline-lint .
```

Critério: 0 violations.

Hipóteses prováveis de violação:

- Regra de visibilidade L4.
- Regra de re-export.

Resolver caso a caso.

### C9 — Documentação ADR-0073

ADR-0073 mantém PROPOSTO. Adicionar nota cirúrgica na
secção "Plano de materialização" confirmando que P204E
foi concluído (1 linha).

### C10 — Critério de fecho de P204E

P204E concluído quando:

- C1 inventário completo.
- C2 forma fixada.
- C3 edição aplicada.
- C4 documentação inline.
- C5 sentinela adicionada.
- C6 compilação verde.
- C7 tests workspace verdes.
- C8 linter 0 violations.
- C9 ADR-0073 anotada.
- Inventário registado.
- Relatório escrito.

### C11 — Sem cláusulas condicionais

C1 produz dados. C2 fixa **uma** alternativa. C3–C9
executam.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204E-inventario.md`.

Conteúdo:
- §1 C1 — inventário (5 sub-secções).
- §2 C2 — forma fixada com justificação.
- §3 C3 — edição aplicada.
- §4 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-204E-relatorio.md`.

Conteúdo:
- O que foi feito.
- Tempo de execução.
- Métricas.
- Decisões.
- Sugestão para próximo sub-passo (P204F).

### Ficheiro 3 — Alterações em código

Não é ficheiro discreto. Conjunto de alterações em:

- 1 ficheiro novo ou modificado em `04_wiring/src/`.
- `04_wiring/Cargo.toml` (se `comemo` precisar de
  adição).
- Test sentinela (mesmo crate ou crate de tests).
- ADR-0073 (anotação cirúrgica).

---

## §5 Critério de progressão para P204F

P204E fechado quando C10 cumprido.

Em caso de divergência empírica relevante (ex: crate
L4 não estruturado como esperado, `comemo::evict` ter
assinatura diferente), registar em `P204E.div-N` e:

- Resolver dentro de P204E (preferido — wrapper é
  trivial).
- Recuar para P204A re-fixar C6 (improvável).

P204F só começa quando P204E fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 3 outputs.
- Inventário empírico antes de implementação.
- Localização canónica:
  `00_nucleo/diagnosticos/` para inventário;
  `00_nucleo/materialization/` para relatório.
- Sem inflação retórica.

---

## §7 Não-objectivos

P204E não:

- Adiciona ficheiros ao corpus de paridade (P204F).
- Adiciona benchmarks (P204G).
- Transita ADR-0073 para ACEITE (P204H).
- Transita ADR-0066 para superseded (P204H).
- Cria ADR nova.
- Integra `crystalline_evict` em CLI ou em fluxo
  automatizado de compilação.
- Toca em consumers de `Introspector`, Layouter, ou
  qualquer trait L1.
- Modifica trait `Introspector` ou impl
  `TagIntrospector`.

---

## §8 Erro a não repetir

Padrão das séries P203 + P204 — premissas erradas em
specs detectadas via inventário empírico. P204E mantém
o padrão: C1 verifica antes de C2 decidir.

Hipótese específica a testar em C1: `04_wiring/` pode
não ter o nome esperado, ou pode estar reorganizado
desde a última auditoria. P201 detectou a localização
canónica `00_nucleo/diagnosticos/` em vez de
`00_nucleo/`. Localização do crate L4 wiring pode
similarmente divergir do esperado.

C2 fica para o inventário decidir entre passthrough e
policy. **Não pré-fixei**.

---

## §9 Particularidade — execução

P204E é trabalho de código mínimo:

- 1–4 linhas de função wrapper.
- 1–6 linhas de doc-comment.
- 1 test sentinela.
- Possível adição em `Cargo.toml`.

Volume baixíssimo. Magnitude S.

Recomendado pela sessão actual (Opus, com bash_tool)
se houver disponibilidade — P204E é o sub-passo mais
simples de M8 e não exige iteração rápida com cargo
build. Caso contrário, Claude Code segue padrão
habitual (overkill para o tamanho do trabalho, mas
consistente).
