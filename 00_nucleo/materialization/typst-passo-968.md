# Passo 968 — 74% do texto do documento usa `Tr 2` (preenchimento+contorno) em vez de `Tr 0` como o vanilla

**Precede este passo**: achado novo, auditoria externa (2026-08-05, seção 8.3) — vanilla usa `Tr 0`
em 100% dos blocos de texto; cristalino usa `Tr 2` (preenchimento + contorno fino, `w` de 0.073pt
ou 0.051pt) em **1541 de 2074 blocos (74%)**, só 26% em `Tr 0` como o vanilla. `Tr 2` engrossa
visualmente o traço do glifo sem trocar de fonte — explica a impressão relatada de "texto meio
negrito".

**Conexão com achado já catalogado**: P956 (Fase C) já tinha registado "`2 Tr + w` de faux-bold"
como diferença residual conhecida, atribuída a "o vanilla usa fonte bold real — frente tipográfica
pré-existente", tratada como fora de escopo. **Mas 74% do documento é muito mais que a proporção
esperada de texto genuinamente em negrito** — o documento de teste não tem 74% do conteúdo em
negrito. Isto sugere que o mecanismo de "faux-bold" pode estar disparando para texto que não devia
receber contorno nenhum, não só para os casos legítimos de negrito sem fonte bold disponível.

**Pré-condição de árvore**: `git status`. Confirmar P967 (se já executado) presente.

---

## Fase A — confirmar se é sobre-disparo do faux-bold ou outra causa

1. Confirmar quanto do documento de 30 secções é conteúdo genuinamente marcado como negrito
   (`*texto*`/`#strong[...]`) — se for muito menos que 74% dos blocos de texto, o mecanismo de
   faux-bold está a disparar além do esperado.
2. Ler o código que decide quando emitir `Tr 2`+`w` (faux-bold) — confirmar a condição exata que
   ativa isso. Candidatos: heurística de peso de fonte mal calibrada, condição que deveria
   verificar "está em negrito E não há fonte bold real disponível" mas só verifica uma das duas
   partes, ou um valor por defeito errado que trata a maioria do texto como precisando de
   simulação de negrito.
3. Confirmar se os 26% que ficam em `Tr 0` têm algo em comum (por exemplo, são os que já usam uma
   fonte com peso bold real disponível) — isso ajudaria a confirmar a causa exata.
4. Confirmar o mecanismo real do vanilla — por que ele nunca precisa de `Tr 2`/faux-bold (sempre
   tem a fonte bold real disponível, ou trata negrito de forma diferente).

## Fase B — Implementação (TDD directo se for correção de condição pontual; protocolo de dois
agentes se envolver mudança mais ampla na seleção de fonte bold)

1. Teste confirmando que texto **não-negrito** nunca recebe `Tr 2`; texto genuinamente negrito
   recebe `Tr 2`+contorno só quando não há fonte bold real disponível (guarda: se houver fonte
   bold real, deve preferir carregá-la, não simular).
2. Implementar a correção da condição.
3. Suíte completa verde, discriminada por crate.
4. Confirmação visual — documento de 30 secções, contagem de blocos `Tr 2` antes/depois, deveria
   cair para perto da proporção real de conteúdo negrito.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Recontar blocos `Tr 0`/`Tr 2` no documento de 30 secções — confirmar que a proporção bate com o
   conteúdo genuinamente negrito, não 74%.
2. Confirmação visual lado a lado com o vanilla (peso de traço, não só presença/ausência de
   negrito).
3. Benchmark completo, 7 cenários, `depois/antes`, zero regressão.

## Resultado esperado

- Causa exacta do sobre-disparo confirmada (condição de código, não presumida).
- Proporção de `Tr 2` no documento reduzida para bater com o conteúdo genuinamente negrito.
- Se ainda sobrar uso legítimo de faux-bold (fonte bold real de facto ausente): mantido, registado
  como decisão consciente, não removido às cegas.
- Benchmark sem regressão.
