# Passo 950 — `BaseFont`/`FontName` genérico (`CrystallineFontN`) em vez do nome real da fonte

**Precede este passo**: achado do dono durante a investigação de P949 — a fonte embutida no PDF do
cristalino aparece como `CrystallineFont1`-`CrystallineFont4` (genérico, sem referência à família
real), enquanto o vanilla embute com nomes reais (`NewCM10-Bold`, `NewCMMath-Book`,
`LibertinusSerif-Regular`, `NewCM10-Regular`). Isso já causou confusão real nesta investigação
(interpretação inicial de "fonte diferente" quando na verdade é subsetting normal com nome
genérico) e é um defeito de interoperabilidade, não só estético.

**Pré-condição de árvore**: `git status`. Confirmar P949 (se já executado) presente.

---

## Fase A — confirmar exatamente quais nomes estão genéricos, e por quê

1. Ler o código de export de PDF (`03_infra/src/export/`) e confirmar, separadamente:
   - O **nome do recurso** (`/CrystallineFont1` referenciado no content stream, dentro do
     dicionário `/Resources << /Font ... >>`) — isto é interno, não precisa de significar nada
     para o PDF funcionar, mas confirmar se é isto que o dono está a ver ou se é o próximo item.
   - O **`BaseFont`** do objeto `/Font` e o **`FontName`** dentro do `/FontDescriptor` — estes
     **devem** reflectir o nome real da fonte, por convenção PDF e por interoperabilidade (leitura
     por outras ferramentas, substituição de fonte se a extração falhar).
2. Confirmar se o cristalino tem acesso à informação de nome real da fonte no ponto onde estes
   campos são escritos — a família/nome deveria já estar disponível (é a mesma informação usada
   para seleccionar a fonte via `FontBook`/`covering()`).
3. Confirmar a convenção exacta usada pelo vanilla/padrão PDF — prefixo de 6 letras maiúsculas
   aleatórias + `+` + nome PostScript real da fonte (`ABCDEF+NewCMMath-Book`), per a especificação
   PDF (secção sobre subsetting) — confirmar se o vanilla usa exactamente isto ou uma variação.

## Fase B — Implementação (TDD directo — mapeamento de nome, sem geometria nova)

1. Teste confirmando que `BaseFont`/`FontName` no PDF exportado contém o nome real da fonte
   (verificado por leitura do PDF gerado, não só do código) — para pelo menos duas fontes
   diferentes usadas no mesmo documento (confirma que não é hardcoded para uma só).
2. Implementar: usar o nome real da fonte (já disponível via `FontInfo`/`FontBook`) na construção
   do `BaseFont`/`FontName`, com o prefixo de subsetting gerado (aleatório ou determinístico —
   decidir e documentar qual, confirmando se isso importa para reprodutibilidade de build, já
   que este projeto se importa com isso noutros contextos).
3. **Manter o nome do recurso interno (`/F1`, `/CrystallineFontN` ou equivalente) como está**, a
   menos que a Fase A confirme que também precisa mudar — não é o mesmo problema.
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Recompilar um documento com múltiplas fontes e confirmar, por leitura directa do PDF
   (`pikepdf`/`fontTools`, mesmo método usado nesta investigação inteira), que os nomes agora
   reflectem as fontes reais.
2. Confirmar que isto não quebra nada que dependa do nome anterior (grep por `CrystallineFont` no
   código e nos testes, para não deixar uma asserção presa ao nome antigo sem querer).
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão (mudança de nome não
   deveria ter custo de performance, mas confirmar).

## Resultado esperado

- `BaseFont`/`FontName` no PDF exportado reflectindo o nome real da fonte usada, não um
  identificador genérico.
- Nome do recurso interno inalterado, a menos que a Fase A justifique mudar também.
- Investigações futuras (deste tipo) deixam de correr o risco de interpretar subsetting normal
  como fonte diferente, só por olhar o nome.
