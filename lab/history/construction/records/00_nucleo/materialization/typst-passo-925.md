# Passo 925 — diagnóstico: fallback lazy de fontes do sistema é lento para CJK/emoji (achado de P923)

**Precede este passo**: `typst-passo-923-relatorio.md`, Fase E.2 — outlier `05-utf8` (25.91×
cristalino/vanilla), isolado até `SystemWorld::candidates_for_char` (`03_infra/src/world.rs:517`):
o cristalino faz parsing sob demanda das fontes do sistema no primeiro carácter não coberto pela
fonte primária (custo medido: ~4.5s para um único carácter CJK, ~9.4s para um único emoji); o
vanilla usa `fontdb` com coverage pré-computada.

**Este é o início de uma frente nova, não continuação da frente de geometria matemática
(P885-924), que está fechada.** Segue o mesmo padrão de abertura que a frente anterior usou em
P872/873: diagnosticar e desenhar opções antes de implementar.

**Pré-condição de árvore**: `git status`. Confirmar P922-924 commitados.

---

## Fase A — medir o mecanismo actual e o do vanilla, não presumir

1. Ler `SystemWorld::candidates_for_char` (`world.rs:517`) linha a linha — confirmar exactamente
   o que acontece no primeiro carácter não coberto: quantas fontes são abertas, se o parsing é de
   ficheiro inteiro ou só cabeçalho/tabela `cmap`, e se o resultado fica cacheado para os
   caracteres seguintes do mesmo bloco Unicode ou só para o carácter exacto repetido.
2. Confirmar quantas fontes o sistema de testes tem disponíveis para CJK/emoji (`fc-list` ou
   equivalente) — o custo medido (~4.5-9.4s) pode ser proporcional ao número de fontes instaladas
   na máquina, o que mudaria a gravidade percebida do problema conforme o ambiente.
3. Ler como `fontdb` (crate já em uso, per `ADR-0055` e outras) constrói a sua coverage
   pré-computada — é feito no arranque (custo pago uma vez, adiantado) ou é lazy também, só que
   mais eficiente por alguma razão estrutural? Confirmar antes de assumir que "pré-computar" é
   automaticamente a resposta certa.
4. Confirmar como o vanilla real usa `fontdb` neste caso específico (`lab/typst-original/`) —
   mesmo padrão de todo o resto do projecto, ler o código-fonte, não inferir do nome da crate.

## Fase A.1 — desenhar opções (sem implementar ainda)

Candidatos, com custo/risco de cada, a confirmar/expandir na Fase A:

1. **Pré-computar coverage no arranque** (mesmo que o vanilla, se confirmado ser isso que ele faz)
   — paga o custo uma vez, adiantado, não no meio da compilação; risco: pode alongar o tempo de
   arranque para documentos que nunca precisam de fallback CJK/emoji.
2. **Cache mais amplo do lado do cristalino** (por bloco Unicode, não só por carácter exacto) —
   menos invasivo, mas só ajuda documentos com múltiplos caracteres do mesmo bloco (caso comum,
   mas não todos).
3. **Parsing parcial** (só tabela `cmap`, não a fonte inteira) no momento da descoberta — reduz o
   custo por fonte aberta, sem mudar a estratégia lazy vs. eager.

Registar qual(is) opção(ões) fazem sentido, com o dono, antes de implementar — pode ser mais que
uma combinada.

## Fase B — Implementação (TDD; protocolo de dois agentes se a solução escolhida envolver mudança
estrutural no momento de inicialização do `World`)

1. Teste com medição real (mesmo padrão de P890 — `/usr/bin/time -v`/contagem de aberturas de
   ficheiro, não só resultado final) confirmando o custo antes/depois.
2. Implementar a opção escolhida.
3. Suíte completa verde, discriminada por crate.
4. Recompilar os 4 casos isolados de P923 (`utf8-latin`, `utf8-greek`, `utf8-cjk`, `utf8-emoji`) e
   confirmar melhoria, com números — mesma disciplina de `L11`.
5. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários canônicos, `--warmup 5 -m 20`, razão `depois/antes` (não
`cristalino/vanilla` — lição do achado de disciplina em P922/923). Attestation completa.

## Resultado esperado

- Mecanismo do cristalino e do vanilla confirmados por leitura, não suposição.
- Opções de correcção desenhadas com custo/risco, decisão registada com o dono.
- Números antes/depois para os 4 casos de bloco Unicode de P923.
- Benchmark canônico completo, atestado.
