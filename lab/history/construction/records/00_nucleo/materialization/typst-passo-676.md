---
# P676 — Porque a poupança interna de P675 não aparece no tempo de parede?

> **Passo:** 676
> **Data:** 2026-07-10
> **Foco:** P675 mediu `total_ms` a cair de 5222ms para 4668ms (−10,6%) depois de eliminar um walk duplicado, mas o `hyperfine` directo mostra exactamente o mesmo rácio de antes (1,26×) — a poupança interna não aparece no tempo de parede medido de fora. O relatório atribuiu isto a "ruído do sistema", mas uma poupança consistente e precisa de 554ms não costuma desaparecer por ruído. Este passo confirma se há custo fora das fases instrumentadas (o mesmo tipo já encontrado por P674 no arranque) a compensar a poupança.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** Uma explicação de "ruído" para uma discrepância deste tamanho, sem mais confirmação, não é uma explicação — é uma suposição confortável.

---

## Verificação

### Medir com mais repetições, para reduzir a margem de ruído genuíno

```bash
hyperfine --warmup 5 --runs 30 './target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p676.pdf'
```

Com 30 execuções em vez de poucas, confirmar se a média e o desvio padrão suportam mesmo "ruído", ou se o tempo de parede está consistentemente a não reflectir a poupança interna.

### Confirmar quanto tempo fica fora das fases instrumentadas

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p676-timings.pdf --timings-json /tmp/p676-timings.json
python3 -c "
import json
d = json.load(open('/tmp/p676-timings.json'))
print('total_ms instrumentado:', d['total_ms'])
"
time ./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p676-time.pdf
```

Comparar `total_ms` do JSON com o tempo real medido por `time`. A diferença entre os dois é o tempo fora de todas as fases instrumentadas — arranque do processo, escrita do PDF em disco, ou outra coisa ainda não isolada.

### Repetir esta comparação antes e depois da correcção de P675

Se possível, fazer checkout do commit imediatamente antes de P675 e medir a mesma diferença, para confirmar se esse "tempo fora das fases" já existia antes, do mesmo tamanho, ou se cresceu justamente para compensar a poupança de P675 (o que seria uma coincidência improvável e merece mais escrutínio).

### Critério de fecho da verificação

- [ ] Medição com mais repetições, confirmando ou refutando "ruído".
- [ ] Tempo fora das fases instrumentadas, medido directamente, antes e depois de P675.
- [ ] Se houver um custo fixo fora das fases, do mesmo tamanho da poupança: localizado, com a mesma disciplina já usada em P674.

---

## Decisão

Se confirmado que há mesmo um custo fora das fases a compensar a poupança: localizar e corrigir, ou pelo menos documentar com precisão o que é, em vez de atribuir a "ruído".

Se a medição com mais repetições confirmar que era mesmo variação normal, e a poupança de facto aparece no tempo de parede quando medida com cuidado: corrigir o relatório de P675, que tinha esta conclusão com uma amostra pequena de mais.

---

## Critério de fecho do passo

- [ ] Medição com amostra maior, conclusão revista com mais confiança.
- [ ] Tempo fora das fases instrumentadas, quantificado directamente.
- [ ] Causa da discrepância explicada com números, não com "ruído" sem mais.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p676.md`, com hash do commit.
- [ ] Se P675 estava optimista: registo corrigido com o número real.
