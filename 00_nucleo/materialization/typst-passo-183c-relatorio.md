# Relatório P183C — caso de bloqueio

**Data**: 2026-05-03
**Passo**: P183C — auditoria semântica explícita antes de migrar C2 (equation counter)
**Resultado**: **bloqueado em `.B` por gate substancial**. Zero código tocado.

---

## §1 Resumo

P183C tentou migrar o consumer C2
(`01_core/src/rules/layout/equation.rs:97`,
`self.counter.get_flat("equation")`) de legacy para
`Introspector::flat_counter("equation")` com fallback. A auditoria
semântica `.B` (auditoria que faltou em P183B) detectou **gate
substancial em duas dimensões independentes**:

1. **Asimetria temporal** snapshot-during-walk vs snapshot-final —
   mesma natureza de C1 (P183B).
2. **Ausência de arm `Equation` em `from_tags`** — `ElementPayload`
   não tem variant `Equation`; o walk não emite Tag para equations;
   `CounterRegistry` nunca recebe entry para a key `"equation"`.
   `flat_counter("equation")` (hipotético) retornaria sempre `None`.

A categoria de bloqueio é confirmada: **"consumers que precisam de
valor durante o walk, não do final"**. Para C2 a categoria é
**agravada** pela ausência de dados — não basta fixar a semântica
temporal, é também necessário emitir Tag para equation.

Diagnóstico completo em
`00_nucleo/diagnosticos/diagnostico-p183c-bloqueio.md`.

---

## §2 Sub-passos executados

| Sub-passo | Estado | Notas |
|-----------|--------|-------|
| `.A` Auditoria L0 | ✅ | C2 confirmado em `equation.rs:97`; `get_flat` legacy retorna `usize`; `flat_counter` ausente do trait Introspector; sub-store seria `CounterRegistry`. |
| `.B` Auditoria semântica | ❌ **gate substancial** | Falsa em duas dimensões: snapshot-final + sem arm Equation em `from_tags`. |
| `.C` Actualizar L0 | — não executado | gate em `.B` impede |
| `.D` Adicionar método trait + impl | — não executado | gate em `.B` impede |
| `.E` Migrar consumer C2 | — não executado | gate em `.B` impede |
| `.F` Tests E2E | — não executado | gate em `.B` impede |
| `.G` Escalada para DEBT | ✅ | Diagnóstico escrito; DEBT formal será aberto em P183F |
| `.H` Verificação estrutural | n/a (não aplica em bloqueio) | — |
| `.I` Encerramento | ✅ | Este relatório |

---

## §3 Confirmação de "tudo revertido"

`.A` e `.B` foram apenas leituras (Read tool); zero edições em
ficheiros de produção. Confirmação via `git status --short`:

```
?? 00_nucleo/diagnosticos/diagnostico-p183c-bloqueio.md
?? 00_nucleo/materialization/typst-passo-183b.md
?? 00_nucleo/materialization/typst-passo-183c.md
?? 00_nucleo/materialization/typst-passo-183c-relatorio.md  (este ficheiro)
```

- Único ficheiro novo introduzido por P183C em L0: o diagnóstico em
  `00_nucleo/diagnosticos/`.
- Zero ficheiros L1–L4 modificados.
- Zero L0 prompts (`00_nucleo/prompts/`) modificados.
- Zero linhas em ficheiros de código.

---

## §4 Estado pós-P183C

- **Tests workspace**: baseline P183A mantido — **1.756 verdes**; zero
  violations linter. (Sem alteração de código → sem possibilidade de
  regressão.)
- **Hashes L0**: inalterados (nenhum L0 modificado).
- **Trait `Introspector`**: 15 métodos (inalterado; `flat_counter` **não**
  foi adicionado).
- **P183 série**:
  - `A` ✅ (diagnóstico)
  - `B` ❌ (bloqueado, escalado — C1 heading prefix)
  - `C` ❌ (bloqueado, escalado — C2 equation counter)
  - `D` pendente (C3 figure auto-number; provável bloqueio pela mesma
    categoria — humano decide se executa ou salta)
  - `E` pendente (C4 resolved label; categoria diferente — labels são
    identidade, plausivelmente migrável)
  - `F` pendente (fecho da série; abre DEBT M4-residual)
- **Progresso M5/M4**: 5 read-sites migrados (P168 + P181G ×2 + P182D
  ×2). **C1 e C2 permanecem legacy** até DEBT M4-residual ser tratado.
- **36 passos executados** (P183A + P183B + P183C contam como 3 dos 36).

---

## §5 Próximo passo — decisão humana

A categoria "snapshot-during-walk" é agora confirmada empiricamente
em dois consumers (C1, C2). A inferência razoável é que C3 está
bloqueado pela mesma natureza. Há duas opções:

### Opção A — saltar P183D, ir directo a P183E + P183F

- **P183E**: migrar C4 (resolved label, `references.rs:53`).
  Categoria diferente (identidade, não contador incremental). Provável
  sucesso.
- **P183F**: fecho da série + abrir DEBT M4-residual cobrindo
  **C1 + C2 + C3** simultaneamente.
- **Ganho**: poupar ~30 min de auditoria redundante em P183D.
- **Custo**: assumir bloqueio C3 sem confirmação empírica.

### Opção B — manter rigor empírico

- **P183D**: auditoria semântica explícita C3 com mesma estrutura
  `.A`–`.B`–`.G`/`.C`–`.H`. Esperar gate substancial em `.B`.
- **P183E**: idem A.
- **P183F**: fecho com DEBT triplo confirmado.
- **Ganho**: rigor empírico. Documenta que C3 foi auditada (não inferida).
- **Custo**: ~30 min adicionais; conclusão pré-vista.

**Recomendação operacional**: Opção A se o humano confia no padrão (P183B
+ P183C convergem para a mesma categoria por mecanismos relacionados);
Opção B se preferir histórico empírico completo na série P183.

---

## §6 Aprendizado consolidado

A regra operacional para passos M4-residual e M5+ que migrem consumers
de contador:

> **Auditar antes de migrar em dois eixos:**
>
> 1. **Existem dados** no sub-store correspondente para a chave?
>    (Verificar arm correspondente em `from_tags`.)
> 2. **A semântica temporal coincide**? (Snapshot-final do Introspector
>    vs valor mutável durante walk no legacy.)
>
> Se qualquer dos dois falhar, declarar gate substancial e escalar para
> DEBT — não tentar substitution-with-fallback.

P183B descobriu o eixo 2. P183C ratificou o eixo 2 e adicionou o eixo 1.
Esta regra é o invariante operacional consolidado da série P183 para
guiar P183D, P183E e passos M+ análogos.

---

## §7 Pendência paralela (M6+ ou M+)

A solução estrutural que desbloqueia simultaneamente C1, C2 e (provavelmente)
C3 é:

1. **Trait `Introspector::flat_counter_at(key, location) -> Option<&[usize]>`**
   — primitiva já existe em `CounterRegistry::value_at`; falta o método
   trait + impl delegando.
2. **Layouter conhece a sua `Location`** no ponto da consulta — walk de
   layout produz `Location`s sincronizadas com walk de introspect, ou
   propaga `Location` actual via parâmetro.
3. **Para C2 especificamente**: emissão de Tag `Equation` no walk de
   introspect — `ElementPayload::Equation` + arm em `from_tags`
   chamando `apply_at("equation", Step, loc)`.

Pendência paralela P182E §5.2 já cobre (1) e (2). (3) é trabalho
independente que pode ser feito isoladamente.
