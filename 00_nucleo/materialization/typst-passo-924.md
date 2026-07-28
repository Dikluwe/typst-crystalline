# Passo 924 — flake isolado em `typst-infra` (742/1 failed numa execução, registado em P920)

**Precede este passo**: `typst-passo-920-relatorio.md`, "Achados em aberto" — uma execução de
`cargo test --workspace` mostrou `typst-infra: 742 passed, 1 failed`; três execuções seguintes
deram `743/0 failed`. Suspeita registada: testes sensíveis a descoberta de fontes do sistema.

**Prioridade baixa — investigação leve, não abrir escopo maior que o necessário.**

**Pré-condição de árvore**: `git status`.

---

## Fase A

1. Rodar `cargo test -p typst-infra --workspace -- --test-threads=1` várias vezes (10+) e também
   com paralelismo default, para tentar reproduzir a falha isolada — se depender de ordem/paralelismo
   de execução, isso já aponta para estado partilhado entre testes (fontes do sistema, ficheiros
   temporários, variável de ambiente), não para um bug de lógica.
2. Se reproduzir: identificar qual teste falhou e por quê (mensagem de erro completa, não só
   "falhou"). Confirmar a suspeita registada (descoberta de fontes do sistema) ou encontrar outra
   causa.
3. Se não reproduzir depois de tentativa razoável (10-20 execuções): registar como não
   reproduzido, não fechar como "não existe" — deixar nota para se voltar a aparecer, mesma
   disciplina de não inventar conclusão sem evidência.

## Fase B — só se reproduzido e a causa for confirmada

Corrigir (provavelmente: isolar o teste de estado de sistema real, usar fixture pinada em vez de
depender de fontes instaladas — mesmo padrão já corrigido para `pdf_tounicode_...` em P916).

## Resultado esperado

- Reproduzido ou não, com número de tentativas registado.
- Se reproduzido: causa confirmada e corrigida, ou registada com mais precisão para investigação
  futura se a causa não for óbvia.
- Se não reproduzido: registado como tal, sem fechar a possibilidade de reaparecer.
