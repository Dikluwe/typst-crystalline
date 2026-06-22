# Análise P410 — A, B, C: Field Access Version / Field Access Duration / Aritmética Version

**Data**: 2026-06-22.  
**Objetivo**: Determinar, via sonda de substrato, quais dos 3 caminhos já estão implementados (redundantes) e quais necessitam materialização.  
**Padrão**: ADR-0108 (medir-antes-de-decidir).

---

## Matriz de análise

| Caminho | Descrição | Tamanho estimado | Bloqueios | Risco de redundência |
|---------|-----------|------------------|-----------|----------------------|
| **A** | Field access `.major`/`.minor`/`.patch`/`.pre`/`.build` em `Value::Version` | S | Nenhum | Médio — field access é padrão comum em L1 |
| **B** | Field access `.seconds`/`.minutes`/`.hours`/`.days` em `Value::Duration` | S | Nenhum | Médio — field access é padrão comum em L1 |
| **C** | Aritmética/comparações `Version` (semver ordering) | S/M | Pode depender de infra de eval binário | Alto — se P405 já fez Duration, talvez Version também tenha sido feito |

---

## Sonda A — Field Access Version (Caminho A)

### Comandos de sonda

```bash
# 1. Confirmar que Value::Version existe no enum
grep -n "Value::Version" entities/value.rs
# Esperado: 1+ linha confirmando variant

# 2. Confirmar que Version tipo L1 tem campos acessíveis
grep -n "pub major\|pub minor\|pub patch\|pub pre\|pub build" entities/version.rs
# Esperado: campos públicos ou getters

# 3. Verificar se já existe field access para Version em eval
grep -rn "Value::Version" rules/eval/ | grep -i "field\|get_field\|dot\|access\|major\|minor\|patch"
# Esperado: zero hits (se não implementado) OU match arms (se já implementado)

# 4. Verificar padrão de field access existente (ex.: Int, Float, Str)
grep -rn "get_field\|field_access" rules/eval/ | head -20
# Esperado: infra de field access para outros tipos

# 5. Verificar se testes de field access Version já existem
grep -rn "version.*major\|version.*minor\|version.*patch" rules/eval/tests.rs entities/tests/
# Esperado: zero hits (se não implementado)
```

### Critério de passagem para "necessita implementação"

- (1) ✅ + (2) ✅ + (3) **zero hits** + (4) ✅ = **Caminho A é necessário** (field access não existe).
- (1) ✅ + (2) ✅ + (3) **hits encontrados** = **Caminho A é redundante** (já implementado).

---

## Sonda B — Field Access Duration (Caminho B)

### Comandos de sonda

```bash
# 1. Confirmar que Value::Duration existe no enum
grep -n "Value::Duration" entities/value.rs
# Esperado: 1+ linha confirmando variant

# 2. Confirmar que Duration tipo L1 tem métodos de decomposição
grep -n "pub fn as_secs\|pub fn as_millis\|pub fn as_nanos\|pub fn days\|pub fn hours\|pub fn minutes\|pub fn seconds" entities/duration.rs
# Esperado: métodos de extração de componentes

# 3. Verificar se já existe field access para Duration em eval
grep -rn "Value::Duration" rules/eval/ | grep -i "field\|get_field\|dot\|access\|days\|hours\|minutes\|seconds"
# Esperado: zero hits (se não implementado) OU match arms (se já implementado)

# 4. Verificar padrão de field access existente (mesmo que Sonda A.4)
grep -rn "get_field\|field_access" rules/eval/ | head -20
# Esperado: infra de field access para outros tipos

# 5. Verificar se testes de field access Duration já existem
grep -rn "duration.*days\|duration.*hours\|duration.*minutes\|duration.*seconds" rules/eval/tests.rs entities/tests/
# Esperado: zero hits (se não implementado)
```

### Critério de passagem para "necessita implementação"

- (1) ✅ + (2) ✅ + (3) **zero hits** + (4) ✅ = **Caminho B é necessário**.
- (1) ✅ + (2) ✅ + (3) **hits encontrados** = **Caminho B é redundante**.

---

## Sonda C — Aritmética/Comparações Version (Caminho C)

### Comandos de sonda

```bash
# 1. Confirmar que Value::Version existe no enum
grep -n "Value::Version" entities/value.rs
# Esperado: 1+ linha confirmando variant

# 2. Confirmar que Version tipo L1 tem PartialOrd/Eq
grep -n "impl.*PartialOrd\|impl.*Ord\|impl.*Eq\|impl.*PartialEq" entities/version.rs
# Esperado: derives ou impls de comparação

# 3. Verificar se Version já está nos operadores de eval
grep -rn "Value::Version" rules/eval/ | grep -i "add\|sub\|mul\|div\|cmp\|eq\|lt\|gt\|BinaryOp"
# Esperado: zero hits (se não implementado) OU match arms (se já implementado)

# 4. Verificar infra de eval binário (mesmo que P409)
grep -rn "BinaryOp::Eq\|BinaryOp::Lt" rules/eval/ | head -10
# Esperado: match arms existentes para outros tipos

# 5. Verificar se testes de comparação Version já existem
grep -rn "version.*==\|version.*<\|version.*cmp" rules/eval/tests.rs entities/tests/
# Esperado: zero hits (se não implementado)
```

### Critério de passagem para "necessita implementação"

- (1) ✅ + (2) ✅ + (3) **zero hits** + (4) ✅ = **Caminho C é necessário**.
- (1) ✅ + (2) ✅ + (3) **hits encontrados** = **Caminho C é redundante** (possivelmente implementado junto com P401 ou P403).

**Nota especial**: Version não tem operadores aritméticos (`+`, `-`, `*`, `/`) no vanilla — apenas comparações (`==`, `!=`, `<`, `<=`, `>`, `>=`) e concatenação de pré-release. O Caminho C, se necessário, é **apenas comparações** (não aritmética).

---

## Instruções de execução

1. Copiar os 3 blocos de comandos acima.
2. Executar no terminal do projeto (raiz do workspace Rust).
3. Colar o output bruto (não filtrado) desta conversa.
4. Eu classifico cada caminho como:
   - **VERDE** → já implementado, nenhum passo necessário.
   - **AMARELO** → parcialmente implementado, precisa de complemento.
   - **VERMELHO** → não implementado, passo necessário.

---

## Hipótese preliminar (baseada em padrões do projeto)

| Caminho | Hipótese | Confiança |
|---------|----------|-----------|
| A | **VERMELHO** — field access em L1 tipos primitivos é feito sob demanda; Version é recente (P401) | 70% |
| B | **VERMELHO** — mesmo raciocínio; Duration é P400, mas field access não foi mencionado em P403/P405/P409 | 70% |
| C | **AMARELO/VERMELHO** — comparações de Version podem ter sido implementadas em P401 (modelagem) ou podem estar pendentes; aritmética não existe (Version não é numérico) | 50% |

**A hipótese pode estar errada.** A sonda é a única fonte de verdade. Não iniciar nenhum passo antes da sonda confirmar.
