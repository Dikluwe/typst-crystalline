# L0 — Passo 1066: Auditoria V21 — 12 Casos de `2.0 × margin`

**Escopo**: só auditoria e classificação, mesmo formato do P1064. Nenhuma anotação
de silêncio, nenhuma mudança de código neste passo.

**Gate**: nenhum — auditoria pura.

**Base**: a pendência original desta conversa (documento de continuação) dizia
"`/2.0`, `2.0 × margin`" — dois padrões, desde o início. O L0 do P1064 (escrito por
mim) só endereçou `/2.0`, por omissão minha, não por decisão consciente de excluir
`2.0 × margin`. O P1065 revelou 12 avisos V21 desse padrão, ainda não auditados.

---

## 1. Inventário

```bash
grep -rnE "2\.0\s*\*|\*\s*2\.0" 01_core/src/compiler/ | grep -v "/2\.0\|2\.0/"
```

(ajustar o padrão para não recapturar os 46 casos de divisão já anotados no P1065 —
confirmar manualmente que os 12 avisos V21 restantes de `crystalline-lint --checks
v21` são todos deste padrão de multiplicação, não uma mistura com algo mais).

Reconciliar contra o número "12" já citado no relatório do P1065 (discriminação dos
36 avisos restantes). Se o `grep` achar um número diferente, registar a diferença
explicitamente, não ajustar silenciosamente o alvo.

## 2. Hipótese a testar

`2.0 × margin` (dobrar uma margem para obter a largura/altura total de um espaço
simétrico, ou o inverso — dividir algo por `2.0 × margin` para achar quantas
unidades cabem) é a mesma classe de geometria auto-evidente que `/2.0` — não
precisa de citação externa, porque a prova é a própria álgebra, não uma decisão de
design do vanilla.

**Não presumir isto por analogia com o P1064** — confirmar por amostra, mesmo
processo: cada caso, contra o ponto correspondente no vanilla, com citação literal
(linha + código), não descrição em prosa.

## 3. Amostragem

Mínimo de 6 casos (metade do total, dado que 12 é menos da metade de 46) com:

1. Transcrição da linha exacta e contexto.
2. Confirmação de que a fórmula é genuinamente "duplicar/dividir por margem
   simétrica" — não outra coisa que só parece o padrão de relance.
3. Comparação linha a linha com o vanilla, mesmo rigor exigido no P1064 (revisão
   anterior rejeitou citações sem número de linha ou com a mesma linha citada para
   expressões diferentes sem explicação — não repetir esses dois erros aqui).

## 4. Critério de conclusão

- Inventário reconciliado contra "12".
- Pelo menos 6 casos com prova completa (citação literal ou álgebra + medição,
  mesmo padrão exigido e só aceite no P1064 depois de duas rodadas de correcção).
- Veredicto por caso — confirma/não confirma/inconclusivo, sem generalizar se
  algum falhar.
- Recomendação ao dono (não anotação executada) se a hipótese for confirmada.

---

## Nota

Se esta auditoria confirmar a hipótese, o passo seguinte (anotação) pode reutilizar
directamente o mecanismo já homologado no P1065 (`// rationale:`, confirmado contra
avisos reais do linter) — não é preciso reabrir a validação da convenção.
