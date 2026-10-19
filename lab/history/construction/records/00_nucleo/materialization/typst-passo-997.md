# Passo 997 — Verificar ancoragem de `=` (e resto da expressão) em `(n \ k) = ...`

**Tipo**: Verificação primeiro; fix só se confirmado (fluxo contínuo, ADR-0127 — não é
mudança de contrato, é correcção de âncora dentro de grelha já existente).
**Origem**: confirmação externa independente (investigação paralela, documento
`CORREÇÃO GRANDE — (n \ k)`, 2026-08-12) — depois de testar contra Typst 0.15.1 genuíno
(binário oficial, não um ficheiro de referência), confirmou que parêntese solto por linha
(sem esticar) é o comportamento correcto — **o mesmo que P996 já implementou e fechou**.
Acrescenta um achado novo, fora do âmbito de P996: no Typst genuíno, `=` (e o resto da
expressão que se segue) fica ancorado à **linha de baixo** (junto de `k)`), não à linha de
cima (junto de `(n`).

**Pré-condição**: `git status`. Confirmar HEAD ≥ `9989adb24` (P996).

---

## Passo 1 — Reproduzir o construto completo, não só `(n \ k)` isolado

P996 validou `(n \ k)` sozinho e `(a = b \ c = d)` (duas linhas, ambas completas). O achado
novo é sobre um padrão diferente: **conteúdo depois do grupo de linhas, na mesma
expressão** — `(n \ k) = n!/(k!(n-k)!)`, onde o `=` e a fracção vêm a seguir ao delimitador,
não dentro dele.

Construir o repro mínimo exacto:
```
$ (n \ k) = n!/(k!(n-k)!) $
```
Compilar com o cristalino actual (HEAD ≥ P996) e com o vanilla de referência (baseline
ratificado, `a51e02804` — confirmar identidade por string, per P934).

## Passo 2 — Medir onde o `=` se ancora nos dois

`pdftotext -bbox`, ou renders a alta resolução se a extracção de texto não distinguir bem
qual linha. Confirmar:
- Vanilla: `=` (e o resto) na mesma linha de `k)`, ou na de `(n`?
- Cristalino: idem.

**Não presumir o resultado do cristalino a partir do relatório externo** — a versão deles
(23:51 de 2026-08-11) pode não corresponder exactamente ao nosso commit de P996
(`9989adb24`), que foi produzido de forma independente. Medir o nosso próprio binário.

## Passo 3 — Se confirmado o mesmo bug

Localizar por que o `=`/resto da expressão está a ser anexado à primeira linha do grupo
`(n \ k)` em vez de continuar depois da quebra, na posição em que apareceria naturalmente
se o `Linebreak` fosse tratado como fronteira real de linha para o resto da sequência
matemática (não só dentro do delimitador, que já foi corrigido por P996).

Hipótese a confirmar por leitura de código, não presumir: o fix de P996 mudou
`Expr::MathDelimited` para emitir `MathSequence[MathText(open), …corpo com Linebreak…,
MathText(close)]` em vez de emparelhar. Se o `=` e o resto vierem **depois** desse
`MathSequence` completo (fora dele, na sequência matemática maior), a pergunta é: o `close`
(`)`) está a ser anexado à primeira ou à última linha do corpo, e o que vem depois do
`MathSequence` continua daí? Verificar `file:line` exacto em `eval/math.rs` e no caminho de
grelha (`layout_grid`/`layout_sequence`) tocado por P996 e por P51 (`needs_grid_layout`).

## Passo 4 — Fix (só depois de Passo 3 confirmar causa por leitura, não suposição)

TDD directo per o padrão já estabelecido nesta frente. Teste: `(n \ k) = x` — `=`/`x` na
mesma linha de `k)`; guarda de não-regressão: `(n \ k)` sozinho continua correcto (P996);
`(a = b \ c = d)` sozinho continua correcto.

## Passo 5 — Revalidação

Secção 7 do canónico completa (não só o delimitador isolado). Benchmark, zero regressão.
Relatório com hash do commit final (regra já reforçada três vezes nesta frente — sem
placeholder).

## Se NÃO confirmado (o cristalino já ancora correctamente)

Registar no relatório que a medição da investigação paralela (23:51) não corresponde ao
estado do nosso HEAD pós-P996 — sem necessidade de fix, fechar como não-aplicável, com a
medição como prova.

---

## Nota de proveniência

Este passo nasce de uma fonte externa (não medida directamente por nós antes de escrever
o passo) — por isso o Passo 1/2 exige reprodução e medição própria antes de qualquer
suposição sobre o estado actual do cristalino, per a mesma disciplina já aplicada em
P995/P996 (nunca aceitar um achado externo sem verificação directa, mesmo quando parece
plausível).
