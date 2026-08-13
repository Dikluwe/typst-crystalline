# Passo 1025 — Reconciliar o alvo de paridade: é o vanilla mais novo, não 0.15.0

**Tipo**: Correcção de registo + verificação de estado real. O alvo de paridade **já foi
decidido**, na retificação P990-P992 (adendo do dono, 2026-08-11): upstream/main
`a51e02804`, pinado (não "main contínuo" — re-sync futuro exige passo explícito). O
Passo 1024 declarou "lab/typst-original, 0.15.0" como referência sem verificar contra
essa decisão já existente — isto é reconciliação de uma contradição, não uma decisão
nova.
**Pré-condição**: `git status` limpo.

---

## Fase A — Confirmar o estado real de `lab/typst-original`

```bash
cd lab/typst-original && git log -1 --format='%H %D' 2>/dev/null || echo "sem .git próprio"
./target/release/typst --version
```

Três hipóteses a distinguir, não presumir qual:
1. `lab/typst-original` está no commit ratificado (`a51e02804`) — o relatório do P1024
   está simplesmente errado ao chamar-lhe "0.15.0" (pode ser confusão com a string de
   versão enganosa já documentada na retificação — `typst 0.15.1 (e0e8ca4d)`, hash do
   repo local, não do Typst oficial).
2. `lab/typst-original` **regrediu** para 0.15.0 desde a ratificação (alguém fez
   checkout/reset sem passo explícito) — nesse caso é preciso resincronizar.
3. Nunca chegou a ser sincronizado como a retificação registou — mesma acção de (2).

## Fase B — Corrigir consoante o resultado

- **Hipótese 1**: corrigir só a declaração no relatório do P1024 e em qualquer sítio que
  repita "0.15.0" como alvo — não é preciso remedir nada, as medições desse passo já
  estavam contra o binário certo, só mal identificado.
- **Hipóteses 2/3**: resincronizar `lab/typst-original` para `a51e02804` (mesmo
  procedimento já documentado na retificação P990-P992), rebuild, **remedir** o achado da
  margem de 10% (`cases.md`/`matrix.md`, Passo 1024) contra o binário correcto antes de o
  dar como fechado — se `a51e02804` divergir de 0.15.0 nesse ponto específico, a medição
  muda.

## Fase C — Corrigir a documentação do projecto

- `00_nucleo/prompts/auditar-fatiamento.md` (ou onde fizer mais sentido, confirmar):
  adicionar nota permanente — **"O alvo de paridade é o vanilla ratificado
  (upstream/main, hash pinado — confirmar o actual em `lab/typst-original`), não a tag
  0.15.0 nem o binário de sistema. `/usr/local/bin/typst` pode estar noutra versão
  (0.15.1 confirmado nalgum ponto) — nunca usar sem confirmar contra o hash pinado
  primeiro."**
- Corrigir qualquer prompt/relatório activo (não histórico — relatórios de passos
  antigos ficam como registo do que foi medido na altura) que declare "0.15.0" como alvo
  actual.

## Fase D — Validar

```
crystalline-lint .
cargo test --workspace
```
Zero regressão — é correcção de declaração/documentação, só a Fase B (hipóteses 2/3)
poderia gerar mudança de medição real, reportar se acontecer.

---

## Resultado esperado

Estado de `lab/typst-original` confirmado e, se necessário, corrigido para o hash
ratificado. Declaração de alvo de paridade consistente em todo o projecto. Se a
Hipótese 2/3 se confirmar, a medição do Passo 1024 (margem de 10%) é refeita antes de
alimentar o Passo 1026.
