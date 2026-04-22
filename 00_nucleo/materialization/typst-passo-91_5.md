# Passo 91.5 — Registar ADR-0036 (atomização progressiva) no índice

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/README.md` — índice canónico dos ADRs. Formato
  actual: 36 ADRs (35 números únicos; ADR-0026 tem variante -R1),
  distribuição com `EM VIGOR: 7`.
- `00_nucleo/adr/typst-adr-0035-ecovec-autorizacao.md` — último
  ADR actual, para referência de formato.
- `00_nucleo/DEBT.md` — DEBT-44 e DEBT-45 (abertos no Passo 91)
  referenciarão o ADR-0036 quando o `00_nucleo/adr/README.md` for
  actualizado.
- Ficheiro do ADR-0036 fornecido pelo utilizador (a colocar em
  `00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md`).

Pré-condição: `cargo test` — 747 L1 + 174 L3 + 6 ignorados, zero
violations. Passo 91 concluído (DEBT-44 e DEBT-45 abertos).

---

## Natureza deste passo

Sub-passo de governança. Não altera código. Regista um ADR
produzido fora do ciclo normal de passos — o ADR-0036 foi
escrito durante a discussão que precedeu o Passo 91, formaliza
o princípio arquitectural que o Passo 92 (DEBT-44) vai aplicar.

Justificação para sub-passo: a política "sub-passos apenas para
pagar DEBTs ou verificações" abrange também **registos de
governança** que não cabem num passo de construção. Este passo
é **governança pura** — move um ficheiro já escrito para o seu
lugar canónico e actualiza o índice.

Regra absoluta: **não altera código**, **não altera outros
ADRs**, **não altera `DEBT.md`**. Altera apenas:

- `00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md` (ficheiro
  novo, conteúdo fornecido pelo utilizador).
- `00_nucleo/adr/README.md` (4 actualizações mínimas no índice).

---

## Decisões formalizadas neste passo

Nenhuma. O conteúdo do ADR-0036 já está decidido (discussão
pré-Passo 91). Este passo materializa a decisão no repositório.

---

## Tarefa A — Colocar o ficheiro do ADR-0036

### A.1 — Localização do conteúdo

O conteúdo do ADR-0036 vem de ficheiro fornecido pelo utilizador
(partilhado na conversa da sessão, exportado via o workflow de
outputs do Claude). Conteúdo esperado:

- Título: `⚖️ ADR-0036: Atomização progressiva — estado partilhado
  como dívida`.
- **Status**: `EM VIGOR`.
- **Data**: 2026-04-22.
- Sem campos de relação (não revoga, não é revisão).
- Secções: Contexto, Decisão (com 5 regras operacionais),
  Alternativas Consideradas, Consequências, Plano de redução
  progressiva, Referências.
- Tamanho aproximado: 170-190 linhas.

### A.2 — Caminho do ficheiro

`00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md`.

### A.3 — Verificação de formato

Após criação, validar:

```bash
# Ficheiro existe:
ls -la 00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md

# Cabeçalho canónico:
head -10 00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md

# Tamanho razoável:
wc -l 00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md
```

Esperado:
- `Status: EM VIGOR` com backticks.
- `Data: 2026-04-22`.
- Entre 150 e 250 linhas.

Se o conteúdo diverge substancialmente do esperado (ex: não tem
as 5 regras operacionais), **parar** e reportar antes de
prosseguir.

---

## Tarefa B — Actualizar `00_nucleo/adr/README.md`

O `README.md` tem 4 pontos a actualizar. **Ordem sugerida: de
baixo para cima**, para que as contagens agregadas sejam
consistentes com a tabela.

### B.1 — Adicionar linha na tabela "Estado por ADR"

Localizar a tabela (formato `| ADR | Título curto | Status |`).
Actual última linha é `| 0035 | EcoVec autorizado | EM VIGOR |`.

Adicionar imediatamente após:

```markdown
| 0036 | Atomização progressiva — estado partilhado como dívida | `EM VIGOR` |
```

### B.2 — Actualizar contagem por status

Na secção "Distribuição de status" (texto abaixo da tabela),
encontrar a linha:

```
- `EM VIGOR`: 7 ADRs (regras/políticas activas).
```

Alterar para:

```
- `EM VIGOR`: 8 ADRs (regras/políticas activas).
```

### B.3 — Actualizar total de ADRs

Na mesma secção, encontrar a linha de total:

```
**Total**: 36 ADRs (35 números únicos; ADR-0026 tem variante -R1
por revisão).
```

Alterar para:

```
**Total**: 37 ADRs (36 números únicos; ADR-0026 tem variante -R1
por revisão).
```

### B.4 — Actualizar preâmbulo

No topo do `README.md`, encontrar a frase do preâmbulo que
menciona número total. Actualmente é algo como:

```
Lista os 36 ADRs em vigor, as meta-regras que governam o
projecto, [...]
```

Alterar para:

```
Lista os 37 ADRs em vigor, as meta-regras que governam o
projecto, [...]
```

**Importante**: o texto exacto do preâmbulo pode variar. Não
alterar mais do que o número. Se a frase não contiver número
explícito, saltar este item e reportar.

### B.5 — Opcional: adicionar ADR-0036 à secção "Meta-regras em vigor"

Se o ADR-0036 formaliza regra arquitectural transversal (o que
faz), pode ser adicionado à lista de meta-regras no início do
`README.md`. Verificar se essa secção existe:

```bash
grep -A 5 "Meta-regras em vigor" 00_nucleo/adr/README.md
```

Se existe e lista outras meta-regras (ADR-0018, 0029, 0030,
0032, 0033, 0034), adicionar entrada para o ADR-0036:

```markdown
7. **Atomização progressiva** — ADR-0036. Funções de L1 declaram
   explicitamente todas as dependências na assinatura. Estado
   partilhado mutável é dívida a reduzir progressivamente.
   Primeiro pagamento concreto: DEBT-44 (integração estrutural
   do `Route<'a>`).
```

Se a secção não existe ou tem formato diferente, saltar e
reportar.

---

## Critérios de conclusão

- [ ] Ficheiro
      `00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md`
      existe com `Status: EM VIGOR` e data correcta.
- [ ] `README.md` tem nova linha na tabela "Estado por ADR" para
      o ADR-0036.
- [ ] Contagem `EM VIGOR` actualizada de 7 para 8.
- [ ] Total de ADRs actualizado de 36 para 37.
- [ ] Preâmbulo (se contém número) actualizado.
- [ ] Opcional (B.5): ADR-0036 listado em "Meta-regras em vigor"
      se essa secção existir.
- [ ] Nenhum outro ADR alterado.
- [ ] Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
      `04_wiring/` alterado.
- [ ] `00_nucleo/DEBT.md` não alterado.
- [ ] `cargo test` passa com os mesmos 747 L1 + 174 L3 + 6
      ignorados. `crystalline-lint` → zero violations.

---

## Ao terminar, reportar

Tarefa A:
- Caminho e tamanho do ADR-0036 criado.
- Data confirmada no cabeçalho.

Tarefa B:
- Número de linhas alteradas em `README.md`.
- Confirmação dos 4 (ou 5) pontos actualizados.
- Se B.5 foi aplicado ou saltado (com razão).

Verificação:
- Contagem de testes inalterada.
- Zero violations.

Go/No-Go para Passo 92:
- **Go** se ADR-0036 está registado no índice e pré-condição
  preservada. Passo 92 pode citar ADR-0036 explicitamente na
  sua justificação.
- **No-Go** se o conteúdo do ADR-0036 divergiu do esperado ou
  se o `README.md` tem formato que não aceita as alterações
  directamente. Nesse caso, reportar a divergência e decidir
  em conversa antes do Passo 92.
