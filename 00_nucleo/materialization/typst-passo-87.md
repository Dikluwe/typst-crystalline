# Passo 87 — Verificação do estado de autorização de `EcoVec` em L1

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0024-*.md` — ADR que autorizou `ecow`
  (verificar se é autorização pontual para `Value::Str` ou global
  para a crate).
- `00_nucleo/adr/typst-adr-0018-*.md` — critério de autorização
  externa (pureza funcional, não origem).
- `00_nucleo/diagnosticos/diagnostico-stubs-comemo-passo-86.md` —
  identifica `EcoVec` como "externo não autorizado" para `Sink` e
  `Styles`.
- `01_core/Cargo.toml` — verificar se `ecow` está listada em
  `[dependencies]` e em `[l1_allowed_external]`.

Pré-condição: `cargo test` — 911 testes (737 L1 + 174 L3, 6
ignorados pré-existentes), zero violations. Passo 86 concluído.

---

## Natureza deste passo

Sub-passo de verificação (não altera código). Responde a uma
pergunta factual que ficou em aberto no Passo 86: "o ADR-0024
autoriza `ecow` globalmente em L1 ou apenas para `Value::Str`?".

A resposta determina se `EcoVec` pode ser usado em `Sink` e
`Styles` sem novo ADR, ou se é necessário passo adicional para
autorizar (extensão à ADR-0024 ou ADR nova).

Este sub-passo é justificado pela política "sub-passos apenas
para pagar DEBTs ou verificações" — é uma verificação que evita
decisão incorrecta nos passos de construção futuros.

Regra absoluta: **não edita código**, **não edita ADRs**, **não
edita `DEBT.md`**. Apenas lê e produz um relatório curto.

---

## Tarefa única — Classificar o estado de autorização de `ecow`

### 1 — Ler o ADR-0024

Ler o ficheiro completo. Identificar:

- **Escopo declarado**: o ADR diz "autorizar `ecow` para
  `Value::Str`" ou "autorizar `ecow` em L1"?
- **Justificação**: qual a razão dada para a autorização? Se
  a razão é específica a `Value::Str`, a autorização é pontual.
  Se a razão é geral (pureza da crate, performance), é global.
- **Tipos mencionados**: o ADR cita apenas `EcoString`, ou
  também `EcoVec`, `EcoMap`, etc.?

### 2 — Verificar o `Cargo.toml` e a configuração do linter

```bash
# ecow está declarada como dependência?
grep -A 2 "ecow" 01_core/Cargo.toml

# Está listada em [l1_allowed_external] do linter?
# (Localização exacta depende da configuração do crystalline-lint)
grep -rn "ecow\|l1_allowed_external" 01_core/ --include="*.toml" \
    --include="*.rs"
```

Se o linter permite `use ecow::*` em L1 sem reportar violação,
**na prática** o uso está permitido. Se o linter só permite
`use ecow::EcoString`, a autorização é pontual.

### 3 — Procurar usos actuais de tipos `ecow` em L1

```bash
# Usos de EcoString, EcoVec, EcoMap em L1:
grep -rn "EcoString\|EcoVec\|EcoMap\|ecow::" 01_core/src/ \
    --include="*.rs" | head -40
```

Se apenas `EcoString` aparece, a autorização é **de facto**
pontual. Se `EcoVec` ou outros tipos `ecow` já aparecem, a
autorização é **de facto** mais ampla do que o ADR-0024 declara.

### 4 — Classificar o estado

Após os passos 1-3, classificar em uma de três categorias:

**A. Pontual explícita**: ADR-0024 diz literalmente "para
`Value::Str`" e o código só usa `EcoString`. Conclusão: usar
`EcoVec` requer novo ADR ou extensão ao ADR-0024.

**B. Global explícita**: ADR-0024 diz "autorizar crate `ecow`"
(sem restrição a tipo). Conclusão: `EcoVec` pode ser usado sem
novo ADR.

**C. Ambígua**: ADR-0024 é específico mas o código e o linter
permitem mais do que o ADR declara. Conclusão: ambiguidade é
uma decisão pendente — ADR de clarificação é necessário antes
de materializar `Sink`/`Styles`.

### 5 — Produzir relatório

Produzir ficheiro
`00_nucleo/diagnosticos/diagnostico-ecow-autorizacao-passo-87.md`
com as seguintes secções:

1. **Texto literal do escopo do ADR-0024** — citar 2-4 linhas
   da decisão formal do ADR (secção "Decisão" ou equivalente).
2. **Configuração do linter** — listar como `ecow` aparece em
   `[l1_allowed_external]` ou equivalente. Se não aparece,
   registar.
3. **Uso actual em código** — lista de tipos `ecow::*` usados
   em `01_core/src/`, por frequência aproximada.
4. **Classificação** — A, B, ou C (ver ponto 4 acima) com
   justificação de 1-2 frases.
5. **Implicação directa para `Sink`/`Styles`** — uma frase:
   "`EcoVec` pode ser usado em L1 sem novo ADR" ou "`EcoVec`
   requer ADR novo/extensão antes de ser usado em L1".

Tamanho alvo: 40–80 linhas (relatório curto).

---

## Critérios de conclusão

- [ ] Ficheiro
      `00_nucleo/diagnosticos/diagnostico-ecow-autorizacao-passo-87.md`
      existe com as 5 secções.
- [ ] Classificação final (A, B, ou C) registada no relatório.
- [ ] Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
      `04_wiring/` foi alterado.
- [ ] Nenhum ficheiro em `00_nucleo/adr/` foi alterado.
- [ ] `00_nucleo/DEBT.md` não foi alterado.
- [ ] `cargo test` passa com 911 testes, zero violations.

---

## Ao terminar, reportar

- Classificação: A (pontual), B (global), ou C (ambígua).
- Tipos `ecow::*` actualmente usados em L1.
- Resposta directa de uma frase: "autorização é [pontual/
  global/ambígua]; para materializar `Sink` e `Styles`,
  `EcoVec` [pode ser usado / precisa de ADR]".

Go/No-Go para Passo 88 (materializar `Traced`):
- **Go incondicional** para `Traced` — não depende de `EcoVec`.
- **Condicional para passos que materializam `Sink`/`Styles`**
  com base na classificação:
  - Se A ou C: passo intermédio para estender/clarificar
    ADR-0024 antes desses stubs.
  - Se B: materialização directa, sem ADR intermédio.
