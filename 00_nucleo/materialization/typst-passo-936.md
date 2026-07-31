# Passo 936 — reverter P935 (Fase B) e estudar o pipeline completo do vanilla para entender a uniformidade de tempo

**Precede este passo**: `typst-passo-935-relatorio.md` — Fase A (diagnóstico) correta e valiosa;
Fase B implementou "coverage eager" sem a mudança de I/O que a própria Fase A identificou como
necessária (mmap), regrediu `utf8-latin` (88ms→147.6ms) sem medir contra os 7 cenários canônicos,
e piorou `utf8-emoji` (25.34×→31.30× mais lento que o vanilla).

---

## Parte A — reverter

1. Reverter todo o código de `03_infra/src/fonts.rs`, `03_infra/src/fontdb.rs`,
   `03_infra/src/world.rs`, `04_wiring/src/main.rs` e `03_infra/Cargo.toml` para o estado
   P933-fixed (correção do shaper, `face_covers_char` reintroduzido; scan condicional de P927
   intacto; sem coverage eager).
2. Reverter os 4 L0s que P935 tinha actualizado (`fonts.md`, `fontdb.md`, `system-world.md`,
   `wiring.md`) para o estado anterior a P935 — confirmar com `crystalline-lint --fix-hashes .`
   que os hashes voltam a bater com o código revertido.
3. Confirmar o estado revertido: `cargo test --workspace` (números por crate, não só "passou"),
   `crystalline-lint .` (0 drift), e um benchmark rápido dos 7 cenários canônicos contra o estado
   P933-fixed já conhecido — confirmar que voltámos exactamente ao mesmo lugar, não a um estado
   parecido mas diferente.
4. Manter o relatório de P935 como está (a Fase A não é revertida, só o código da Fase B) — é
   diagnóstico correto, só a implementação que se seguiu estava errada.

---

## Parte B — estudo holístico do pipeline completo do vanilla

**Objetivo**: não instrumentar uma função isolada (`fontdb::load_system_fonts`, já feito em P935),
mas traçar o caminho **inteiro**, do início ao fim, para um documento CJK e um documento emoji no
vanilla real, e entender por que o tempo total fica sempre ~270-300ms **independente do tipo de
caractere** — enquanto o cristalino varia de 88ms a 8771ms conforme o caractere. Essa uniformidade
é a pista mais importante e ainda não explicada.

### Hipótese a confirmar (não presumir — é uma hipótese, construída a partir de achados anteriores, não um facto)

P934 encontrou que o filtro de candidatos do cristalino (`candidates_for_char`, bitmap por bloco
de 256 codepoints) tem falsos positivos reais (866 candidatos para grego, a maioria sem o glifo
exacto) — por isso P932/933 precisaram de verificar cada candidato sobrevivente abrindo a face de
verdade (`face_covers_char`), o que é caro e proporcional ao número de candidatos que sobrevivem
ao filtro grosseiro.

P935 encontrou que `FontInfo::new` do vanilla extrai coverage **exacta** (itera a cmap de verdade,
por glifo, não por bloco de 256) para cada fonte, eager, no arranque — barato porque usa mmap.

**A hipótese**: se a coverage do vanilla já é exacta (não aproximada), o `FontBook::select_
fallback` do vanilla nunca precisa de abrir uma face extra para confirmar um candidato — a
resposta já está certa na primeira consulta. O cristalino, por ter uma coverage aproximada (mais
barata de construir, mas imprecisa), é forçado a pagar uma verificação extra por candidato durante
o shaping — e é **esse** custo repetido, não só o custo de arranque, que faz o tempo do cristalino
variar tanto conforme o número de falsos positivos de cada script (CJK tem poucos, 32; emoji tem
muitos, 238; grego tem muitíssimos, 866).

Se isto for confirmado: a causa raiz não é só "usar mmap" (P935 já mostrou isso ajudar pouco
sozinho) — é a **combinação** de I/O barato (mmap) permitindo coverage exacta barata (em vez de
aproximada), o que por sua vez elimina a necessidade de verificação repetida no caminho quente do
shaping. As duas peças são interdependentes; portar só uma não resolve.

### Fase B.1 — traçar o caminho completo no vanilla real

1. Instrumentar (ou usar `perf`/`strace` com granularidade suficiente) o binário vanilla real
   compilando um documento CJK e um documento emoji, separadamente. Para cada um, medir e listar,
   em ordem: tempo de descoberta de fontes, tempo de extração de `FontInfo`/coverage por fonte,
   tempo gasto em `select_fallback` durante o shaping, número de faces abertas **depois** do
   arranque (durante o shaping em si, não na descoberta).
2. Confirmar directamente no código-fonte do vanilla (`typst-library/src/text/font/book.rs`,
   `select_fallback`) se a verificação de cobertura durante o fallback é só um lookup em bitmap/
   estrutura já pronta (`info.coverage.contains(c)`, sem abrir face nenhuma) — confirmar que
   nenhuma face é aberta nesse ponto, só na escolha final para shaping de facto.
3. Repetir a mesma instrumentação para o cristalino (estado P933-fixed, revertido na Parte A) nos
   mesmos dois documentos — confirmar quantas faces são abertas **depois** do arranque, durante o
   shaping, e comparar directamente com o número do vanilla.
4. Se a hipótese acima for confirmada (vanilla: 0 ou quase 0 aberturas de face durante o shaping
   fallback, por ter coverage exacta; cristalino: dezenas a centenas, por ter coverage aproximada
   e precisar verificar): isto explica a uniformidade do vanilla e a variação do cristalino com um
   único mecanismo, não dois separados.
5. Se a hipótese não for confirmada: registar o que de facto se observou, sem forçar a hipótese —
   e procurar a explicação real da uniformidade a partir do que a instrumentação mostrar.

### Fase B.2 — não implementar neste passo

Este passo é só de estudo — a Parte A já reverteu o código, e a Parte B é para entender, não para
corrigir. Registar a conclusão (hipótese confirmada, refutada, ou parcialmente confirmada) e deixar
o desenho da correção para um passo seguinte, agora com uma base de entendimento mais sólida do que
P935 tinha.

---

## Resultado esperado

- Estado do código de volta a P933-fixed, confirmado por testes e benchmark, não só por "reverti".
- Traçado completo do pipeline do vanilla para CJK e emoji, com números por etapa.
- Mesmo traçado para o cristalino, nos mesmos dois documentos.
- Veredicto sobre a hipótese "coverage exacta elimina verificação repetida no shaping" — confirmada,
  refutada, ou parcial, com números, não suposição.
- Nenhuma implementação neste passo — só entendimento, para informar o desenho do próximo.
